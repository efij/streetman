use crate::security::classify_sensitive;
use anyhow::Context;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use zeroize::Zeroizing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveRecord {
    pub hash: String,
    pub created_at: DateTime<Utc>,
    pub original_tokens_estimate: usize,
    pub compressed_tokens_estimate: usize,
    pub note: String,
}

pub struct Archive {
    root: PathBuf,
    conn: Connection,
    cipher: ChaCha20Poly1305,
}

impl Archive {
    pub fn open_default() -> anyhow::Result<Self> {
        let root = dirs::home_dir()
            .context("home directory not available")?
            .join(".streetman");
        Self::open(root)
    }

    pub fn open(root: impl AsRef<Path>) -> anyhow::Result<Self> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(root.join("archive"))?;
        let conn = Connection::open(root.join("streetman.sqlite3"))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS archive (
                hash TEXT PRIMARY KEY,
                created_at TEXT NOT NULL,
                original_tokens INTEGER NOT NULL,
                compressed_tokens INTEGER NOT NULL,
                note TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                created_at TEXT NOT NULL,
                kind TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                prev_hash TEXT NOT NULL DEFAULT '',
                event_hash TEXT NOT NULL DEFAULT ''
            );",
        )?;
        ensure_event_hash_columns(&conn)?;
        let key = derive_local_key(&root);
        let cipher = ChaCha20Poly1305::new((&key).into());
        Ok(Self { root, conn, cipher })
    }

    pub fn store(
        &self,
        original: &str,
        compressed: &str,
        note: impl Into<String>,
    ) -> anyhow::Result<ArchiveRecord> {
        let sensitive = classify_sensitive(original);
        if !sensitive.is_empty() {
            let kinds = sensitive
                .iter()
                .map(|finding| finding.kind.as_str())
                .collect::<Vec<_>>()
                .join(",");
            anyhow::bail!("archive rejected sensitive original before persistence: {kinds}");
        }
        let hash = blake3::hash(original.as_bytes()).to_hex().to_string();
        let nonce_bytes = nonce_for_hash(&hash);
        let original_bytes = Zeroizing::new(original.as_bytes().to_vec());
        let encrypted = self
            .cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), original_bytes.as_slice())
            .map_err(|err| anyhow::anyhow!("archive encryption failed: {err}"))?;
        fs::write(
            self.root.join("archive").join(format!("{hash}.bin")),
            encrypted,
        )?;
        let note = note.into();
        let record = ArchiveRecord {
            hash: hash.clone(),
            created_at: Utc::now(),
            original_tokens_estimate: original.split_whitespace().count(),
            compressed_tokens_estimate: compressed.split_whitespace().count(),
            note,
        };
        self.conn.execute(
            "INSERT OR REPLACE INTO archive
             (hash, created_at, original_tokens, compressed_tokens, note)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                record.hash,
                record.created_at.to_rfc3339(),
                record.original_tokens_estimate as i64,
                record.compressed_tokens_estimate as i64,
                record.note
            ],
        )?;
        Ok(record)
    }

    pub fn retrieve(&self, hash: &str, query: Option<&str>) -> anyhow::Result<String> {
        let encrypted = fs::read(self.root.join("archive").join(format!("{hash}.bin")))
            .with_context(|| format!("archive record not found: {hash}"))?;
        let nonce_bytes = nonce_for_hash(hash);
        let bytes = self
            .cipher
            .decrypt(Nonce::from_slice(&nonce_bytes), encrypted.as_ref())
            .map_err(|err| anyhow::anyhow!("archive decryption failed: {err}"))?;
        let text = String::from_utf8(bytes)?;
        if let Some(query) = query {
            Ok(search_lines(&text, query))
        } else {
            Ok(text)
        }
    }

    pub fn list_records(&self, limit: usize) -> anyhow::Result<Vec<ArchiveRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT hash, created_at, original_tokens, compressed_tokens, note
             FROM archive ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            let created_at: String = row.get(1)?;
            Ok(ArchiveRecord {
                hash: row.get(0)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                original_tokens_estimate: row.get::<_, i64>(2)? as usize,
                compressed_tokens_estimate: row.get::<_, i64>(3)? as usize,
                note: row.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn totals(&self) -> anyhow::Result<(usize, usize, usize)> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*), COALESCE(SUM(original_tokens), 0), COALESCE(SUM(compressed_tokens), 0)
             FROM archive",
        )?;
        let (count, original, compressed): (i64, i64, i64) =
            stmt.query_row([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
        Ok((count as usize, original as usize, compressed as usize))
    }

    pub fn log_event<T: Serialize>(&self, kind: &str, payload: &T) -> anyhow::Result<()> {
        let payload_json = serde_json::to_string(payload)?;
        let prev_hash: String = self
            .conn
            .query_row(
                "SELECT event_hash FROM events ORDER BY id DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or_default();
        let created_at = Utc::now().to_rfc3339();
        let event_hash =
            blake3::hash(format!("{created_at}:{kind}:{prev_hash}:{payload_json}").as_bytes())
                .to_hex()
                .to_string();
        self.conn.execute(
            "INSERT INTO events (created_at, kind, payload_json, prev_hash, event_hash)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![created_at, kind, payload_json, prev_hash, event_hash],
        )?;
        Ok(())
    }
}

fn ensure_event_hash_columns(conn: &Connection) -> anyhow::Result<()> {
    let columns = conn
        .prepare("PRAGMA table_info(events)")?
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<Result<Vec<_>, _>>()?;
    if !columns.iter().any(|column| column == "prev_hash") {
        conn.execute(
            "ALTER TABLE events ADD COLUMN prev_hash TEXT NOT NULL DEFAULT ''",
            [],
        )?;
    }
    if !columns.iter().any(|column| column == "event_hash") {
        conn.execute(
            "ALTER TABLE events ADD COLUMN event_hash TEXT NOT NULL DEFAULT ''",
            [],
        )?;
    }
    Ok(())
}

fn derive_local_key(root: &Path) -> [u8; 32] {
    if let Ok(value) = std::env::var("STREETMAN_ARCHIVE_KEY") {
        if !value.trim().is_empty() {
            return blake3::derive_key("streetman archive byok v1", value.as_bytes());
        }
    }
    let material = format!(
        "streetman-local-archive:{}:{}",
        std::env::var("USER").unwrap_or_default(),
        root.display()
    );
    blake3::derive_key("streetman archive v1", material.as_bytes())
}

fn nonce_for_hash(hash: &str) -> [u8; 12] {
    let decoded = hex::decode(hash).unwrap_or_else(|_| hash.as_bytes().to_vec());
    let digest = blake3::hash(&decoded);
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&digest.as_bytes()[..12]);
    nonce
}

fn search_lines(text: &str, query: &str) -> String {
    let terms: Vec<_> = query
        .to_ascii_lowercase()
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let mut scored = text
        .lines()
        .map(|line| {
            let lower = line.to_ascii_lowercase();
            let score = terms
                .iter()
                .filter(|term| lower.contains(term.as_str()))
                .count();
            (score, line)
        })
        .filter(|(score, _)| *score > 0)
        .collect::<Vec<_>>();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored
        .into_iter()
        .take(25)
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn retrieval_marker(hash: &str) -> String {
    format!("[streetman original archived: retrieve {hash}]")
}

pub fn encode_hash_for_display(hash: &str) -> String {
    STANDARD.encode(hash.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_and_retrieves_exact_original() {
        let dir = tempfile::tempdir().unwrap();
        let archive = Archive::open(dir.path()).unwrap();
        let record = archive.store("alpha\nfatal beta", "alpha", "test").unwrap();
        assert_eq!(
            archive.retrieve(&record.hash, None).unwrap(),
            "alpha\nfatal beta"
        );
        assert_eq!(
            archive.retrieve(&record.hash, Some("fatal")).unwrap(),
            "fatal beta"
        );
    }

    #[test]
    fn rejects_sensitive_archives_and_logs_hash_chain() {
        let dir = tempfile::tempdir().unwrap();
        let archive = Archive::open(dir.path()).unwrap();
        let err = archive
            .store("OPENAI_API_KEY=sk-testsecret123", "redacted", "test")
            .unwrap_err();
        assert!(err.to_string().contains("archive rejected sensitive"));
        archive
            .log_event("test", &serde_json::json!({"ok": true}))
            .unwrap();
        archive
            .log_event("test", &serde_json::json!({"ok": false}))
            .unwrap();
        let count: i64 = archive
            .conn
            .query_row(
                "SELECT COUNT(*) FROM events WHERE event_hash != ''",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 2);
    }
}

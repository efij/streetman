//! Host wiring for the one-liner install: pure functions that produce the
//! per-turn enforcement instruction text and the host config files. No
//! filesystem access lives here so every transform is unit-testable. The CLI
//! `streetman init` command reads/writes files and calls into these.

use serde_json::{json, Map, Value};

/// Marker appended to every streetman-managed hook command. It is a shell
/// comment (ignored at runtime) and lets us find/replace/remove exactly the
/// hooks we added without disturbing the user's own hooks.
pub const HOOK_SENTINEL: &str = "# streetman-managed";

/// Sentinels bracketing the streetman block in an AGENTS.md / instruction file.
pub const AGENTS_START: &str = "<!-- streetman:start -->";
pub const AGENTS_END: &str = "<!-- streetman:end -->";

/// The per-turn instruction text injected into the model context (Layer A).
/// Deterministic; `mode` is one of off/lite/full/ultra, anything else => full.
pub fn compression_instructions(mode: &str, host: &str) -> String {
    let mode = normalize_mode(mode);
    if mode == "off" {
        return "STREETMAN COMPRESSION OFF".to_string();
    }
    let mode_rule = match mode {
        "lite" => "Trim filler and hedging. Keep full sentences where clarity needs them.",
        "ultra" => "Maximum brevity. Fragments and symbols welcome. Drop every non-essential word.",
        _ => "Drop articles, filler, and pleasantries. Short synonyms. Fragments OK.",
    };
    format!(
        r#"STREETMAN COMPRESSION ACTIVE - level: {mode}
Host: {host}

Compress your prose output to save tokens while keeping every fact intact.

Rules:
- {mode_rule}
- NEVER alter code, commands, file paths, URLs, identifiers, version numbers, or security terms — reproduce them verbatim.
- Preserve all technical meaning; compression is lossless on facts, lossy only on filler.
- Pattern: [thing] [action] [reason]. [next step].
- Code blocks, commit messages, and security-sensitive text: write normal.

Say "stop streetman" or "normal mode" to turn this off for the session."#
    )
}

fn normalize_mode(mode: &str) -> &str {
    match mode.to_ascii_lowercase().as_str() {
        "off" => "off",
        "lite" => "lite",
        "ultra" => "ultra",
        _ => "full",
    }
}

fn streetman_hook_group(bin: &str, mode: &str) -> Value {
    let command = format!("{bin} instructions --mode {mode} --host claude {HOOK_SENTINEL}");
    json!({ "hooks": [ { "type": "command", "command": command } ] })
}

fn is_streetman_group(group: &Value) -> bool {
    group
        .get("hooks")
        .and_then(Value::as_array)
        .map(|hooks| {
            hooks.iter().any(|h| {
                h.get("command")
                    .and_then(Value::as_str)
                    .map(|c| c.contains(HOOK_SENTINEL))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

/// Idempotently merge streetman's SessionStart + UserPromptSubmit instruction
/// hooks into an existing Claude `settings.json`. Re-running replaces our own
/// hooks (picking up a new mode/bin) and never touches the user's other hooks.
pub fn merge_claude_settings_hooks(
    existing: Option<&str>,
    bin: &str,
    mode: &str,
) -> Result<String, serde_json::Error> {
    let mode = normalize_mode(mode);
    let mut root: Value = match existing {
        Some(s) if !s.trim().is_empty() => serde_json::from_str(s)?,
        _ => Value::Object(Map::new()),
    };
    if !root.is_object() {
        root = Value::Object(Map::new());
    }
    let obj = root.as_object_mut().unwrap();
    let hooks = obj
        .entry("hooks")
        .or_insert_with(|| Value::Object(Map::new()));
    if !hooks.is_object() {
        *hooks = Value::Object(Map::new());
    }
    let hooks = hooks.as_object_mut().unwrap();

    for event in ["SessionStart", "UserPromptSubmit"] {
        let arr = hooks
            .entry(event)
            .or_insert_with(|| Value::Array(Vec::new()));
        let groups = match arr.as_array_mut() {
            Some(a) => a,
            None => {
                *arr = Value::Array(Vec::new());
                arr.as_array_mut().unwrap()
            }
        };
        groups.retain(|g| !is_streetman_group(g));
        groups.push(streetman_hook_group(bin, mode));
    }
    serde_json::to_string_pretty(&root)
}

/// Remove only streetman-managed hooks from a Claude `settings.json`, leaving
/// the user's own hooks and any other settings untouched.
pub fn strip_claude_settings_hooks(existing: &str) -> Result<String, serde_json::Error> {
    let mut root: Value = if existing.trim().is_empty() {
        Value::Object(Map::new())
    } else {
        serde_json::from_str(existing)?
    };
    if let Some(hooks) = root
        .get_mut("hooks")
        .and_then(Value::as_object_mut)
    {
        for event in ["SessionStart", "UserPromptSubmit", "PreToolUse"] {
            if let Some(arr) = hooks.get_mut(event).and_then(Value::as_array_mut) {
                arr.retain(|g| !is_streetman_group(g));
            }
        }
        hooks.retain(|_, v| v.as_array().map(|a| !a.is_empty()).unwrap_or(true));
    }
    serde_json::to_string_pretty(&root)
}

fn agents_block(mode: &str, host: &str) -> String {
    format!(
        "{AGENTS_START}\n{}\n{AGENTS_END}",
        compression_instructions(mode, host)
    )
}

/// Insert or replace the streetman block in an AGENTS.md-style instruction file
/// (Codex and other AGENTS.md-reading hosts). Idempotent.
pub fn upsert_agents_block(existing: Option<&str>, mode: &str, host: &str) -> String {
    let block = agents_block(mode, host);
    match existing {
        Some(s) if s.contains(AGENTS_START) && s.contains(AGENTS_END) => {
            let start = s.find(AGENTS_START).unwrap();
            let end = s.find(AGENTS_END).unwrap() + AGENTS_END.len();
            let mut out = String::with_capacity(s.len());
            out.push_str(&s[..start]);
            out.push_str(&block);
            out.push_str(&s[end..]);
            out
        }
        Some(s) if s.trim().is_empty() => format!("{block}\n"),
        Some(s) => {
            let sep = if s.ends_with('\n') { "\n" } else { "\n\n" };
            format!("{s}{sep}{block}\n")
        }
        None => format!("{block}\n"),
    }
}

/// Remove the streetman block (and its surrounding blank lines) from an
/// AGENTS.md-style file. Returns the file unchanged if no block is present.
pub fn strip_agents_block(existing: &str) -> String {
    let (Some(start), Some(end_idx)) = (existing.find(AGENTS_START), existing.find(AGENTS_END))
    else {
        return existing.to_string();
    };
    let end = end_idx + AGENTS_END.len();
    let mut out = String::with_capacity(existing.len());
    out.push_str(existing[..start].trim_end());
    let tail = existing[end..].trim_start_matches('\n');
    if !out.is_empty() && !tail.is_empty() {
        out.push('\n');
    }
    out.push_str(tail);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_off_is_explicit() {
        assert_eq!(compression_instructions("off", "claude"), "STREETMAN COMPRESSION OFF");
    }

    #[test]
    fn instructions_default_to_full_and_protect_code() {
        let txt = compression_instructions("bogus", "claude");
        assert!(txt.contains("level: full"));
        assert!(txt.contains("reproduce them verbatim"));
    }

    #[test]
    fn merge_is_idempotent() {
        let once = merge_claude_settings_hooks(None, "/x/streetman", "full").unwrap();
        let twice = merge_claude_settings_hooks(Some(&once), "/x/streetman", "full").unwrap();
        let v: Value = serde_json::from_str(&twice).unwrap();
        for event in ["SessionStart", "UserPromptSubmit"] {
            let groups = v["hooks"][event].as_array().unwrap();
            assert_eq!(groups.len(), 1, "{event} must not duplicate on re-run");
        }
    }

    #[test]
    fn merge_preserves_user_hooks_and_settings() {
        let existing = r#"{"model":"opus","hooks":{"SessionStart":[{"hooks":[{"type":"command","command":"echo mine"}]}]}}"#;
        let merged = merge_claude_settings_hooks(Some(existing), "streetman", "full").unwrap();
        let v: Value = serde_json::from_str(&merged).unwrap();
        assert_eq!(v["model"], "opus");
        let groups = v["hooks"]["SessionStart"].as_array().unwrap();
        assert_eq!(groups.len(), 2, "user hook + streetman hook");
        assert!(merged.contains("echo mine"));
    }

    #[test]
    fn strip_restores_user_only() {
        let existing = r#"{"model":"opus","hooks":{"SessionStart":[{"hooks":[{"type":"command","command":"echo mine"}]}]}}"#;
        let merged = merge_claude_settings_hooks(Some(existing), "streetman", "full").unwrap();
        let stripped = strip_claude_settings_hooks(&merged).unwrap();
        let v: Value = serde_json::from_str(&stripped).unwrap();
        assert_eq!(v["model"], "opus");
        assert!(stripped.contains("echo mine"));
        assert!(!stripped.contains(HOOK_SENTINEL));
        let groups = v["hooks"]["SessionStart"].as_array().unwrap();
        assert_eq!(groups.len(), 1);
    }

    #[test]
    fn agents_block_round_trips() {
        let base = "# My project\n\nSome notes.\n";
        let with = upsert_agents_block(Some(base), "full", "codex");
        assert!(with.contains(AGENTS_START));
        assert!(with.contains("STREETMAN COMPRESSION ACTIVE"));
        // re-running replaces, never stacks
        let again = upsert_agents_block(Some(&with), "ultra", "codex");
        assert_eq!(again.matches(AGENTS_START).count(), 1);
        assert!(again.contains("level: ultra"));
        let without = strip_agents_block(&again);
        assert!(!without.contains(AGENTS_START));
        assert!(without.contains("# My project"));
    }
}

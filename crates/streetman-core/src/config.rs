use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StreetmanConfig {
    pub default_mode: String,
    pub default_domain: String,
    pub telemetry: bool,
    pub archive_ttl_days: u32,
    pub dashboard_port: u16,
    pub protected_patterns: Vec<String>,
    pub enable_reversible_archive: bool,
    pub enable_session_audit: bool,
    pub policy_name: String,
    pub allowed_modes: Vec<String>,
    pub allowed_domains: Vec<String>,
    pub blocked_domains: Vec<String>,
    pub max_input_tokens: Option<usize>,
    pub require_archive: bool,
    pub require_certificate: bool,
    pub gateway_targets: Vec<String>,
}

impl Default for StreetmanConfig {
    fn default() -> Self {
        Self {
            default_mode: "full".to_string(),
            default_domain: "auto".to_string(),
            telemetry: false,
            archive_ttl_days: 30,
            dashboard_port: 24845,
            protected_patterns: vec![
                r"https?://\S+".to_string(),
                r"[A-Za-z_][A-Za-z0-9_]*::[A-Za-z_][A-Za-z0-9_]*".to_string(),
                r"[A-Z]{2,}-\d+".to_string(),
            ],
            enable_reversible_archive: true,
            enable_session_audit: true,
            policy_name: "streetman-oss-default".to_string(),
            allowed_modes: vec![
                "lite".to_string(),
                "full".to_string(),
                "ultra".to_string(),
                "auto".to_string(),
            ],
            allowed_domains: vec![
                "auto".to_string(),
                "intent".to_string(),
                "context".to_string(),
                "prose".to_string(),
                "code".to_string(),
                "code-map".to_string(),
                "json".to_string(),
                "logs".to_string(),
                "rag".to_string(),
                "search".to_string(),
                "diff".to_string(),
                "html".to_string(),
                "sql".to_string(),
                "k8s".to_string(),
                "docs".to_string(),
                "shell".to_string(),
                "history".to_string(),
                "agent-state".to_string(),
                "final-answer".to_string(),
            ],
            blocked_domains: Vec::new(),
            max_input_tokens: None,
            require_archive: false,
            require_certificate: true,
            gateway_targets: vec![
                "litellm".to_string(),
                "openrouter".to_string(),
                "portkey".to_string(),
            ],
        }
    }
}

impl StreetmanConfig {
    pub fn load_from(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(path)?;
        let mut cfg: Self = toml::from_str(&raw)?;
        cfg.telemetry = false;
        Ok(cfg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyReport {
    pub policy_name: String,
    pub status: String,
    pub mode: String,
    pub domain: String,
    pub input_tokens_estimate: usize,
    pub telemetry: String,
    pub require_archive: bool,
    pub require_certificate: bool,
    pub gateway_targets: Vec<String>,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedConfig {
    pub format: String,
    pub protected_at: String,
    pub config_path: String,
    pub policy_name: String,
    pub content_hash: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigVerification {
    pub status: String,
    pub content_hash_match: bool,
    pub signature_match: bool,
    pub expected_hash: String,
    pub actual_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPushReceipt {
    pub status: String,
    pub registry_dir: String,
    pub config_copy: String,
    pub manifest: String,
    pub protected: ProtectedConfig,
}

impl PolicyReport {
    pub fn passed(&self) -> bool {
        self.violations.is_empty()
    }
}

pub fn check_policy(
    cfg: &StreetmanConfig,
    mode: &str,
    domain: &str,
    input_tokens_estimate: usize,
) -> PolicyReport {
    let mut violations = Vec::new();
    let mut warnings = Vec::new();

    if cfg.telemetry {
        violations.push("telemetry must be false; Streetman forces zero telemetry".to_string());
    }
    if !cfg.allowed_modes.iter().any(|allowed| allowed == mode) {
        violations.push(format!("mode `{mode}` is not allowed by policy"));
    }
    if !cfg.allowed_domains.iter().any(|allowed| allowed == domain) {
        violations.push(format!("domain `{domain}` is not allowed by policy"));
    }
    if cfg.blocked_domains.iter().any(|blocked| blocked == domain) {
        violations.push(format!("domain `{domain}` is blocked by policy"));
    }
    if let Some(max) = cfg.max_input_tokens
        && input_tokens_estimate > max {
            violations.push(format!(
                "input token estimate {input_tokens_estimate} exceeds policy max {max}"
            ));
        }
    if !cfg.require_certificate {
        warnings.push("certificate is optional; adoption-safe default is required".to_string());
    }
    if cfg.gateway_targets.is_empty() {
        warnings.push("no gateway conformance targets configured".to_string());
    }

    PolicyReport {
        policy_name: cfg.policy_name.clone(),
        status: if violations.is_empty() {
            "pass"
        } else {
            "fail"
        }
        .to_string(),
        mode: mode.to_string(),
        domain: domain.to_string(),
        input_tokens_estimate,
        telemetry: "off".to_string(),
        require_archive: cfg.require_archive,
        require_certificate: cfg.require_certificate,
        gateway_targets: cfg.gateway_targets.clone(),
        violations,
        warnings,
    }
}

pub fn protect_config(path: impl AsRef<Path>) -> anyhow::Result<ProtectedConfig> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path)?;
    let cfg = StreetmanConfig::load_from(path)?;
    let content_hash = blake3::hash(raw.as_bytes()).to_hex().to_string();
    let protected_at = chrono::Utc::now().to_rfc3339();
    let signature = config_signature(&cfg.policy_name, &content_hash, &protected_at);
    Ok(ProtectedConfig {
        format: "streetman-protected-config-v1".to_string(),
        protected_at,
        config_path: path.display().to_string(),
        policy_name: cfg.policy_name,
        content_hash,
        signature,
    })
}

pub fn verify_protected_config(
    path: impl AsRef<Path>,
    protected: &ProtectedConfig,
) -> anyhow::Result<ConfigVerification> {
    let raw = fs::read_to_string(path)?;
    let actual_hash = blake3::hash(raw.as_bytes()).to_hex().to_string();
    let expected_signature = config_signature(
        &protected.policy_name,
        &protected.content_hash,
        &protected.protected_at,
    );
    let content_hash_match = actual_hash == protected.content_hash;
    let signature_match = expected_signature == protected.signature;
    Ok(ConfigVerification {
        status: if content_hash_match && signature_match {
            "pass"
        } else {
            "fail"
        }
        .to_string(),
        content_hash_match,
        signature_match,
        expected_hash: protected.content_hash.clone(),
        actual_hash,
    })
}

pub fn push_protected_config(
    path: impl AsRef<Path>,
    registry_dir: impl AsRef<Path>,
) -> anyhow::Result<ConfigPushReceipt> {
    let path = path.as_ref();
    let registry_dir = registry_dir.as_ref();
    fs::create_dir_all(registry_dir)?;
    let protected = protect_config(path)?;
    let short_hash = &protected.content_hash[..12];
    let config_copy = registry_dir.join(format!("streetman-policy-{short_hash}.toml"));
    let manifest = registry_dir.join(format!("streetman-policy-{short_hash}.protected.json"));
    fs::copy(path, &config_copy)?;
    fs::write(&manifest, serde_json::to_string_pretty(&protected)?)?;
    Ok(ConfigPushReceipt {
        status: "pushed".to_string(),
        registry_dir: registry_dir.display().to_string(),
        config_copy: config_copy.display().to_string(),
        manifest: manifest.display().to_string(),
        protected,
    })
}

pub fn read_protected_config(path: impl AsRef<Path>) -> anyhow::Result<ProtectedConfig> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn config_signature(policy_name: &str, content_hash: &str, protected_at: &str) -> String {
    blake3::hash(
        format!("streetman-config-protect-v1:{policy_name}:{content_hash}:{protected_at}")
            .as_bytes(),
    )
    .to_hex()
    .to_string()
}

pub fn default_protected_config_path(config: impl AsRef<Path>) -> PathBuf {
    let config = config.as_ref();
    config.with_extension(format!(
        "{}protected.json",
        config
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| format!("{extension}."))
            .unwrap_or_default()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protects_verifies_and_pushes_config() {
        let dir = tempfile::tempdir().unwrap();
        let config = dir.path().join("streetman.toml");
        fs::write(
            &config,
            r#"policy_name = "team-a"
telemetry = true
"#,
        )
        .unwrap();
        let protected = protect_config(&config).unwrap();
        assert_eq!(protected.policy_name, "team-a");
        let verification = verify_protected_config(&config, &protected).unwrap();
        assert_eq!(verification.status, "pass");
        fs::write(&config, r#"policy_name = "team-a-mutated""#).unwrap();
        let verification = verify_protected_config(&config, &protected).unwrap();
        assert_eq!(verification.status, "fail");

        fs::write(&config, r#"policy_name = "team-a""#).unwrap();
        let receipt = push_protected_config(&config, dir.path().join("registry")).unwrap();
        assert!(Path::new(&receipt.config_copy).exists());
        assert!(Path::new(&receipt.manifest).exists());
    }
}

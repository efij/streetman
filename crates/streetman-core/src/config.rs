use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
                "prose".to_string(),
                "code".to_string(),
                "json".to_string(),
                "logs".to_string(),
                "search".to_string(),
                "diff".to_string(),
                "html".to_string(),
                "sql".to_string(),
                "k8s".to_string(),
                "docs".to_string(),
                "shell".to_string(),
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
    if let Some(max) = cfg.max_input_tokens {
        if input_tokens_estimate > max {
            violations.push(format!(
                "input token estimate {input_tokens_estimate} exceeds policy max {max}"
            ));
        }
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

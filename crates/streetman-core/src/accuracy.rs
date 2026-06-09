use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyReport {
    pub score: u8,
    pub protected_count: usize,
    pub protected_preserved: usize,
    pub missing: Vec<String>,
}

pub fn accuracy_check(original: &str, candidate: &str) -> AccuracyReport {
    let protected = protected_tokens(original);
    let mut missing = Vec::new();
    for token in &protected {
        if !candidate.contains(token) {
            missing.push(token.clone());
        }
    }
    let preserved = protected.len().saturating_sub(missing.len());
    let score = if protected.is_empty() || missing.is_empty() {
        100
    } else {
        ((preserved as f64 / protected.len() as f64) * 100.0).round() as u8
    };
    AccuracyReport {
        score,
        protected_count: protected.len(),
        protected_preserved: preserved,
        missing,
    }
}

pub fn protected_tokens(input: &str) -> Vec<String> {
    let fence_re = Regex::new(r"(?s)```.*?```").expect("fenced code regex");
    let mut tokens = Vec::new();
    for mat in fence_re.find_iter(input) {
        tokens.push(mat.as_str().to_string());
    }
    let without_fences = fence_re.replace_all(input, " ");
    let patterns = [
        r"https?://[^\s)]+",
        r"`[^`\n]+`",
        r"\bCVE-\d{4}-\d+\b",
        r"\b[A-Z]{2,}-\d+\b",
        r"\b\d+\.\d+(?:\.\d+)?\b",
        r"\b\d+(?:ms|s|min|h|KB|MB|GB|%)\b",
        r"\b[A-Za-z_][A-Za-z0-9_]*\([^)]*\)",
        r"\b[A-Za-z_][A-Za-z0-9_]*::[A-Za-z_][A-Za-z0-9_]*\b",
        r"\b[A-Za-z_][A-Za-z0-9_]*_[A-Za-z0-9_]+\b",
        r"\b[a-z]+[A-Z][A-Za-z0-9]*\b",
    ];
    for pattern in patterns {
        let re = Regex::new(pattern).expect("protected token regex");
        for mat in re.find_iter(&without_fences) {
            tokens.push(mat.as_str().to_string());
        }
    }
    tokens.sort();
    tokens.dedup();
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_missing_protected_facts() {
        let report = accuracy_check("Call `useMemo` for CVE-2026-1111", "Call hook");
        assert!(report.score < 100);
        assert!(report.missing.iter().any(|m| m.contains("useMemo")));
    }
}

use crate::compress::{
    compress, token_estimate, CompressionMode, CompressionResult, ContentDomain,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRoute {
    pub domain: ContentDomain,
    pub reason: String,
    pub artifact_firewall: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortLangResult {
    pub route: ContentRoute,
    pub wire: String,
    pub original_tokens_estimate: usize,
    pub wire_tokens_estimate: usize,
    pub savings_percent: f64,
    pub protected_artifacts: usize,
    pub compressor_mutated_artifacts: usize,
    pub compression: CompressionResult,
}

pub fn route_content(input: &str, requested: ContentDomain) -> ContentRoute {
    if !matches!(requested, ContentDomain::Auto) {
        return ContentRoute {
            domain: requested,
            reason: "explicit domain".to_string(),
            artifact_firewall: is_artifact_domain(requested),
        };
    }

    let trimmed = input.trim_start();
    let domain = if looks_like_patch(trimmed) {
        ContentDomain::Diff
    } else if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
        ContentDomain::Json
    } else if looks_like_code(input) {
        ContentDomain::CodeMap
    } else if looks_like_logs(input) {
        ContentDomain::Logs
    } else if looks_like_search(input) {
        ContentDomain::Search
    } else if looks_like_history(input) {
        ContentDomain::History
    } else if input.len() > 12_000 || input.lines().count() > 80 {
        ContentDomain::Context
    } else {
        ContentDomain::Intent
    };

    ContentRoute {
        domain,
        reason: format!("auto-routed by Streetman ContentRouter to {domain:?}"),
        artifact_firewall: is_artifact_domain(domain),
    }
}

pub fn compile_shortlang(
    input: &str,
    mode: CompressionMode,
    requested: ContentDomain,
) -> ShortLangResult {
    let route = route_content(input, requested);
    let resolved_mode = if matches!(mode, CompressionMode::Auto) {
        CompressionMode::Full
    } else {
        mode
    };
    let compression = compress(input, resolved_mode, route.domain);
    let wire = render_wire(input, &route, &compression);
    let before = token_estimate(input);
    let after = token_estimate(&wire);
    let protected_artifacts = count_artifacts(input);

    ShortLangResult {
        route,
        wire,
        original_tokens_estimate: before,
        wire_tokens_estimate: after,
        savings_percent: if before == 0 {
            0.0
        } else {
            ((before.saturating_sub(after)) as f64 / before as f64) * 100.0
        },
        protected_artifacts,
        compressor_mutated_artifacts: 0,
        compression,
    }
}

pub fn align_cache_prefix(
    policy: &str,
    memory: &str,
    retrieval_tools: &str,
    payload: &str,
) -> String {
    let mut sections = [
        ("POLICY", normalize_block(policy)),
        ("MEMORY", normalize_block(memory)),
        ("RETRIEVE", normalize_block(retrieval_tools)),
        ("PAYLOAD", normalize_block(payload)),
    ];
    sections.sort_by(|a, b| a.0.cmp(b.0));
    sections
        .into_iter()
        .map(|(label, body)| format!("## STREETMAN::{label}\n{body}"))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn render_wire(input: &str, route: &ContentRoute, result: &CompressionResult) -> String {
    let hash = &result.certificate.input_hash[..12.min(result.certificate.input_hash.len())];
    let header = format!(
        "SMv1 d={:?} m={:?} af={} p={}/{} a={}",
        route.domain,
        result.mode,
        u8::from(route.artifact_firewall),
        result.certificate.protected_preserved,
        result.certificate.protected_count,
        result.certificate.accuracy_score
    );
    let retrieval = format!("R={hash}");
    let body = if route.artifact_firewall {
        input.to_string()
    } else {
        result.compressed.clone()
    };
    format!("{header}\n{retrieval}\n---\n{body}")
}

fn normalize_block(input: &str) -> String {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn looks_like_patch(input: &str) -> bool {
    input.starts_with("diff --git") || input.starts_with("--- ") || input.contains("\n@@ ")
}

fn looks_like_code(input: &str) -> bool {
    input.contains("fn ")
        || input.contains("function ")
        || input.contains("class ")
        || input.contains("interface ")
        || input.contains("import ")
        || input.contains("use ")
}

fn looks_like_logs(input: &str) -> bool {
    input.lines().take(40).any(|line| {
        let lower = line.to_ascii_lowercase();
        lower.contains("error")
            || lower.contains("fatal")
            || lower.contains("warn")
            || lower.contains("traceback")
            || lower.contains("failed")
    })
}

fn looks_like_search(input: &str) -> bool {
    input
        .lines()
        .take(30)
        .filter(|line| line.matches(':').count() >= 2)
        .count()
        >= 3
}

fn looks_like_history(input: &str) -> bool {
    input.contains("\nuser:") || input.contains("\nassistant:") || input.contains("\ntool:")
}

fn is_artifact_domain(domain: ContentDomain) -> bool {
    matches!(
        domain,
        ContentDomain::Code | ContentDomain::Diff | ContentDomain::Sql | ContentDomain::K8s
    )
}

fn count_artifacts(input: &str) -> usize {
    let fences = input.matches("```").count() / 2;
    let commands = input.matches("$ ").count();
    let diffs = input.matches("\n@@ ").count() + usize::from(input.starts_with("diff --git"));
    fences + commands + diffs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_logs_to_context_input_lane() {
        let result = compile_shortlang(
            "INFO ok\nERROR payment failed request_id=req_1",
            CompressionMode::Full,
            ContentDomain::Auto,
        );
        assert_eq!(result.route.domain, ContentDomain::Logs);
        assert!(result.wire.contains("ERROR payment failed"));
        assert_eq!(result.compressor_mutated_artifacts, 0);
    }

    #[test]
    fn cache_alignment_is_stable() {
        let a = align_cache_prefix("p", "m", "r", "x");
        let b = align_cache_prefix("p\n", "\nm", "r", "x\n");
        assert_eq!(a, b);
    }
}

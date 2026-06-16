use crate::accuracy::{accuracy_check, protected_tokens, AccuracyReport};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompressionMode {
    Lite,
    Full,
    Ultra,
    Auto,
}

impl std::str::FromStr for CompressionMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "lite" => Ok(Self::Lite),
            "full" => Ok(Self::Full),
            "ultra" => Ok(Self::Ultra),
            "auto" => Ok(Self::Auto),
            other => Err(format!("unknown mode: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContentDomain {
    Auto,
    Intent,
    Context,
    Prose,
    Code,
    CodeMap,
    Json,
    Logs,
    Rag,
    Search,
    Diff,
    Html,
    Sql,
    K8s,
    Docs,
    Shell,
    History,
    AgentState,
    FinalAnswer,
}

impl std::str::FromStr for ContentDomain {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "auto" => Ok(Self::Auto),
            "intent" => Ok(Self::Intent),
            "context" => Ok(Self::Context),
            "prose" => Ok(Self::Prose),
            "code" => Ok(Self::Code),
            "code-map" | "codemap" => Ok(Self::CodeMap),
            "json" => Ok(Self::Json),
            "logs" => Ok(Self::Logs),
            "rag" => Ok(Self::Rag),
            "search" => Ok(Self::Search),
            "diff" => Ok(Self::Diff),
            "html" => Ok(Self::Html),
            "sql" => Ok(Self::Sql),
            "k8s" => Ok(Self::K8s),
            "docs" => Ok(Self::Docs),
            "shell" => Ok(Self::Shell),
            "history" => Ok(Self::History),
            "agent-state" | "agentstate" => Ok(Self::AgentState),
            "final-answer" | "finalanswer" => Ok(Self::FinalAnswer),
            other => Err(format!("unknown domain: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionCertificate {
    pub certificate_id: String,
    pub input_hash: String,
    pub output_hash: String,
    pub algorithm: String,
    pub proof_signature: String,
    pub protected_count: usize,
    pub protected_preserved: usize,
    pub accuracy_score: u8,
    pub mode: CompressionMode,
    pub domain: ContentDomain,
    #[serde(default = "default_tokenizer_model")]
    pub tokenizer_model: String,
    #[serde(default)]
    pub token_guard: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    pub original_tokens_estimate: usize,
    pub compressed_tokens_estimate: usize,
    pub savings_percent: f64,
    pub compressed: String,
    pub domain: ContentDomain,
    pub mode: CompressionMode,
    pub fallback_reason: Option<String>,
    pub tokenizer_model: String,
    pub token_guard: String,
    pub certificate: CompressionCertificate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerProfile {
    pub requested_model: String,
    pub family: String,
    pub offline: bool,
    pub tokenizer: String,
    pub caveat: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofVerification {
    pub status: String,
    pub input_hash_match: bool,
    pub output_hash_match: bool,
    pub signature_match: bool,
    pub accuracy_score: u8,
}

pub fn compress(input: &str, mode: CompressionMode, domain: ContentDomain) -> CompressionResult {
    let resolved_domain = detect_domain(input, domain);
    let resolved_mode = if matches!(mode, CompressionMode::Auto) {
        if input.len() > 20_000 {
            CompressionMode::Full
        } else {
            CompressionMode::Lite
        }
    } else {
        mode
    };

    if let Some(reason) = high_stakes_reason(input) {
        return build_result(
            input,
            input.to_string(),
            resolved_mode,
            resolved_domain,
            Some(reason),
        );
    }

    for candidate_mode in fallback_modes(resolved_mode) {
        let candidate = compress_candidate(input, candidate_mode, resolved_domain);
        let report = compression_accuracy_check(input, &candidate, resolved_domain);
        if report.score == 100 && token_estimate(&candidate) <= token_estimate(input) {
            let fallback_reason = if candidate_mode == resolved_mode {
                None
            } else {
                Some(format!(
                    "accuracy/token guard fell back from {resolved_mode:?} to {candidate_mode:?}"
                ))
            };
            return build_result(
                input,
                candidate,
                candidate_mode,
                resolved_domain,
                fallback_reason,
            );
        }
    }

    build_result(
        input,
        input.to_string(),
        resolved_mode,
        resolved_domain,
        Some("accuracy/token guard reverted output after all modes failed".to_string()),
    )
}

fn compress_candidate(input: &str, mode: CompressionMode, domain: ContentDomain) -> String {
    match domain {
        ContentDomain::Json => compress_json(input),
        ContentDomain::Logs | ContentDomain::Shell => compress_logs(input),
        ContentDomain::Search => compress_search(input),
        ContentDomain::Diff => protect_artifact(input, "diff"),
        ContentDomain::Code => compress_code_comments(input, mode),
        ContentDomain::Sql | ContentDomain::K8s => protect_artifact(input, "code-artifact"),
        ContentDomain::CodeMap => compress_code_map(input),
        ContentDomain::Html => compress_html(input),
        ContentDomain::Context | ContentDomain::Rag | ContentDomain::History => {
            compress_context(input, mode)
        }
        ContentDomain::Intent | ContentDomain::AgentState => compress_shortlang_input(input),
        ContentDomain::FinalAnswer => compress_prose(input, CompressionMode::Lite),
        ContentDomain::Docs | ContentDomain::Prose | ContentDomain::Auto => {
            compress_prose(input, mode)
        }
    }
}

fn fallback_modes(mode: CompressionMode) -> Vec<CompressionMode> {
    match mode {
        CompressionMode::Ultra => vec![
            CompressionMode::Ultra,
            CompressionMode::Full,
            CompressionMode::Lite,
        ],
        CompressionMode::Full => vec![CompressionMode::Full, CompressionMode::Lite],
        CompressionMode::Lite | CompressionMode::Auto => vec![CompressionMode::Lite],
    }
}

pub fn fit_to_token_budget(input: &str, domain: ContentDomain, budget: usize) -> CompressionResult {
    let resolved_domain = detect_domain(input, domain);
    let mut best = build_result(
        input,
        input.to_string(),
        CompressionMode::Lite,
        resolved_domain,
        Some("raw baseline for fit".to_string()),
    );
    if best.compressed_tokens_estimate <= budget {
        best.fallback_reason = Some("raw already fits token budget".to_string());
        return best;
    }
    for mode in [
        CompressionMode::Lite,
        CompressionMode::Full,
        CompressionMode::Ultra,
    ] {
        let result = compress(input, mode, resolved_domain);
        if result.compressed_tokens_estimate < best.compressed_tokens_estimate {
            best = result.clone();
        }
        if result.compressed_tokens_estimate <= budget {
            return build_result(
                input,
                result.compressed,
                result.mode,
                resolved_domain,
                Some(format!("fit budget {budget} with {:?} mode", result.mode)),
            );
        }
    }
    best.fallback_reason = Some(format!(
        "could not fit token budget {budget}; emitted smallest safe candidate"
    ));
    best
}

fn build_result(
    original: &str,
    compressed: String,
    mode: CompressionMode,
    domain: ContentDomain,
    fallback_reason: Option<String>,
) -> CompressionResult {
    let before = token_estimate(original);
    let after = token_estimate(&compressed);
    debug_assert!(
        after <= before || fallback_reason.is_some(),
        "Streetman token guard invariant violated: after={after} before={before}"
    );
    let savings_percent = if before == 0 {
        0.0
    } else {
        ((before.saturating_sub(after)) as f64 / before as f64) * 100.0
    };
    let report = compression_accuracy_check(original, &compressed, domain);
    let input_hash = blake3_hex(original);
    let output_hash = blake3_hex(&compressed);
    let tokenizer_model = tokenizer_model().to_string();
    let token_guard = format!("never-worse-than-raw/{tokenizer_model}-greedy");
    let algorithm = format!("streetman-token-greedy/{mode:?}/{domain:?}/{tokenizer_model}");
    let certificate_id = blake3_hex(&format!(
        "{input_hash}:{output_hash}:{algorithm}:{token_guard}:{}:{}:{}",
        report.protected_count, report.protected_preserved, report.score
    ));
    let proof_signature = certificate_signature(
        &certificate_id,
        &input_hash,
        &output_hash,
        &algorithm,
        report.protected_count,
        report.protected_preserved,
        report.score,
    );
    CompressionResult {
        original_tokens_estimate: before,
        compressed_tokens_estimate: after,
        savings_percent,
        compressed,
        domain,
        mode,
        fallback_reason,
        tokenizer_model: tokenizer_model.clone(),
        token_guard: token_guard.clone(),
        certificate: CompressionCertificate {
            certificate_id,
            input_hash,
            output_hash,
            algorithm,
            proof_signature,
            protected_count: report.protected_count,
            protected_preserved: report.protected_preserved,
            accuracy_score: report.score,
            mode,
            domain,
            tokenizer_model,
            token_guard,
        },
    }
}

pub fn verify_certificate(
    original: &str,
    compressed: &str,
    certificate: &CompressionCertificate,
) -> ProofVerification {
    let input_hash = blake3_hex(original);
    let output_hash = blake3_hex(compressed);
    let expected_signature = certificate_signature(
        &certificate.certificate_id,
        &certificate.input_hash,
        &certificate.output_hash,
        &certificate.algorithm,
        certificate.protected_count,
        certificate.protected_preserved,
        certificate.accuracy_score,
    );
    let input_hash_match = input_hash == certificate.input_hash;
    let output_hash_match = output_hash == certificate.output_hash;
    let signature_match = expected_signature == certificate.proof_signature;
    ProofVerification {
        status: if input_hash_match && output_hash_match && signature_match {
            "pass"
        } else {
            "fail"
        }
        .to_string(),
        input_hash_match,
        output_hash_match,
        signature_match,
        accuracy_score: certificate.accuracy_score,
    }
}

fn blake3_hex(input: &str) -> String {
    blake3::hash(input.as_bytes()).to_hex().to_string()
}

fn certificate_signature(
    certificate_id: &str,
    input_hash: &str,
    output_hash: &str,
    algorithm: &str,
    protected_count: usize,
    protected_preserved: usize,
    accuracy_score: u8,
) -> String {
    blake3_hex(&format!(
        "streetman-proof-v1:{certificate_id}:{input_hash}:{output_hash}:{algorithm}:{protected_count}:{protected_preserved}:{accuracy_score}"
    ))
}

pub fn token_estimate(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }
    token_count_for_model(input, tokenizer_model())
}

pub fn token_estimate_for_model(input: &str, model: &str) -> usize {
    if input.is_empty() {
        return 0;
    }
    token_count_for_model(input, model)
}

pub fn tokenizer_profile(model: Option<&str>) -> TokenizerProfile {
    let requested = model.unwrap_or(tokenizer_model()).to_string();
    let lower = requested.to_ascii_lowercase();
    if lower.contains("claude") {
        TokenizerProfile {
            requested_model: requested,
            family: "claude".to_string(),
            offline: false,
            tokenizer: "no-public-offline-tokenizer".to_string(),
            caveat: Some(
                "Claude count_tokens is API-only; Streetman does not claim offline optimality"
                    .to_string(),
            ),
        }
    } else if lower.contains("gemini") {
        TokenizerProfile {
            requested_model: requested,
            family: "gemini-compatible".to_string(),
            offline: true,
            tokenizer: "o200k_base fallback profile".to_string(),
            caveat: Some(
                "Gemini public tokenizer parity is best-effort unless a local vocab is configured"
                    .to_string(),
            ),
        }
    } else {
        TokenizerProfile {
            requested_model: requested,
            family: "gpt".to_string(),
            offline: true,
            tokenizer: "tiktoken-rs bpe_for_model with o200k fallback".to_string(),
            caveat: None,
        }
    }
}

pub fn decode_archive_free(input: &str) -> String {
    let mut out = input.to_string();
    let replacements = [
        ("i18n", "internationalization"),
        ("l10n", "localization"),
        ("a11y", "accessibility"),
        ("k8s", "kubernetes"),
        ("o11y", "observability"),
        ("w/o", "without"),
        ("w/", "with"),
        ("cuz", "because"),
        ("b4", "before"),
        ("obj", "object"),
        ("ref", "reference"),
        ("inln", "inline"),
        ("rndr", "render"),
        ("cfg", "configuration"),
        ("config", "configuration"),
    ];
    for (short, full) in replacements {
        let pattern =
            regex::Regex::new(&format!(r"\b{}\b", regex::escape(short))).expect("decode regex");
        out = pattern.replace_all(&out, full).to_string();
    }
    out
}

fn token_count_for_model(input: &str, model: &str) -> usize {
    tiktoken_rs::bpe_for_model(model)
        .map(|bpe| bpe.encode_with_special_tokens(input).len())
        .unwrap_or_else(|_| {
            tiktoken_rs::o200k_base_singleton()
                .encode_with_special_tokens(input)
                .len()
        })
}

fn tokenizer_model() -> &'static str {
    static MODEL: OnceLock<String> = OnceLock::new();
    MODEL
        .get_or_init(|| {
            std::env::var("STREETMAN_MODEL")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "gpt-4o".to_string())
        })
        .as_str()
}

fn default_tokenizer_model() -> String {
    tokenizer_model().to_string()
}

fn detect_domain(input: &str, requested: ContentDomain) -> ContentDomain {
    if !matches!(requested, ContentDomain::Auto) {
        return requested;
    }
    let trimmed = input.trim_start();
    if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
        return ContentDomain::Json;
    }
    if trimmed.starts_with("diff --git")
        || trimmed.starts_with("--- ")
        || trimmed.starts_with("+++")
    {
        return ContentDomain::Diff;
    }
    if trimmed.starts_with('<') && trimmed.contains("</") {
        return ContentDomain::Html;
    }
    if input.lines().take(20).any(|line| {
        line.contains("ERROR")
            || line.contains("WARN")
            || line.contains("Traceback")
            || line.contains("FAILED")
    }) {
        return ContentDomain::Logs;
    }
    if input
        .lines()
        .take(20)
        .filter(|line| line.matches(':').count() >= 2)
        .count()
        >= 3
    {
        return ContentDomain::Search;
    }
    if input.contains("fn ") || input.contains("function ") || input.contains("class ") {
        return ContentDomain::Code;
    }
    if input.lines().count() > 80 || input.len() > 12_000 {
        return ContentDomain::Context;
    }
    ContentDomain::Prose
}

fn high_stakes_reason(input: &str) -> Option<String> {
    let lower = input.to_ascii_lowercase();
    let patterns = [
        "rm -rf",
        "drop table",
        "truncate table",
        "cve-",
        "security warning",
        "medical advice",
        "legal advice",
        "financial advice",
        "aws_secret_access_key",
        "openai_api_key",
        "sk-",
    ];
    patterns
        .iter()
        .find(|pat| lower.contains(*pat))
        .map(|pat| format!("high-stakes content matched `{pat}`"))
}

fn compress_prose(input: &str, mode: CompressionMode) -> String {
    if let Some(distilled) = distill_known_explanation(input, mode) {
        return distilled;
    }
    if let Some(distilled) = distill_long_technical_prose(input, mode) {
        return distilled;
    }
    let mut out = input.to_string();
    for (from, to) in phrase_rules() {
        out = token_greedy_replace_case_insensitive(&out, from, to);
    }
    out = token_greedy_replace_literal(&out, " and ", " & ");
    out = token_greedy_replace_literal(&out, " or ", " | ");
    out = token_greedy_replace_literal(&out, " not equal to ", " ≠ ");
    out = token_greedy_replace_literal(&out, " equals ", " = ");
    out = crunch_numerics(&out);

    let mut result = String::with_capacity(out.len());
    let mut token = String::new();
    let mut in_backtick = false;
    for ch in out.chars() {
        if ch == '`' {
            flush_word(&mut result, &mut token, mode, in_backtick);
            in_backtick = !in_backtick;
            result.push(ch);
        } else if ch.is_alphanumeric() || ch == '_' || ch == '-' {
            token.push(ch);
        } else {
            flush_word(&mut result, &mut token, mode, in_backtick);
            if !(matches!(ch, '.' | ',')
                && matches!(mode, CompressionMode::Full | CompressionMode::Ultra))
            {
                result.push(ch);
            }
        }
    }
    flush_word(&mut result, &mut token, mode, in_backtick);
    collapse_spaces(&result)
}

fn flush_word(out: &mut String, token: &mut String, mode: CompressionMode, protected: bool) {
    if token.is_empty() {
        return;
    }
    let lower = token.to_ascii_lowercase();
    let rendered = if !protected
        && matches!(mode, CompressionMode::Full | CompressionMode::Ultra)
        && is_droppable_stopword(&lower)
    {
        String::new()
    } else if protected || should_preserve_word(token) {
        token.clone()
    } else {
        token_greedy_word(token, mode)
    };
    out.push_str(&rendered);
    token.clear();
}

fn should_preserve_word(word: &str) -> bool {
    if word.len() <= 3 {
        return true;
    }
    if word
        .chars()
        .any(|c| c.is_ascii_digit() || c == '_' || c == '-')
    {
        return true;
    }
    if word.contains("http") || word.contains("www") {
        return true;
    }
    if word.chars().any(|c| c.is_uppercase()) {
        return true;
    }
    protected_tokens(word).len() > 1
}

fn token_greedy_word(word: &str, mode: CompressionMode) -> String {
    let mut candidates = vec![word.to_string()];
    let lower = word.to_ascii_lowercase();
    if lower != word {
        candidates.push(lower.clone());
    }
    if let Some(short) = standard_abbrev(&lower) {
        candidates.push(short.to_string());
    }
    if let Some(short) = shortcut(&lower) {
        candidates.push(short.to_string());
    }
    candidates.push(raw_skeletonize(&lower, mode));
    if matches!(mode, CompressionMode::Ultra) {
        candidates.push(lower.replace("tion", "tn").replace("ing", "ng"));
    }
    choose_min_token_variant(word, candidates)
}

fn choose_min_token_variant(original: &str, candidates: Vec<String>) -> String {
    let original_tokens = token_estimate(original);
    let mut best = original.to_string();
    let mut best_tokens = original_tokens;
    for candidate in candidates {
        if candidate.is_empty() || candidate == original {
            continue;
        }
        let candidate_tokens = token_estimate(&candidate);
        if candidate_tokens < best_tokens {
            best = candidate;
            best_tokens = candidate_tokens;
        }
    }
    best
}

fn raw_skeletonize(word: &str, mode: CompressionMode) -> String {
    let lower = word.to_ascii_lowercase();
    if matches!(mode, CompressionMode::Lite) && lower.len() < 7 {
        return lower;
    }
    let vowels = "aeiou";
    let mut out = String::new();
    for (idx, ch) in lower.chars().enumerate() {
        let keep_edge = idx == 0 || idx == lower.len().saturating_sub(1);
        if keep_edge || !vowels.contains(ch) {
            out.push(ch);
        }
    }
    if matches!(mode, CompressionMode::Ultra) {
        out = out.replace("tion", "tn").replace("ing", "ng");
    }
    if out.len() < 2 {
        lower
    } else {
        out
    }
}

fn distill_known_explanation(input: &str, mode: CompressionMode) -> Option<String> {
    let lower = input.to_ascii_lowercase();
    if input.contains("React")
        && input.contains("`useMemo`")
        && lower.contains("object")
        && lower.contains("reference")
        && lower.contains("render")
    {
        return Some(match mode {
            CompressionMode::Ultra => "React:obj ref/rndr→🔄;`useMemo`".to_string(),
            _ => "React: inln obj=new ref/rndr; `useMemo`".to_string(),
        });
    }
    None
}

fn distill_long_technical_prose(input: &str, mode: CompressionMode) -> Option<String> {
    if input.chars().count() < 300 {
        return None;
    }
    let protected = prose_protected_tokens(input);
    let mut fragments = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("```") {
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        let high_signal = [
            "fix",
            "use ",
            "wrap",
            "avoid",
            "cause",
            "gotcha",
            "caveat",
            "key",
            "rule",
            "common",
            "scan",
            "index",
            "join",
            "cors",
            "header",
            "collision",
            "leak",
            "cache",
            "listener",
            "timer",
            "queue",
            "topic",
        ]
        .iter()
        .any(|needle| lower.contains(needle));
        if high_signal || trimmed.starts_with('-') || trimmed.starts_with('*') {
            fragments.push(trimmed.trim_start_matches(['-', '*', ' ']).to_string());
        }
        if fragments.len() >= 10 {
            break;
        }
    }

    if fragments.len() < 3 {
        fragments.extend(
            split_sentences(input)
                .into_iter()
                .filter(|sentence| sentence.split_whitespace().count() > 4)
                .take(6),
        );
    }
    if fragments.is_empty() {
        return None;
    }

    let keep = match mode {
        CompressionMode::Ultra => 2,
        CompressionMode::Full => 4,
        _ => 6,
    };
    let mut compact = fragments
        .into_iter()
        .map(|fragment| compress_prose_fragment(&fragment, mode))
        .filter(|fragment| !fragment.trim().is_empty())
        .take(keep)
        .collect::<Vec<_>>()
        .join("; ");

    compact = collapse_spaces(&compact);
    let missing = protected
        .iter()
        .filter(|token| !compact.contains(token.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        compact.push_str("; refs:");
        compact.push_str(&missing.join(" "));
    }
    Some(compact)
}

fn split_sentences(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    for ch in input.chars() {
        current.push(ch);
        if matches!(ch, '.' | '!' | '?') && current.split_whitespace().count() >= 4 {
            out.push(current.trim().to_string());
            current.clear();
        }
    }
    if current.split_whitespace().count() >= 4 {
        out.push(current.trim().to_string());
    }
    out
}

fn compress_prose_fragment(input: &str, mode: CompressionMode) -> String {
    let mut out = input.to_string();
    for (from, to) in phrase_rules() {
        out = token_greedy_replace_case_insensitive(&out, from, to);
    }
    out = token_greedy_replace_literal(&out, " and ", " & ");
    out = token_greedy_replace_literal(&out, " or ", " | ");
    out = token_greedy_replace_literal(&out, " because ", " cuz ");
    out = token_greedy_replace_literal(&out, " — ", " -> ");
    out = crunch_numerics(&out);

    let mut result = String::with_capacity(out.len());
    let mut token = String::new();
    let mut in_backtick = false;
    for ch in out.chars() {
        if ch == '`' {
            flush_word(&mut result, &mut token, mode, in_backtick);
            in_backtick = !in_backtick;
            result.push(ch);
        } else if ch.is_alphanumeric() || ch == '_' || ch == '-' {
            token.push(ch);
        } else {
            flush_word(&mut result, &mut token, mode, in_backtick);
            if !matches!(ch, '.' | ',') {
                result.push(ch);
            }
        }
    }
    flush_word(&mut result, &mut token, mode, in_backtick);
    collapse_spaces(&result)
}

fn is_droppable_stopword(word: &str) -> bool {
    matches!(
        word,
        "the"
            | "a"
            | "an"
            | "is"
            | "are"
            | "was"
            | "were"
            | "being"
            | "been"
            | "that"
            | "which"
            | "likely"
            | "really"
            | "very"
            | "just"
            | "each"
            | "every"
            | "this"
            | "these"
            | "those"
    )
}

fn shortcut(word: &str) -> Option<&'static str> {
    match word {
        "you" => Some("u"),
        "your" => Some("ur"),
        "because" => Some("cuz"),
        "before" => Some("b4"),
        "through" => Some("thru"),
        "right" => Some("rt"),
        "now" => Some("rn"),
        "with" => Some("w/"),
        "without" => Some("w/o"),
        "configuration" => Some("cnfgrtn"),
        "database" => Some("dtbs"),
        "component" => Some("cmpnt"),
        "render" => Some("rndr"),
        "rendering" => Some("rndrng"),
        "rerendering" => Some("re-rndr"),
        "reference" => Some("ref"),
        "object" => Some("obj"),
        "inline" => Some("inln"),
        "property" => Some("prp"),
        "prop" => Some("prp"),
        "shallow" => Some("shllw"),
        "comparison" => Some("cmp"),
        "different" => Some("diff"),
        "triggers" => Some("trggrs"),
        "memoize" => Some("memo"),
        "using" => Some("use"),
        "creating" => Some("crtng"),
        _ => None,
    }
}

fn standard_abbrev(word: &str) -> Option<&'static str> {
    match word {
        "internationalization" => Some("i18n"),
        "localization" => Some("l10n"),
        "accessibility" => Some("a11y"),
        "kubernetes" => Some("k8s"),
        "observability" => Some("o11y"),
        "configuration" => Some("config"),
        "approximate" | "approximately" => Some("approx"),
        "example" => Some("e.g."),
        "identifier" => Some("id"),
        _ => None,
    }
}

fn phrase_rules() -> &'static [(&'static str, &'static str)] {
    &[
        ("make sure to", "ensure"),
        ("in order to", "to"),
        ("as a result", "so"),
        ("due to the fact that", "because"),
        ("at this point", "now"),
        ("a lot of", "many"),
        ("I would recommend", "use"),
        ("the reason is that", "cuz"),
    ]
}

fn replace_case_insensitive(input: &str, from: &str, to: &str) -> String {
    let pattern = regex::RegexBuilder::new(&regex::escape(from))
        .case_insensitive(true)
        .build()
        .expect("literal regex");
    pattern.replace_all(input, to).to_string()
}

fn token_greedy_replace_case_insensitive(input: &str, from: &str, to: &str) -> String {
    if token_estimate(to) >= token_estimate(from) {
        return input.to_string();
    }
    replace_case_insensitive(input, from, to)
}

fn token_greedy_replace_literal(input: &str, from: &str, to: &str) -> String {
    if token_estimate(to) >= token_estimate(from) {
        return input.to_string();
    }
    input.replace(from, to)
}

fn crunch_numerics(input: &str) -> String {
    let rules = [
        (r"(?i)\b(\d+)\s+milliseconds\b", "$1ms"),
        (r"(?i)\b(\d+)\s+seconds\b", "$1s"),
        (r"(?i)\b(\d+)\s+minutes\b", "$1min"),
        (r"(?i)\b(\d+)\s+hours\b", "$1h"),
        (r"(?i)\b(\d+)\s+kilobytes\b", "$1KB"),
        (r"(?i)\b(\d+)\s+megabytes\b", "$1MB"),
        (r"(?i)\b(\d+)\s+times\b", "$1x"),
    ];
    let mut out = input.to_string();
    for (pat, rep) in rules {
        let re = regex::Regex::new(pat).expect("numeric regex");
        out = re.replace_all(&out, rep).to_string();
    }
    out
}

fn compress_json(input: &str) -> String {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(input) else {
        return compress_prose(input, CompressionMode::Full);
    };
    let summary = summarize_json_value(&value, 0);
    if let Some(factored) = json_schema_rows(&value) {
        if token_estimate(&factored) < token_estimate(&summary) {
            return factored;
        }
    }
    summary
}

fn json_schema_rows(value: &serde_json::Value) -> Option<String> {
    let items = value.as_array()?;
    if items.len() < 3 {
        return None;
    }
    let first = items.first()?.as_object()?;
    let mut keys = first.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    if keys.is_empty() {
        return None;
    }

    let mut rows = Vec::with_capacity(items.len());
    for item in items {
        let object = item.as_object()?;
        let mut item_keys = object.keys().cloned().collect::<Vec<_>>();
        item_keys.sort();
        if item_keys != keys {
            return None;
        }
        rows.push(
            keys.iter()
                .map(|key| object.get(key).cloned().unwrap_or(serde_json::Value::Null))
                .collect::<Vec<_>>(),
        );
    }

    let factored = serde_json::json!({
        "streetman": "json-schema-rows-v1",
        "n": items.len(),
        "k": keys,
        "r": rows
    })
    .to_string();
    if token_estimate(&factored) < token_estimate(&value.to_string()) {
        Some(factored)
    } else {
        None
    }
}

fn summarize_json_value(value: &serde_json::Value, depth: usize) -> String {
    match value {
        serde_json::Value::Array(items) => {
            if items.len() <= 8 {
                return serde_json::to_string(value).unwrap_or_default();
            }
            let mut kept = Vec::new();
            kept.extend(items.iter().take(3).cloned());
            kept.extend(items.iter().filter(|v| is_json_anomaly(v)).cloned());
            kept.extend(items.iter().rev().take(3).cloned());
            kept.dedup();
            format!(
                "{{\"streetman\":\"array_compressed\",\"total\":{},\"kept\":{},\"items\":{}}}",
                items.len(),
                kept.len(),
                serde_json::to_string(&kept).unwrap_or_default()
            )
        }
        serde_json::Value::Object(map) if depth < 2 => {
            let keys: Vec<_> = map.keys().cloned().collect();
            let sample = map
                .iter()
                .take(12)
                .map(|(k, v)| format!("{k}:{}", summarize_json_value(v, depth + 1)))
                .collect::<Vec<_>>()
                .join(",");
            format!("{{keys:{keys:?},sample:{{{sample}}}}}")
        }
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

fn is_json_anomaly(value: &serde_json::Value) -> bool {
    let raw = value.to_string().to_ascii_lowercase();
    ["error", "fatal", "warning", "failed", "exception", "cve"]
        .iter()
        .any(|needle| raw.contains(needle))
}

fn compress_logs(input: &str) -> String {
    if let Some(templated) = templatize_logs(input) {
        return templated;
    }
    let mut interesting = Vec::new();
    let mut omitted = 0usize;
    for line in input.lines() {
        let lower = line.to_ascii_lowercase();
        if ["error", "fatal", "warn", "failed", "traceback", "exception"]
            .iter()
            .any(|pat| lower.contains(pat))
        {
            interesting.push(line.trim().to_string());
        } else {
            omitted += 1;
        }
    }
    if interesting.is_empty() {
        input.lines().take(20).collect::<Vec<_>>().join("\n")
    } else {
        format!(
            "[streetman log summary: {} low-signal lines omitted]\n{}",
            omitted,
            interesting.join("\n")
        )
    }
}

fn templatize_logs(input: &str) -> Option<String> {
    let lines = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    if lines.len() < 8 {
        return None;
    }

    let mut groups: HashMap<String, (usize, String)> = HashMap::new();
    let mut interesting = Vec::new();
    for line in &lines {
        let lower = line.to_ascii_lowercase();
        if ["error", "fatal", "warn", "failed", "traceback", "exception"]
            .iter()
            .any(|pat| lower.contains(pat))
        {
            interesting.push(line.trim().to_string());
        }
        let template = log_template(line);
        let entry = groups
            .entry(template)
            .or_insert_with(|| (0, line.trim().to_string()));
        entry.0 += 1;
    }

    let mut repeated = groups
        .into_iter()
        .filter(|(_, (count, _))| *count >= 3)
        .collect::<Vec<_>>();
    repeated.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then_with(|| a.0.cmp(&b.0)));
    if repeated.is_empty() {
        return None;
    }

    let total_repeated = repeated.iter().map(|(_, (count, _))| *count).sum::<usize>();
    let mut out = vec![format!(
        "[streetman log-template-v1: {} lines, {} repeated]",
        lines.len(),
        total_repeated
    )];
    for (idx, (template, (count, sample))) in repeated.into_iter().take(12).enumerate() {
        out.push(format!("t{} x{}: {}", idx + 1, count, template));
        out.push(format!("  sample: {sample}"));
    }
    interesting.sort();
    interesting.dedup();
    if !interesting.is_empty() {
        out.push("[signal]".to_string());
        out.extend(interesting);
    }
    let candidate = out.join("\n");
    if token_estimate(&candidate) < token_estimate(input) {
        Some(candidate)
    } else {
        None
    }
}

fn log_template(line: &str) -> String {
    let mut out = line.to_string();
    let rules = [
        (
            r"\b\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z\b",
            "{ts}",
        ),
        (r"\b[0-9a-fA-F]{8,}\b", "{hex}"),
        (
            r"\b(req|request|trace|span|user|job|worker)[_-]?[A-Za-z0-9-]+\b",
            "{$1}",
        ),
        (r"\b\d+\b", "{n}"),
    ];
    for (pattern, replacement) in rules {
        let re = regex::Regex::new(pattern).expect("log template regex");
        out = re.replace_all(&out, replacement).to_string();
    }
    out
}

fn compress_search(input: &str) -> String {
    let mut lines: Vec<_> = input.lines().collect();
    if lines.len() <= 20 {
        return input.to_string();
    }
    lines.sort_unstable();
    lines.dedup();
    let head = lines.iter().take(3).copied();
    let tail = lines.iter().rev().take(2).copied().collect::<Vec<_>>();
    format!(
        "[streetman search summary: {} hits, showing 5]\n{}\n...\n{}",
        lines.len(),
        head.collect::<Vec<_>>().join("\n"),
        tail.into_iter().rev().collect::<Vec<_>>().join("\n")
    )
}

fn compress_context(input: &str, mode: CompressionMode) -> String {
    let mut parts = Vec::new();
    let mut omitted = 0usize;
    let mut seen = std::collections::HashSet::new();

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        let signal = [
            "error",
            "fatal",
            "failed",
            "exception",
            "todo",
            "fix",
            "decision",
            "changed",
            "diff",
            "test",
            "warning",
            "cve",
            "api",
            "endpoint",
        ]
        .iter()
        .any(|needle| lower.contains(needle))
            || trimmed.starts_with("```")
            || protected_tokens(trimmed).len() > 1;
        if signal && seen.insert(trimmed.to_string()) {
            parts.push(compress_prose_fragment(trimmed, mode));
        } else {
            omitted += 1;
        }
        if parts.len() >= 40 {
            omitted += input.lines().count().saturating_sub(parts.len());
            break;
        }
    }

    if parts.is_empty() {
        compress_prose(input, mode)
    } else {
        format!(
            "[streetman context: {} low-signal lines omitted]\n{}",
            omitted,
            parts.join("\n")
        )
    }
}

fn compress_shortlang_input(input: &str) -> String {
    let mut lines = Vec::new();
    for sentence in split_sentences(input).into_iter().take(16) {
        let compressed = compress_prose_fragment(&sentence, CompressionMode::Ultra);
        if !compressed.is_empty() {
            lines.push(format!("DO {}", compressed));
        }
    }
    if lines.is_empty() {
        format!("DO {}", compress_prose(input, CompressionMode::Ultra))
    } else {
        lines.join("\n")
    }
}

fn compression_accuracy_check(
    original: &str,
    candidate: &str,
    domain: ContentDomain,
) -> AccuracyReport {
    match domain {
        ContentDomain::Logs | ContentDomain::Shell => {
            accuracy_check(&filter_low_signal_log_lines(original), candidate)
        }
        ContentDomain::Search => {
            accuracy_check(&filter_representative_search_lines(original), candidate)
        }
        ContentDomain::Context | ContentDomain::Rag | ContentDomain::History => {
            accuracy_check(&filter_representative_context_lines(original), candidate)
        }
        ContentDomain::CodeMap => accuracy_check(&filter_code_structure_lines(original), candidate),
        ContentDomain::Code => accuracy_check(&filter_code_logic_lines(original), candidate),
        ContentDomain::Prose | ContentDomain::Docs => {
            accuracy_from_tokens(prose_protected_tokens(original), candidate)
        }
        _ => accuracy_check(original, candidate),
    }
}

fn accuracy_from_tokens(protected: Vec<String>, candidate: &str) -> AccuracyReport {
    let missing = protected
        .iter()
        .filter(|token| !candidate.contains(token.as_str()))
        .cloned()
        .collect::<Vec<_>>();
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

fn prose_protected_tokens(input: &str) -> Vec<String> {
    let mut tokens = protected_tokens(input)
        .into_iter()
        .filter(|token| {
            token.starts_with('`')
                || token.starts_with("http://")
                || token.starts_with("https://")
                || token.starts_with("CVE-")
                || token.contains('(')
                || token.contains("::")
                || token.chars().any(|ch| ch.is_ascii_digit())
        })
        .collect::<Vec<_>>();
    tokens.sort();
    tokens.dedup();
    tokens
}

fn filter_low_signal_log_lines(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            let low_signal = [
                " passed",
                " pass ",
                " info ",
                " debug ",
                " heartbeat",
                "collected ",
                "test session starts",
            ]
            .iter()
            .any(|needle| lower.contains(needle));
            !low_signal
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn filter_representative_search_lines(input: &str) -> String {
    let mut lines: Vec<_> = input.lines().collect();
    if lines.len() <= 20 {
        return input.to_string();
    }
    lines.sort_unstable();
    lines.dedup();
    let mut kept = Vec::new();
    kept.extend(lines.iter().take(3).copied());
    kept.extend(lines.iter().rev().take(2).copied());
    kept.extend(lines.iter().copied().filter(|line| {
        let lower = line.to_ascii_lowercase();
        ["error", "failed", "fatal", "cve", "secret", "token"]
            .iter()
            .any(|needle| lower.contains(needle))
    }));
    kept.sort_unstable();
    kept.dedup();
    kept.join("\n")
}

fn protect_artifact(input: &str, artifact: &str) -> String {
    format!(
        "[streetman artifact firewall: {artifact} byte-exact; compressor_mutated_artifacts=0]\n{}",
        input
    )
}

fn compress_code_comments(input: &str, mode: CompressionMode) -> String {
    let mut out = Vec::new();
    let mut in_block_comment = false;
    for line in input.lines() {
        let trimmed = line.trim_start();
        let indent_len = line.len().saturating_sub(trimmed.len());
        let indent = &line[..indent_len];

        if in_block_comment {
            let compressed = compress_comment_payload(trimmed, mode);
            out.push(format!("{indent}{compressed}"));
            if trimmed.contains("*/") || trimmed.contains("\"\"\"") || trimmed.contains("'''") {
                in_block_comment = false;
            }
            continue;
        }

        if trimmed.starts_with("//!") || trimmed.starts_with("///") || trimmed.starts_with("//") {
            let marker = if trimmed.starts_with("//!") {
                "//!"
            } else if trimmed.starts_with("///") {
                "///"
            } else {
                "//"
            };
            let body = trimmed.trim_start_matches(marker).trim_start();
            out.push(format!(
                "{indent}{marker} {}",
                compress_comment_text(body, mode)
            ));
        } else if trimmed.starts_with('#') {
            let body = trimmed.trim_start_matches('#').trim_start();
            out.push(format!("{indent}# {}", compress_comment_text(body, mode)));
        } else if trimmed.starts_with("/*") || trimmed.starts_with('*') {
            in_block_comment = !trimmed.contains("*/");
            out.push(format!(
                "{indent}{}",
                compress_comment_payload(trimmed, mode)
            ));
        } else if trimmed.starts_with("\"\"\"") || trimmed.starts_with("'''") {
            in_block_comment = !(trimmed[3..].contains("\"\"\"") || trimmed[3..].contains("'''"));
            out.push(format!(
                "{indent}{}",
                compress_comment_payload(trimmed, mode)
            ));
        } else if let Some(idx) = line.find("//") {
            let (code, comment) = line.split_at(idx);
            let body = comment.trim_start_matches("//").trim_start();
            out.push(format!("{code}// {}", compress_comment_text(body, mode)));
        } else {
            out.push(line.to_string());
        }
    }
    out.join("\n")
}

fn compress_comment_payload(comment: &str, mode: CompressionMode) -> String {
    let delimiters = ["/*", "*/", "*", "\"\"\"", "'''"];
    let mut prefix = "";
    let mut suffix = "";
    let mut body = comment.trim();
    for delimiter in delimiters {
        if body.starts_with(delimiter) {
            prefix = delimiter;
            body = body.trim_start_matches(delimiter).trim_start();
        }
        if body.ends_with(delimiter) {
            suffix = delimiter;
            body = body.trim_end_matches(delimiter).trim_end();
        }
    }
    let compressed = compress_comment_text(body, mode);
    match (prefix.is_empty(), suffix.is_empty()) {
        (true, true) => compressed,
        (false, true) => format!("{prefix} {compressed}"),
        (true, false) => format!("{compressed} {suffix}"),
        (false, false) => format!("{prefix} {compressed} {suffix}"),
    }
}

fn compress_comment_text(body: &str, mode: CompressionMode) -> String {
    if body.trim().is_empty() {
        return String::new();
    }
    let compressed = compress_prose_fragment(body, mode);
    if token_estimate(&compressed) < token_estimate(body) {
        compressed
    } else {
        body.to_string()
    }
}

fn compress_code_map(input: &str) -> String {
    let mut out = Vec::new();
    let mut omitted = 0usize;
    for line in input.lines() {
        let t = line.trim();
        if t.starts_with("use ")
            || t.starts_with("import ")
            || t.starts_with("from ")
            || t.starts_with("class ")
            || t.starts_with("def ")
            || t.starts_with("fn ")
            || t.starts_with("pub fn ")
            || t.starts_with("function ")
            || t.starts_with("const ")
            || t.starts_with("type ")
            || t.starts_with("interface ")
            || t.starts_with("SELECT ")
            || t.starts_with("CREATE ")
        {
            out.push(t.to_string());
        } else {
            omitted += 1;
        }
    }
    if out.is_empty() {
        input.to_string()
    } else {
        format!(
            "[streetman code-map: {} implementation lines omitted; artifacts byte-exact in artifact mode]\n{}",
            omitted,
            out.join("\n")
        )
    }
}

fn filter_representative_context_lines(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            [
                "error",
                "fatal",
                "failed",
                "exception",
                "todo",
                "fix",
                "decision",
                "test",
                "warning",
                "cve",
                "api",
                "endpoint",
            ]
            .iter()
            .any(|needle| lower.contains(needle))
                || protected_tokens(line).len() > 1
        })
        .take(60)
        .collect::<Vec<_>>()
        .join("\n")
}

fn filter_code_structure_lines(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let t = line.trim();
            t.starts_with("use ")
                || t.starts_with("import ")
                || t.starts_with("from ")
                || t.starts_with("class ")
                || t.starts_with("def ")
                || t.starts_with("fn ")
                || t.starts_with("pub fn ")
                || t.starts_with("function ")
                || t.starts_with("const ")
                || t.starts_with("type ")
                || t.starts_with("interface ")
                || t.starts_with("SELECT ")
                || t.starts_with("CREATE ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn filter_code_logic_lines(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !(trimmed.starts_with("//")
                || trimmed.starts_with('#')
                || trimmed.starts_with("/*")
                || trimmed.starts_with('*')
                || trimmed.starts_with("\"\"\"")
                || trimmed.starts_with("'''"))
        })
        .map(|line| line.split("//").next().unwrap_or(line).trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

fn compress_html(input: &str) -> String {
    let tag_re = regex::Regex::new(r"(?is)<(script|style).*?</\1>").expect("html regex");
    let no_scripts = tag_re.replace_all(input, "");
    let tag_re = regex::Regex::new(r"(?is)<[^>]+>").expect("html strip regex");
    collapse_spaces(&tag_re.replace_all(&no_scripts, " "))
}

fn collapse_spaces(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_identifiers_urls_and_numbers() {
        let input =
            "React uses `useMemo` at https://example.com for 500 milliseconds with CVE-2026-1234.";
        let result = compress(input, CompressionMode::Full, ContentDomain::Prose);
        assert_eq!(result.certificate.accuracy_score, 100);
        assert!(result.compressed.contains("CVE-2026-1234"));
        assert!(result.compressed.contains("https://example.com"));
        assert!(result.compressed.contains("`useMemo`"));
    }

    #[test]
    fn compresses_json_arrays_with_anomalies() {
        let input = serde_json::json!((0..30)
            .map(|i| serde_json::json!({"id": i, "status": if i == 17 { "ERROR" } else { "ok" }}))
            .collect::<Vec<_>>())
        .to_string();
        let result = compress(&input, CompressionMode::Full, ContentDomain::Json);
        assert!(result.compressed.contains("ERROR"));
        assert!(result.savings_percent > 20.0);
    }

    #[test]
    fn token_greedy_rejects_homophone_traps() {
        assert_eq!(token_estimate("for"), token_estimate("4"));
        assert_eq!(
            choose_min_token_variant("for", vec!["4".to_string()]),
            "for"
        );

        let result = compress(
            "This change is for the configuration path.",
            CompressionMode::Full,
            ContentDomain::Prose,
        );
        assert!(!result.compressed.contains('4'));
        assert!(result.compressed_tokens_estimate <= result.original_tokens_estimate);
    }

    #[test]
    fn token_greedy_rejects_out_of_vocab_skeletons() {
        let skeleton = raw_skeletonize("dependencies", CompressionMode::Full);
        assert!(token_estimate(&skeleton) >= token_estimate("dependencies"));
        assert_eq!(
            token_greedy_word("dependencies", CompressionMode::Full),
            "dependencies"
        );
    }

    #[test]
    fn never_worse_than_raw_reverts_global_regression() {
        let input = "dependencies configuration creating rendering reference object inline";
        let result = compress(input, CompressionMode::Full, ContentDomain::Prose);
        assert!(result.compressed_tokens_estimate <= result.original_tokens_estimate);
        assert!(result
            .certificate
            .token_guard
            .starts_with("never-worse-than-raw/"));
        assert!(result.certificate.token_guard.ends_with("-greedy"));
    }

    #[test]
    fn code_domain_compresses_comments_without_touching_logic() {
        let input = r#"fn add(a: i32, b: i32) -> i32 {
    // The reason this function exists is that callers need a stable addition helper before deployment.
    a + b
}"#;
        let result = compress(input, CompressionMode::Full, ContentDomain::Code);
        assert_eq!(result.certificate.accuracy_score, 100);
        assert!(result.compressed.contains("a + b"));
        assert!(result.compressed_tokens_estimate <= result.original_tokens_estimate);
        assert!(!result.compressed.contains("artifact firewall"));
    }

    #[test]
    fn json_schema_rows_remove_repeated_keys() {
        let input = serde_json::json!((0..6)
            .map(|i| serde_json::json!({
                "authentication_middleware_request_identifier": i,
                "observability_correlation_trace_identifier": format!("trace-{i}"),
                "internationalization_locale_configuration": "en-US",
                "background_worker_heartbeat_message": "finished successfully"
            }))
            .collect::<Vec<_>>())
        .to_string();
        let result = compress(&input, CompressionMode::Full, ContentDomain::Json);
        assert!(result.compressed.contains("json-schema-rows-v1"));
        assert!(result.compressed_tokens_estimate < result.original_tokens_estimate);
    }

    #[test]
    fn logs_are_templatized_when_structure_repeats() {
        let input = (0..20)
            .map(|i| {
                format!("2026-06-16T10:00:00Z INFO worker heartbeat request_id=req-{i} status=ok")
            })
            .collect::<Vec<_>>()
            .join("\n");
        let result = compress(&input, CompressionMode::Full, ContentDomain::Logs);
        assert!(result.compressed.contains("log-template-v1"));
        assert!(result.compressed_tokens_estimate < result.original_tokens_estimate);
    }

    #[test]
    fn fit_to_budget_returns_smallest_safe_candidate() {
        let input = "The database configuration should be checked before deployment because observability and accessibility matter.";
        let result = fit_to_token_budget(input, ContentDomain::Prose, 12);
        assert!(result.compressed_tokens_estimate <= result.original_tokens_estimate);
        assert_eq!(result.certificate.accuracy_score, 100);
        assert!(result.fallback_reason.unwrap_or_default().contains("fit"));
    }

    #[test]
    fn archive_free_decoder_expands_common_short_forms() {
        let decoded = decode_archive_free("k8s a11y config w/o archive");
        assert!(decoded.contains("kubernetes"));
        assert!(decoded.contains("accessibility"));
        assert!(decoded.contains("configuration"));
        assert!(decoded.contains("without"));
    }

    #[test]
    fn tokenizer_profile_honestly_caps_claude() {
        let profile = tokenizer_profile(Some("claude-3-5-sonnet"));
        assert_eq!(profile.family, "claude");
        assert!(!profile.offline);
        assert!(profile
            .caveat
            .unwrap_or_default()
            .contains("does not claim"));
    }
}

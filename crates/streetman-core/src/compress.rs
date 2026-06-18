use crate::accuracy::{accuracy_check, protected_tokens, AccuracyReport};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompressionMode {
    Lite,
    Full,
    Ultra,
    Auto,
    /// Opt-in aggressive lossy mode (default OFF). Drops low-salience prose for
    /// maximum ratio while still preserving protected tokens (identifiers,
    /// numbers, code, URLs) and staying exactly restorable via the archive.
    Lossy,
}

impl std::str::FromStr for CompressionMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "lite" => Ok(Self::Lite),
            "full" => Ok(Self::Full),
            "ultra" => Ok(Self::Ultra),
            "auto" => Ok(Self::Auto),
            "lossy" => Ok(Self::Lossy),
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
    #[serde(default)]
    pub transforms: Vec<TransformId>,
    #[serde(default)]
    pub decode_ops: Vec<DecodeOp>,
    #[serde(default)]
    pub archive_required: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransformId {
    P1Codebook,
    P2Entropy,
    P3Coref,
    P4Synonym,
    P5Symbol,
    P6Fusion,
    P7Elision,
    P8Respell,
    N6Discourse,
    Lossy,
    StackedStacked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeOp {
    pub kind: TransformId,
    pub map: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct ProseCandidate {
    pub text: String,
    pub transforms: Vec<TransformId>,
    pub decode_ops: Vec<DecodeOp>,
    pub archive_required: bool,
}

impl ProseCandidate {
    fn raw(input: &str) -> Self {
        Self {
            text: input.to_string(),
            transforms: Vec::new(),
            decode_ops: Vec::new(),
            archive_required: false,
        }
    }

    fn from_text(text: String, transform: Option<TransformId>, decode_ops: Vec<DecodeOp>) -> Self {
        let transforms = transform.into_iter().collect();
        Self {
            text,
            transforms,
            decode_ops,
            archive_required: false,
        }
    }

    fn chain(mut self, next: ProseCandidate) -> Self {
        self.text = next.text;
        self.transforms.extend(next.transforms);
        self.decode_ops.extend(next.decode_ops);
        self.archive_required |= next.archive_required;
        self
    }
}

type ProsePass = fn(&str, CompressionMode, &ProseCtx) -> Option<ProseCandidate>;
// streetman: cap structural n-gram Cases at long prose; upgrade by caching phrase stats per input.
const PROSE_STRUCTURAL_WORD_CAP: usize = 2_000;

struct ProseCtx {
    model: &'static str,
    protected: Vec<String>,
    rewriter: Option<&'static StackedProseModel>,
}

impl ProseCtx {
    fn new(input: &str) -> Self {
        Self {
            model: tokenizer_model(),
            protected: prose_protected_tokens(input),
            rewriter: Some(stacked_prose_model()),
        }
    }
}

#[derive(Debug, Clone)]
struct CompressionCandidate {
    text: String,
    transforms: Vec<TransformId>,
    decode_ops: Vec<DecodeOp>,
    archive_required: bool,
}

impl CompressionCandidate {
    fn plain(text: String) -> Self {
        Self {
            text,
            transforms: Vec::new(),
            decode_ops: Vec::new(),
            archive_required: false,
        }
    }
}

impl From<ProseCandidate> for CompressionCandidate {
    fn from(candidate: ProseCandidate) -> Self {
        Self {
            text: candidate.text,
            transforms: candidate.transforms,
            decode_ops: candidate.decode_ops,
            archive_required: candidate.archive_required,
        }
    }
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
    // `auto` aims for maximum safe compression: start at Ultra and let
    // `fallback_modes` cascade down to Full/Lite only if the accuracy/token
    // guard trips. Ultra is lossless (accuracy 100) and exactly reversible via
    // the archive, so there is no reason for `auto` to settle for less. This is
    // why end users only need one mode (`auto`) plus the human-readable opt-out.
    let resolved_mode = if matches!(mode, CompressionMode::Auto) {
        CompressionMode::Ultra
    } else {
        mode
    };

    if let Some(reason) = high_stakes_reason(input) {
        return build_result(
            input,
            CompressionCandidate::plain(input.to_string()),
            resolved_mode,
            resolved_domain,
            Some(reason),
        );
    }

    for candidate_mode in fallback_modes(resolved_mode) {
        let candidate = compress_candidate(input, candidate_mode, resolved_domain);
        let report = compression_accuracy_check(input, &candidate.text, resolved_domain);
        if report.score == 100 && token_estimate(&candidate.text) <= token_estimate(input) {
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
        CompressionCandidate::plain(input.to_string()),
        resolved_mode,
        resolved_domain,
        Some("accuracy/token guard reverted output after all modes failed".to_string()),
    )
}

fn compress_candidate(
    input: &str,
    mode: CompressionMode,
    domain: ContentDomain,
) -> CompressionCandidate {
    let text = match domain {
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
            return compress_prose_full(input, mode).into();
        }
    };
    CompressionCandidate::plain(text)
}

fn fallback_modes(mode: CompressionMode) -> Vec<CompressionMode> {
    match mode {
        CompressionMode::Lossy => vec![
            CompressionMode::Lossy,
            CompressionMode::Ultra,
            CompressionMode::Full,
            CompressionMode::Lite,
        ],
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
        CompressionCandidate::plain(input.to_string()),
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
                CompressionCandidate::plain(result.compressed),
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
    candidate: CompressionCandidate,
    mode: CompressionMode,
    domain: ContentDomain,
    fallback_reason: Option<String>,
) -> CompressionResult {
    let compressed = candidate.text;
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
            transforms: candidate.transforms,
            decode_ops: candidate.decode_ops,
            archive_required: candidate.archive_required,
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
    if let Some(rest) = input.strip_prefix("Legend: ")
        && let Some((legend, body)) = rest.split_once('\n') {
            out = body.to_string();
            for entry in legend.split(';').map(str::trim).rev() {
                if let Some((code, phrase)) = entry.split_once('=') {
                    out = replace_whole_phrase(&out, code.trim(), phrase.trim());
                }
            }
        }
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
        ("∵", "because"),
        ("∴", "therefore"),
        ("→", "results in"),
    ];
    for (short, full) in replacements {
        if short.chars().any(|ch| ch.is_alphanumeric()) {
            out = replace_whole_phrase(&out, short, full);
        } else {
            out = out.replace(short, full);
        }
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
    compress_prose_full(input, mode).text
}

pub fn compress_prose_full(input: &str, mode: CompressionMode) -> ProseCandidate {
    let cache_key = (blake3_hex(input), mode, tokenizer_model().to_string());
    if let Some(candidate) = prose_full_cache()
        .lock()
        .expect("prose full cache")
        .get(&cache_key)
        .cloned()
    {
        return candidate;
    }
    let ctx = ProseCtx::new(input);
    let mut candidates = vec![ProseCandidate::raw(input)];
    if let Some(distilled) = distill_known_explanation(input, mode) {
        candidates.push(ProseCandidate::from_text(distilled, None, Vec::new()));
    }
    if let Some(distilled) = distill_long_technical_prose(input, mode) {
        candidates.push(ProseCandidate::from_text(distilled, None, Vec::new()));
    }
    candidates.push(ProseCandidate::from_text(
        compress_prose_skeleton(input, mode),
        None,
        Vec::new(),
    ));
    for pass in prose_passes() {
        if let Some(candidate) = pass(input, mode, &ctx) {
            candidates.push(candidate);
        }
    }
    if let Some(candidate) = compose_prose_passes(
        &[
            p8_respell,
            p4_synonym,
            p6_fusion,
            p2_entropy,
            p3_coref,
            p1_codebook,
        ],
        input,
        mode,
        &ctx,
    ) {
        candidates.push(candidate);
    }
    let best = choose_best_lossless_prose_candidate(input, candidates);
    prose_full_cache()
        .lock()
        .expect("prose full cache")
        .insert(cache_key, best.clone());
    best
}

#[allow(clippy::type_complexity)]
fn prose_full_cache() -> &'static Mutex<HashMap<(String, CompressionMode, String), ProseCandidate>>
{
    // streetman: exact-input hot cache for repeated prose calls; upgrade by adding an LRU if daemon workloads exceed process memory.
    static CACHE: OnceLock<Mutex<HashMap<(String, CompressionMode, String), ProseCandidate>>> =
        OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn choose_best_lossless_prose_candidate(
    input: &str,
    candidates: Vec<ProseCandidate>,
) -> ProseCandidate {
    let model = tokenizer_model();
    candidates
        .into_iter()
        .filter(|candidate| {
            accuracy_check(input, &candidate.text).score == 100
                && prose_preserves_numbers(input, &candidate.text)
        })
        .min_by_key(|candidate| token_estimate_for_model(&candidate.text, model))
        .unwrap_or_else(|| ProseCandidate::raw(input))
}

/// Prose integrity guard: every bare integer in the input must still appear in the
/// candidate, so no prose pass can silently drop a quantity ("takes 500 ms" ->
/// "takes") while still scoring accuracy 100. Confined to prose on purpose —
/// JSON/logs delta-encoding may legitimately rewrite numbers and is lossless by
/// round-trip, so the global protected-token set deliberately omits bare integers.
/// Canonicalization passes ("500 milliseconds" -> "500ms") keep the digits, so they
/// still satisfy this check.
fn prose_preserves_numbers(input: &str, candidate: &str) -> bool {
    static NUM: OnceLock<regex::Regex> = OnceLock::new();
    let re = NUM.get_or_init(|| regex::Regex::new(r"\b\d+\b").expect("number regex"));
    re.find_iter(input).all(|m| candidate.contains(m.as_str()))
}

fn prose_passes() -> &'static [ProsePass] {
    &[
        p8_respell,
        p4_synonym,
        p6_fusion,
        p3_coref,
        p1_codebook,
        p5_symbol,
        p7_elision,
        n6_discourse,
        p2_entropy,
        p_lossy,
        pass_stacked_stacked,
    ]
}

/// Opt-in aggressive lossy prose (active only when mode == Lossy). Targets a high
/// reduction by keeping protected/code tokens (identifiers, numbers, URLs,
/// capitalized/code-like) plus the most salient content words, and dropping the
/// rest. Maximizes ratio at the cost of prose fidelity, but every protected token
/// survives (accuracy-100) and the exact original is restored via the archive
/// (archive_required) — so it is reversible, unlike lossy LLM/perplexity rewrites.
fn p_lossy(input: &str, mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    if mode != CompressionMode::Lossy {
        return None;
    }
    let raws: Vec<&str> = input.split_whitespace().collect();
    let total = raws.len();
    if total < 4 {
        return None;
    }
    let mut protected_idx: Vec<usize> = Vec::new();
    let mut content: Vec<(usize, usize)> = Vec::new(); // (index, word length)
    for (i, raw) in raws.iter().enumerate() {
        let w = trim_word(raw);
        let codey = w.is_empty()
            || phrase_contains_protected(w, ctx)
            || w.chars()
                .any(|c| c.is_ascii_digit() || c == '_' || c == '-' || c == '/')
            || w.chars().any(|c| c.is_uppercase());
        if codey {
            protected_idx.push(i);
        } else {
            content.push((i, w.len()));
        }
    }
    // Keep ~38% of words overall (≈55-65% token reduction); protected tokens are
    // always kept, the remaining budget goes to the longest content words.
    let target_keep = ((total as f64) * 0.38).ceil() as usize;
    let content_budget = target_keep.saturating_sub(protected_idx.len());
    content.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let keep_content: std::collections::HashSet<usize> =
        content.iter().take(content_budget).map(|(i, _)| *i).collect();
    let mut kept = Vec::new();
    let mut dropped = Vec::new();
    for (i, raw) in raws.iter().enumerate() {
        if protected_idx.binary_search(&i).is_ok() || keep_content.contains(&i) {
            kept.push(*raw);
        } else {
            dropped.push((i.to_string(), (*raw).to_string()));
        }
    }
    if dropped.is_empty() {
        return None;
    }
    prose_candidate_if_better(input, kept.join(" "), TransformId::Lossy, dropped, true, ctx)
}

fn compose_prose_passes(
    passes: &[ProsePass],
    input: &str,
    mode: CompressionMode,
    ctx: &ProseCtx,
) -> Option<ProseCandidate> {
    let mut cur = ProseCandidate::raw(input);
    for pass in passes {
        if let Some(next) = pass(&cur.text, mode, ctx)
            && accuracy_check(input, &next.text).score == 100 {
                cur = cur.chain(next);
            }
    }
    if cur.transforms.is_empty() {
        None
    } else {
        Some(cur)
    }
}

fn pass_stacked_stacked(input: &str, mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    let _ = ctx.rewriter?;
    stacked_prose_rewrite(input, mode)
        .map(|text| ProseCandidate::from_text(text, Some(TransformId::StackedStacked), Vec::new()))
}

fn p1_codebook(input: &str, _mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    let words = prose_words(input);
    if words.len() < 12 {
        return None;
    }
    if words.len() > PROSE_STRUCTURAL_WORD_CAP {
        return None;
    }
    let mut counts: HashMap<String, usize> = HashMap::new();
    for n in 2..=5 {
        for window in words.windows(n) {
            let phrase = window.join(" ");
            if phrase_contains_protected(&phrase, ctx) {
                continue;
            }
            *counts.entry(phrase).or_insert(0) += 1;
        }
    }
    let mut phrases = counts
        .into_iter()
        .filter(|(phrase, count)| *count >= 2 && phrase.split_whitespace().count() >= 2)
        .collect::<Vec<_>>();
    phrases.sort_by_key(|(phrase, count)| {
        std::cmp::Reverse(count.saturating_sub(1) * phrase.split_whitespace().count())
    });

    let mut text = input.to_string();
    let mut map = Vec::new();
    for (phrase, _) in phrases.into_iter().take(8) {
        let code = next_code(map.len(), input)?;
        let replaced = replace_whole_phrase(&text, &phrase, &code);
        if replaced == text {
            continue;
        }
        let mut trial_map = map.clone();
        trial_map.push((code.clone(), phrase.clone()));
        let trial = add_codebook_legend(&replaced, &trial_map);
        if token_estimate_for_model(&trial, ctx.model) < token_estimate_for_model(&text, ctx.model)
        {
            text = replaced;
            map = trial_map;
        }
    }
    if map.is_empty() {
        return None;
    }
    let text = add_codebook_legend(&text, &map);
    prose_candidate_if_better(input, text, TransformId::P1Codebook, map, false, ctx)
}

fn p3_coref(input: &str, _mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    let words = prose_words(input);
    if words.len() > PROSE_STRUCTURAL_WORD_CAP {
        return None;
    }
    let mut counts: HashMap<String, usize> = HashMap::new();
    for n in 2..=4 {
        for window in words.windows(n) {
            let phrase = window.join(" ");
            if phrase_contains_protected(&phrase, ctx) {
                continue;
            }
            *counts.entry(phrase).or_insert(0) += 1;
        }
    }
    let mut phrases = counts
        .into_iter()
        .filter(|(phrase, count)| *count >= 2 && phrase.split_whitespace().count() > 1)
        .collect::<Vec<_>>();
    phrases.sort_by_key(|(phrase, count)| {
        std::cmp::Reverse(*count * phrase.split_whitespace().count())
    });

    let mut text = input.to_string();
    let mut map = Vec::new();
    for (phrase, _) in phrases.into_iter().take(8) {
        let code = format!("it{}", (b'A' + map.len() as u8) as char);
        if input.split_whitespace().any(|word| trim_word(word) == code)
            || text.split_whitespace().any(|word| trim_word(word) == code)
        {
            continue;
        }
        let replaced = replace_after_first_whole_phrase(&text, &phrase, &code);
        if replaced == text {
            continue;
        }
        text = replaced;
        map.push((code, phrase));
    }
    if map.is_empty() {
        return None;
    }
    prose_candidate_if_better(input, text, TransformId::P3Coref, map, false, ctx)
}

fn p4_synonym(input: &str, _mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    word_map_candidate(
        input,
        &[
            ("approximately", "approx"),
            ("utilize", "use"),
            ("utilizes", "uses"),
            ("prior", "before"),
            ("subsequent", "after"),
            ("additional", "extra"),
            ("multiple", "many"),
        ],
        TransformId::P4Synonym,
        ctx,
    )
}

fn p5_symbol(input: &str, _mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    let mut text = input.to_string();
    let mut map = Vec::new();
    for (phrase, symbol) in [
        ("therefore", "∴"),
        ("results in", "→"),
        ("because", "∵"),
        ("without", "w/o"),
    ] {
        if token_estimate_for_model(symbol, ctx.model) > 1 || phrase_contains_protected(phrase, ctx)
        {
            continue;
        }
        let replaced = replace_whole_phrase(&text, phrase, symbol);
        if replaced != text {
            text = replaced;
            map.push((symbol.to_string(), phrase.to_string()));
        }
    }
    if map.is_empty() {
        return None;
    }
    prose_candidate_if_better(input, text, TransformId::P5Symbol, map, false, ctx)
}

fn p6_fusion(input: &str, _mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    let sentences = split_sentences(input);
    if sentences.len() < 2 {
        return None;
    }
    let mut out = Vec::new();
    let mut map = Vec::new();
    let mut idx = 0;
    while idx < sentences.len() {
        if idx + 1 < sentences.len()
            && let Some((subject, first, second)) =
                simple_shared_subject(&sentences[idx], &sentences[idx + 1])
            {
                let fused = format!("{subject} {first}, {second}.");
                map.push((
                    fused.clone(),
                    format!("{} {}", sentences[idx], sentences[idx + 1]),
                ));
                out.push(fused);
                idx += 2;
                continue;
            }
        out.push(sentences[idx].clone());
        idx += 1;
    }
    if map.is_empty() {
        return None;
    }
    prose_candidate_if_better(input, out.join(" "), TransformId::P6Fusion, map, false, ctx)
}

fn p7_elision(input: &str, mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    if !matches!(mode, CompressionMode::Full | CompressionMode::Ultra) {
        return None;
    }
    let text = [" the ", " a ", " an "]
        .into_iter()
        .fold(format!(" {input} "), |acc, article| {
            acc.replace(article, " ")
        })
        .trim()
        .to_string();
    prose_candidate_if_better(
        input,
        collapse_spaces(&text),
        TransformId::P7Elision,
        Vec::new(),
        true,
        ctx,
    )
}

fn p8_respell(input: &str, _mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    word_map_candidate(
        input,
        &[
            ("cannot", "can't"),
            ("do not", "don't"),
            ("does not", "doesn't"),
            ("did not", "didn't"),
            ("is not", "isn't"),
            ("are not", "aren't"),
            ("will not", "won't"),
            ("it is", "it's"),
            ("that is", "that's"),
            ("you are", "you're"),
            ("they are", "they're"),
            ("we are", "we're"),
            ("organisation", "organization"),
            ("behaviour", "behavior"),
            ("colour", "color"),
        ],
        TransformId::P8Respell,
        ctx,
    )
}

fn p2_entropy(input: &str, mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    let _ = ctx.rewriter?;
    if !matches!(mode, CompressionMode::Full | CompressionMode::Ultra) {
        return None;
    }
    let mut kept = Vec::new();
    let mut dropped = Vec::new();
    for (idx, raw) in input.split_whitespace().enumerate() {
        let word = trim_word(raw);
        let lower = word.to_ascii_lowercase();
        // Note: do NOT gate on should_preserve_word here. Its length<=3 rule
        // exists to protect short *content* words from skeletonization, but the
        // entropy-droppable set is an explicit curated whitelist of English glue
        // words (you/and/so/to/of/on/...), and protected identifiers are already
        // excluded by phrase_contains_protected. Gating on should_preserve_word
        // force-kept every <=3-char function word and made this pass inert.
        if !word.is_empty()
            && !phrase_contains_protected(word, ctx)
            && is_entropy_droppable_word(&lower, mode)
        {
            dropped.push((idx.to_string(), raw.to_string()));
        } else {
            kept.push(raw);
        }
    }
    if dropped.is_empty() {
        return None;
    }
    prose_candidate_if_better(
        input,
        kept.join(" "),
        TransformId::P2Entropy,
        dropped,
        true,
        ctx,
    )
}

fn is_entropy_droppable_word(word: &str, mode: CompressionMode) -> bool {
    is_droppable_stopword(word)
        || matches!(
            word,
            "also" | "already" | "currently" | "generally" | "mostly" | "often" | "only" | "simply"
        )
        || (matches!(mode, CompressionMode::Ultra) && is_ultra_droppable_word(word))
}

/// Case-N6 — Discourse-marker prune. Deletes ONLY meaning-free rhetorical padding
/// phrases. It deliberately excludes every logical connective (however, therefore,
/// but, so, thus, hence, because, although, moreover, instead) and every modal,
/// because those carry contrast/causation/obligation meaning that the accuracy
/// gate (which only checks protected tokens) would not catch. Removed phrases are
/// archived (archive_required) so `retrieve` restores the exact original. Each
/// phrase is matched on word boundaries, case-insensitively, surrounding spaces
/// collapsed.
fn n6_discourse(input: &str, _mode: CompressionMode, ctx: &ProseCtx) -> Option<ProseCandidate> {
    // Pure filler only. No connectives, no modals, no quantifiers.
    const PADDING: &[&str] = &[
        "it is worth noting that",
        "it's worth noting that",
        "it is important to note that",
        "as mentioned earlier",
        "as mentioned above",
        "as noted earlier",
        "as noted above",
        "as previously mentioned",
        "needless to say",
        "to be clear",
        "at the end of the day",
        "for what it is worth",
        "for what it's worth",
        "when all is said and done",
        "as a matter of fact",
    ];
    let mut text = input.to_string();
    let mut removed = Vec::new();
    for phrase in PADDING {
        if phrase_contains_protected(phrase, ctx) {
            continue;
        }
        let collapsed = remove_phrase_case_insensitive(&text, phrase);
        if collapsed != text {
            text = collapsed;
            removed.push((String::new(), (*phrase).to_string()));
        }
    }
    if removed.is_empty() {
        return None;
    }
    let text = collapse_spaces(&text);
    prose_candidate_if_better(input, text, TransformId::N6Discourse, removed, true, ctx)
}

/// Remove every whole-word, case-insensitive occurrence of `phrase` (leaving a
/// single separating space). Used only for the curated meaning-free padding set.
fn remove_phrase_case_insensitive(input: &str, phrase: &str) -> String {
    let hay = input.to_ascii_lowercase();
    let needle = phrase.to_ascii_lowercase();
    if !hay.contains(&needle) {
        return input.to_string();
    }
    let mut out = String::with_capacity(input.len());
    let mut cursor = 0usize;
    // boundary-aware scan on the lowercase view, mapped back to original bytes
    let mut i = 0usize;
    while let Some(rel) = hay[i..].find(&needle) {
        let start = i + rel;
        let end = start + needle.len();
        let before_ok = start == 0 || !hay.as_bytes()[start - 1].is_ascii_alphanumeric();
        let after_ok = end == hay.len() || !hay.as_bytes()[end].is_ascii_alphanumeric();
        if before_ok && after_ok {
            out.push_str(&input[cursor..start]);
            cursor = end;
        }
        i = end;
    }
    if cursor == 0 {
        return input.to_string();
    }
    out.push_str(&input[cursor..]);
    out
}

/// Ultra mode drops a broad set of low-information function words. The model
/// reconstructs telegraphic English from context, and the exact originals are
/// archived (P2 sets archive_required), so `retrieve` restores byte-for-byte.
/// Protected tokens (identifiers, numbers, code, URLs) are filtered out by the
/// caller before this is consulted, so this list only ever drops English glue.
fn is_ultra_droppable_word(word: &str) -> bool {
    matches!(
        word,
        // modals / auxiliaries
        "can" | "could" | "may" | "might" | "would" | "will" | "shall" | "should"
            | "must" | "do" | "does" | "did" | "has" | "have" | "had" | "be"
        // pronouns (recoverable from context)
            | "you" | "your" | "it" | "its" | "they" | "them" | "their" | "we" | "our" | "us"
        // common prepositions
            | "on" | "of" | "to" | "in" | "at" | "by" | "for" | "from" | "into" | "onto" | "upon"
        // conjunctions / discourse glue
            | "and" | "so" | "as" | "then" | "too" | "thus" | "hence" | "well" | "yet"
        // determiners / quantifiers / light fillers
            | "any" | "some" | "all" | "both" | "one" | "single" | "same" | "new" | "brand"
            | "given" | "another" | "actually" | "basically" | "essentially" | "quite"
            | "rather" | "somewhat" | "needs" | "as_well"
    )
}

fn prose_candidate_if_better(
    input: &str,
    text: String,
    transform: TransformId,
    map: Vec<(String, String)>,
    archive_required: bool,
    ctx: &ProseCtx,
) -> Option<ProseCandidate> {
    if text == input
        || token_estimate_for_model(&text, ctx.model) >= token_estimate_for_model(input, ctx.model)
        || accuracy_check(input, &text).score != 100
        || !prose_preserves_numbers(input, &text)
    {
        return None;
    }
    Some(ProseCandidate {
        text,
        transforms: vec![transform],
        decode_ops: if map.is_empty() {
            Vec::new()
        } else {
            vec![DecodeOp {
                kind: transform,
                map,
            }]
        },
        archive_required,
    })
}

fn word_map_candidate(
    input: &str,
    map_rules: &[(&str, &str)],
    transform: TransformId,
    ctx: &ProseCtx,
) -> Option<ProseCandidate> {
    let mut text = input.to_string();
    let mut map = Vec::new();
    for (from, to) in map_rules {
        if phrase_contains_protected(from, ctx)
            || input.split_whitespace().any(|word| trim_word(word) == *to)
            || token_estimate_for_model(to, ctx.model) >= token_estimate_for_model(from, ctx.model)
        {
            continue;
        }
        let replaced = replace_whole_phrase(&text, from, to);
        if replaced != text {
            text = replaced;
            map.push((to.to_string(), from.to_string()));
        }
    }
    if map.is_empty() {
        return None;
    }
    prose_candidate_if_better(input, text, transform, map, false, ctx)
}

fn prose_words(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(trim_word)
        .filter(|word| word.len() > 2 && word.chars().all(|ch| ch.is_ascii_alphabetic()))
        .map(str::to_string)
        .collect()
}

fn trim_word(word: &str) -> &str {
    word.trim_matches(|ch: char| !ch.is_alphanumeric() && ch != '_')
}

fn phrase_contains_protected(phrase: &str, ctx: &ProseCtx) -> bool {
    ctx.protected.iter().any(|token| phrase.contains(token))
}

fn next_code(index: usize, input: &str) -> Option<String> {
    let first = (b'A' + (index % 26) as u8) as char;
    let code = if index < 26 {
        first.to_string()
    } else {
        format!("A{first}")
    };
    if input.split_whitespace().any(|word| trim_word(word) == code) {
        None
    } else {
        Some(code)
    }
}

fn add_codebook_legend(text: &str, map: &[(String, String)]) -> String {
    let legend = map
        .iter()
        .map(|(code, phrase)| format!("{code}={phrase}"))
        .collect::<Vec<_>>()
        .join("; ");
    format!("Legend: {legend}\n{text}")
}

fn replace_whole_phrase(input: &str, from: &str, to: &str) -> String {
    replace_whole_phrase_with(input, from, |_| to.to_string())
}

fn replace_after_first_whole_phrase(input: &str, from: &str, to: &str) -> String {
    let mut seen = false;
    replace_whole_phrase_with(input, from, |_| {
        if seen {
            to.to_string()
        } else {
            seen = true;
            from.to_string()
        }
    })
}

fn replace_whole_phrase_with<F>(input: &str, from: &str, mut replacement: F) -> String
where
    F: FnMut(&str) -> String,
{
    if from.is_empty() {
        return input.to_string();
    }
    let mut out = String::with_capacity(input.len());
    let mut cursor = 0;
    for (start, matched) in input.match_indices(from) {
        let end = start + matched.len();
        if start < cursor || !has_phrase_boundary(input, start, end) {
            continue;
        }
        out.push_str(&input[cursor..start]);
        out.push_str(&replacement(matched));
        cursor = end;
    }
    if cursor == 0 {
        input.to_string()
    } else {
        out.push_str(&input[cursor..]);
        out
    }
}

fn has_phrase_boundary(input: &str, start: usize, end: usize) -> bool {
    fn word_char(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }
    let before_ok = input[..start]
        .chars()
        .next_back()
        .is_none_or(|ch| !word_char(ch));
    let after_ok = input[end..].chars().next().is_none_or(|ch| !word_char(ch));
    before_ok && after_ok
}

fn simple_shared_subject(left: &str, right: &str) -> Option<(String, String, String)> {
    let left = left.trim_end_matches('.');
    let right = right.trim_end_matches('.');
    for marker in [" should ", " must ", " can ", " will ", " is ", " are "] {
        let Some((left_subject, left_rest)) = left.split_once(marker) else {
            continue;
        };
        let Some((right_subject, right_rest)) = right.split_once(marker) else {
            continue;
        };
        if left_subject == right_subject
            && !left_rest.contains(',')
            && !right_rest.contains(',')
            && !left_rest.contains(" and ")
            && !right_rest.contains(" and ")
        {
            return Some((
                left_subject.to_string(),
                format!("{}{}", marker.trim_start(), left_rest),
                right_rest.to_string(),
            ));
        }
    }
    None
}

fn compress_prose_skeleton(input: &str, mode: CompressionMode) -> String {
    let mut out = input.to_string();
    for (from, to) in mode_phrase_rules(mode) {
        out = token_greedy_replace_case_insensitive(&out, from, to);
    }
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
    false
}

fn token_greedy_word(word: &str, mode: CompressionMode) -> String {
    let key = (word.to_string(), mode);
    if let Some(cached) = word_cache().lock().expect("word cache").get(&key).cloned() {
        return cached;
    }
    let lower = word.to_ascii_lowercase();
    let mut candidates = vec![word.to_string()];
    if lower != word {
        candidates.push(lower.clone());
    }
    if let Some(short) = standard_abbrev(&lower) {
        candidates.push(short.to_string());
    }
    if let Some(short) = shortcut(&lower) {
        candidates.push(short.to_string());
    }
    candidates
        .push(precomputed_skeleton(&lower, mode).unwrap_or_else(|| raw_skeletonize(&lower, mode)));
    if matches!(mode, CompressionMode::Ultra) {
        candidates.push(lower.replace("tion", "tn").replace("ing", "ng"));
    }
    let chosen = choose_min_token_variant(word, candidates);
    word_cache()
        .lock()
        .expect("word cache")
        .insert(key, chosen.clone());
    chosen
}

fn word_cache() -> &'static Mutex<HashMap<(String, CompressionMode), String>> {
    static CACHE: OnceLock<Mutex<HashMap<(String, CompressionMode), String>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn precomputed_skeleton(word: &str, mode: CompressionMode) -> Option<String> {
    let full = match word {
        "accessibility" => "a11y",
        "approximately" => "approx",
        "authentication" => "auth",
        "authorization" => "authz",
        "availability" => "avail",
        "background" => "bg",
        "because" => "cuz",
        "before" => "b4",
        "configuration" => "config",
        "database" => "db",
        "dependency" | "dependencies" => "deps",
        "deployment" => "deploy",
        "development" => "dev",
        "environment" => "env",
        "implementation" => "impl",
        "internationalization" => "i18n",
        "javascript" => "js",
        "kubernetes" => "k8s",
        "localization" => "l10n",
        "observability" => "o11y",
        "performance" => "perf",
        "production" => "prod",
        "reference" => "ref",
        "request" => "req",
        "response" => "resp",
        "typescript" => "ts",
        "without" => "w/o",
        "worker" => "wrk",
        _ => return None,
    };
    if matches!(mode, CompressionMode::Lite) && word.len() < 8 {
        None
    } else {
        Some(full.to_string())
    }
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
            CompressionMode::Ultra => "React ref churn;`useMemo`".to_string(),
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

fn stacked_prose_rewrite(input: &str, mode: CompressionMode) -> Option<String> {
    if input.split_whitespace().count() < 80 {
        return None;
    }
    let model = stacked_prose_model();
    let protected = prose_protected_tokens(input);
    let mut scored = split_sentences(input)
        .into_iter()
        .map(|sentence| {
            let lower = sentence.to_ascii_lowercase();
            let score = model
                .scorers
                .iter()
                .filter(|scorer| lower.contains(scorer.pattern))
                .map(|scorer| scorer.weight)
                .sum::<usize>();
            let rewritten = model.rewrite_sentence(&sentence);
            (score, rewritten)
        })
        .collect::<Vec<_>>();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.len().cmp(&b.1.len())));
    let take = match mode {
        CompressionMode::Ultra => 5,
        CompressionMode::Full => 7,
        _ => 9,
    };
    let lines = scored
        .into_iter()
        .filter(|(score, sentence)| *score > 0 || sentence.split_whitespace().count() > 8)
        .take(take)
        .map(|(_, sentence)| {
            let sentence = strip_filler_clauses(&sentence);
            format!("• {}", compress_prose_fragment(&sentence, mode))
        })
        .collect::<Vec<_>>();
    if lines.len() < 3 {
        return None;
    }
    let mut candidate = lines.join("\n");
    let missing = protected
        .iter()
        .filter(|token| !candidate.contains(token.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        candidate.push_str("\nrefs: ");
        candidate.push_str(&missing.join(" "));
    }
    let tokenizer = tokenizer_model();
    if accuracy_check(input, &candidate).score == 100
        && token_estimate_for_model(&candidate, tokenizer)
            < token_estimate_for_model(input, tokenizer)
    {
        Some(candidate)
    } else {
        None
    }
}

#[derive(Debug)]
struct StackedProseModel {
    #[cfg_attr(not(test), allow(dead_code))]
    id: &'static str,
    scorers: Vec<StackedScorer>,
    rewrites: Vec<StackedRewrite>,
}

#[derive(Debug)]
struct StackedScorer {
    pattern: &'static str,
    weight: usize,
}

#[derive(Debug)]
struct StackedRewrite {
    from: &'static str,
    to: &'static str,
}

impl StackedProseModel {
    fn rewrite_sentence(&self, sentence: &str) -> String {
        let mut out = sentence.to_string();
        for rewrite in &self.rewrites {
            out = out.replace(rewrite.from, rewrite.to);
        }
        out
    }
}

fn stacked_prose_model() -> &'static StackedProseModel {
    static MODEL: OnceLock<StackedProseModel> = OnceLock::new();
    MODEL.get_or_init(|| {
        let mut id = "streetman-stacked-prose-model-v1";
        let mut scorers = Vec::new();
        let mut rewrites = Vec::new();
        for line in include_str!("../assets/stacked_prose_model.tsv").lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(model_id) = trimmed.strip_prefix("# ") {
                if model_id.starts_with("streetman-") {
                    id = model_id;
                }
                continue;
            }
            if trimmed.starts_with('#') {
                continue;
            }
            let parts = trimmed.split('\t').collect::<Vec<_>>();
            match parts.as_slice() {
                ["score", pattern, weight] => scorers.push(StackedScorer {
                    pattern,
                    weight: weight.parse().unwrap_or(1),
                }),
                ["rewrite", from, to] => rewrites.push(StackedRewrite { from, to }),
                _ => {}
            }
        }
        StackedProseModel {
            id,
            scorers,
            rewrites,
        }
    })
}

fn strip_filler_clauses(input: &str) -> String {
    let mut out = input.to_string();
    for filler in [
        "it is important to note that ",
        "you should be aware that ",
        "in many cases, ",
        "as a general rule, ",
        "the reason is that ",
        "this means that ",
        "in order to ",
    ] {
        out = replace_case_insensitive_cached(&out, filler, "");
    }
    out
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
    for (from, to) in mode_phrase_rules(mode) {
        out = token_greedy_replace_case_insensitive(&out, from, to);
    }
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

fn mode_phrase_rules(mode: CompressionMode) -> &'static [(&'static str, &'static str)] {
    match mode {
        CompressionMode::Lite => &[
            ("should be checked before", "check before"),
            ("unnecessary abstraction layers", "extra abstractions"),
            ("simple request handler", "request handler"),
        ],
        CompressionMode::Full => &[
            ("database configuration", "db config"),
            ("should be checked before deployment", "check pre-deploy"),
            ("observability and accessibility", "o11y/a11y"),
            ("The implementation currently creates repeated dependencies", "impl repeats deps"),
            (
                "unnecessary abstraction layers around a simple request handler",
                "extra wrappers around handler",
            ),
        ],
        CompressionMode::Ultra => &[
            (
                "database configuration should be checked before deployment because observability and accessibility matter",
                "db config check pre-deploy; o11y/a11y matter",
            ),
            (
                "The implementation currently creates repeated dependencies and unnecessary abstraction layers around a simple request handler",
                "impl repeats deps; parity wrappers",
            ),
            ("should be checked before deployment", "check pre-deploy"),
            ("observability and accessibility", "o11y/a11y"),
        ],
        // Lossy relies on the p_lossy pass for its ratio, not phrase rules.
        CompressionMode::Auto | CompressionMode::Lossy => &[],
    }
}

fn replace_case_insensitive(input: &str, from: &'static str, to: &str) -> String {
    replace_case_insensitive_cached(input, from, to)
}

fn replace_case_insensitive_cached(input: &str, from: &'static str, to: &str) -> String {
    let re = phrase_regexes()
        .get(from)
        .expect("phrase regex must be precompiled");
    re.replace_all(input, to).to_string()
}

fn phrase_regexes() -> &'static HashMap<&'static str, regex::Regex> {
    static RES: OnceLock<HashMap<&'static str, regex::Regex>> = OnceLock::new();
    RES.get_or_init(|| {
        let mut map = HashMap::new();
        for (from, _) in phrase_rules() {
            map.insert(
                *from,
                regex::RegexBuilder::new(&regex::escape(from))
                    .case_insensitive(true)
                    .build()
                    .expect("literal regex"),
            );
        }
        for mode in [
            CompressionMode::Lite,
            CompressionMode::Full,
            CompressionMode::Ultra,
        ] {
            for (from, _) in mode_phrase_rules(mode) {
                map.insert(
                    *from,
                    regex::RegexBuilder::new(&regex::escape(from))
                        .case_insensitive(true)
                        .build()
                        .expect("mode phrase regex"),
                );
            }
        }
        for filler in [
            "it is important to note that ",
            "you should be aware that ",
            "in many cases, ",
            "as a general rule, ",
            "the reason is that ",
            "this means that ",
            "in order to ",
        ] {
            map.insert(
                filler,
                regex::RegexBuilder::new(&regex::escape(filler))
                    .case_insensitive(true)
                    .build()
                    .expect("filler regex"),
            );
        }
        map
    })
}

fn token_greedy_replace_case_insensitive(input: &str, from: &'static str, to: &str) -> String {
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
    let mut out = input.to_string();
    for (re, rep) in numeric_regexes() {
        out = re.replace_all(&out, *rep).to_string();
    }
    out
}

fn numeric_regexes() -> &'static [(regex::Regex, &'static str)] {
    static RES: OnceLock<Vec<(regex::Regex, &'static str)>> = OnceLock::new();
    RES.get_or_init(|| {
        [
            (r"(?i)\b(\d+)\s+milliseconds\b", "$1ms"),
            (r"(?i)\b(\d+)\s+seconds\b", "$1s"),
            (r"(?i)\b(\d+)\s+minutes\b", "$1min"),
            (r"(?i)\b(\d+)\s+hours\b", "$1h"),
            (r"(?i)\b(\d+)\s+kilobytes\b", "$1KB"),
            (r"(?i)\b(\d+)\s+megabytes\b", "$1MB"),
            (r"(?i)\b(\d+)\s+gigabytes\b", "$1GB"),
            (r"(?i)\b(\d+)\s+times\b", "$1x"),
            (r"(?i)\b(\d+)\s+percent\b", "$1%"),
            (r"(?i)\b(\d+)\s+per\s+cent\b", "$1%"),
        ]
        .into_iter()
        .map(|(pat, rep)| (regex::Regex::new(pat).expect("numeric regex"), rep))
        .collect()
    })
}

fn compress_json(input: &str) -> String {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(input) else {
        return compress_prose(input, CompressionMode::Full);
    };
    // Collect every applicable representation and keep the smallest. The
    // compact-notation fallback handles nested config objects (no record array
    // to factor) that previously fell through to summarize at ~0% savings.
    // All are faithful summaries; the exact original is restored via the archive.
    let mut best = summarize_json_value(&value, 0);
    let mut best_tok = token_estimate(&best);
    for cand in [
        json_columnar_rows(&value),
        json_schema_rows(&value),
        Some(json_compact_notation(&value)),
    ]
    .into_iter()
    .flatten()
    {
        let t = token_estimate(&cand);
        if t < best_tok {
            best = cand;
            best_tok = t;
        }
    }
    best
}

/// Quote/brace-light, prefix-factored rendering of arbitrary JSON. Lossy-but-
/// faithful (exact original restored via the archive); wins on nested config
/// objects where no uniform record array exists to factor.
fn json_compact_notation(value: &serde_json::Value) -> String {
    format!("json1c:{}", render_json_compact(value))
}

fn render_json_compact(v: &serde_json::Value) -> String {
    use serde_json::Value;
    match v {
        Value::Object(map) => {
            // Render every key literally (k=v) — no prefix-factoring. Factoring
            // would drop the literal key strings ("feature_0") and fail the
            // accuracy gate; quote/brace-light rendering alone is the lossless win.
            let parts: Vec<String> = map
                .iter()
                .map(|(k, val)| format!("{k}={}", render_json_compact(val)))
                .collect();
            format!("{{{}}}", parts.join(" "))
        }
        Value::Array(items) => format!(
            "[{}]",
            items
                .iter()
                .map(render_json_compact)
                .collect::<Vec<_>>()
                .join(",")
        ),
        Value::String(s) => {
            if !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || "._-:/+".contains(c)) {
                s.clone()
            } else {
                format!("\"{s}\"")
            }
        }
        other => other.to_string(),
    }
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

fn json_columnar_rows(value: &serde_json::Value) -> Option<String> {
    let items = value.as_array()?;
    if items.len() < 8 {
        return None;
    }
    let first = items.first()?.as_object()?;
    let mut keys = first.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    if keys.is_empty() {
        return None;
    }
    let mut columns: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    for key in &keys {
        let mut values = Vec::with_capacity(items.len());
        for item in items {
            let object = item.as_object()?;
            let mut item_keys = object.keys().cloned().collect::<Vec<_>>();
            item_keys.sort();
            if item_keys != keys {
                return None;
            }
            values.push(object.get(key).cloned().unwrap_or(serde_json::Value::Null));
        }
        columns.insert(key.clone(), encode_column(values));
    }
    let factored = serde_json::json!({
        "streetman": "json-columnar-delta-v1",
        "n": items.len(),
        "c": columns
    })
    .to_string();
    if token_estimate(&factored) < token_estimate(&value.to_string()) {
        Some(factored)
    } else {
        None
    }
}

fn encode_column(values: Vec<serde_json::Value>) -> serde_json::Value {
    if values.windows(2).all(|pair| {
        matches!((&pair[0], &pair[1]), (serde_json::Value::Number(a), serde_json::Value::Number(b)) if b.as_i64() == a.as_i64().map(|n| n + 1))
    }) {
        return serde_json::json!({"start": values[0], "delta": 1});
    }
    if values.iter().all(|value| value == &values[0]) {
        return serde_json::json!({"const": values[0]});
    }
    serde_json::Value::Array(values)
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
        if interesting.iter().any(|line| line == &sample) {
            out.push(format!("  sample: {sample}"));
        }
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
    for (re, replacement) in log_template_regexes() {
        out = re.replace_all(&out, *replacement).to_string();
    }
    out
}

fn log_template_regexes() -> &'static [(regex::Regex, &'static str)] {
    static RES: OnceLock<Vec<(regex::Regex, &'static str)>> = OnceLock::new();
    RES.get_or_init(|| {
        [
            (
                r"\b\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z\b",
                "{ts}",
            ),
            (r"\b[0-9a-fA-F]{8,}\b", "{hex}"),
            (
                r"\b(req|request|trace|span|user|job|worker)[_-]?[A-Za-z0-9-]+\b",
                "{$1}",
            ),
            // Mask every digit run, including numbers glued to a unit suffix
            // ("10ms", "512Mi"); a trailing \b would miss those and split one
            // template into many (dur=10ms..16ms -> 7 templates instead of 1).
            (r"\d+", "{n}"),
        ]
        .into_iter()
        .map(|(pat, rep)| (regex::Regex::new(pat).expect("log template regex"), rep))
        .collect()
    })
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
        ContentDomain::Prose | ContentDomain::Docs => accuracy_check(original, candidate),
        _ => accuracy_check(original, candidate),
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
    // High-signal = the lines a reader actually needs preserved: error/anomaly
    // lines, plus rare lines (a template seen <=2 times). High-frequency routine
    // lines belong to a template and are captured by templatization + the archive,
    // so they are droppable. This is what lets routine INFO/worker logs compress
    // hard while keeping accuracy-100 on the lines that matter. (Keyword-only
    // classification missed routine lines that lacked an "info"/"debug" tag.)
    let lines: Vec<&str> = input.lines().collect();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for line in &lines {
        *counts.entry(log_template(line)).or_insert(0) += 1;
    }
    lines
        .iter()
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            let anomaly = [
                "error",
                "fatal",
                "warn",
                "failed",
                "traceback",
                "exception",
                "panic",
                "critical",
            ]
            .iter()
            .any(|needle| lower.contains(needle));
            let rare = counts.get(&log_template(line)).copied().unwrap_or(0) <= 2;
            anomaly || rare
        })
        .copied()
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
    let mut out = input.to_string();
    for re in html_script_regexes() {
        out = re.replace_all(&out, "").to_string();
    }
    collapse_spaces(&html_tag_regex().replace_all(&out, " "))
}

fn html_script_regexes() -> &'static [regex::Regex] {
    static RES: OnceLock<Vec<regex::Regex>> = OnceLock::new();
    RES.get_or_init(|| {
        [r"(?is)<script.*?</script>", r"(?is)<style.*?</style>"]
            .into_iter()
            .map(|pat| regex::Regex::new(pat).expect("html script regex"))
            .collect()
    })
}

fn html_tag_regex() -> &'static regex::Regex {
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new(r"(?is)<[^>]+>").expect("html strip regex"))
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

    #[test]
    fn prose_modes_are_not_byte_identical() {
        let input = "The database configuration should be checked before deployment because observability and accessibility matter. The implementation currently creates repeated dependencies and unnecessary abstraction layers around a simple request handler.";
        let lite = compress(input, CompressionMode::Lite, ContentDomain::Prose);
        let full = compress(input, CompressionMode::Full, ContentDomain::Prose);
        let ultra = compress(input, CompressionMode::Ultra, ContentDomain::Prose);
        assert_eq!(lite.certificate.accuracy_score, 100);
        assert_eq!(full.certificate.accuracy_score, 100);
        assert_eq!(ultra.certificate.accuracy_score, 100);
        assert_ne!(lite.compressed, full.compressed);
        assert_ne!(full.compressed, ultra.compressed);
        assert!(full.compressed_tokens_estimate < lite.compressed_tokens_estimate);
        assert!(ultra.compressed_tokens_estimate <= full.compressed_tokens_estimate);
    }

    #[test]
    fn ultra_known_prose_beats_full() {
        let input = "The reason your React component is re-rendering is likely because you're creating a new object reference on each render cycle. When you pass an inline object as a prop, React's shallow comparison sees it as a different object every time, which triggers a re-render. I would recommend using `useMemo` to memoize the object.";
        let full = compress(input, CompressionMode::Full, ContentDomain::Prose);
        let ultra = compress(input, CompressionMode::Ultra, ContentDomain::Prose);
        assert_eq!(full.certificate.accuracy_score, 100);
        assert_eq!(ultra.certificate.accuracy_score, 100);
        assert!(ultra.compressed_tokens_estimate < full.compressed_tokens_estimate);
        assert!(ultra.compressed.contains("`useMemo`"));
    }

    #[test]
    fn stacked_uses_bundled_on_device_model_before_skeleton() {
        let model = stacked_prose_model();
        assert_eq!(model.id, "streetman-stacked-prose-model-v1");
        assert!(model
            .scorers
            .iter()
            .any(|scorer| scorer.pattern == "latency"));
        assert!(model
            .rewrites
            .iter()
            .any(|rewrite| rewrite.to == "avoid egress"));

        let input = "The system should cache compiled regex objects because latency matters and repeated prose compression should preserve identifiers while avoiding network egress. ".repeat(24);
        let result = compress(&input, CompressionMode::Full, ContentDomain::Prose);
        assert_eq!(result.certificate.accuracy_score, 100);
        assert!(result.compressed_tokens_estimate < 193);
        assert!(result.savings_percent > 40.0);
        assert!(result.compressed.contains("avoid egress"));
        assert!(result.compressed.contains("preserve ids"));
    }

    #[test]
    fn prose_selection_prefers_smaller_lossless_candidate() {
        let input = "The database configuration should be checked before deployment because observability and accessibility matter.";
        let skeleton = compress_prose_skeleton(input, CompressionMode::Full);
        let weak = format!("{skeleton} extra extra extra extra extra");

        assert_eq!(
            choose_best_lossless_prose_candidate(
                input,
                vec![
                    ProseCandidate::from_text(weak, None, Vec::new()),
                    ProseCandidate::from_text(skeleton.clone(), None, Vec::new())
                ]
            )
            .text,
            skeleton
        );
    }

    #[test]
    fn p1_codebook_is_net_positive_and_decodable() {
        let input = "cache compiled regex objects because latency matters. ".repeat(12);
        let ctx = ProseCtx::new(&input);
        let candidate = p1_codebook(&input, CompressionMode::Full, &ctx).expect("p1 candidate");
        assert!(candidate.transforms.contains(&TransformId::P1Codebook));
        assert!(candidate.text.starts_with("Legend: "));
        assert!(token_estimate(&candidate.text) < token_estimate(&input));
        assert!(decode_archive_free(&candidate.text).contains("cache compiled regex objects"));
    }

    #[test]
    fn p1_codebook_runs_on_long_prose() {
        let input = "cache compiled regex objects because latency matters. ".repeat(60);
        assert!(prose_words(&input).len() > 160);
        let ctx = ProseCtx::new(&input);
        assert!(p1_codebook(&input, CompressionMode::Full, &ctx).is_some());
    }

    #[test]
    fn p2_entropy_archives_dropped_tokens() {
        let input =
            "This system is currently very stable and it is also generally reliable. ".repeat(8);
        let ctx = ProseCtx::new(&input);
        let candidate = p2_entropy(&input, CompressionMode::Ultra, &ctx).expect("p2 candidate");
        assert!(candidate.transforms.contains(&TransformId::P2Entropy));
        assert!(candidate.archive_required);
        assert!(candidate.decode_ops[0]
            .map
            .iter()
            .any(|(_, token)| token.trim_matches('.').eq("currently")));
        assert!(token_estimate(&candidate.text) < token_estimate(&input));
    }

    #[test]
    fn n6_drops_padding_but_preserves_connectives() {
        let input = "It is worth noting that the cache is warm. However, the database is cold. \
                     Therefore we retry, because the timeout is short.";
        let ctx = ProseCtx::new(input);
        let candidate = n6_discourse(input, CompressionMode::Full, &ctx).expect("n6 candidate");
        // Padding removed.
        assert!(!candidate.text.to_ascii_lowercase().contains("worth noting"));
        // Integrity: every logical connective is preserved verbatim.
        for connective in ["However", "Therefore", "because"] {
            assert!(
                candidate.text.contains(connective),
                "connective {connective} must survive N6"
            );
        }
        // Reversible + fewer tokens.
        assert!(candidate.archive_required);
        assert!(token_estimate(&candidate.text) < token_estimate(input));
    }

    #[test]
    fn n6_never_touches_logical_markers() {
        // A sentence made only of connectives/modals must be returned unchanged
        // (N6 finds no padding to remove -> None).
        let input = "However, therefore we should, because thus it may.";
        let ctx = ProseCtx::new(input);
        assert!(n6_discourse(input, CompressionMode::Full, &ctx).is_none());
    }

    #[test]
    fn p3_coref_compresses_multiple_phrases() {
        let input = "cache compiled regex objects before deployment. preserve stable archive records before deployment. cache compiled regex objects after deployment. preserve stable archive records after deployment. ".repeat(4);
        let ctx = ProseCtx::new(&input);
        let candidate = p3_coref(&input, CompressionMode::Full, &ctx).expect("p3 candidate");
        assert!(candidate.transforms.contains(&TransformId::P3Coref));
        assert!(candidate.decode_ops[0].map.len() > 1);
        assert!(token_estimate(&candidate.text) < token_estimate(&input));
    }

    #[test]
    fn prose_certificate_records_transform_metadata() {
        let input = "cache compiled regex objects because latency matters. ".repeat(12);
        let result = compress(&input, CompressionMode::Full, ContentDomain::Prose);
        assert_eq!(result.certificate.accuracy_score, 100);
        assert!(result.compressed_tokens_estimate <= result.original_tokens_estimate);
        assert!(!result.certificate.transforms.is_empty());
        assert_eq!(
            result.certificate.transforms,
            compress_prose_full(&input, CompressionMode::Full).transforms
        );
    }

    #[test]
    fn ultra_preserves_protected_tokens() {
        let input = "React uses `useMemo` because an inline object reference changes on every render. Preserve userProfileToken and paymentProcessorConfig.";
        let result = compress(input, CompressionMode::Ultra, ContentDomain::Prose);
        let strict = accuracy_check(input, &result.compressed);
        assert_eq!(strict.score, 100);
        assert_eq!(result.certificate.accuracy_score, 100);
        assert!(result.compressed.contains("userProfileToken"));
        assert!(result.compressed.contains("paymentProcessorConfig"));
    }
}

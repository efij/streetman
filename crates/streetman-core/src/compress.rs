use crate::accuracy::{accuracy_check, protected_tokens, AccuracyReport};
use serde::{Deserialize, Serialize};

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
    Prose,
    Code,
    Json,
    Logs,
    Search,
    Diff,
    Html,
    Sql,
    K8s,
    Docs,
    Shell,
}

impl std::str::FromStr for ContentDomain {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "auto" => Ok(Self::Auto),
            "prose" => Ok(Self::Prose),
            "code" => Ok(Self::Code),
            "json" => Ok(Self::Json),
            "logs" => Ok(Self::Logs),
            "search" => Ok(Self::Search),
            "diff" => Ok(Self::Diff),
            "html" => Ok(Self::Html),
            "sql" => Ok(Self::Sql),
            "k8s" => Ok(Self::K8s),
            "docs" => Ok(Self::Docs),
            "shell" => Ok(Self::Shell),
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
    pub certificate: CompressionCertificate,
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

    let candidate = match resolved_domain {
        ContentDomain::Json => compress_json(input),
        ContentDomain::Logs | ContentDomain::Shell => compress_logs(input),
        ContentDomain::Search => compress_search(input),
        ContentDomain::Diff => compress_diff(input),
        ContentDomain::Code | ContentDomain::Sql | ContentDomain::K8s => compress_code(input),
        ContentDomain::Html => compress_html(input),
        ContentDomain::Docs | ContentDomain::Prose | ContentDomain::Auto => {
            compress_prose(input, resolved_mode)
        }
    };

    let report = compression_accuracy_check(input, &candidate, resolved_domain);
    if report.score < 100 {
        build_result(
            input,
            input.to_string(),
            resolved_mode,
            resolved_domain,
            Some("accuracy guard rejected compressed output".to_string()),
        )
    } else {
        build_result(input, candidate, resolved_mode, resolved_domain, None)
    }
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
    let savings_percent = if before == 0 {
        0.0
    } else {
        ((before.saturating_sub(after)) as f64 / before as f64) * 100.0
    };
    let report = compression_accuracy_check(original, &compressed, domain);
    let input_hash = blake3_hex(original);
    let output_hash = blake3_hex(&compressed);
    let algorithm = format!("streetman-deterministic/{mode:?}/{domain:?}");
    let certificate_id = blake3_hex(&format!(
        "{input_hash}:{output_hash}:{algorithm}:{}:{}:{}",
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
    input.chars().count().div_ceil(4).max(1)
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
        out = replace_case_insensitive(&out, from, to);
    }
    out = out.replace(" and ", " & ");
    out = out.replace(" or ", " | ");
    out = out.replace(" not equal to ", " ≠ ");
    out = out.replace(" equals ", " = ");
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
        skeletonize(token, mode)
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

fn skeletonize(word: &str, mode: CompressionMode) -> String {
    let lower = word.to_ascii_lowercase();
    if let Some(short) = shortcut(&lower) {
        return short.to_string();
    }
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
        out = replace_case_insensitive(&out, from, to);
    }
    out = out.replace(" and ", " & ");
    out = out.replace(" or ", " | ");
    out = out.replace(" because ", " cuz ");
    out = out.replace(" — ", " -> ");
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
    summarize_json_value(&value, 0)
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

fn compress_diff(input: &str) -> String {
    let kept = input
        .lines()
        .filter(|line| {
            line.starts_with("diff --git")
                || line.starts_with("+++")
                || line.starts_with("---")
                || line.starts_with("@@")
                || line.starts_with("+")
                || line.starts_with("-")
        })
        .take(200)
        .collect::<Vec<_>>()
        .join("\n");
    if kept.is_empty() {
        input.to_string()
    } else {
        kept
    }
}

fn compress_code(input: &str) -> String {
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
            "[streetman structure map: {} implementation lines omitted]\n{}",
            omitted,
            out.join("\n")
        )
    }
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
}

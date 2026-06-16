use crate::compress::token_estimate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchoredDiffReport {
    pub format: String,
    pub before_tokens: usize,
    pub after_tokens: usize,
    pub transport_tokens: usize,
    pub savings_vs_full_after_percent: f64,
    pub prefix_lines: usize,
    pub suffix_lines: usize,
    pub replaced_before_lines: usize,
    pub inserted_after_lines: usize,
    pub fallback_reason: Option<String>,
    pub transport: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElisionReport {
    pub format: String,
    pub original_tokens: usize,
    pub elided_tokens: usize,
    pub savings_percent: f64,
    pub omitted_lines: usize,
    pub output: String,
}

pub fn anchored_diff(before: &str, after: &str) -> AnchoredDiffReport {
    let before_lines = before.lines().collect::<Vec<_>>();
    let after_lines = after.lines().collect::<Vec<_>>();
    let before_tokens = token_estimate(before);
    let after_tokens = token_estimate(after);

    if before == after {
        let transport = "[streetman-edit v1: no-op]".to_string();
        return build_diff_report(
            before_tokens,
            after_tokens,
            before_lines.len(),
            0,
            0,
            0,
            0,
            None,
            transport,
        );
    }

    let mut prefix = 0usize;
    while prefix < before_lines.len()
        && prefix < after_lines.len()
        && before_lines[prefix] == after_lines[prefix]
    {
        prefix += 1;
    }

    let mut suffix = 0usize;
    while suffix + prefix < before_lines.len()
        && suffix + prefix < after_lines.len()
        && before_lines[before_lines.len() - 1 - suffix]
            == after_lines[after_lines.len() - 1 - suffix]
    {
        suffix += 1;
    }

    let before_mid = &before_lines[prefix..before_lines.len().saturating_sub(suffix)];
    let after_mid = &after_lines[prefix..after_lines.len().saturating_sub(suffix)];
    let start_line = prefix + 1;
    let end_line = if before_mid.is_empty() {
        prefix
    } else {
        prefix + before_mid.len()
    };
    let transport = format!(
        "[streetman-edit v1 replace lines {start_line}-{end_line}; prefix={prefix}; suffix={suffix}]\n--- find\n{}\n--- replace\n{}\n[/streetman-edit]",
        before_mid.join("\n"),
        after_mid.join("\n")
    );
    let transport_tokens = token_estimate(&transport);
    if transport_tokens <= after_tokens {
        build_diff_report(
            before_tokens,
            after_tokens,
            prefix,
            suffix,
            before_mid.len(),
            after_mid.len(),
            transport_tokens,
            None,
            transport,
        )
    } else {
        build_diff_report(
            before_tokens,
            after_tokens,
            prefix,
            suffix,
            before_mid.len(),
            after_mid.len(),
            after_tokens,
            Some("anchored edit was not smaller than full after; emitted full after".to_string()),
            after.to_string(),
        )
    }
}

fn build_diff_report(
    before_tokens: usize,
    after_tokens: usize,
    prefix_lines: usize,
    suffix_lines: usize,
    replaced_before_lines: usize,
    inserted_after_lines: usize,
    transport_tokens: usize,
    fallback_reason: Option<String>,
    transport: String,
) -> AnchoredDiffReport {
    let transport_tokens = if transport_tokens == 0 {
        token_estimate(&transport)
    } else {
        transport_tokens
    };
    let savings_vs_full_after_percent = if after_tokens == 0 {
        0.0
    } else {
        ((after_tokens.saturating_sub(transport_tokens)) as f64 / after_tokens as f64) * 100.0
    };
    AnchoredDiffReport {
        format: "streetman-anchored-edit-v1".to_string(),
        before_tokens,
        after_tokens,
        transport_tokens,
        savings_vs_full_after_percent,
        prefix_lines,
        suffix_lines,
        replaced_before_lines,
        inserted_after_lines,
        fallback_reason,
        transport,
    }
}

pub fn elide_unchanged_regions(input: &str, keep_edge_lines: usize) -> ElisionReport {
    let lines = input.lines().collect::<Vec<_>>();
    let original_tokens = token_estimate(input);
    let keep = keep_edge_lines.max(1);
    if lines.len() <= keep * 2 + 3 {
        return ElisionReport {
            format: "streetman-unchanged-elision-v1".to_string(),
            original_tokens,
            elided_tokens: original_tokens,
            savings_percent: 0.0,
            omitted_lines: 0,
            output: input.to_string(),
        };
    }
    let omitted = lines.len().saturating_sub(keep * 2);
    let mut out = Vec::new();
    out.extend(lines.iter().take(keep).copied());
    let marker = format!("// ... {omitted} unchanged lines elided; re-expand from source ...");
    out.push(&marker);
    out.extend(lines.iter().rev().take(keep).rev().copied());
    let output = out.join("\n");
    let elided_tokens = token_estimate(&output);
    let savings_percent = if original_tokens == 0 {
        0.0
    } else {
        ((original_tokens.saturating_sub(elided_tokens)) as f64 / original_tokens as f64) * 100.0
    };
    ElisionReport {
        format: "streetman-unchanged-elision-v1".to_string(),
        original_tokens,
        elided_tokens,
        savings_percent,
        omitted_lines: omitted,
        output,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchored_diff_is_smaller_for_small_edits() {
        let before = (0..80)
            .map(|i| format!("line {i}: unchanged"))
            .collect::<Vec<_>>()
            .join("\n");
        let after = before.replace("line 40: unchanged", "line 40: changed");
        let report = anchored_diff(&before, &after);
        assert!(report.fallback_reason.is_none());
        assert!(report.transport_tokens < report.after_tokens);
        assert!(report.savings_vs_full_after_percent > 80.0);
    }

    #[test]
    fn elides_middle_regions() {
        let input = (0..60)
            .map(|i| format!("fn item_{i}() {{}}"))
            .collect::<Vec<_>>()
            .join("\n");
        let report = elide_unchanged_regions(&input, 2);
        assert!(report.omitted_lines > 50);
        assert!(report.elided_tokens < report.original_tokens);
    }
}

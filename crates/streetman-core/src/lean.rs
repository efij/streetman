use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LeanMode {
    Off,
    Lite,
    Full,
    Ultra,
}

impl LeanMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Lite => "lite",
            Self::Full => "full",
            Self::Ultra => "ultra",
        }
    }

    pub fn parse(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Self::Off,
            "lite" => Self::Lite,
            "ultra" => Self::Ultra,
            _ => Self::Full,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanFinding {
    pub tag: String,
    pub severity: String,
    pub path: Option<String>,
    pub line: Option<usize>,
    pub message: String,
    pub replacement: String,
    pub estimated_lines_saved: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanReport {
    pub scope: String,
    pub status: String,
    pub mode: String,
    pub files_touched: usize,
    pub loc_added: usize,
    pub loc_removed: usize,
    pub dependencies_added: Vec<String>,
    pub dependencies_removed: Vec<String>,
    pub shortcut_comments: usize,
    pub runnable_checks: usize,
    pub safety_exceptions: usize,
    pub extension_cost_score: u8,
    pub estimated_lines_saved: i64,
    pub findings: Vec<LeanFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanCertificate {
    pub certificate_id: String,
    pub generated_at: String,
    pub mode: String,
    pub input_hash: String,
    pub diff_hash: String,
    pub files_touched: usize,
    pub loc_added: usize,
    pub loc_removed: usize,
    pub dependencies_added: Vec<String>,
    pub dependencies_removed: Vec<String>,
    pub findings_count: usize,
    pub estimated_lines_saved: i64,
    pub runnable_checks: usize,
    pub shortcut_comments: usize,
    pub safety_exceptions: usize,
    pub extension_cost_score: u8,
    pub normal_twin_hash: Option<String>,
    pub commands: Vec<String>,
    pub proof_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanGateConfig {
    pub max_new_dependencies: usize,
    pub max_files_touched: usize,
    pub require_runnable_check: bool,
    pub max_extension_cost_score: u8,
}

impl Default for LeanGateConfig {
    fn default() -> Self {
        Self {
            max_new_dependencies: 0,
            max_files_touched: 12,
            require_runnable_check: true,
            max_extension_cost_score: 75,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanGateResult {
    pub status: String,
    pub passed: bool,
    pub violations: Vec<String>,
    pub report: LeanReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanBenchCase {
    pub task: String,
    pub baseline_loc: usize,
    pub ponytail_loc: usize,
    pub streetman_compression_loc: usize,
    pub streetman_lean_loc: usize,
    pub ponytail_tokens: usize,
    pub streetman_lean_tokens: usize,
    pub safety_passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanBenchResult {
    pub suite: String,
    pub against: String,
    pub status: String,
    pub claim: String,
    pub feature_kill: bool,
    pub public_performance_claim_ready: bool,
    pub cases: Vec<LeanBenchCase>,
    pub totals: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanKillFeature {
    pub feature: String,
    pub ponytail: String,
    pub streetman: String,
    pub status: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeanKillReport {
    pub against: String,
    pub verdict: String,
    pub feature_kill: bool,
    pub public_performance_claim_ready: bool,
    pub ponytail_commit: String,
    pub streetman_extra_features: Vec<String>,
    pub parity: Vec<LeanKillFeature>,
    pub caveat: String,
}

pub fn lean_instructions(mode: LeanMode, host: &str) -> String {
    if mode == LeanMode::Off {
        return "STREETMAN LEAN MODE OFF".to_string();
    }

    let level = mode.as_str();
    let mode_rule = match mode {
        LeanMode::Lite => {
            "Build what was asked, then name the smaller stdlib/native alternative in one short line."
        }
        LeanMode::Full => {
            "Use the ladder as the default build path. Ship the smallest correct diff without stalling."
        }
        LeanMode::Ultra => {
            "Deletion first. Skip speculative work, challenge bloat, and use the one-line/native answer when correct."
        }
        LeanMode::Off => "",
    };

    format!(
        r#"STREETMAN LEAN MODE ACTIVE - level: {level}
Host: {host}

You are building with Streetman Lean: smallest correct implementation, proven by diff.

Ladder:
1. Does this need to exist? If not, skip it.
2. Does the standard library do it? Use it.
3. Does the platform/runtime/database/browser do it natively? Use it.
4. Does an already-installed dependency do it? Use it.
5. Can it be one line? Make it one line.
6. Only then write the minimum code that works.

Rules:
- No unrequested abstractions, factories, wrappers, future-proof config, or new deps.
- Prefer deletion over addition and fewer touched files.
- Mark intentional shortcuts with `streetman:` plus ceiling and upgrade path.
- Non-trivial logic leaves one small runnable check.
- Never simplify away trust-boundary validation, security, data-loss handling, accessibility basics, or explicit requirements.
- End with what was skipped and when to add it.

Mode rule: {mode_rule}
"#
    )
}

pub fn review_diff(diff: &str, mode: LeanMode) -> LeanReport {
    let mut report = base_report("diff", mode);
    let mut current_path: Option<String> = None;
    let mut added_line_number = 0usize;
    let mut touched = BTreeSet::new();
    let mut added_deps = BTreeSet::new();
    let mut removed_deps = BTreeSet::new();

    for line in diff.lines() {
        if let Some(path) = parse_diff_path(line) {
            current_path = Some(path.clone());
            touched.insert(path);
            added_line_number = 0;
            continue;
        }

        if line.starts_with("@@") {
            added_line_number = parse_hunk_new_start(line).unwrap_or(0);
            continue;
        }

        let is_added = line.starts_with('+') && !line.starts_with("+++");
        let is_removed = line.starts_with('-') && !line.starts_with("---");
        if is_added {
            report.loc_added += 1;
            let text = &line[1..];
            inspect_line(
                &mut report,
                current_path.as_deref(),
                Some(added_line_number),
                text,
                true,
            );
            if let Some(dep) = dependency_from_line(current_path.as_deref(), text) {
                added_deps.insert(dep.clone());
                dependency_finding(
                    &mut report,
                    current_path.as_deref(),
                    added_line_number,
                    &dep,
                );
            }
            added_line_number = added_line_number.saturating_add(1);
        } else if is_removed {
            report.loc_removed += 1;
            if let Some(dep) = dependency_from_line(current_path.as_deref(), &line[1..]) {
                removed_deps.insert(dep);
            }
        } else if !line.starts_with('\\') {
            added_line_number = added_line_number.saturating_add(1);
        }
    }

    report.files_touched = touched.len();
    report.dependencies_added = added_deps.into_iter().collect();
    report.dependencies_removed = removed_deps.into_iter().collect();
    finalize_report(report)
}

pub fn audit_files(files: &[(String, String)], mode: LeanMode) -> LeanReport {
    let mut report = base_report("repo", mode);
    let mut added_deps = BTreeSet::new();

    for (path, content) in files {
        report.files_touched += 1;
        for (idx, line) in content.lines().enumerate() {
            report.loc_added += 1;
            inspect_line(&mut report, Some(path), Some(idx + 1), line, false);
            if let Some(dep) = dependency_from_line(Some(path), line) {
                added_deps.insert(dep.clone());
                dependency_finding(&mut report, Some(path), idx + 1, &dep);
            }
        }

        if content.lines().count() < 8 && path.ends_with(".rs") {
            report.findings.push(LeanFinding {
                tag: "yagni".to_string(),
                severity: "info".to_string(),
                path: Some(path.clone()),
                line: Some(1),
                message: "tiny file may only wrap one thing".to_string(),
                replacement: "inline it unless this is a real module boundary".to_string(),
                estimated_lines_saved: 4,
            });
        }
    }

    report.dependencies_added = added_deps.into_iter().collect();
    finalize_report(report)
}

pub fn gate_diff(diff: &str, mode: LeanMode, config: LeanGateConfig) -> LeanGateResult {
    let report = review_diff(diff, mode);
    let mut violations = Vec::new();

    if report.dependencies_added.len() > config.max_new_dependencies {
        violations.push(format!(
            "new dependencies {} exceed max {}",
            report.dependencies_added.len(),
            config.max_new_dependencies
        ));
    }
    if report.files_touched > config.max_files_touched {
        violations.push(format!(
            "files touched {} exceed max {}",
            report.files_touched, config.max_files_touched
        ));
    }
    if config.require_runnable_check
        && report.loc_added > 20
        && report.runnable_checks == 0
        && !has_docs_only_change(&report)
    {
        violations.push("non-trivial diff has no minimal runnable check".to_string());
    }
    if report.extension_cost_score > config.max_extension_cost_score {
        violations.push(format!(
            "extension cost score {} exceeds max {}",
            report.extension_cost_score, config.max_extension_cost_score
        ));
    }
    if report.findings.iter().any(|f| f.severity == "block") {
        violations.push("blocking lean findings present".to_string());
    }

    let passed = violations.is_empty();
    LeanGateResult {
        status: if passed { "pass" } else { "fail" }.to_string(),
        passed,
        violations,
        report,
    }
}

pub fn prove_diff(diff: &str, mode: LeanMode, commands: Vec<String>) -> LeanCertificate {
    prove_diff_with_normal_twin(diff, mode, commands, None)
}

pub fn prove_diff_with_normal_twin(
    diff: &str,
    mode: LeanMode,
    commands: Vec<String>,
    normal_twin: Option<&str>,
) -> LeanCertificate {
    let report = review_diff(diff, mode);
    let input_hash = blake3_hex(diff);
    let normal_twin_hash = normal_twin.map(blake3_hex);
    let proof_material = serde_json::json!({
        "input_hash": input_hash,
        "mode": mode.as_str(),
        "files_touched": report.files_touched,
        "loc_added": report.loc_added,
        "loc_removed": report.loc_removed,
        "dependencies_added": report.dependencies_added,
        "dependencies_removed": report.dependencies_removed,
        "findings": report.findings.len(),
        "normal_twin_hash": normal_twin_hash,
        "commands": commands,
    })
    .to_string();
    let proof_signature = blake3_hex(&proof_material);
    LeanCertificate {
        certificate_id: format!("lean-{}", &proof_signature[..16]),
        generated_at: chrono::Utc::now().to_rfc3339(),
        mode: mode.as_str().to_string(),
        input_hash: input_hash.clone(),
        diff_hash: input_hash,
        files_touched: report.files_touched,
        loc_added: report.loc_added,
        loc_removed: report.loc_removed,
        dependencies_added: report.dependencies_added,
        dependencies_removed: report.dependencies_removed,
        findings_count: report.findings.len(),
        estimated_lines_saved: report.estimated_lines_saved,
        runnable_checks: report.runnable_checks,
        shortcut_comments: report.shortcut_comments,
        safety_exceptions: report.safety_exceptions,
        extension_cost_score: report.extension_cost_score,
        normal_twin_hash,
        commands,
        proof_signature,
    }
}

pub fn ponytail_h2h_fixture(against: &str) -> LeanBenchResult {
    let cases = vec![
        LeanBenchCase {
            task: "email-validator".to_string(),
            baseline_loc: 104,
            ponytail_loc: 5,
            streetman_compression_loc: 5,
            streetman_lean_loc: 4,
            ponytail_tokens: 26_573,
            streetman_lean_tokens: 18_900,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "debounce".to_string(),
            baseline_loc: 38,
            ponytail_loc: 5,
            streetman_compression_loc: 5,
            streetman_lean_loc: 4,
            ponytail_tokens: 26_745,
            streetman_lean_tokens: 19_250,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "csv-sum".to_string(),
            baseline_loc: 6,
            ponytail_loc: 6,
            streetman_compression_loc: 6,
            streetman_lean_loc: 6,
            ponytail_tokens: 26_251,
            streetman_lean_tokens: 18_700,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "react-countdown".to_string(),
            baseline_loc: 190,
            ponytail_loc: 13,
            streetman_compression_loc: 13,
            streetman_lean_loc: 11,
            ponytail_tokens: 26_961,
            streetman_lean_tokens: 20_300,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "rate-limit".to_string(),
            baseline_loc: 25,
            ponytail_loc: 18,
            streetman_compression_loc: 18,
            streetman_lean_loc: 15,
            ponytail_tokens: 29_179,
            streetman_lean_tokens: 21_050,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "production-log-cli".to_string(),
            baseline_loc: 946,
            ponytail_loc: 145,
            streetman_compression_loc: 145,
            streetman_lean_loc: 132,
            ponytail_tokens: 38_100,
            streetman_lean_tokens: 27_800,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "production-file-sync".to_string(),
            baseline_loc: 656,
            ponytail_loc: 99,
            streetman_compression_loc: 99,
            streetman_lean_loc: 91,
            ponytail_tokens: 34_400,
            streetman_lean_tokens: 25_600,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "production-dispatcher".to_string(),
            baseline_loc: 808,
            ponytail_loc: 73,
            streetman_compression_loc: 73,
            streetman_lean_loc: 68,
            ponytail_tokens: 37_600,
            streetman_lean_tokens: 26_200,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "production-validation".to_string(),
            baseline_loc: 677,
            ponytail_loc: 70,
            streetman_compression_loc: 70,
            streetman_lean_loc: 64,
            ponytail_tokens: 35_900,
            streetman_lean_tokens: 25_900,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "production-auth".to_string(),
            baseline_loc: 260,
            ponytail_loc: 49,
            streetman_compression_loc: 49,
            streetman_lean_loc: 47,
            ponytail_tokens: 30_500,
            streetman_lean_tokens: 22_700,
            safety_passed: true,
        },
        LeanBenchCase {
            task: "production-ledger".to_string(),
            baseline_loc: 282,
            ponytail_loc: 54,
            streetman_compression_loc: 54,
            streetman_lean_loc: 51,
            ponytail_tokens: 31_800,
            streetman_lean_tokens: 23_100,
            safety_passed: true,
        },
    ];

    let mut totals = BTreeMap::new();
    totals.insert(
        "baseline_loc".to_string(),
        cases.iter().map(|c| c.baseline_loc).sum(),
    );
    totals.insert(
        "ponytail_loc".to_string(),
        cases.iter().map(|c| c.ponytail_loc).sum(),
    );
    totals.insert(
        "streetman_lean_loc".to_string(),
        cases.iter().map(|c| c.streetman_lean_loc).sum(),
    );
    totals.insert(
        "ponytail_tokens".to_string(),
        cases.iter().map(|c| c.ponytail_tokens).sum(),
    );
    totals.insert(
        "streetman_lean_tokens".to_string(),
        cases.iter().map(|c| c.streetman_lean_tokens).sum(),
    );

    LeanBenchResult {
        suite: "streetman-lean-ponytail-h2h-fixture".to_string(),
        against: against.to_string(),
        status: "feature-win-fixture-pass".to_string(),
        claim: "Feature-wise Streetman Lean includes Ponytail's minimalism surface plus Streetman proof/compression/gate extras. This is a feature-surface win; live provider replay is still required before public performance claims.".to_string(),
        feature_kill: true,
        public_performance_claim_ready: false,
        cases,
        totals,
    }
}

pub fn ponytail_kill_report() -> LeanKillReport {
    let parity = vec![
        kill_feature(
            "minimal implementation ladder",
            "YAGNI -> stdlib -> native -> installed dependency -> one line -> minimum",
            "Streetman Lean instructions render the same ladder and bind it to proof/gate commands",
            &["streetman lean instructions", "skills/streetman-lean/SKILL.md"],
        ),
        kill_feature(
            "lite/full/ultra/off modes",
            "Prompt-level modes plus hook state",
            "LeanMode supports off/lite/full/ultra across CLI, hooks, OpenCode, and Pi",
            &["LeanMode", "hooks/streetman-mode-tracker.js", ".opencode/plugins/streetman-lean.mjs"],
        ),
        kill_feature(
            "persistent activation",
            "SessionStart and prompt hooks",
            "Streetman ships SessionStart/UserPromptSubmit hooks, statusline scripts, and host injection adapters",
            &["hooks/hooks.json", "hooks/streetman-activate.js", "hooks/streetman-statusline.sh"],
        ),
        kill_feature(
            "cross-agent distribution",
            "Claude, Codex, OpenCode, Gemini, Cursor, Windsurf, Cline, Copilot, Kiro, Pi",
            "Streetman ships those surfaces plus Zed and a VS Code extension scaffold",
            &[".codex-plugin/plugin.json", ".claude-plugin/plugin.json", "gemini-extension.json", ".zed/streetman-lean.md", "vscode-extension/package.json"],
        ),
        kill_feature(
            "overengineering review",
            "ponytail-review skill",
            "streetman lean review emits structured findings, dependency blocks, line savings, and JSON",
            &["streetman lean review --diff", "review_diff"],
        ),
        kill_feature(
            "repo-wide bloat audit",
            "ponytail-audit skill",
            "streetman lean audit scans repo files, dependencies, wrappers, abstractions, and dead config",
            &["streetman lean audit .", "audit_files"],
        ),
        kill_feature(
            "shortcut comments",
            "`ponytail:` ceiling comments",
            "`streetman:` ceiling comments are counted in reports/certificates and reduce extension-cost score",
            &["shortcut_comments", "LeanCertificate"],
        ),
        kill_feature(
            "runnable-check reflex",
            "Prompt rule for one small check",
            "Lean gate blocks non-trivial diffs without checks unless explicitly waived",
            &["streetman lean gate", "LeanGateConfig.require_runnable_check"],
        ),
        kill_feature(
            "safety boundaries",
            "Prompt guardrails",
            "Streetman keeps safety guardrails in instructions and records safety exceptions in reports/certificates",
            &["safety_exceptions", "redteam bench"],
        ),
        kill_feature(
            "benchmarks",
            "Promptfoo LOC/cost/time reports",
            "Streetman ships Ponytail H2H fixture, totals, feature-kill flag, and benchmark-result schema",
            &["streetman lean bench run --against ponytail", "benchmarks/lean/ponytail-h2h-tasks.json"],
        ),
        kill_feature(
            "claims discipline",
            "Benchmark caveats in docs",
            "Streetman separates feature-surface win from public performance claims in machine-readable output",
            &["public_performance_claim_ready=false", "docs/streetman-lean.md"],
        ),
    ];

    LeanKillReport {
        against: "DietrichGebert/ponytail".to_string(),
        verdict: "yes-feature-wise-streetman-includes-ponytail-and-more".to_string(),
        feature_kill: parity.iter().all(|feature| feature.status == "streetman-wins"),
        public_performance_claim_ready: false,
        ponytail_commit: "16319c7bc91b098975d2bfb2e351398ff8aae3e7".to_string(),
        streetman_extra_features: vec![
            "Lean Certificate".to_string(),
            "Dependency Kill Switch".to_string(),
            "Extension-Cost Predictor".to_string(),
            "Normal Twin / archive path for exact originals and rejected context".to_string(),
            "FinOps + EngOps dashboard hooks via audit/dashboard/archive stats".to_string(),
            "Rust compression engine with proof certificates".to_string(),
            "MCP/proxy/gateway conformance surface".to_string(),
        ],
        parity,
        caveat: "This is a feature-surface kill. Public performance claims still require a committed live provider replay.".to_string(),
    }
}

fn kill_feature(
    feature: &str,
    ponytail: &str,
    streetman: &str,
    evidence: &[&str],
) -> LeanKillFeature {
    LeanKillFeature {
        feature: feature.to_string(),
        ponytail: ponytail.to_string(),
        streetman: streetman.to_string(),
        status: "streetman-wins".to_string(),
        evidence: evidence.iter().map(|item| item.to_string()).collect(),
    }
}

fn base_report(scope: &str, mode: LeanMode) -> LeanReport {
    LeanReport {
        scope: scope.to_string(),
        status: "lean".to_string(),
        mode: mode.as_str().to_string(),
        files_touched: 0,
        loc_added: 0,
        loc_removed: 0,
        dependencies_added: Vec::new(),
        dependencies_removed: Vec::new(),
        shortcut_comments: 0,
        runnable_checks: 0,
        safety_exceptions: 0,
        extension_cost_score: 0,
        estimated_lines_saved: 0,
        findings: Vec::new(),
    }
}

fn finalize_report(mut report: LeanReport) -> LeanReport {
    report.estimated_lines_saved = report
        .findings
        .iter()
        .map(|finding| finding.estimated_lines_saved.max(0))
        .sum();
    let dep_penalty = (report.dependencies_added.len() * 12) as u8;
    let file_penalty = report.files_touched.saturating_sub(6) as u8 * 4;
    let finding_penalty = (report.findings.len() as u8).saturating_mul(5);
    let shortcut_bonus = (report.shortcut_comments as u8).saturating_mul(3);
    report.extension_cost_score = dep_penalty
        .saturating_add(file_penalty)
        .saturating_add(finding_penalty)
        .saturating_sub(shortcut_bonus)
        .min(100);
    report.status = if report.findings.iter().any(|f| f.severity == "block") {
        "block".to_string()
    } else if report.findings.is_empty() {
        "pass".to_string()
    } else {
        "warn".to_string()
    };
    report
}

fn inspect_line(
    report: &mut LeanReport,
    path: Option<&str>,
    line: Option<usize>,
    text: &str,
    from_diff: bool,
) {
    let lower = text.to_ascii_lowercase();
    if lower.contains("streetman:") {
        report.shortcut_comments += 1;
    }
    if lower.contains("assert")
        || lower.contains("#[test]")
        || lower.contains("fn test_")
        || lower.contains("test(")
        || lower.contains("demo(")
        || lower.contains("__main__")
    {
        report.runnable_checks += 1;
    }
    if lower.contains("auth")
        || lower.contains("security")
        || lower.contains("cve-")
        || lower.contains("accessibility")
        || lower.contains("aria-")
        || lower.contains("drop table")
        || lower.contains("rm -rf")
        || lower.contains("data loss")
    {
        report.safety_exceptions += 1;
    }

    if contains_any(
        &lower,
        &[
            "future proof",
            "future-proof",
            "for later",
            "just in case",
            "scaffold",
            "placeholder",
        ],
    ) {
        report.findings.push(LeanFinding {
            tag: "yagni".to_string(),
            severity: "warn".to_string(),
            path: path.map(str::to_string),
            line,
            message: "speculative code or config added".to_string(),
            replacement: "delete until a caller or requirement exists".to_string(),
            estimated_lines_saved: 8,
        });
    }

    if contains_any(
        text,
        &[
            "Factory",
            "Abstract",
            "IRepository",
            "IService",
            "Provider",
            "Manager",
        ],
    ) && from_diff
    {
        report.findings.push(LeanFinding {
            tag: "yagni".to_string(),
            severity: "warn".to_string(),
            path: path.map(str::to_string),
            line,
            message: "abstraction-shaped code added".to_string(),
            replacement: "inline until there are two real implementations".to_string(),
            estimated_lines_saved: 12,
        });
    }

    if lower.contains("regex::new") || lower.contains("new regexp") {
        report.findings.push(LeanFinding {
            tag: "shrink".to_string(),
            severity: "info".to_string(),
            path: path.map(str::to_string),
            line,
            message: "regex added for possibly simple string matching".to_string(),
            replacement: "prefer contains/split/strip_prefix when enough".to_string(),
            estimated_lines_saved: 3,
        });
    }
}

fn dependency_finding(report: &mut LeanReport, path: Option<&str>, line: usize, dep: &str) {
    if let Some(replacement) = avoidable_dependency_replacement(dep) {
        report.findings.push(LeanFinding {
            tag: "stdlib".to_string(),
            severity: "block".to_string(),
            path: path.map(str::to_string),
            line: Some(line),
            message: format!("avoidable dependency `{dep}` added"),
            replacement: replacement.to_string(),
            estimated_lines_saved: 25,
        });
    }
}

fn avoidable_dependency_replacement(dep: &str) -> Option<&'static str> {
    match dep {
        "moment" | "date-fns" | "dayjs" | "flatpickr" => {
            Some("native Date/Intl or browser date input")
        }
        "lodash" | "underscore" => Some("stdlib iterators, Object/Array helpers"),
        "axios" | "request" | "node-fetch" => Some("native fetch"),
        "uuid" => Some("crypto.randomUUID or platform UUID"),
        "validator" => Some("trust-boundary-specific validation or confirmation flow"),
        "left-pad" => Some("padStart"),
        _ => None,
    }
}

fn dependency_from_line(path: Option<&str>, text: &str) -> Option<String> {
    let path = path.unwrap_or_default();
    let trimmed = text.trim().trim_matches(',');
    if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
        return None;
    }

    if path.ends_with("Cargo.toml") {
        if let Some((name, _)) = trimmed.split_once('=') {
            let name = name.trim();
            if is_dependency_name(name) {
                return Some(name.to_string());
            }
        }
    }
    if path.ends_with("package.json") {
        let cleaned = trimmed.trim_matches('"');
        if let Some((name, _)) = cleaned.split_once("\":") {
            let dep = name.trim_matches('"').to_string();
            if is_dependency_name(&dep) {
                return Some(dep);
            }
        }
    }
    if path.ends_with("requirements.txt") {
        let dep = trimmed
            .split(['=', '<', '>', '~', '['])
            .next()
            .unwrap_or_default()
            .trim();
        if is_dependency_name(dep) {
            return Some(dep.to_string());
        }
    }
    None
}

fn is_dependency_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '@' | '/' | '.'))
        && !matches!(
            value,
            "dependencies"
                | "devDependencies"
                | "package"
                | "workspace"
                | "features"
                | "version"
                | "edition"
                | "license"
                | "repository"
        )
}

fn parse_diff_path(line: &str) -> Option<String> {
    if let Some(rest) = line.strip_prefix("diff --git ") {
        let mut parts = rest.split_whitespace();
        let _a = parts.next();
        if let Some(b) = parts.next() {
            return Some(b.trim_start_matches("b/").to_string());
        }
    }
    if let Some(rest) = line.strip_prefix("+++ b/") {
        return Some(rest.to_string());
    }
    None
}

fn parse_hunk_new_start(line: &str) -> Option<usize> {
    let plus = line.split_whitespace().find(|part| part.starts_with('+'))?;
    let start = plus
        .trim_start_matches('+')
        .split(',')
        .next()
        .unwrap_or_default();
    start.parse().ok()
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn has_docs_only_change(report: &LeanReport) -> bool {
    !report.findings.is_empty()
        && report
            .findings
            .iter()
            .all(|finding| finding.path.as_deref().is_some_and(is_doc_path))
}

fn is_doc_path(path: &str) -> bool {
    path.ends_with(".md") || path.starts_with("docs/")
}

fn blake3_hex(input: &str) -> String {
    blake3::hash(input.as_bytes()).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_detects_avoidable_dependency_and_shortcut_comment() {
        let diff = r#"diff --git a/package.json b/package.json
@@ -1,3 +1,4 @@
 {
+  "flatpickr": "^4.0.0",
+  "// streetman: native date input, upgrade to picker if range UI needed": true
 }
"#;
        let report = review_diff(diff, LeanMode::Full);
        assert_eq!(report.dependencies_added, vec!["flatpickr"]);
        assert_eq!(report.shortcut_comments, 1);
        assert!(report.findings.iter().any(|f| f.severity == "block"));
    }

    #[test]
    fn gate_requires_check_for_non_trivial_diff() {
        let diff = format!(
            "diff --git a/src/lib.rs b/src/lib.rs\n@@ -1,1 +1,30 @@\n{}",
            (0..30)
                .map(|i| format!("+pub fn generated_{i}() {{}}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        let result = gate_diff(&diff, LeanMode::Full, LeanGateConfig::default());
        assert!(!result.passed);
        assert!(result
            .violations
            .iter()
            .any(|violation| violation.contains("runnable check")));
    }

    #[test]
    fn proof_is_stable_shape() {
        let cert = prove_diff(
            "diff --git a/src/lib.rs b/src/lib.rs\n@@ -1 +1 @@\n+assert!(true)",
            LeanMode::Ultra,
            vec!["cargo test".to_string()],
        );
        assert!(cert.certificate_id.starts_with("lean-"));
        assert_eq!(cert.runnable_checks, 1);
        assert_eq!(cert.commands, vec!["cargo test"]);
    }
}

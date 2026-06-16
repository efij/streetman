use std::process::{Command, Stdio};

#[test]
fn cli_compress_smoke() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args([
            "compress",
            "--mode",
            "full",
            "--domain",
            "prose",
            "--json",
            "--no-archive",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn streetman");
    {
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(b"The database configuration should be checked before deployment.")
            .expect("write stdin");
    }
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("json");
    assert!(
        value["compressed_tokens_estimate"].as_u64().unwrap()
            <= value["original_tokens_estimate"].as_u64().unwrap()
    );
    let token_guard = value["token_guard"].as_str().expect("token guard");
    assert!(token_guard.starts_with("never-worse-than-raw/"));
    assert!(token_guard.ends_with("-greedy"));
}

#[test]
fn cli_compile_shortlang_smoke() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args([
            "compile",
            "--mode",
            "full",
            "--domain",
            "context",
            "--json",
            "--no-archive",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn streetman");
    {
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(b"INFO ok\nERROR payment failed request_id=req_123")
            .expect("write stdin");
    }
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"wire\""));
    assert!(stdout.contains("compressor_mutated_artifacts"));
}

#[test]
fn cli_run_receipt_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args([
            "run",
            "--json",
            "--",
            "sh",
            "-c",
            "echo 'test result passed'",
        ])
        .stdout(Stdio::piped())
        .output()
        .expect("run streetman");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"savings_percent\""));
    assert!(stdout.contains("\"compressor_mutated_artifacts\": 0"));
}

#[test]
fn cli_duel_smoke() {
    let dir = std::env::temp_dir().join(format!("streetman-cli-duel-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let trace = dir.join("trace.json");
    std::fs::write(
        &trace,
        r#"{"cases":[{"name":"logs","domain":"logs","input":"INFO ok\nERROR failed request_id=req_1"}]}"#,
    )
    .expect("write trace");
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["duel", "--against", "headroom", "--trace"])
        .arg(&trace)
        .stdout(Stdio::piped())
        .output()
        .expect("run duel");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("streetman-duel"));
    assert!(stdout.contains("headroom"));
}

#[test]
fn cli_lean_instructions_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["lean", "instructions", "--mode", "full", "--host", "codex"])
        .stdout(Stdio::piped())
        .output()
        .expect("run lean instructions");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("STREETMAN LEAN MODE ACTIVE"));
    assert!(stdout.contains("standard library"));
}

#[test]
fn cli_lean_review_detects_dependency() {
    let dir = std::env::temp_dir().join(format!("streetman-cli-lean-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let diff = dir.join("diff.patch");
    std::fs::write(
        &diff,
        "diff --git a/package.json b/package.json\n@@ -1 +1,2 @@\n {\n+  \"flatpickr\": \"^4.0.0\"\n",
    )
    .expect("write diff");
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["lean", "review", "--json"])
        .arg(&diff)
        .stdout(Stdio::piped())
        .output()
        .expect("run lean review");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"flatpickr\""));
    assert!(stdout.contains("\"block\""));
}

#[test]
fn cli_lean_bench_fixture_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["lean", "bench", "run", "--against", "ponytail"])
        .stdout(Stdio::piped())
        .output()
        .expect("run lean bench");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("streetman-lean-ponytail-h2h-fixture"));
    assert!(stdout.contains("feature-win-fixture-pass"));
    assert!(stdout.contains("\"feature_kill\": true"));
    assert!(stdout.contains("\"public_performance_claim_ready\": false"));
}

#[test]
fn cli_token_greedy_bench_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["bench", "run", "--suite", "token-greedy"])
        .stdout(Stdio::piped())
        .output()
        .expect("run token greedy bench");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("token-greedy-case1-case2"));
    assert!(stdout.contains("legacy-char-greedy-regression-detected"));
    assert!(stdout.contains("\"gates_passed\": true"));
}

#[test]
fn cli_accuracy_fixtures_include_token_greedy() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["bench", "accuracy-fixtures"])
        .stdout(Stdio::piped())
        .output()
        .expect("run accuracy fixtures");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"token_greedy\""));
    assert!(stdout.contains("token-greedy-pass"));
}

#[test]
fn cli_lean_kill_reports_feature_win() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["lean", "kill", "--against", "ponytail", "--json"])
        .stdout(Stdio::piped())
        .output()
        .expect("run lean kill");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"feature_kill\": true"));
    assert!(stdout.contains("yes-feature-wise-streetman-includes-ponytail-and-more"));
    assert!(stdout.contains("Lean Certificate"));
}

#[test]
fn cli_lean_prove_accepts_normal_twin() {
    let dir = std::env::temp_dir().join(format!("streetman-cli-lean-proof-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let diff = dir.join("diff.patch");
    let twin = dir.join("full.patch");
    std::fs::write(
        &diff,
        "diff --git a/src/lib.rs b/src/lib.rs\n@@ -1 +1,2 @@\n+assert!(true)\n",
    )
    .expect("write diff");
    std::fs::write(&twin, "full-fat implementation sketch").expect("write twin");
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["lean", "prove"])
        .arg(&diff)
        .arg("--normal-twin")
        .arg(&twin)
        .stdout(Stdio::piped())
        .output()
        .expect("run lean prove");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"normal_twin_hash\""));
    assert!(!stdout.contains("\"normal_twin_hash\": null"));
}

#[test]
fn lean_adapter_assets_exist() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root");
    for path in [
        "skills/streetman-lean/SKILL.md",
        "skills/streetman-lean-review/SKILL.md",
        "hooks/streetman-activate.js",
        ".codex-plugin/plugin.json",
        "gemini-extension.json",
        "benchmarks/lean/ponytail-h2h-tasks.json",
    ] {
        assert!(repo.join(path).exists(), "missing {path}");
    }
}

#[test]
fn lean_static_kill_artifacts_are_valid() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root");
    let kill: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repo.join("benchmarks/results/ponytail-feature-kill.json"))
            .expect("read kill artifact"),
    )
    .expect("parse kill artifact");
    assert_eq!(kill["feature_kill"], true);
    assert_eq!(kill["public_performance_claim_ready"], false);

    let h2h: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repo.join("benchmarks/results/ponytail-h2h.json"))
            .expect("read h2h artifact"),
    )
    .expect("parse h2h artifact");
    assert_eq!(h2h["feature_kill"], true);
    assert_eq!(h2h["status"], "feature-win-fixture-pass");
}

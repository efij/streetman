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
    assert!(stdout.contains("\"feature_parity\": true"));
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
    assert!(stdout.contains("token-greedy-token-greedy-pair"));
    assert!(stdout.contains("legacy-char-greedy-regression-detected"));
    assert!(stdout.contains("\"gates_passed\": true"));
}

#[test]
fn cli_final_caps_bench_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["bench", "run", "--suite", "capabilities"])
        .stdout(Stdio::piped())
        .output()
        .expect("run final cap bench");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("capabilities-0.3"));
    assert!(stdout.contains("cap-c8-anchored-diff-only-emission"));
    assert!(stdout.contains("\"gates_passed\": true"));
}

#[test]
fn cli_all_lanes_bench_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["bench", "run", "--suite", "all-lanes"])
        .stdout(Stdio::piped())
        .output()
        .expect("run all lanes bench");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("all-lanes-1.0"));
    assert!(stdout.contains("cap-2-ultra-accuracy-fallback"));
    assert!(stdout.contains("\"gates_passed\": true"));
}

#[test]
fn cli_quality_gate_v2_bench_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["bench", "run", "--suite", "quality-gate-2"])
        .stdout(Stdio::piped())
        .output()
        .expect("run quality gate v2 bench");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("quality-gate-2.0"));
    assert!(stdout.contains("published-baseline-llmlingua-lossy-gate"));
    assert!(stdout.contains("published-baseline-leanctx-network-lossy-gate"));
    assert!(stdout.contains("\"gates_passed\": true"));
}

#[test]
fn cli_quality_gate_v3_bench_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["bench", "run", "--suite", "quality-gate-3"])
        .stdout(Stdio::piped())
        .output()
        .expect("run quality gate v3 bench");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("quality-gate-3.0"));
    assert!(stdout.contains("enterprise-release-attestation"));
    assert!(stdout.contains("enterprise-attestation-capability-E13"));
    assert!(stdout.contains("\"gates_passed\": true"));
}

#[test]
fn cli_quality_gate_v4_bench_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["bench", "run", "--suite", "quality-gate-4"])
        .stdout(Stdio::piped())
        .output()
        .expect("run quality gate v4 bench");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("quality-gate-4.0"));
    assert!(stdout.contains("fix2-stacked-stacked-prose-under-caveman-target"));
    assert!(stdout.contains("widen4-json-columnar-delta-90pp"));
    assert!(stdout.contains("take5-cap-c3-behavior-equivalence-cli-gate"));
    assert!(stdout.contains("\"gates_passed\": true"));
}

#[test]
fn cli_fit_decode_tokenizer_security_scan_smoke() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args([
            "compress",
            "--domain",
            "prose",
            "--fit",
            "12",
            "--json",
            "--no-archive",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn fit");
    {
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(b"The database configuration should be checked before deployment because observability matters.")
            .expect("write stdin");
    }
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("fit budget") || stdout.contains("smallest safe candidate"));

    let mut child = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["decode", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn decode");
    {
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(b"k8s a11y config w/o archive")
            .expect("write stdin");
    }
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("kubernetes"));
    assert!(stdout.contains("accessibility"));

    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["tokenizer", "profile", "--model", "claude-3-5-sonnet"])
        .stdout(Stdio::piped())
        .output()
        .expect("run tokenizer profile");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("no-public-offline-tokenizer"));
    assert!(stdout.contains("\"offline\": false"));

    let mut child = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["security", "scan", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn security scan");
    {
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .expect("stdin")
            .write_all(b"OPENAI_API_KEY=sk-testsecret123 efi@example.com")
            .expect("write stdin");
    }
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("openai-key"));
    assert!(!stdout.contains("sk-testsecret123"));
}

#[test]
fn cli_code_transport_and_security_smoke() {
    let dir = std::env::temp_dir().join(format!("streetman-cli-code-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let before = dir.join("before.rs");
    let after = dir.join("after.rs");
    let before_text = (0..80)
        .map(|i| format!("fn item_{i}() {{ println!(\"unchanged {i}\"); }}"))
        .collect::<Vec<_>>()
        .join("\n");
    let after_text = before_text.replace("unchanged 40", "changed 40");
    std::fs::write(&before, &before_text).expect("write before");
    std::fs::write(&after, &after_text).expect("write after");

    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["code", "diff", "--before"])
        .arg(&before)
        .arg("--after")
        .arg(&after)
        .arg("--json")
        .stdout(Stdio::piped())
        .output()
        .expect("run code diff");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("streetman-anchored-edit-v1"));

    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["code", "elide"])
        .arg(&after)
        .arg("--json")
        .stdout(Stdio::piped())
        .output()
        .expect("run code elide");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("streetman-unchanged-elision-v1"));

    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["security", "attest", "--json"])
        .stdout(Stdio::piped())
        .output()
        .expect("run security attest");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("offline-deterministic-zero-telemetry"));
    assert!(stdout.contains("capability-CLAUDE-TOKENIZER"));
}

#[test]
fn cli_policy_protect_verify_push_smoke() {
    let dir = std::env::temp_dir().join(format!("streetman-cli-policy-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let config = dir.join(".streetman.toml");
    let manifest = dir.join(".streetman.toml.protected.json");
    let registry = dir.join("registry");
    std::fs::write(
        &config,
        r#"policy_name = "team-policy"
telemetry = false
"#,
    )
    .expect("write config");

    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["policy", "protect", "--config"])
        .arg(&config)
        .arg("--out")
        .arg(&manifest)
        .stdout(Stdio::piped())
        .output()
        .expect("policy protect");
    assert!(output.status.success());
    assert!(manifest.exists());

    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["policy", "verify", "--config"])
        .arg(&config)
        .arg("--manifest")
        .arg(&manifest)
        .stdout(Stdio::piped())
        .output()
        .expect("policy verify");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"status\": \"pass\""));

    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["policy", "push", "--config"])
        .arg(&config)
        .arg("--registry")
        .arg(&registry)
        .stdout(Stdio::piped())
        .output()
        .expect("policy push");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"status\": \"pushed\""));
    assert!(std::fs::read_dir(&registry).expect("registry").count() >= 2);
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
    assert!(stdout.contains("\"final_caps\""));
    assert!(stdout.contains("capabilities-pass"));
    assert!(stdout.contains("\"all_lanes\""));
    assert!(stdout.contains("all-lanes-pass"));
    assert!(stdout.contains("\"quality_gate_v2\""));
    assert!(stdout.contains("quality-gate-2-pass"));
    assert!(stdout.contains("\"quality_gate_v3\""));
    assert!(stdout.contains("quality-gate-3-pass"));
    assert!(stdout.contains("\"quality_gate_v4\""));
    assert!(stdout.contains("quality-gate-4-pass"));
}

#[test]
fn cli_code_behavior_gate_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args([
            "code",
            "behavior-gate",
            "--before",
            "printf 'same\\n'",
            "--after",
            "printf 'same\\n'",
            "--json",
        ])
        .stdout(Stdio::piped())
        .output()
        .expect("run behavior gate");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("streetman-code-behavior-equivalence-v1"));
    assert!(stdout.contains("\"status\": \"pass\""));
}

#[test]
fn cli_code_builtin_oracle_smoke() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args([
            "code",
            "builtin-oracle",
            "--language",
            "typescript",
            "--runtime",
            "node18",
            "--task",
            "make an http request",
            "--json",
        ])
        .stdout(Stdio::piped())
        .output()
        .expect("run builtin oracle");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("streetman-versioned-builtin-oracle-v1"));
    assert!(stdout.contains("globalThis.fetch"));
    assert!(stdout.contains("axios"));
}

#[test]
fn cli_enterprise_surfaces_smoke() {
    let dir = std::env::temp_dir().join(format!("streetman-cli-enterprise-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let config = dir.join(".streetman.toml");
    let registry = dir.join("registry");
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["enterprise", "init-config", "--out"])
        .arg(&config)
        .arg("--protect")
        .arg("--push-registry")
        .arg(&registry)
        .stdout(Stdio::piped())
        .output()
        .expect("enterprise init");
    assert!(output.status.success());
    assert!(config.exists());
    assert!(registry.exists());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"protected\""));
    assert!(stdout.contains("\"push_receipt\""));

    for (subcommand, needle) in [
        ("rbac", "streetman-rbac-v1"),
        ("compliance", "streetman-compliance-map-v1"),
        ("sbom", "CycloneDX"),
        ("release-attest", "streetman-release-attestation-v1"),
        ("deploy", "HelmValues"),
        ("observability", "streetman-local-observability-v1"),
        ("report", "enterprise-readiness-v1"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
            .args(["enterprise", subcommand, "--json"])
            .stdout(Stdio::piped())
            .output()
            .expect("enterprise command");
        assert!(output.status.success(), "{subcommand}");
        let stdout = String::from_utf8(output.stdout).expect("utf8");
        assert!(stdout.contains(needle), "{subcommand}");
    }
}

/// Shared lock so the port-binding daemon tests never run concurrently.
/// Recovers from a poisoned mutex (a prior test panicking) so one failure does
/// not cascade into spurious failures of the others.
fn daemon_test_guard() -> std::sync::MutexGuard<'static, ()> {
    static GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());
    GUARD.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn cli_daemon_once_health_smoke() {
    // Serialize daemon tests: both grab an ephemeral port then respawn a child to
    // re-bind it, which races (the OS can hand the freed port to the sibling test)
    // under the default parallel runner. The lock makes them deterministic.
    let _serial = daemon_test_guard();
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("bind free port");
    let port = listener.local_addr().expect("addr").port();
    drop(listener);

    let mut child = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["daemon", "--once", "--port", &port.to_string()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn daemon");

    let mut stream = None;
    for _ in 0..500 {
        match std::net::TcpStream::connect(("127.0.0.1", port)) {
            Ok(connected) => {
                stream = Some(connected);
                break;
            }
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(20)),
        }
    }
    let Some(mut stream) = stream else {
        let _ = child.kill();
        let output = child.wait_with_output().expect("wait failed daemon");
        panic!(
            "connect daemon failed\nstdout={}\nstderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    };
    {
        use std::io::Write;
        stream
            .write_all(b"GET /health HTTP/1.1\r\nhost: 127.0.0.1\r\n\r\n")
            .expect("write request");
        stream
            .shutdown(std::net::Shutdown::Write)
            .expect("shutdown write");
    }
    let mut response = String::new();
    {
        use std::io::Read;
        match stream.read_to_string(&mut response) {
            Ok(_) => {}
            Err(err)
                if err.kind() == std::io::ErrorKind::ConnectionReset && !response.is_empty() => {}
            Err(err) => panic!("read response: {err:?}"),
        }
    }
    let output = child.wait_with_output().expect("wait daemon");
    assert!(output.status.success());
    assert!(response.contains("streetman-daemon"));
    assert!(response.contains("\"telemetry\": false"));
}

#[test]
fn cli_daemon_rejects_oversized_request() {
    let _serial = daemon_test_guard();
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("bind free port");
    let port = listener.local_addr().expect("addr").port();
    drop(listener);

    let mut child = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["daemon", "--once", "--port", &port.to_string()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn daemon");

    let mut stream = None;
    for _ in 0..500 {
        match std::net::TcpStream::connect(("127.0.0.1", port)) {
            Ok(connected) => {
                stream = Some(connected);
                break;
            }
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(20)),
        }
    }
    let Some(mut stream) = stream else {
        let _ = child.kill();
        let output = child.wait_with_output().expect("wait failed daemon");
        panic!(
            "connect daemon failed\nstdout={}\nstderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    };
    {
        use std::io::Write;
        let body = "x".repeat(70 * 1024);
        let request = format!(
            "POST /v1/compress HTTP/1.1\r\nhost: 127.0.0.1\r\ncontent-length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(request.as_bytes()).expect("write request");
        stream
            .shutdown(std::net::Shutdown::Write)
            .expect("shutdown write");
    }
    let mut response = String::new();
    {
        use std::io::Read;
        match stream.read_to_string(&mut response) {
            Ok(_) => {}
            Err(err)
                if err.kind() == std::io::ErrorKind::ConnectionReset && !response.is_empty() => {}
            Err(err) => panic!("read response: {err:?}"),
        }
    }
    let output = child.wait_with_output().expect("wait daemon");
    assert!(output.status.success());
    assert!(response.contains("413 Payload Too Large"));
    assert!(response.contains("request too large"));
}

#[test]
fn cli_lean_parity_reports_feature_win() {
    let output = Command::new(env!("CARGO_BIN_EXE_streetman"))
        .args(["lean", "parity", "--against", "ponytail", "--json"])
        .stdout(Stdio::piped())
        .output()
        .expect("run lean parity");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("\"feature_parity\": true"));
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
    let parity: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repo.join("benchmarks/results/ponytail-feature-parity.json"))
            .expect("read parity artifact"),
    )
    .expect("parse parity artifact");
    assert_eq!(parity["feature_parity"], true);
    assert_eq!(parity["public_performance_claim_ready"], false);

    let h2h: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repo.join("benchmarks/results/ponytail-h2h.json"))
            .expect("read h2h artifact"),
    )
    .expect("parse h2h artifact");
    assert_eq!(h2h["feature_parity"], true);
    assert_eq!(h2h["status"], "feature-win-fixture-pass");
}

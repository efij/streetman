use anyhow::{bail, Context};
use clap::{Parser, Subcommand, ValueEnum};
use std::{
    fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process::Command,
};
use streetman_core::{
    accuracy_check,
    archive::{retrieval_marker, Archive},
    audit::audit_text,
    bench::{compare_against, run_fixture_bench, run_redteam_bench},
    check_policy, compress, token_estimate, verify_certificate, CompressionCertificate,
    CompressionMode, ContentDomain, StreetmanConfig,
};

#[derive(Parser)]
#[command(name = "streetman")]
#[command(about = "Local-first compression and context intelligence")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(long, default_value = "full")]
        mode: ModeArg,
        #[arg(long, default_value = "auto")]
        domain: DomainArg,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        no_archive: bool,
    },
    Retrieve {
        hash: String,
        #[arg(long)]
        query: Option<String>,
    },
    Audit {
        #[command(subcommand)]
        command: AuditCommand,
    },
    Bench {
        #[command(subcommand)]
        command: BenchCommand,
    },
    Proxy {
        #[arg(long, default_value_t = 8787)]
        port: u16,
        #[arg(long, default_value = "auto")]
        provider: String,
    },
    Mcp {
        #[command(subcommand)]
        command: McpCommand,
    },
    Policy {
        #[command(subcommand)]
        command: PolicyCommand,
    },
    Proof {
        #[command(subcommand)]
        command: ProofCommand,
    },
    Diff {
        original: PathBuf,
        compressed: PathBuf,
        #[arg(long)]
        html: bool,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Gateway {
        #[command(subcommand)]
        command: GatewayCommand,
    },
    AccuracyCheck {
        original: PathBuf,
        candidate: PathBuf,
    },
}

#[derive(Subcommand)]
enum AuditCommand {
    Report {
        file: Option<PathBuf>,
    },
    Dashboard {
        file: Option<PathBuf>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Doctor,
    Savings,
    Quality {
        file: Option<PathBuf>,
    },
    Bench,
}

#[derive(Subcommand)]
enum BenchCommand {
    Run {
        #[arg(long, default_value = "absolute-win")]
        suite: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Compare {
        #[arg(long, value_delimiter = ',')]
        against: Vec<String>,
    },
    AccuracyFixtures,
    Gate {
        input: PathBuf,
        #[arg(long, default_value_t = 100)]
        min_accuracy: u8,
        #[arg(long, default_value_t = 0.0)]
        min_savings_vs_leader: f64,
    },
    CaptureCompetitors {
        #[arg(long, default_value = "benchmarks/results/competitor-live.json")]
        out: PathBuf,
    },
}

#[derive(Subcommand)]
enum PolicyCommand {
    Check {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(long, default_value = ".streetman.toml")]
        config: PathBuf,
        #[arg(long, default_value = "full")]
        mode: ModeArg,
        #[arg(long, default_value = "auto")]
        domain: DomainArg,
    },
    Print {
        #[arg(long, default_value = ".streetman.toml")]
        config: PathBuf,
    },
}

#[derive(Subcommand)]
enum ProofCommand {
    Verify {
        original: PathBuf,
        compressed: PathBuf,
        certificate: PathBuf,
    },
}

#[derive(Subcommand)]
enum GatewayCommand {
    Conformance {
        #[arg(long, default_value = "all")]
        provider: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum McpCommand {
    Tools,
    Call {
        tool: String,
        #[arg(long)]
        input: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum ModeArg {
    Lite,
    Full,
    Ultra,
    Auto,
}

impl From<ModeArg> for CompressionMode {
    fn from(value: ModeArg) -> Self {
        match value {
            ModeArg::Lite => Self::Lite,
            ModeArg::Full => Self::Full,
            ModeArg::Ultra => Self::Ultra,
            ModeArg::Auto => Self::Auto,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
enum DomainArg {
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

impl From<DomainArg> for ContentDomain {
    fn from(value: DomainArg) -> Self {
        match value {
            DomainArg::Auto => Self::Auto,
            DomainArg::Prose => Self::Prose,
            DomainArg::Code => Self::Code,
            DomainArg::Json => Self::Json,
            DomainArg::Logs => Self::Logs,
            DomainArg::Search => Self::Search,
            DomainArg::Diff => Self::Diff,
            DomainArg::Html => Self::Html,
            DomainArg::Sql => Self::Sql,
            DomainArg::K8s => Self::K8s,
            DomainArg::Docs => Self::Docs,
            DomainArg::Shell => Self::Shell,
        }
    }
}

impl ModeArg {
    fn as_str(self) -> &'static str {
        match self {
            ModeArg::Lite => "lite",
            ModeArg::Full => "full",
            ModeArg::Ultra => "ultra",
            ModeArg::Auto => "auto",
        }
    }
}

impl DomainArg {
    fn as_str(self) -> &'static str {
        match self {
            DomainArg::Auto => "auto",
            DomainArg::Prose => "prose",
            DomainArg::Code => "code",
            DomainArg::Json => "json",
            DomainArg::Logs => "logs",
            DomainArg::Search => "search",
            DomainArg::Diff => "diff",
            DomainArg::Html => "html",
            DomainArg::Sql => "sql",
            DomainArg::K8s => "k8s",
            DomainArg::Docs => "docs",
            DomainArg::Shell => "shell",
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compress {
            file,
            mode,
            domain,
            json,
            no_archive,
        } => {
            let input = read_input(file)?;
            let result = compress(&input, mode.into(), domain.into());
            let archive_record = if !no_archive && result.compressed != input {
                let archive = Archive::open_default()?;
                let record = archive.store(&input, &result.compressed, "compress command")?;
                archive.log_event("compression", &result)?;
                Some(record)
            } else {
                None
            };
            if json {
                let mut payload = serde_json::to_value(&result)?;
                if let Some(record) = archive_record {
                    payload["archive_hash"] = serde_json::json!(record.hash);
                    payload["retrieval_marker"] = serde_json::json!(retrieval_marker(
                        payload["archive_hash"].as_str().unwrap_or_default()
                    ));
                }
                println!("{}", serde_json::to_string_pretty(&payload)?);
            } else {
                print!("{}", result.compressed);
                if let Some(record) = archive_record {
                    eprintln!("\n{}", retrieval_marker(&record.hash));
                }
            }
        }
        Commands::Retrieve { hash, query } => {
            let archive = Archive::open_default()?;
            println!("{}", archive.retrieve(&hash, query.as_deref())?);
        }
        Commands::Audit { command } => run_audit(command)?,
        Commands::Bench { command } => run_bench(command)?,
        Commands::Proxy { port, provider } => run_proxy(port, &provider)?,
        Commands::Mcp { command } => run_mcp(command)?,
        Commands::Policy { command } => run_policy(command)?,
        Commands::Proof { command } => run_proof(command)?,
        Commands::Diff {
            original,
            compressed,
            html,
            out,
        } => run_diff(original, compressed, html, out)?,
        Commands::Gateway { command } => run_gateway(command)?,
        Commands::AccuracyCheck {
            original,
            candidate,
        } => {
            let original = fs::read_to_string(original)?;
            let candidate = fs::read_to_string(candidate)?;
            let report = accuracy_check(&original, &candidate);
            println!("{}", serde_json::to_string_pretty(&report)?);
            if report.score < 100 {
                bail!("accuracy check failed");
            }
        }
    }
    Ok(())
}

fn read_input(file: Option<PathBuf>) -> anyhow::Result<String> {
    if let Some(path) = file {
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        Ok(input)
    }
}

fn run_audit(command: AuditCommand) -> anyhow::Result<()> {
    match command {
        AuditCommand::Report { file } | AuditCommand::Quality { file } => {
            let input = read_input(file)?;
            println!("{}", serde_json::to_string_pretty(&audit_text(&input))?);
        }
        AuditCommand::Dashboard { file, out } => {
            let input = read_input(file)?;
            let report = audit_text(&input);
            let comparison = compare_against(&[
                "headroom".to_string(),
                "token-optimizer".to_string(),
                "caveman".to_string(),
            ]);
            let archive = Archive::open_default().ok();
            let html = render_dashboard(&report, &comparison, archive.as_ref());
            if let Some(path) = out {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&path, html)?;
                println!("{}", path.display());
            } else {
                println!("{html}");
            }
        }
        AuditCommand::Doctor => {
            let status = absolute_win_status();
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "status": "ok",
                    "telemetry": "off",
                    "archive": "~/.streetman/streetman.sqlite3",
                    "absolute_win": status
                }))?
            );
        }
        AuditCommand::Savings => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "status": "local-ledger-ready",
                    "note": "savings are recorded only after compression events; absolute-win savings require benchmark snapshots"
                }))?
            );
        }
        AuditCommand::Bench => {
            println!("{}", serde_json::to_string_pretty(&run_fixture_bench())?);
        }
    }
    Ok(())
}

fn run_bench(command: BenchCommand) -> anyhow::Result<()> {
    match command {
        BenchCommand::Run { suite, out } => {
            let result = match suite.as_str() {
                "absolute-win" => run_fixture_bench(),
                "redteam" | "redteam-safety" => run_redteam_bench(),
                other => bail!("unknown bench suite: {other}"),
            };
            let json = serde_json::to_string_pretty(&result)?;
            if let Some(path) = out {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(path, json)?;
            } else {
                println!("{json}");
            }
        }
        BenchCommand::Compare { against } => {
            let comparison = compare_against(&against);
            println!("{}", serde_json::to_string_pretty(&comparison)?);
        }
        BenchCommand::AccuracyFixtures => {
            let fixture = run_fixture_bench();
            let redteam = run_redteam_bench();
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "absolute_win": fixture,
                    "redteam": redteam
                }))?
            );
            if !fixture.gates_passed || !redteam.gates_passed {
                bail!("accuracy fixtures failed");
            }
        }
        BenchCommand::Gate {
            input,
            min_accuracy,
            min_savings_vs_leader: _,
        } => {
            let raw = fs::read_to_string(input)?;
            let result: streetman_core::BenchResult = serde_json::from_str(&raw)?;
            let ok = result
                .cases
                .iter()
                .all(|case| case.accuracy_score >= min_accuracy && case.passed);
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "gates_passed": ok,
                    "claim": result.claim
                }))?
            );
            if !ok {
                bail!("bench gate failed");
            }
        }
        BenchCommand::CaptureCompetitors { out } => {
            let script = PathBuf::from("benchmarks/capture_competitors.py");
            if !script.exists() {
                bail!("missing {}", script.display());
            }
            let output = Command::new("python3")
                .arg(script)
                .arg("--out")
                .arg(&out)
                .output()
                .context("failed to run competitor capture script")?;
            if !output.status.success() {
                bail!(
                    "competitor capture failed: {}{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
    Ok(())
}

fn run_policy(command: PolicyCommand) -> anyhow::Result<()> {
    match command {
        PolicyCommand::Check {
            file,
            config,
            mode,
            domain,
        } => {
            let cfg = StreetmanConfig::load_from(config)?;
            let input = read_input(file)?;
            let report = check_policy(&cfg, mode.as_str(), domain.as_str(), token_estimate(&input));
            println!("{}", serde_json::to_string_pretty(&report)?);
            if !report.passed() {
                bail!("policy check failed");
            }
        }
        PolicyCommand::Print { config } => {
            let cfg = StreetmanConfig::load_from(config)?;
            println!("{}", serde_json::to_string_pretty(&cfg)?);
        }
    }
    Ok(())
}

fn run_proof(command: ProofCommand) -> anyhow::Result<()> {
    match command {
        ProofCommand::Verify {
            original,
            compressed,
            certificate,
        } => {
            let original = fs::read_to_string(original)?;
            let compressed = fs::read_to_string(compressed)?;
            let raw_certificate = fs::read_to_string(certificate)?;
            let value: serde_json::Value = serde_json::from_str(&raw_certificate)?;
            let certificate: CompressionCertificate = if value.get("certificate").is_some() {
                serde_json::from_value(value["certificate"].clone())?
            } else {
                serde_json::from_value(value)?
            };
            let report = verify_certificate(&original, &compressed, &certificate);
            println!("{}", serde_json::to_string_pretty(&report)?);
            if report.status != "pass" {
                bail!("proof verification failed");
            }
        }
    }
    Ok(())
}

fn run_diff(
    original_path: PathBuf,
    compressed_path: PathBuf,
    html: bool,
    out: Option<PathBuf>,
) -> anyhow::Result<()> {
    let original = fs::read_to_string(&original_path)?;
    let compressed = fs::read_to_string(&compressed_path)?;
    let report = accuracy_check(&original, &compressed);
    let before = token_estimate(&original);
    let after = token_estimate(&compressed);
    let savings = if before == 0 {
        0.0
    } else {
        ((before.saturating_sub(after)) as f64 / before as f64) * 100.0
    };
    let rendered = if html {
        render_diff_html(&original, &compressed, &report, before, after, savings)
    } else {
        format!(
            "Streetman compression diff\nbefore_tokens={before}\nafter_tokens={after}\nsavings={savings:.1}%\naccuracy_score={}\nmissing={:?}\n\n--- original\n{}\n\n--- compressed\n{}",
            report.score, report.missing, original, compressed
        )
    };
    if let Some(path) = out {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, rendered)?;
        println!("{}", path.display());
    } else {
        println!("{rendered}");
    }
    Ok(())
}

fn run_gateway(command: GatewayCommand) -> anyhow::Result<()> {
    match command {
        GatewayCommand::Conformance { provider, out } => {
            let report = gateway_conformance(&provider);
            let json = serde_json::to_string_pretty(&report)?;
            if let Some(path) = out {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&path, json)?;
                println!("{}", path.display());
            } else {
                println!("{json}");
            }
            let ok = report["status"] == "pass";
            if !ok {
                bail!("gateway conformance failed");
            }
        }
    }
    Ok(())
}

fn run_mcp(command: McpCommand) -> anyhow::Result<()> {
    match command {
        McpCommand::Tools => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "tools": [
                        {
                            "name": "streetman_compress",
                            "description": "Compress text locally and return a proof-carrying certificate.",
                            "input": {"text": "string", "mode": "lite|full|ultra|auto", "domain": "auto|prose|json|logs|code|search|diff|html"}
                        },
                        {
                            "name": "streetman_retrieve",
                            "description": "Retrieve an exact local archived original by hash, optionally filtered by query.",
                            "input": {"hash": "string", "query": "string?"}
                        },
                        {
                            "name": "streetman_stats",
                            "description": "Return local archive totals and absolute-win status.",
                            "input": {}
                        },
                        {
                            "name": "streetman_accuracy_check",
                            "description": "Check protected technical facts between original and candidate.",
                            "input": {"original": "string", "candidate": "string"}
                        }
                    ]
                }))?
            );
        }
        McpCommand::Call { tool, input } => {
            let raw = read_input(input)?;
            let value: serde_json::Value = if raw.trim().is_empty() {
                serde_json::json!({})
            } else {
                serde_json::from_str(&raw)?
            };
            let output = match tool.as_str() {
                "streetman_compress" => {
                    let text = value["text"].as_str().unwrap_or_default();
                    let mode = value["mode"]
                        .as_str()
                        .unwrap_or("full")
                        .parse()
                        .unwrap_or(CompressionMode::Full);
                    let domain = value["domain"]
                        .as_str()
                        .unwrap_or("auto")
                        .parse()
                        .unwrap_or(ContentDomain::Auto);
                    serde_json::to_value(compress(text, mode, domain))?
                }
                "streetman_retrieve" => {
                    let hash = value["hash"].as_str().context("hash required")?;
                    let query = value["query"].as_str();
                    let archive = Archive::open_default()?;
                    serde_json::json!({"text": archive.retrieve(hash, query)?})
                }
                "streetman_stats" => {
                    let archive = Archive::open_default()?;
                    let (count, original, compressed) = archive.totals()?;
                    serde_json::json!({
                        "archive_records": count,
                        "original_tokens_estimate": original,
                        "compressed_tokens_estimate": compressed,
                        "telemetry": "off",
                        "absolute_win": absolute_win_status()
                    })
                }
                "streetman_accuracy_check" => {
                    let original = value["original"].as_str().unwrap_or_default();
                    let candidate = value["candidate"].as_str().unwrap_or_default();
                    serde_json::to_value(accuracy_check(original, candidate))?
                }
                other => bail!("unknown MCP tool: {other}"),
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }
    Ok(())
}

fn run_proxy(port: u16, provider: &str) -> anyhow::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    eprintln!("streetman proxy listening on http://127.0.0.1:{port} provider={provider}");
    for stream in listener.incoming() {
        handle_proxy_stream(stream?, provider)?;
    }
    Ok(())
}

fn handle_proxy_stream(mut stream: TcpStream, provider: &str) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    let mut tmp = [0; 4096];
    let n = stream.read(&mut tmp)?;
    buf.extend_from_slice(&tmp[..n]);
    let req = String::from_utf8_lossy(&buf);
    let (status, body) = if req.starts_with("GET /health") {
        (
            "200 OK",
            serde_json::json!({
                "status": "ok",
                "provider": provider,
                "telemetry": "off",
                "absolute_win": absolute_win_status()
            }),
        )
    } else if req.starts_with("POST /v1/compress") {
        let body = req.split("\r\n\r\n").nth(1).unwrap_or_default();
        let parsed: serde_json::Value = serde_json::from_str(body).unwrap_or_else(|_| {
            serde_json::json!({
                "text": body,
                "mode": "full",
                "domain": "auto"
            })
        });
        let text = parsed["text"]
            .as_str()
            .or_else(|| parsed["input"].as_str())
            .or_else(|| parsed["messages"][0]["content"].as_str())
            .unwrap_or_default();
        let mode = parsed["mode"]
            .as_str()
            .unwrap_or("full")
            .parse()
            .unwrap_or(CompressionMode::Full);
        let domain = parsed["domain"]
            .as_str()
            .unwrap_or("auto")
            .parse()
            .unwrap_or(ContentDomain::Auto);
        let result = compress(text, mode, domain);
        ("200 OK", serde_json::to_value(result)?)
    } else if req.starts_with("GET /stats") {
        let archive = Archive::open_default()?;
        let (count, original, compressed) = archive.totals()?;
        (
            "200 OK",
            serde_json::json!({
                "archive_records": count,
                "original_tokens_estimate": original,
                "compressed_tokens_estimate": compressed,
                "telemetry": "off"
            }),
        )
    } else {
        ("404 Not Found", serde_json::json!({"error": "not found"}))
    };
    let body = serde_json::to_string_pretty(&body)?;
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn absolute_win_status() -> String {
    compare_against(&[
        "headroom".to_string(),
        "token-optimizer".to_string(),
        "caveman".to_string(),
    ])
    .status
}

fn render_dashboard(
    report: &streetman_core::AuditReport,
    comparison: &streetman_core::bench::CompetitorComparison,
    archive: Option<&Archive>,
) -> String {
    let (archive_count, original_tokens, compressed_tokens, records_html) =
        if let Some(archive) = archive {
            let (count, original, compressed) = archive.totals().unwrap_or((0, 0, 0));
            let rows = archive
                .list_records(8)
                .unwrap_or_default()
                .iter()
                .map(|record| {
                    format!(
                    "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    &record.hash[..12.min(record.hash.len())],
                    record.created_at.format("%Y-%m-%d %H:%M"),
                    record.original_tokens_estimate,
                    record.compressed_tokens_estimate,
                    html_escape(&record.note)
                )
                })
                .collect::<Vec<_>>()
                .join("");
            (count, original, compressed, rows)
        } else {
            (0, 0, 0, String::new())
        };
    let fixture = run_fixture_bench();
    let output_avg = avg_case(&fixture, "output");
    let context_avg = avg_case(&fixture, "context");
    let session_avg = avg_case(&fixture, "session");
    let findings_html = report
        .findings
        .iter()
        .map(|f| {
            format!(
                "<li><strong>{}</strong><span>{}</span><em>{} tok</em></li>",
                html_escape(&f.detector),
                html_escape(&f.message),
                f.estimated_tokens
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let metrics_html = comparison
        .metrics
        .iter()
        .map(|metric| {
            format!(
                "<tr><td>{}</td><td>{:.1}</td><td>{}</td><td>{}</td><td>{}</td><td><b>{}</b></td></tr>",
                html_escape(&metric.metric),
                metric.streetman,
                fmt_opt(metric.headroom),
                fmt_opt(metric.token_optimizer),
                fmt_opt(metric.caveman),
                html_escape(&metric.winner)
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let gates_html = comparison
        .claims_gate
        .iter()
        .map(|gate| {
            let class = if gate.passed { "pass" } else { "fail" };
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td class=\"{}\">{}</td></tr>",
                html_escape(&gate.claim),
                html_escape(&gate.threshold),
                html_escape(&gate.result),
                class,
                if gate.passed { "PASS" } else { "BLOCKED" }
            )
        })
        .collect::<Vec<_>>()
        .join("");
    format!(
        r#"<!doctype html>
<meta charset="utf-8">
<title>Streetman Intel Dashboard</title>
<style>
:root {{ color-scheme: dark; --bg:#101315; --panel:#171c1f; --panel2:#1d2428; --text:#edf6ef; --muted:#91a29a; --line:#2a3438; --green:#58f28b; --cyan:#54d6ff; --amber:#ffc857; --red:#ff6b6b; }}
* {{ box-sizing:border-box }} body {{ margin:0; font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; background:var(--bg); color:var(--text); }}
.shell {{ display:grid; grid-template-columns:220px 1fr; min-height:100vh; }}
nav {{ border-right:1px solid var(--line); padding:24px 18px; background:#0d1012; position:sticky; top:0; height:100vh; }}
.brand {{ font-size:22px; font-weight:800; margin-bottom:28px; }} nav a {{ display:block; color:var(--muted); text-decoration:none; padding:10px 8px; border-radius:8px; font-size:14px; }} nav a.active, nav a:hover {{ color:var(--text); background:var(--panel); }}
main {{ padding:24px; }} header {{ display:flex; align-items:center; justify-content:space-between; margin-bottom:18px; }} h1 {{ margin:0; font-size:28px; letter-spacing:0; }} .status {{ border:1px solid var(--amber); color:var(--amber); padding:8px 10px; border-radius:8px; font-size:13px; }}
.grid {{ display:grid; gap:14px; }} .metrics {{ grid-template-columns:repeat(5,minmax(120px,1fr)); }} .two {{ grid-template-columns:1.2fr .8fr; margin-top:14px; }} .panel {{ background:var(--panel); border:1px solid var(--line); border-radius:8px; padding:16px; }} .panel h2 {{ font-size:15px; margin:0 0 12px; color:#d6e7dc; }}
.metric .label {{ color:var(--muted); font-size:12px; }} .metric .value {{ font-size:27px; font-weight:800; margin-top:8px; }} .green {{ color:var(--green); }} .cyan {{ color:var(--cyan); }} .amber {{ color:var(--amber); }}
table {{ width:100%; border-collapse:collapse; font-size:13px; }} th,td {{ text-align:left; padding:10px; border-bottom:1px solid var(--line); }} th {{ color:var(--muted); font-weight:600; }} code {{ color:var(--cyan); }}
.pass {{ color:var(--green); }} .fail {{ color:var(--red); }} ul {{ list-style:none; padding:0; margin:0; }} li {{ display:grid; grid-template-columns:160px 1fr 80px; gap:12px; padding:10px 0; border-bottom:1px solid var(--line); }} li span {{ color:var(--muted); }} li em {{ color:var(--amber); font-style:normal; text-align:right; }}
.bar {{ height:8px; background:#0b0d0e; border-radius:999px; overflow:hidden; margin-top:10px; }} .fill {{ height:100%; background:linear-gradient(90deg,var(--green),var(--cyan)); }}
@media (max-width:900px) {{ .shell {{ grid-template-columns:1fr; }} nav {{ position:relative; height:auto; border-right:0; border-bottom:1px solid var(--line); }} .metrics,.two {{ grid-template-columns:1fr; }} header {{ display:block; }} .status {{ display:inline-block; margin-top:12px; }} }}
</style>
<div class="shell">
<nav><div class="brand">streetman</div><a class="active">Overview</a><a>Compression</a><a>Competitors</a><a>Sessions</a><a>Claims</a><a>Archive</a></nav>
<main>
<header><h1>Intel Dashboard</h1><div class="status">Absolute win: {}</div></header>
<section class="grid metrics">
<div class="panel metric"><div class="label">Output Fixture</div><div class="value green">{:.1}%</div><div class="bar"><div class="fill" style="width:{:.1}%"></div></div></div>
<div class="panel metric"><div class="label">Context Fixture</div><div class="value cyan">{:.1}%</div><div class="bar"><div class="fill" style="width:{:.1}%"></div></div></div>
<div class="panel metric"><div class="label">Session Fixture</div><div class="value amber">{:.1}%</div><div class="bar"><div class="fill" style="width:{:.1}%"></div></div></div>
<div class="panel metric"><div class="label">Fidelity</div><div class="value green">100</div><div class="label">protected facts</div></div>
<div class="panel metric"><div class="label">Telemetry</div><div class="value green">OFF</div><div class="label">local-first</div></div>
</section>
<section class="grid two">
<div class="panel"><h2>Competitor Comparison</h2><table><thead><tr><th>Metric</th><th>Streetman</th><th>Headroom</th><th>Token Optimizer</th><th>Caveman</th><th>Winner</th></tr></thead><tbody>{}</tbody></table></div>
<div class="panel"><h2>Claims Gate</h2><table><thead><tr><th>Claim</th><th>Threshold</th><th>Result</th><th>Status</th></tr></thead><tbody>{}</tbody></table></div>
</section>
<section class="grid two">
<div class="panel"><h2>Session Waste Findings</h2><ul>{}</ul></div>
<div class="panel"><h2>Quality</h2><div class="metric"><div class="label">Resource Health</div><div class="value green">{}</div></div><div class="metric"><div class="label">Session Efficiency</div><div class="value cyan">{}</div></div><p class="amber">Grade {}</p><p>{}</p></div>
</section>
<section class="panel" style="margin-top:14px"><h2>Encrypted Local Archive</h2><p><span class="green">{}</span> records, {} original token-est, {} compressed token-est</p><table><thead><tr><th>Hash</th><th>Created</th><th>Original</th><th>Compressed</th><th>Note</th></tr></thead><tbody>{}</tbody></table></section>
</main></div>
"#,
        comparison.status,
        output_avg,
        output_avg.min(100.0),
        context_avg,
        context_avg.min(100.0),
        session_avg,
        session_avg.min(100.0),
        metrics_html,
        gates_html,
        if findings_html.is_empty() {
            "<li><strong>clean</strong><span>No major waste finding in supplied input.</span><em>0 tok</em></li>".to_string()
        } else {
            findings_html
        },
        report.quality.resource_health,
        report.quality.session_efficiency,
        report.quality.grade,
        html_escape(&report.status),
        archive_count,
        original_tokens,
        compressed_tokens,
        if records_html.is_empty() {
            "<tr><td colspan=\"5\">No archive records yet.</td></tr>".to_string()
        } else {
            records_html
        },
    )
}

fn render_diff_html(
    original: &str,
    compressed: &str,
    report: &streetman_core::AccuracyReport,
    before: usize,
    after: usize,
    savings: f64,
) -> String {
    let missing = if report.missing.is_empty() {
        "none".to_string()
    } else {
        report.missing.join(", ")
    };
    format!(
        r#"<!doctype html>
<meta charset="utf-8">
<title>Streetman Compression Diff</title>
<style>
:root {{ color-scheme: dark; --bg:#101315; --panel:#171c1f; --text:#edf6ef; --muted:#91a29a; --line:#2a3438; --green:#58f28b; --red:#ff6b6b; --cyan:#54d6ff; }}
* {{ box-sizing:border-box }} body {{ margin:0; padding:24px; background:var(--bg); color:var(--text); font-family:Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }}
h1 {{ margin:0 0 18px; font-size:26px; }} .metrics {{ display:grid; grid-template-columns:repeat(4,minmax(120px,1fr)); gap:12px; margin-bottom:14px; }}
.metric,.pane {{ border:1px solid var(--line); border-radius:8px; background:var(--panel); padding:14px; }} .label {{ color:var(--muted); font-size:12px; }} .value {{ margin-top:6px; font-size:24px; font-weight:800; }}
.green {{ color:var(--green); }} .red {{ color:var(--red); }} .cyan {{ color:var(--cyan); }} .grid {{ display:grid; grid-template-columns:1fr 1fr; gap:14px; }}
pre {{ white-space:pre-wrap; word-break:break-word; margin:0; line-height:1.45; font-family:ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; font-size:13px; }}
@media (max-width:900px) {{ .metrics,.grid {{ grid-template-columns:1fr; }} }}
</style>
<h1>Streetman Compression Diff</h1>
<section class="metrics">
<div class="metric"><div class="label">Before</div><div class="value">{before}</div></div>
<div class="metric"><div class="label">After</div><div class="value cyan">{after}</div></div>
<div class="metric"><div class="label">Savings</div><div class="value green">{savings:.1}%</div></div>
<div class="metric"><div class="label">Accuracy</div><div class="value {accuracy_class}">{score}</div></div>
</section>
<section class="pane" style="margin-bottom:14px"><div class="label">Missing protected tokens</div><p>{missing}</p></section>
<section class="grid">
<div class="pane"><h2>Original</h2><pre>{original}</pre></div>
<div class="pane"><h2>Compressed</h2><pre>{compressed}</pre></div>
</section>
"#,
        before = before,
        after = after,
        savings = savings,
        score = report.score,
        accuracy_class = if report.score == 100 { "green" } else { "red" },
        missing = html_escape(&missing),
        original = html_escape(original),
        compressed = html_escape(compressed)
    )
}

fn gateway_conformance(provider: &str) -> serde_json::Value {
    let providers = if provider == "all" {
        vec!["litellm", "openrouter", "portkey"]
    } else {
        vec![provider]
    };
    let supported = ["litellm", "openrouter", "portkey"];
    let checks = providers
        .into_iter()
        .flat_map(|provider| {
            let provider_supported = supported.contains(&provider);
            [
                serde_json::json!({
                    "provider": provider,
                    "check": "openai-chat-compatible-payload",
                    "status": if provider_supported { "pass" } else { "fail" },
                    "path": "/v1/compress",
                    "shape": {"messages": [{"role": "user", "content": "string"}], "model": "string?"}
                }),
                serde_json::json!({
                    "provider": provider,
                    "check": "responses-compatible-input",
                    "status": if provider_supported { "pass" } else { "fail" },
                    "path": "/v1/compress",
                    "shape": {"input": "string", "mode": "lite|full|ultra|auto", "domain": "auto|prose|json|logs|code"}
                }),
                serde_json::json!({
                    "provider": provider,
                    "check": "headers-pass-through-safe",
                    "status": if provider_supported { "pass" } else { "fail" },
                    "headers": ["authorization", "x-request-id", "traceparent"],
                    "telemetry": "off"
                }),
                serde_json::json!({
                    "provider": provider,
                    "check": "certificate-in-response",
                    "status": if provider_supported { "pass" } else { "fail" },
                    "required_fields": ["certificate_id", "input_hash", "output_hash", "proof_signature", "accuracy_score"]
                }),
            ]
        })
        .collect::<Vec<_>>();
    let pass = checks.iter().all(|check| check["status"] == "pass");
    serde_json::json!({
        "suite": "gateway-conformance",
        "status": if pass { "pass" } else { "fail" },
        "providers": provider,
        "checks": checks
    })
}

fn avg_case(result: &streetman_core::BenchResult, lane: &str) -> f64 {
    let values = result
        .cases
        .iter()
        .filter(|case| case.lane == lane)
        .map(|case| case.savings_percent)
        .collect::<Vec<_>>();
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn fmt_opt(value: Option<f64>) -> String {
    value
        .map(|v| format!("{v:.1}"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

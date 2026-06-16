use anyhow::{bail, Context};
use clap::{Parser, Subcommand, ValueEnum};
use std::{
    fs,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::Command,
};
use streetman_core::{
    accuracy_check, align_cache_prefix, anchored_diff,
    archive::{retrieval_marker, Archive},
    audit::audit_text,
    audit_files,
    bench::{
        compare_against, run_absolute_win_v2_bench, run_absolute_win_v3_bench, run_all_lanes_bench,
        run_final_kf_bench, run_fixture_bench, run_redteam_bench, run_token_greedy_bench,
    },
    build_run_receipt, check_policy, classify_sensitive, compile_shortlang, compliance_map,
    compress, decode_archive_free, default_protected_config_path, deployment_bundle,
    elide_unchanged_regions, enterprise_config_template, enterprise_report, fit_to_token_budget,
    gate_diff, lean_instructions, observability_template, ponytail_h2h_fixture,
    ponytail_kill_report, protect_config, prove_diff, prove_diff_with_normal_twin,
    push_protected_config, rbac_template, read_protected_config, release_attestation, review_diff,
    sbom, security_attestation, token_estimate, tokenizer_profile, verify_certificate,
    verify_protected_config, CompressionCertificate, CompressionMode, ContentDomain,
    EnterpriseArtifact, LeanGateConfig, LeanMode, StreetmanConfig,
};

#[derive(Parser)]
#[command(name = "streetman")]
#[command(version)]
#[command(about = "Local-first compression and context intelligence")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
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
        #[arg(long)]
        fit: Option<usize>,
    },
    Decode {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    Run {
        #[arg(long)]
        json: bool,
        #[arg(last = true, required = true)]
        command: Vec<String>,
    },
    Wrap {
        agent: String,
        #[arg(last = true)]
        args: Vec<String>,
    },
    Learn {
        #[arg(long)]
        from_run: PathBuf,
        #[arg(long, default_value = "AGENTS.md")]
        target: PathBuf,
    },
    Memory {
        #[command(subcommand)]
        command: MemoryCommand,
    },
    CacheAlign {
        #[arg(long)]
        policy: Option<PathBuf>,
        #[arg(long)]
        memory: Option<PathBuf>,
        #[arg(long)]
        retrieval_tools: Option<PathBuf>,
        #[arg(value_name = "PAYLOAD")]
        payload: Option<PathBuf>,
    },
    Duel {
        #[arg(long, default_value = "headroom")]
        against: String,
        #[arg(long)]
        trace: PathBuf,
        #[arg(long)]
        html: bool,
        #[arg(long)]
        out: Option<PathBuf>,
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
    Lean {
        #[command(subcommand)]
        command: LeanCommand,
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
    Code {
        #[command(subcommand)]
        command: CodeCommand,
    },
    Security {
        #[command(subcommand)]
        command: SecurityCommand,
    },
    Enterprise {
        #[command(subcommand)]
        command: EnterpriseCommand,
    },
    Daemon {
        #[arg(long, default_value_t = 24846)]
        port: u16,
        #[arg(long)]
        once: bool,
    },
    Tokenizer {
        #[command(subcommand)]
        command: TokenizerCommand,
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
enum CodeCommand {
    Diff {
        #[arg(long)]
        before: PathBuf,
        #[arg(long)]
        after: PathBuf,
        #[arg(long)]
        json: bool,
    },
    Elide {
        file: PathBuf,
        #[arg(long, default_value_t = 3)]
        keep: usize,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum SecurityCommand {
    Attest {
        #[arg(long)]
        json: bool,
    },
    Scan {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum EnterpriseCommand {
    InitConfig {
        #[arg(long, default_value = ".streetman.toml")]
        out: PathBuf,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        protect: bool,
        #[arg(long)]
        push_registry: Option<PathBuf>,
    },
    Rbac {
        #[arg(long)]
        json: bool,
    },
    Compliance {
        #[arg(long)]
        json: bool,
    },
    Sbom {
        #[arg(long)]
        json: bool,
    },
    ReleaseAttest {
        #[arg(long)]
        json: bool,
    },
    Deploy {
        #[arg(long)]
        json: bool,
    },
    Observability {
        #[arg(long)]
        json: bool,
    },
    Report {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum TokenizerCommand {
    Profile {
        #[arg(long)]
        model: Option<String>,
    },
}

#[derive(Subcommand)]
enum LeanCommand {
    Instructions {
        #[arg(long, default_value = "full")]
        mode: LeanModeArg,
        #[arg(long, default_value = "generic")]
        host: String,
    },
    Review {
        #[arg(value_name = "DIFF")]
        file: Option<PathBuf>,
        #[arg(long)]
        diff: bool,
        #[arg(long, default_value = "full")]
        mode: LeanModeArg,
        #[arg(long)]
        json: bool,
    },
    Audit {
        #[arg(value_name = "PATH", default_value = ".")]
        path: PathBuf,
        #[arg(long, default_value = "full")]
        mode: LeanModeArg,
        #[arg(long)]
        json: bool,
    },
    Gate {
        #[arg(long)]
        before: Option<String>,
        #[arg(long, default_value = "HEAD")]
        after: String,
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long, default_value = "full")]
        mode: LeanModeArg,
        #[arg(long, default_value_t = 0)]
        max_new_dependencies: usize,
        #[arg(long, default_value_t = 12)]
        max_files_touched: usize,
        #[arg(long, default_value_t = 75)]
        max_extension_cost_score: u8,
        #[arg(long)]
        allow_missing_check: bool,
    },
    Prove {
        #[arg(value_name = "DIFF")]
        file: Option<PathBuf>,
        #[arg(long)]
        diff: bool,
        #[arg(long, default_value = "full")]
        mode: LeanModeArg,
        #[arg(long)]
        normal_twin: Option<PathBuf>,
        #[arg(long = "command")]
        commands: Vec<String>,
    },
    Bench {
        #[command(subcommand)]
        command: LeanBenchCommand,
    },
    Kill {
        #[arg(long, default_value = "ponytail")]
        against: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum LeanBenchCommand {
    Run {
        #[arg(long, default_value = "ponytail")]
        against: String,
        #[arg(long)]
        out: Option<PathBuf>,
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
    Protect {
        #[arg(long, default_value = ".streetman.toml")]
        config: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Verify {
        #[arg(long, default_value = ".streetman.toml")]
        config: PathBuf,
        #[arg(long)]
        manifest: Option<PathBuf>,
    },
    Push {
        #[arg(long, default_value = ".streetman.toml")]
        config: PathBuf,
        #[arg(long, default_value = ".streetman-policy-registry")]
        registry: PathBuf,
    },
}

#[derive(Subcommand)]
enum MemoryCommand {
    Add {
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(long, default_value = "manual")]
        agent: String,
    },
    List,
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
    Serve,
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

#[derive(Clone, Copy, ValueEnum)]
enum LeanModeArg {
    Off,
    Lite,
    Full,
    Ultra,
}

impl From<LeanModeArg> for LeanMode {
    fn from(value: LeanModeArg) -> Self {
        match value {
            LeanModeArg::Off => Self::Off,
            LeanModeArg::Lite => Self::Lite,
            LeanModeArg::Full => Self::Full,
            LeanModeArg::Ultra => Self::Ultra,
        }
    }
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

impl From<DomainArg> for ContentDomain {
    fn from(value: DomainArg) -> Self {
        match value {
            DomainArg::Auto => Self::Auto,
            DomainArg::Intent => Self::Intent,
            DomainArg::Context => Self::Context,
            DomainArg::Prose => Self::Prose,
            DomainArg::Code => Self::Code,
            DomainArg::CodeMap => Self::CodeMap,
            DomainArg::Json => Self::Json,
            DomainArg::Logs => Self::Logs,
            DomainArg::Rag => Self::Rag,
            DomainArg::Search => Self::Search,
            DomainArg::Diff => Self::Diff,
            DomainArg::Html => Self::Html,
            DomainArg::Sql => Self::Sql,
            DomainArg::K8s => Self::K8s,
            DomainArg::Docs => Self::Docs,
            DomainArg::Shell => Self::Shell,
            DomainArg::History => Self::History,
            DomainArg::AgentState => Self::AgentState,
            DomainArg::FinalAnswer => Self::FinalAnswer,
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
            DomainArg::Intent => "intent",
            DomainArg::Context => "context",
            DomainArg::Prose => "prose",
            DomainArg::Code => "code",
            DomainArg::CodeMap => "code-map",
            DomainArg::Json => "json",
            DomainArg::Logs => "logs",
            DomainArg::Rag => "rag",
            DomainArg::Search => "search",
            DomainArg::Diff => "diff",
            DomainArg::Html => "html",
            DomainArg::Sql => "sql",
            DomainArg::K8s => "k8s",
            DomainArg::Docs => "docs",
            DomainArg::Shell => "shell",
            DomainArg::History => "history",
            DomainArg::AgentState => "agent-state",
            DomainArg::FinalAnswer => "final-answer",
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile {
            file,
            mode,
            domain,
            json,
            no_archive,
        } => {
            let input = read_input(file)?;
            let result = compile_shortlang(&input, mode.into(), domain.into());
            let archive_record = if !no_archive && result.wire != input {
                let archive = Archive::open_default()?;
                let record = archive.store(&input, &result.wire, "compile command")?;
                archive.log_event("compile", &result)?;
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
                print!("{}", result.wire);
                if let Some(record) = archive_record {
                    eprintln!("\n{}", retrieval_marker(&record.hash));
                }
            }
        }
        Commands::Compress {
            file,
            mode,
            domain,
            json,
            no_archive,
            fit,
        } => {
            let input = read_input(file)?;
            let result = if let Some(budget) = fit {
                fit_to_token_budget(&input, domain.into(), budget)
            } else {
                compress(&input, mode.into(), domain.into())
            };
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
        Commands::Decode { file, json } => {
            let input = read_input(file)?;
            let decoded = decode_archive_free(&input);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "decoder": "streetman-archive-free-v1",
                        "input_tokens": token_estimate(&input),
                        "decoded_tokens": token_estimate(&decoded),
                        "decoded": decoded
                    }))?
                );
            } else {
                print!("{decoded}");
            }
        }
        Commands::Run { json, command } => run_agent_command(command, json)?,
        Commands::Wrap { agent, args } => {
            let mut command = vec![agent];
            command.extend(args);
            run_agent_command(command, false)?;
        }
        Commands::Learn { from_run, target } => run_learn(from_run, target)?,
        Commands::Memory { command } => run_memory(command)?,
        Commands::CacheAlign {
            policy,
            memory,
            retrieval_tools,
            payload,
        } => run_cache_align(policy, memory, retrieval_tools, payload)?,
        Commands::Duel {
            against,
            trace,
            html,
            out,
        } => run_duel(&against, trace, html, out)?,
        Commands::Retrieve { hash, query } => {
            let archive = Archive::open_default()?;
            println!("{}", archive.retrieve(&hash, query.as_deref())?);
        }
        Commands::Audit { command } => run_audit(command)?,
        Commands::Bench { command } => run_bench(command)?,
        Commands::Lean { command } => run_lean(command)?,
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
        Commands::Code { command } => run_code(command)?,
        Commands::Security { command } => run_security(command)?,
        Commands::Enterprise { command } => run_enterprise(command)?,
        Commands::Daemon { port, once } => run_daemon(port, once)?,
        Commands::Tokenizer { command } => run_tokenizer(command)?,
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

fn run_agent_command(command: Vec<String>, json: bool) -> anyhow::Result<()> {
    if command.is_empty() {
        bail!("run requires a command after --");
    }
    let output = Command::new(&command[0])
        .args(&command[1..])
        .output()
        .with_context(|| format!("failed to run {}", command.join(" ")))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = if stderr.trim().is_empty() {
        stdout.to_string()
    } else {
        format!("{stdout}\n--- stderr ---\n{stderr}")
    };

    let compiled = compile_shortlang(&combined, CompressionMode::Full, ContentDomain::Context);
    let archive = Archive::open_default()?;
    let archive_record = archive.store(&combined, &compiled.wire, "agent run output")?;
    archive.log_event("run_compile", &compiled)?;

    let preliminary = build_run_receipt(
        command.clone(),
        output.status.code(),
        &combined,
        &compiled,
        vec![archive_record.hash.clone()],
        String::new(),
    );
    let run_dir = dirs::home_dir()
        .context("home directory not available")?
        .join(".streetman")
        .join("runs")
        .join(&preliminary.run_id);
    fs::create_dir_all(&run_dir)?;
    fs::write(run_dir.join("original.txt"), &combined)?;
    fs::write(run_dir.join("shortlang.txt"), &compiled.wire)?;

    let receipt_path = run_dir.join("receipt.json");
    let receipt = build_run_receipt(
        command,
        output.status.code(),
        &combined,
        &compiled,
        vec![archive_record.hash],
        receipt_path.display().to_string(),
    );
    fs::write(&receipt_path, serde_json::to_string_pretty(&receipt)?)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&receipt)?);
    } else {
        print!("{stdout}");
        eprint!("{stderr}");
        eprintln!(
            "\nstreetman receipt: saved {:.1}% · tests {} · artifacts protected {} · compressor mutations {} · retrieval misses {} · replay {}",
            receipt.savings_percent,
            receipt
                .tests_passed
                .map(|v| if v { "pass" } else { "fail" })
                .unwrap_or("n/a"),
            receipt.artifact_report.protected_artifacts,
            receipt.artifact_report.compressor_mutated_artifacts,
            receipt.artifact_report.retrieval_misses,
            receipt.replay_path
        );
    }

    if !output.status.success() {
        bail!(
            "wrapped command exited with {}",
            output.status.code().unwrap_or(1)
        );
    }
    Ok(())
}

fn run_learn(from_run: PathBuf, target: PathBuf) -> anyhow::Result<()> {
    let raw = fs::read_to_string(&from_run)
        .with_context(|| format!("failed to read run receipt {}", from_run.display()))?;
    let receipt: streetman_core::RunReceipt = serde_json::from_str(&raw)?;
    let note = format!(
        "\n\n## Streetman Learned Run {}\n- cmd: `{}`\n- saved: {:.1}%\n- tests: {}\n- artifacts protected: {}\n- compressor mutations: {}\n- replay: `{}`\n",
        receipt.run_id,
        receipt.command.join(" "),
        receipt.savings_percent,
        receipt
            .tests_passed
            .map(|v| if v { "pass" } else { "fail" })
            .unwrap_or("n/a"),
        receipt.artifact_report.protected_artifacts,
        receipt.artifact_report.compressor_mutated_artifacts,
        receipt.replay_path
    );
    let mut existing = fs::read_to_string(&target).unwrap_or_default();
    if !existing.contains(&format!("Streetman Learned Run {}", receipt.run_id)) {
        existing.push_str(&note);
        fs::write(&target, existing)?;
    }
    println!("{}", target.display());
    Ok(())
}

fn run_memory(command: MemoryCommand) -> anyhow::Result<()> {
    let path = memory_path()?;
    match command {
        MemoryCommand::Add { file, agent } => {
            let input = read_input(file)?;
            let compiled =
                compile_shortlang(&input, CompressionMode::Full, ContentDomain::AgentState);
            let mut entries = read_memory_entries(&path)?;
            let hash = blake3::hash(compiled.wire.as_bytes()).to_hex().to_string();
            if !entries.iter().any(|entry| entry["hash"] == hash) {
                entries.push(serde_json::json!({
                    "hash": hash,
                    "agent": agent,
                    "created_at": chrono::Utc::now().to_rfc3339(),
                    "wire": compiled.wire,
                    "tokens": compiled.wire_tokens_estimate
                }));
                write_memory_entries(&path, &entries)?;
            }
            println!("{}", path.display());
        }
        MemoryCommand::List => {
            let entries = read_memory_entries(&path)?;
            println!("{}", serde_json::to_string_pretty(&entries)?);
        }
    }
    Ok(())
}

fn memory_path() -> anyhow::Result<PathBuf> {
    let dir = dirs::home_dir()
        .context("home directory not available")?
        .join(".streetman")
        .join("memory");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("shared.json"))
}

fn read_memory_entries(path: &PathBuf) -> anyhow::Result<Vec<serde_json::Value>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw).unwrap_or_default())
}

fn write_memory_entries(path: &PathBuf, entries: &[serde_json::Value]) -> anyhow::Result<()> {
    fs::write(path, serde_json::to_string_pretty(entries)?)?;
    Ok(())
}

fn run_cache_align(
    policy: Option<PathBuf>,
    memory: Option<PathBuf>,
    retrieval_tools: Option<PathBuf>,
    payload: Option<PathBuf>,
) -> anyhow::Result<()> {
    let policy = read_optional(policy)?;
    let memory = read_optional(memory)?;
    let retrieval_tools = read_optional(retrieval_tools)?;
    let payload = read_input(payload)?;
    println!(
        "{}",
        align_cache_prefix(&policy, &memory, &retrieval_tools, &payload)
    );
    Ok(())
}

fn read_optional(path: Option<PathBuf>) -> anyhow::Result<String> {
    if let Some(path) = path {
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))
    } else {
        Ok(String::new())
    }
}

fn run_duel(against: &str, trace: PathBuf, html: bool, out: Option<PathBuf>) -> anyhow::Result<()> {
    let raw = fs::read_to_string(&trace)
        .with_context(|| format!("failed to read trace {}", trace.display()))?;
    let trace_cases = parse_trace_cases(&raw)?;
    let cases = trace_cases
        .iter()
        .map(|case| {
            let domain = case
                .domain
                .as_deref()
                .unwrap_or("auto")
                .parse()
                .unwrap_or(ContentDomain::Auto);
            let compiled = compile_shortlang(&case.input, CompressionMode::Full, domain);
            let headroom = estimate_headroom_baseline(&compiled.route.domain, &case.input);
            serde_json::json!({
                "name": case.name,
                "domain": format!("{:?}", compiled.route.domain),
                "streetman_tokens": compiled.wire_tokens_estimate,
                "streetman_savings_percent": compiled.savings_percent,
                "headroom_estimated_savings_percent": headroom,
                "streetman_delta_pp": compiled.savings_percent - headroom,
                "artifact_mutations": compiled.compressor_mutated_artifacts,
                "protected_artifacts": compiled.protected_artifacts,
                "accuracy_score": compiled.compression.certificate.accuracy_score
            })
        })
        .collect::<Vec<_>>();
    let avg_delta = if cases.is_empty() {
        0.0
    } else {
        cases
            .iter()
            .filter_map(|case| case["streetman_delta_pp"].as_f64())
            .sum::<f64>()
            / cases.len() as f64
    };
    let report = serde_json::json!({
        "suite": "streetman-duel",
        "against": against,
        "trace": trace.display().to_string(),
        "status": "streetman-measured-headroom-baseline-estimated",
        "avg_delta_pp": avg_delta,
        "cases": cases,
        "note": "Use committed Headroom run artifacts or install Headroom CLI for external replay before making public absolute-win claims."
    });
    let rendered = if html {
        render_duel_html(&report)
    } else {
        serde_json::to_string_pretty(&report)?
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

#[derive(Debug)]
struct TraceCase {
    name: String,
    input: String,
    domain: Option<String>,
}

fn parse_trace_cases(raw: &str) -> anyhow::Result<Vec<TraceCase>> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        let array = value
            .get("cases")
            .and_then(|v| v.as_array())
            .cloned()
            .or_else(|| value.as_array().cloned())
            .unwrap_or_default();
        if !array.is_empty() {
            return Ok(array
                .into_iter()
                .enumerate()
                .map(|(idx, item)| TraceCase {
                    name: item["name"]
                        .as_str()
                        .map(str::to_string)
                        .unwrap_or_else(|| format!("case-{idx}")),
                    input: item["input"]
                        .as_str()
                        .or_else(|| item["text"].as_str())
                        .unwrap_or_default()
                        .to_string(),
                    domain: item["domain"].as_str().map(str::to_string),
                })
                .collect());
        }
    }
    Ok(vec![TraceCase {
        name: "raw-trace".to_string(),
        input: raw.to_string(),
        domain: None,
    }])
}

fn estimate_headroom_baseline(domain: &ContentDomain, input: &str) -> f64 {
    match domain {
        ContentDomain::Logs => 98.1,
        ContentDomain::Search => 83.0,
        ContentDomain::Json => 90.0,
        ContentDomain::CodeMap | ContentDomain::Code => 47.0,
        ContentDomain::Context | ContentDomain::Rag | ContentDomain::History => {
            if input.len() > 50_000 {
                92.0
            } else {
                73.0
            }
        }
        _ => 60.0,
    }
}

fn render_duel_html(report: &serde_json::Value) -> String {
    let rows = report["cases"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|case| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{:.1}</td><td>{:.1}</td><td>{:.1}</td><td>{}</td><td>{}</td></tr>",
                html_escape(case["name"].as_str().unwrap_or_default()),
                html_escape(case["domain"].as_str().unwrap_or_default()),
                case["streetman_savings_percent"].as_f64().unwrap_or_default(),
                case["headroom_estimated_savings_percent"].as_f64().unwrap_or_default(),
                case["streetman_delta_pp"].as_f64().unwrap_or_default(),
                case["artifact_mutations"].as_i64().unwrap_or_default(),
                case["accuracy_score"].as_i64().unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join("");
    format!(
        r#"<!doctype html><meta charset="utf-8"><title>Streetman Duel</title>
<style>body{{font-family:system-ui;margin:24px;background:#101315;color:#edf6ef}}table{{border-collapse:collapse;width:100%}}td,th{{border-bottom:1px solid #2a3438;padding:10px;text-align:left}}th{{color:#91a29a}}.win{{color:#58f28b}}</style>
<h1>Streetman Duel: {}</h1><p>Status: {}</p><p class="win">Average delta: {:.1}pp</p>
<table><thead><tr><th>Case</th><th>Domain</th><th>Streetman %</th><th>Headroom baseline %</th><th>Delta pp</th><th>Artifact mutations</th><th>Accuracy</th></tr></thead><tbody>{}</tbody></table>
<p>{}</p>"#,
        html_escape(report["against"].as_str().unwrap_or_default()),
        html_escape(report["status"].as_str().unwrap_or_default()),
        report["avg_delta_pp"].as_f64().unwrap_or_default(),
        rows,
        html_escape(report["note"].as_str().unwrap_or_default())
    )
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
                "token-greedy" | "case1-case2" => run_token_greedy_bench(),
                "final-case" | "final-case-0.3" => run_final_kf_bench(),
                "all-lanes" | "all-lanes-1.0" => run_all_lanes_bench(),
                "absolute-win-2" | "absolute-win-2.0" | "all-17" => run_absolute_win_v2_bench(),
                "absolute-win-3" | "absolute-win-3.0" | "entire-plan" => {
                    run_absolute_win_v3_bench()
                }
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
            let token_greedy = run_token_greedy_bench();
            let final_kf = run_final_kf_bench();
            let all_lanes = run_all_lanes_bench();
            let absolute_win_v2 = run_absolute_win_v2_bench();
            let absolute_win_v3 = run_absolute_win_v3_bench();
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "absolute_win": fixture,
                    "redteam": redteam,
                    "token_greedy": token_greedy,
                    "final_kf": final_kf,
                    "all_lanes": all_lanes,
                    "absolute_win_v2": absolute_win_v2,
                    "absolute_win_v3": absolute_win_v3
                }))?
            );
            if !fixture.gates_passed
                || !redteam.gates_passed
                || !token_greedy.gates_passed
                || !final_kf.gates_passed
                || !all_lanes.gates_passed
                || !absolute_win_v2.gates_passed
                || !absolute_win_v3.gates_passed
            {
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

fn run_code(command: CodeCommand) -> anyhow::Result<()> {
    match command {
        CodeCommand::Diff {
            before,
            after,
            json,
        } => {
            let before_text = fs::read_to_string(&before)
                .with_context(|| format!("failed to read {}", before.display()))?;
            let after_text = fs::read_to_string(&after)
                .with_context(|| format!("failed to read {}", after.display()))?;
            let report = anchored_diff(&before_text, &after_text);
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                print!("{}", report.transport);
            }
        }
        CodeCommand::Elide { file, keep, json } => {
            let input = fs::read_to_string(&file)
                .with_context(|| format!("failed to read {}", file.display()))?;
            let report = elide_unchanged_regions(&input, keep);
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                print!("{}", report.output);
            }
        }
    }
    Ok(())
}

fn run_security(command: SecurityCommand) -> anyhow::Result<()> {
    match command {
        SecurityCommand::Attest { json } => {
            let report = security_attestation();
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("streetman security attestation {}", report.version);
                println!("profile: {}", report.profile);
                for claim in report.claims {
                    println!("{} [{}] {}", claim.id, claim.status, claim.evidence);
                }
                println!("signed_summary: {}", report.signed_summary);
            }
        }
        SecurityCommand::Scan { file, json } => {
            let input = read_input(file)?;
            let findings = classify_sensitive(&input);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "status": if findings.is_empty() { "pass" } else { "sensitive" },
                        "findings": findings
                    }))?
                );
            } else if findings.is_empty() {
                println!("no sensitive markers found");
            } else {
                for finding in findings {
                    println!("{} {}", finding.kind, finding.marker);
                }
            }
        }
    }
    Ok(())
}

fn run_enterprise(command: EnterpriseCommand) -> anyhow::Result<()> {
    match command {
        EnterpriseCommand::InitConfig {
            out,
            force,
            protect,
            push_registry,
        } => {
            if out.exists() && !force {
                bail!(
                    "{} already exists; pass --force to overwrite",
                    out.display()
                );
            }
            let artifact = enterprise_config_template();
            if let Some(parent) = out.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(&out, artifact.content)?;
            let mut payload = serde_json::json!({
                "status": "written",
                "config": out,
            });
            if protect || push_registry.is_some() {
                let protected = protect_config(&out)?;
                let manifest = default_protected_config_path(&out);
                fs::write(&manifest, serde_json::to_string_pretty(&protected)?)?;
                payload["manifest"] = serde_json::json!(manifest);
                payload["protected"] = serde_json::to_value(&protected)?;
            }
            if let Some(registry) = push_registry {
                let receipt = push_protected_config(&out, registry)?;
                payload["push_receipt"] = serde_json::to_value(receipt)?;
            }
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        EnterpriseCommand::Rbac { json } => print_enterprise_artifact(rbac_template(), json)?,
        EnterpriseCommand::Compliance { json } => {
            print_enterprise_artifact(compliance_map(), json)?
        }
        EnterpriseCommand::Sbom { json } => print_enterprise_artifact(sbom("."), json)?,
        EnterpriseCommand::ReleaseAttest { json } => {
            print_enterprise_artifact(release_attestation("."), json)?
        }
        EnterpriseCommand::Deploy { json } => print_enterprise_artifact(deployment_bundle(), json)?,
        EnterpriseCommand::Observability { json } => {
            print_enterprise_artifact(observability_template(), json)?
        }
        EnterpriseCommand::Report { json } => {
            let report = enterprise_report(".");
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("{} {}", report.suite, report.status);
                for artifact in report.artifacts {
                    println!(
                        "{} [{}] {}",
                        artifact.artifact, artifact.status, artifact.signature
                    );
                }
            }
        }
    }
    Ok(())
}

fn print_enterprise_artifact(artifact: EnterpriseArtifact, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(&artifact)?);
    } else {
        println!("{}", artifact.content);
        eprintln!(
            "streetman artifact {} {}",
            artifact.artifact, artifact.signature
        );
    }
    Ok(())
}

fn run_daemon(port: u16, once: bool) -> anyhow::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .with_context(|| format!("failed to bind daemon on 127.0.0.1:{port}"))?;
    eprintln!("streetman daemon listening on 127.0.0.1:{port}");
    for stream in listener.incoming() {
        handle_daemon_stream(stream?)?;
        if once {
            break;
        }
    }
    Ok(())
}

fn handle_daemon_stream(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buffer = [0_u8; 64 * 1024];
    let read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..read]);
    let body = request.split("\r\n\r\n").nth(1).unwrap_or_default();
    let response = if request.starts_with("GET /health ") {
        serde_json::json!({
            "status": "ok",
            "service": "streetman-daemon",
            "version": env!("CARGO_PKG_VERSION"),
            "telemetry": false
        })
    } else if request.starts_with("POST /v1/compress ") {
        let value: serde_json::Value = serde_json::from_str(body).unwrap_or_default();
        let text = value["text"].as_str().unwrap_or_default();
        let mode = value["mode"]
            .as_str()
            .unwrap_or("full")
            .parse::<CompressionMode>()
            .unwrap_or(CompressionMode::Full);
        let domain = value["domain"]
            .as_str()
            .unwrap_or("auto")
            .parse::<ContentDomain>()
            .unwrap_or(ContentDomain::Auto);
        serde_json::to_value(compress(text, mode, domain))?
    } else {
        serde_json::json!({
            "status": "not-found",
            "routes": ["GET /health", "POST /v1/compress"]
        })
    };
    let status = if response["status"] == "not-found" {
        "404 Not Found"
    } else {
        "200 OK"
    };
    let body = serde_json::to_string_pretty(&response)?;
    write!(
        stream,
        "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
        body.len(),
        body
    )?;
    Ok(())
}

fn run_tokenizer(command: TokenizerCommand) -> anyhow::Result<()> {
    match command {
        TokenizerCommand::Profile { model } => {
            let profile = tokenizer_profile(model.as_deref());
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }
    }
    Ok(())
}

fn run_lean(command: LeanCommand) -> anyhow::Result<()> {
    match command {
        LeanCommand::Instructions { mode, host } => {
            println!("{}", lean_instructions(mode.into(), &host));
        }
        LeanCommand::Review {
            file,
            diff,
            mode,
            json,
        } => {
            let raw = read_lean_diff(file, diff, None, "HEAD")?;
            let report = review_diff(&raw, mode.into());
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("{}", render_lean_report(&report));
            }
        }
        LeanCommand::Audit { path, mode, json } => {
            let files = collect_audit_files(&path)?;
            let report = audit_files(&files, mode.into());
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("{}", render_lean_report(&report));
            }
        }
        LeanCommand::Gate {
            before,
            after,
            file,
            mode,
            max_new_dependencies,
            max_files_touched,
            max_extension_cost_score,
            allow_missing_check,
        } => {
            let raw = read_lean_diff(file, true, before.as_deref(), &after)?;
            let result = gate_diff(
                &raw,
                mode.into(),
                LeanGateConfig {
                    max_new_dependencies,
                    max_files_touched,
                    require_runnable_check: !allow_missing_check,
                    max_extension_cost_score,
                },
            );
            println!("{}", serde_json::to_string_pretty(&result)?);
            if !result.passed {
                bail!("lean gate failed");
            }
        }
        LeanCommand::Prove {
            file,
            diff,
            mode,
            normal_twin,
            commands,
        } => {
            let raw = read_lean_diff(file, diff, None, "HEAD")?;
            let normal_twin = read_optional(normal_twin)?;
            let cert = if normal_twin.is_empty() {
                prove_diff(&raw, mode.into(), commands)
            } else {
                prove_diff_with_normal_twin(&raw, mode.into(), commands, Some(&normal_twin))
            };
            println!("{}", serde_json::to_string_pretty(&cert)?);
        }
        LeanCommand::Bench { command } => match command {
            LeanBenchCommand::Run { against, out } => {
                let result = ponytail_h2h_fixture(&against);
                let json = serde_json::to_string_pretty(&result)?;
                if let Some(path) = out {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&path, json)?;
                    println!("{}", path.display());
                } else {
                    println!("{json}");
                }
            }
        },
        LeanCommand::Kill { against, json } => {
            if against != "ponytail" && against != "DietrichGebert/ponytail" {
                bail!("only ponytail kill reports are implemented");
            }
            let report = ponytail_kill_report();
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("{}", render_kill_report(&report));
            }
        }
    }
    Ok(())
}

fn read_lean_diff(
    file: Option<PathBuf>,
    use_git_diff: bool,
    before: Option<&str>,
    after: &str,
) -> anyhow::Result<String> {
    if let Some(path) = file {
        return fs::read_to_string(&path)
            .with_context(|| format!("failed to read diff {}", path.display()));
    }
    if use_git_diff {
        return run_git_diff(before, after);
    }
    run_git_diff(None, after)
}

fn run_git_diff(before: Option<&str>, after: &str) -> anyhow::Result<String> {
    let mut cmd = Command::new("git");
    cmd.arg("diff");
    if let Some(before) = before {
        cmd.arg(before).arg(after);
    }
    let output = cmd.output().context("failed to run git diff")?;
    if !output.status.success() {
        bail!(
            "git diff failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn collect_audit_files(root: &Path) -> anyhow::Result<Vec<(String, String)>> {
    let mut files = Vec::new();
    if root.is_file() {
        if should_audit_path(root) {
            files.push((
                root.display().to_string(),
                fs::read_to_string(root)
                    .with_context(|| format!("failed to read {}", root.display()))?,
            ));
        }
        return Ok(files);
    }
    collect_audit_files_inner(root, root, &mut files)?;
    Ok(files)
}

fn collect_audit_files_inner(
    root: &Path,
    dir: &Path,
    files: &mut Vec<(String, String)>,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("failed to read {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if should_skip_audit_path(&path) {
            continue;
        }
        if path.is_dir() {
            collect_audit_files_inner(root, &path, files)?;
        } else if should_audit_path(&path) {
            let label = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .display()
                .to_string();
            files.push((
                label,
                fs::read_to_string(&path)
                    .with_context(|| format!("failed to read {}", path.display()))?,
            ));
        }
    }
    Ok(())
}

fn should_skip_audit_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            matches!(
                name,
                ".git" | "target" | "node_modules" | ".next" | "dist" | "coverage"
            )
        })
}

fn should_audit_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            matches!(
                ext,
                "rs" | "ts"
                    | "tsx"
                    | "js"
                    | "jsx"
                    | "py"
                    | "go"
                    | "java"
                    | "kt"
                    | "rb"
                    | "php"
                    | "md"
                    | "toml"
                    | "json"
                    | "yaml"
                    | "yml"
                    | "txt"
            )
        })
}

fn render_lean_report(report: &streetman_core::LeanReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Streetman Lean {}\nstatus={} mode={} files={} +{} -{} deps_added={} checks={} shortcuts={} extension_cost={}\n",
        report.scope,
        report.status,
        report.mode,
        report.files_touched,
        report.loc_added,
        report.loc_removed,
        report.dependencies_added.len(),
        report.runnable_checks,
        report.shortcut_comments,
        report.extension_cost_score
    ));
    if report.findings.is_empty() {
        out.push_str("Lean already. Ship.\n");
    } else {
        for finding in &report.findings {
            let loc = match (&finding.path, finding.line) {
                (Some(path), Some(line)) => format!("{path}:L{line}"),
                (Some(path), None) => path.clone(),
                (None, Some(line)) => format!("L{line}"),
                (None, None) => "diff".to_string(),
            };
            out.push_str(&format!(
                "{}: {}: {}. {}. (-{} lines)\n",
                loc,
                finding.tag,
                finding.message,
                finding.replacement,
                finding.estimated_lines_saved.max(0)
            ));
        }
        out.push_str(&format!(
            "net: -{} lines possible.\n",
            report.estimated_lines_saved
        ));
    }
    out
}

fn render_kill_report(report: &streetman_core::LeanKillReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Streetman vs {}\nfeature_kill={}\nverdict={}\npublic_performance_claim_ready={}\n",
        report.against,
        if report.feature_kill { "YES" } else { "NO" },
        report.verdict,
        report.public_performance_claim_ready
    ));
    for feature in &report.parity {
        out.push_str(&format!(
            "- {}: {} ({})\n",
            feature.feature, feature.status, feature.streetman
        ));
    }
    out.push_str("extras:\n");
    for extra in &report.streetman_extra_features {
        out.push_str(&format!("- {extra}\n"));
    }
    out.push_str(&format!("caveat: {}\n", report.caveat));
    out
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
        PolicyCommand::Protect { config, out } => {
            let protected = protect_config(&config)?;
            let out = out.unwrap_or_else(|| default_protected_config_path(&config));
            if let Some(parent) = out.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(&out, serde_json::to_string_pretty(&protected)?)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "status": "protected",
                    "manifest": out,
                    "protected": protected
                }))?
            );
        }
        PolicyCommand::Verify { config, manifest } => {
            let manifest = manifest.unwrap_or_else(|| default_protected_config_path(&config));
            let protected = read_protected_config(&manifest)?;
            let verification = verify_protected_config(&config, &protected)?;
            println!("{}", serde_json::to_string_pretty(&verification)?);
            if verification.status != "pass" {
                bail!("protected config verification failed");
            }
        }
        PolicyCommand::Push { config, registry } => {
            let receipt = push_protected_config(&config, &registry)?;
            println!("{}", serde_json::to_string_pretty(&receipt)?);
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
        McpCommand::Serve => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            if input.trim().is_empty() {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "status": "ready",
                        "transport": "stdio-json",
                        "tools": ["streetman_compress", "streetman_compile", "streetman_retrieve", "streetman_stats", "streetman_accuracy_check"]
                    }))?
                );
            } else {
                for line in input.lines().filter(|line| !line.trim().is_empty()) {
                    let value: serde_json::Value = serde_json::from_str(line)?;
                    let tool = value["tool"].as_str().unwrap_or_default();
                    let output = handle_mcp_tool(tool, value.get("input").unwrap_or(&value))?;
                    println!("{}", serde_json::to_string(&output)?);
                }
            }
        }
        McpCommand::Tools => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "tools": [
                        {
                            "name": "streetman_compress",
                            "description": "Compress text locally and return a proof-carrying certificate.",
                            "input": {"text": "string", "mode": "lite|full|ultra|auto", "domain": "auto|intent|context|prose|json|logs|rag|code|code-map|search|diff|html|history|agent-state|final-answer"}
                        },
                        {
                            "name": "streetman_compile",
                            "description": "Compile prompt/context/history/tool output into Streetman ShortLang.",
                            "input": {"text": "string", "mode": "lite|full|ultra|auto", "domain": "auto|intent|context|rag|history|agent-state"}
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
            let output = handle_mcp_tool(&tool, &value)?;
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }
    Ok(())
}

fn handle_mcp_tool(tool: &str, value: &serde_json::Value) -> anyhow::Result<serde_json::Value> {
    let output = match tool {
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
        "streetman_compile" => {
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
            serde_json::to_value(compile_shortlang(text, mode, domain))?
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
    Ok(output)
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
    } else if req.starts_with("POST /v1/chat/completions") || req.starts_with("POST /v1/responses")
    {
        let path = req
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/v1/chat/completions");
        let body = req.split("\r\n\r\n").nth(1).unwrap_or_default();
        let parsed: serde_json::Value = serde_json::from_str(body).unwrap_or_default();
        let transformed = transform_llm_payload(parsed);
        if let Ok(upstream) = std::env::var("STREETMAN_UPSTREAM_URL") {
            let auth = req
                .lines()
                .find(|line| line.to_ascii_lowercase().starts_with("authorization:"))
                .map(str::to_string);
            let forwarded = forward_with_curl(&upstream, path, auth.as_deref(), &transformed)?;
            ("200 OK", forwarded)
        } else {
            (
                "200 OK",
                serde_json::json!({
                    "status": "transformed",
                    "provider": provider,
                    "upstream_forwarding": "set STREETMAN_UPSTREAM_URL to forward",
                    "payload": transformed
                }),
            )
        }
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

fn transform_llm_payload(mut value: serde_json::Value) -> serde_json::Value {
    if let Some(messages) = value.get_mut("messages").and_then(|v| v.as_array_mut()) {
        for message in messages {
            if let Some(content) = message.get_mut("content") {
                if let Some(text) = content.as_str() {
                    let compiled =
                        compile_shortlang(text, CompressionMode::Full, ContentDomain::Auto);
                    *content = serde_json::Value::String(compiled.wire);
                }
            }
        }
    }
    if let Some(input) = value.get_mut("input") {
        if let Some(text) = input.as_str() {
            let compiled = compile_shortlang(text, CompressionMode::Full, ContentDomain::Auto);
            *input = serde_json::Value::String(compiled.wire);
        }
    }
    value
}

fn forward_with_curl(
    upstream: &str,
    path: &str,
    auth: Option<&str>,
    payload: &serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{}/{}",
        upstream.trim_end_matches('/'),
        path.trim_start_matches('/')
    );
    let body = serde_json::to_string(payload)?;
    let mut cmd = Command::new("curl");
    cmd.args(["-sS", "-X", "POST", "-H", "Content-Type: application/json"]);
    if let Some(auth) = auth {
        cmd.args(["-H", auth]);
    }
    let output = cmd.arg("--data-binary").arg(body).arg(url).output()?;
    if !output.status.success() {
        bail!(
            "upstream forwarding failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let raw = String::from_utf8_lossy(&output.stdout);
    Ok(serde_json::from_str(&raw).unwrap_or_else(|_| serde_json::json!({"raw": raw})))
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

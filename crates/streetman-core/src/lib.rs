pub mod accuracy;
pub mod archive;
pub mod audit;
pub mod bench;
pub mod compress;
pub mod config;
pub mod lean;
pub mod run;
pub mod shortlang;

pub use accuracy::{accuracy_check, AccuracyReport};
pub use archive::{Archive, ArchiveRecord};
pub use audit::{AuditReport, QualityScore};
pub use bench::{
    run_fixture_bench, run_redteam_bench, run_token_greedy_bench, AbsoluteWinGate, BenchResult,
};
pub use compress::{
    compress, token_estimate, token_estimate_for_model, verify_certificate, CompressionCertificate,
    CompressionMode, CompressionResult, ContentDomain, ProofVerification,
};
pub use config::{check_policy, PolicyReport, StreetmanConfig};
pub use lean::{
    audit_files, gate_diff, lean_instructions, ponytail_h2h_fixture, ponytail_kill_report,
    prove_diff, prove_diff_with_normal_twin, review_diff, LeanBenchResult, LeanCertificate,
    LeanFinding, LeanGateConfig, LeanGateResult, LeanKillFeature, LeanKillReport, LeanMode,
    LeanReport,
};
pub use run::{build_run_receipt, RunArtifactReport, RunReceipt};
pub use shortlang::{
    align_cache_prefix, compile_shortlang, route_content, ContentRoute, ShortLangResult,
};

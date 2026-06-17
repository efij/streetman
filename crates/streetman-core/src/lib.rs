pub mod accuracy;
pub mod archive;
pub mod audit;
pub mod bench;
pub mod builtin;
pub mod compress;
pub mod config;
pub mod enterprise;
pub mod lean;
pub mod run;
pub mod security;
pub mod shortlang;
pub mod transport;

pub use accuracy::{accuracy_check, AccuracyReport};
pub use archive::{Archive, ArchiveRecord};
pub use audit::{AuditReport, QualityScore};
pub use bench::{
    run_quality_gate_v2_bench, run_quality_gate_v3_bench, run_quality_gate_v4_bench,
    run_all_lanes_bench, run_final_caps_bench, run_fixture_bench, run_redteam_bench,
    run_token_greedy_bench, QualityGate, BenchResult,
};
pub use builtin::{builtin_oracle, BuiltinOracleResult};
pub use compress::{
    compress, decode_archive_free, fit_to_token_budget, token_estimate, token_estimate_for_model,
    tokenizer_profile, verify_certificate, CompressionCertificate, CompressionMode,
    CompressionResult, ContentDomain, ProofVerification, TokenizerProfile,
};
pub use config::{
    check_policy, default_protected_config_path, protect_config, push_protected_config,
    read_protected_config, verify_protected_config, ConfigPushReceipt, ConfigVerification,
    PolicyReport, ProtectedConfig, StreetmanConfig,
};
pub use enterprise::{
    compliance_map, deployment_bundle, enterprise_config_template, enterprise_report,
    observability_template, rbac_template, release_attestation, sbom, EnterpriseArtifact,
    EnterpriseReport,
};
pub use lean::{
    audit_files, gate_diff, lean_instructions, ponytail_h2h_fixture, ponytail_parity_report,
    prove_diff, prove_diff_with_normal_twin, review_diff, LeanBenchResult, LeanCertificate,
    LeanFinding, LeanGateConfig, LeanGateResult, LeanKillFeature, LeanParityReport, LeanMode,
    LeanReport,
};
pub use run::{build_run_receipt, RunArtifactReport, RunReceipt};
pub use security::{
    classify_sensitive, security_attestation, SecurityAttestation, SecurityClaim, SensitiveFinding,
};
pub use shortlang::{
    align_cache_prefix, compile_shortlang, route_content, ContentRoute, ShortLangResult,
};
pub use transport::{anchored_diff, elide_unchanged_regions, AnchoredDiffReport, ElisionReport};

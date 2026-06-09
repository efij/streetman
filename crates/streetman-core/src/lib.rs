pub mod accuracy;
pub mod archive;
pub mod audit;
pub mod bench;
pub mod compress;
pub mod config;

pub use accuracy::{accuracy_check, AccuracyReport};
pub use archive::{Archive, ArchiveRecord};
pub use audit::{AuditReport, QualityScore};
pub use bench::{run_fixture_bench, run_redteam_bench, AbsoluteWinGate, BenchResult};
pub use compress::{
    compress, token_estimate, verify_certificate, CompressionCertificate, CompressionMode,
    CompressionResult, ContentDomain, ProofVerification,
};
pub use config::{check_policy, PolicyReport, StreetmanConfig};

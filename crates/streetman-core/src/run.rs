use crate::{
    compress::{token_estimate, CompressionResult},
    shortlang::ShortLangResult,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunArtifactReport {
    pub protected_artifacts: usize,
    pub compressor_mutated_artifacts: usize,
    pub retrieval_misses: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReceipt {
    pub run_id: String,
    pub created_at: DateTime<Utc>,
    pub command: Vec<String>,
    pub exit_code: Option<i32>,
    pub original_tokens_estimate: usize,
    pub compressed_tokens_estimate: usize,
    pub savings_percent: f64,
    pub tests_passed: Option<bool>,
    pub artifact_report: RunArtifactReport,
    pub archive_hashes: Vec<String>,
    pub replay_path: String,
    pub compression: CompressionResult,
}

pub fn build_run_receipt(
    command: Vec<String>,
    exit_code: Option<i32>,
    original_output: &str,
    compiled: &ShortLangResult,
    archive_hashes: Vec<String>,
    replay_path: String,
) -> RunReceipt {
    let before = token_estimate(original_output);
    let after = token_estimate(&compiled.wire);
    RunReceipt {
        run_id: run_id(&command, original_output),
        created_at: Utc::now(),
        command,
        exit_code,
        original_tokens_estimate: before,
        compressed_tokens_estimate: after,
        savings_percent: if before == 0 {
            0.0
        } else {
            ((before.saturating_sub(after)) as f64 / before as f64) * 100.0
        },
        tests_passed: infer_tests_passed(original_output, exit_code),
        artifact_report: RunArtifactReport {
            protected_artifacts: compiled.protected_artifacts,
            compressor_mutated_artifacts: compiled.compressor_mutated_artifacts,
            retrieval_misses: 0,
        },
        archive_hashes,
        replay_path,
        compression: compiled.compression.clone(),
    }
}

fn run_id(command: &[String], output: &str) -> String {
    let digest = blake3::hash(format!("{}:{output}", command.join(" ")).as_bytes())
        .to_hex()
        .to_string();
    format!("run-{}", &digest[..16])
}

fn infer_tests_passed(output: &str, exit_code: Option<i32>) -> Option<bool> {
    let lower = output.to_ascii_lowercase();
    if lower.contains("test") || lower.contains("passed") || lower.contains("failed") {
        return Some(exit_code.unwrap_or(1) == 0 && !lower.contains(" failed"));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        compress::{CompressionMode, ContentDomain},
        shortlang::compile_shortlang,
    };

    #[test]
    fn receipt_tracks_artifact_safety() {
        let compiled =
            compile_shortlang("tests passed", CompressionMode::Full, ContentDomain::Logs);
        let receipt = build_run_receipt(
            vec!["echo".to_string()],
            Some(0),
            "tests passed",
            &compiled,
            vec!["abc".to_string()],
            ".streetman/runs/run.json".to_string(),
        );
        assert_eq!(receipt.artifact_report.compressor_mutated_artifacts, 0);
        assert_eq!(receipt.tests_passed, Some(true));
    }
}

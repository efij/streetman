use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScore {
    pub resource_health: u8,
    pub session_efficiency: u8,
    pub grade: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteFinding {
    pub detector: String,
    pub severity: String,
    pub message: String,
    pub estimated_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub quality: QualityScore,
    pub findings: Vec<WasteFinding>,
    pub telemetry: String,
    pub status: String,
}

pub fn audit_text(input: &str) -> AuditReport {
    let tokens = input.split_whitespace().count();
    let line_count = input.lines().count();
    let repeated_lines = repeated_line_count(input);
    let resource_health = if tokens > 20_000 {
        45
    } else if tokens > 10_000 {
        65
    } else if tokens > 5_000 {
        80
    } else {
        95
    };
    let session_efficiency = if repeated_lines > line_count / 4 && line_count > 20 {
        55
    } else if input.matches("ERROR").count() > 10 {
        70
    } else {
        90
    };
    let avg = ((resource_health as u16 + session_efficiency as u16) / 2) as u8;
    let mut findings = Vec::new();
    if repeated_lines > 10 {
        findings.push(WasteFinding {
            detector: "duplicate-context".to_string(),
            severity: "medium".to_string(),
            message: "Repeated lines suggest stale context or duplicated tool output.".to_string(),
            estimated_tokens: repeated_lines * 8,
        });
    }
    if input.to_ascii_lowercase().contains("retry") && input.to_ascii_lowercase().contains("failed")
    {
        findings.push(WasteFinding {
            detector: "retry-churn".to_string(),
            severity: "high".to_string(),
            message: "Failure/retry language suggests wasted turns.".to_string(),
            estimated_tokens: 500,
        });
    }
    if input.len() > 50_000 {
        findings.push(WasteFinding {
            detector: "large-context".to_string(),
            severity: "high".to_string(),
            message: "Large context should be compressed or archived before model ingestion."
                .to_string(),
            estimated_tokens: tokens.saturating_sub(5_000),
        });
    }
    AuditReport {
        quality: QualityScore {
            resource_health,
            session_efficiency,
            grade: grade(avg).to_string(),
        },
        findings,
        telemetry: "zero-telemetry-default".to_string(),
        status: "local-only audit; claim status comes from benchmark snapshots".to_string(),
    }
}

fn repeated_line_count(input: &str) -> usize {
    let mut seen = std::collections::HashSet::new();
    let mut repeated = 0;
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if !seen.insert(line.to_string()) {
            repeated += 1;
        }
    }
    repeated
}

fn grade(score: u8) -> &'static str {
    match score {
        90..=100 => "S",
        80..=89 => "A",
        70..=79 => "B",
        60..=69 => "C",
        50..=59 => "D",
        _ => "F",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_duplicate_context() {
        let report = audit_text(&"same\n".repeat(30));
        assert!(report
            .findings
            .iter()
            .any(|f| f.detector == "duplicate-context"));
    }
}

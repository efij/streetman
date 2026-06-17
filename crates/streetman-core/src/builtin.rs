use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinOracleResult {
    pub gate: String,
    pub status: String,
    pub language: String,
    pub runtime: String,
    pub task: String,
    pub builtin: Option<String>,
    pub min_version: Option<String>,
    pub replacement_for: Vec<String>,
    pub caveat: Option<String>,
}

pub fn builtin_oracle(language: &str, runtime: &str, task: &str) -> BuiltinOracleResult {
    let language_l = language.to_ascii_lowercase();
    let runtime_l = runtime.to_ascii_lowercase();
    let task_l = task.to_ascii_lowercase();
    let mut result = BuiltinOracleResult {
        gate: "streetman-versioned-builtin-oracle-v1".to_string(),
        status: "flag".to_string(),
        language: language.to_string(),
        runtime: runtime.to_string(),
        task: task.to_string(),
        builtin: None,
        min_version: None,
        replacement_for: Vec::new(),
        caveat: Some("no native builtin match in the committed oracle table".to_string()),
    };

    let hit = if matches!(
        language_l.as_str(),
        "javascript" | "js" | "typescript" | "ts"
    ) && runtime_l.contains("node")
        && (task_l.contains("http") || task_l.contains("request") || task_l.contains("fetch"))
    {
        Some((
            "globalThis.fetch",
            "node>=18",
            vec!["axios", "node-fetch", "got", "request"],
            "use native fetch before adding an HTTP client dependency",
        ))
    } else if matches!(
        language_l.as_str(),
        "javascript" | "js" | "typescript" | "ts"
    ) && (task_l.contains("clone") || task_l.contains("deep copy"))
    {
        Some((
            "structuredClone",
            "node>=17/browser",
            vec!["lodash.cloneDeep", "rfdc"],
            "native structuredClone preserves common structured data without a dependency",
        ))
    } else if matches!(language_l.as_str(), "python" | "py")
        && (task_l.contains("json") || task_l.contains("parse"))
    {
        Some((
            "json",
            "python>=3.8",
            vec!["simplejson"],
            "stdlib json is the default unless benchmarked feature gaps require more",
        ))
    } else if matches!(language_l.as_str(), "python" | "py")
        && (task_l.contains("path") || task_l.contains("file"))
    {
        Some((
            "pathlib",
            "python>=3.8",
            vec!["path.py"],
            "stdlib pathlib covers path joins, suffixes, globbing, and metadata",
        ))
    } else if language_l == "rust" && (task_l.contains("cache") || task_l.contains("lazy")) {
        Some((
            "std::sync::OnceLock",
            "rust>=1.70",
            vec!["once_cell"],
            "prefer OnceLock for one-time initialization before adding once_cell",
        ))
    } else if language_l == "rust" && (task_l.contains("http") || task_l.contains("server")) {
        Some((
            "std::net::TcpListener",
            "rust>=1.56",
            vec!["tiny_http"],
            "TcpListener is enough for tiny local protocol/health endpoints",
        ))
    } else {
        None
    };

    if let Some((builtin, min_version, replacement_for, caveat)) = hit {
        result.status = "pass".to_string();
        result.builtin = Some(builtin.to_string());
        result.min_version = Some(min_version.to_string());
        result.replacement_for = replacement_for.into_iter().map(str::to_string).collect();
        result.caveat = Some(caveat.to_string());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_node_fetch_before_dependency() {
        let result = builtin_oracle("typescript", "node18", "make an http request");
        assert_eq!(result.status, "pass");
        assert_eq!(result.builtin.as_deref(), Some("globalThis.fetch"));
        assert!(result.replacement_for.iter().any(|dep| dep == "axios"));
    }
}

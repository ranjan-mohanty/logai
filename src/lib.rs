pub mod ai;
pub mod analyzer;
pub mod cli;
pub mod output;
pub mod parser;
pub mod search;
pub mod storage;

pub use anyhow::{Error, Result};

/// Core types used throughout the application
pub mod types {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LogEntry {
        pub timestamp: Option<DateTime<Utc>>,
        pub severity: Severity,
        pub message: String,
        pub metadata: LogMetadata,
        pub raw: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LogMetadata {
        pub file: Option<String>,
        pub line: Option<u32>,
        pub function: Option<String>,
        pub thread: Option<String>,
        pub extra: std::collections::HashMap<String, String>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Severity {
        Error,
        Warning,
        Info,
        Debug,
        Trace,
        Unknown,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ErrorGroup {
        pub id: String,
        pub pattern: String,
        pub count: usize,
        pub first_seen: DateTime<Utc>,
        pub last_seen: DateTime<Utc>,
        pub severity: Severity,
        pub entries: Vec<LogEntry>,
        pub analysis: Option<ErrorAnalysis>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ErrorAnalysis {
        pub explanation: String,
        pub root_cause: Option<String>,
        pub suggestions: Vec<Suggestion>,
        pub related_resources: Vec<Resource>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Suggestion {
        pub description: String,
        pub code_example: Option<String>,
        pub priority: u8,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Resource {
        pub title: String,
        pub url: String,
        pub source: String,
    }
}

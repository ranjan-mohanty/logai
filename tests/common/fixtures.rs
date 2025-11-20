//! Test fixtures for LogAI tests
//!
//! This module provides common test data and utilities for creating
//! test scenarios across the test suite.

#![allow(dead_code)]

use logai::types::{ErrorGroup, LogEntry, Severity};
use std::collections::HashMap;
use tempfile::{NamedTempFile, TempDir};

/// Common test fixtures
pub struct TestFixtures {
    pub temp_dirs: Vec<TempDir>,
}

impl TestFixtures {
    pub fn new() -> Self {
        Self {
            temp_dirs: Vec::new(),
        }
    }

    /// Create a temporary directory
    pub fn create_temp_dir(&mut self) -> &TempDir {
        let dir = TempDir::new().expect("Failed to create temp dir");
        self.temp_dirs.push(dir);
        self.temp_dirs.last().unwrap()
    }
}

impl Default for TestFixtures {
    fn default() -> Self {
        Self::new()
    }
}

/// Sample log entries for testing
pub fn sample_json_log() -> String {
    r#"{"level":"error","message":"Connection failed","timestamp":"2024-11-19T10:30:00Z"}"#
        .to_string()
}

pub fn sample_plain_log() -> String {
    "2024-11-19 10:30:00 ERROR Connection failed".to_string()
}

pub fn sample_apache_log() -> String {
    r#"127.0.0.1 - - [19/Nov/2024:10:30:00 +0000] "GET /api/users HTTP/1.1" 500 1234"#.to_string()
}

pub fn sample_nginx_log() -> String {
    r#"2024/11/19 10:30:00 [error] 12345#12345: *67890 connect() failed (111: Connection refused)"#
        .to_string()
}

pub fn sample_syslog() -> String {
    r#"<34>Nov 19 10:30:00 hostname app[12345]: Connection failed"#.to_string()
}

/// Sample error group for testing
pub fn sample_error_group() -> ErrorGroup {
    ErrorGroup {
        id: "test-group-1".to_string(),
        pattern: "Connection failed".to_string(),
        count: 5,
        severity: Severity::Error,
        first_seen: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        entries: vec![sample_log_entry()],
        analysis: None,
    }
}

/// Sample log entry for testing
pub fn sample_log_entry() -> LogEntry {
    use logai::types::LogMetadata;

    LogEntry {
        timestamp: Some(chrono::Utc::now()),
        severity: Severity::Error,
        message: "Connection failed".to_string(),
        raw: sample_plain_log(),
        metadata: LogMetadata {
            file: None,
            line: None,
            function: None,
            thread: None,
            extra: HashMap::new(),
        },
    }
}

/// Create a temporary log file with content
pub fn create_temp_log_file(content: &str) -> NamedTempFile {
    use std::io::Write;
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");
    file.flush().expect("Failed to flush temp file");
    file
}

/// Sample multi-line log with stack trace
pub fn sample_log_with_stack_trace() -> String {
    r#"2024-11-19 10:30:00 ERROR Failed to process request
    at com.example.Service.method(Service.java:42)
    at com.example.Controller.handle(Controller.java:89)
    at com.example.Main.main(Main.java:15)"#
        .to_string()
}

/// Sample malformed logs for error testing
pub fn sample_malformed_logs() -> Vec<String> {
    vec![
        "".to_string(),                         // Empty line
        "Not a valid log format".to_string(),   // No structure
        "2024-13-45 INVALID".to_string(),       // Invalid timestamp
        "{invalid json}".to_string(),           // Malformed JSON
        "\x00\x01\x02 binary data".to_string(), // Binary data
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_json_log() {
        let log = sample_json_log();
        assert!(log.contains("error"));
        assert!(log.contains("Connection failed"));
    }

    #[test]
    fn test_sample_error_group() {
        let group = sample_error_group();
        assert_eq!(group.count, 5);
        assert_eq!(group.severity, Severity::Error);
        assert!(!group.entries.is_empty());
    }

    #[test]
    fn test_create_temp_log_file() {
        let content = "test log content";
        let file = create_temp_log_file(content);
        assert!(file.path().exists());
    }

    #[test]
    fn test_sample_malformed_logs() {
        let logs = sample_malformed_logs();
        assert_eq!(logs.len(), 5);
        assert!(logs[0].is_empty());
    }
}

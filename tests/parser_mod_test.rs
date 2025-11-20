//! Tests for parser module trait and default implementations
//!
//! Tests cover:
//! - LogParser trait default implementations
//! - parse_lines default behavior
//! - supports_multiline default behavior
//! - is_continuation_line default behavior

use chrono::Utc;
use logai::parser::LogParser;
use logai::types::{LogEntry, LogMetadata, Severity};
use logai::Result;

/// Helper function to create a test LogEntry
fn create_log_entry(message: &str) -> LogEntry {
    LogEntry {
        timestamp: Some(Utc::now()),
        severity: Severity::Info,
        message: message.to_string(),
        metadata: LogMetadata {
            file: None,
            line: None,
            function: None,
            thread: None,
            extra: std::collections::HashMap::new(),
        },
        raw: message.to_string(),
    }
}

/// Mock parser for testing trait default implementations
struct MockParser {
    can_parse_result: bool,
    supports_multiline_override: Option<bool>,
}

impl MockParser {
    fn new(can_parse: bool) -> Self {
        Self {
            can_parse_result: can_parse,
            supports_multiline_override: None,
        }
    }

    fn with_multiline(mut self, supports: bool) -> Self {
        self.supports_multiline_override = Some(supports);
        self
    }
}

impl LogParser for MockParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        if line.is_empty() {
            return Ok(None);
        }

        Ok(Some(create_log_entry(line)))
    }

    fn can_parse(&self, _sample: &str) -> bool {
        self.can_parse_result
    }

    fn supports_multiline(&self) -> bool {
        self.supports_multiline_override.unwrap_or(false)
    }
}

#[test]
fn test_parse_lines_default_implementation() {
    let parser = MockParser::new(true);
    let lines = vec![
        "Line 1".to_string(),
        "Line 2".to_string(),
        "Line 3".to_string(),
    ];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].message, "Line 1");
    assert_eq!(entries[1].message, "Line 2");
    assert_eq!(entries[2].message, "Line 3");
}

#[test]
fn test_parse_lines_with_empty_lines() {
    let parser = MockParser::new(true);
    let lines = vec!["Line 1".to_string(), "".to_string(), "Line 3".to_string()];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert_eq!(entries.len(), 2); // Empty line should be skipped
    assert_eq!(entries[0].message, "Line 1");
    assert_eq!(entries[1].message, "Line 3");
}

#[test]
fn test_parse_lines_empty_input() {
    let parser = MockParser::new(true);
    let lines: Vec<String> = vec![];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert!(entries.is_empty());
}

#[test]
fn test_parse_lines_all_empty() {
    let parser = MockParser::new(true);
    let lines = vec!["".to_string(), "".to_string(), "".to_string()];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert!(entries.is_empty());
}

#[test]
fn test_supports_multiline_default() {
    let parser = MockParser::new(true);
    assert!(!parser.supports_multiline()); // Default should be false
}

#[test]
fn test_supports_multiline_override() {
    let parser = MockParser::new(true).with_multiline(true);
    assert!(parser.supports_multiline());
}

#[test]
fn test_is_continuation_line_default() {
    let parser = MockParser::new(true);
    assert!(!parser.is_continuation_line("    at some.function()"));
    assert!(!parser.is_continuation_line("regular line"));
    assert!(!parser.is_continuation_line(""));
}

#[test]
fn test_can_parse_true() {
    let parser = MockParser::new(true);
    assert!(parser.can_parse("any content"));
}

#[test]
fn test_can_parse_false() {
    let parser = MockParser::new(false);
    assert!(!parser.can_parse("any content"));
}

#[test]
fn test_parse_line_returns_none_for_empty() {
    let parser = MockParser::new(true);
    let result = parser.parse_line("");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_parse_line_returns_entry() {
    let parser = MockParser::new(true);
    let result = parser.parse_line("test message");
    assert!(result.is_ok());

    let entry = result.unwrap();
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().message, "test message");
}

#[test]
fn test_parse_lines_preserves_order() {
    let parser = MockParser::new(true);
    let lines = vec![
        "First".to_string(),
        "Second".to_string(),
        "Third".to_string(),
        "Fourth".to_string(),
    ];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert_eq!(entries.len(), 4);
    assert_eq!(entries[0].message, "First");
    assert_eq!(entries[1].message, "Second");
    assert_eq!(entries[2].message, "Third");
    assert_eq!(entries[3].message, "Fourth");
}

#[test]
fn test_parse_lines_with_whitespace_only() {
    let parser = MockParser::new(true);
    let lines = vec![
        "Valid line".to_string(),
        "   ".to_string(),
        "\t".to_string(),
        "Another valid line".to_string(),
    ];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    // Whitespace-only lines are not empty, so they should be parsed
    assert_eq!(entries.len(), 4);
}

#[test]
fn test_parse_lines_single_line() {
    let parser = MockParser::new(true);
    let lines = vec!["Single line".to_string()];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].message, "Single line");
}

#[test]
fn test_parse_lines_with_special_characters() {
    let parser = MockParser::new(true);
    let lines = vec![
        "Line with 🚀 emoji".to_string(),
        "Line with <html> tags".to_string(),
        "Line with \"quotes\"".to_string(),
    ];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert_eq!(entries.len(), 3);
    assert!(entries[0].message.contains("🚀"));
    assert!(entries[1].message.contains("<html>"));
    assert!(entries[2].message.contains("\"quotes\""));
}

#[test]
fn test_parse_lines_with_long_lines() {
    let parser = MockParser::new(true);
    let long_line = "x".repeat(10000);
    let lines = vec![long_line.clone()];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].message.len(), 10000);
}

#[test]
fn test_parse_lines_many_lines() {
    let parser = MockParser::new(true);
    let lines: Vec<String> = (0..1000).map(|i| format!("Line {}", i)).collect();

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert_eq!(entries.len(), 1000);
    assert_eq!(entries[0].message, "Line 0");
    assert_eq!(entries[999].message, "Line 999");
}

/// Mock parser that returns errors for testing error handling
struct ErrorParser;

impl LogParser for ErrorParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        if line.contains("error") {
            Err(anyhow::anyhow!("Parse error"))
        } else {
            Ok(Some(create_log_entry(line)))
        }
    }

    fn can_parse(&self, _sample: &str) -> bool {
        true
    }
}

#[test]
fn test_parse_lines_propagates_errors() {
    let parser = ErrorParser;
    let lines = vec![
        "Good line".to_string(),
        "error line".to_string(),
        "Another good line".to_string(),
    ];

    let result = parser.parse_lines(&lines);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Parse error"));
}

#[test]
fn test_parse_lines_stops_on_first_error() {
    let parser = ErrorParser;
    let lines = vec![
        "Good line 1".to_string(),
        "Good line 2".to_string(),
        "error line".to_string(),
        "Good line 3".to_string(),
    ];

    let result = parser.parse_lines(&lines);
    assert!(result.is_err());
}

/// Mock parser with custom continuation line logic
struct MultilineParser;

impl LogParser for MultilineParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        if line.is_empty() {
            return Ok(None);
        }

        Ok(Some(create_log_entry(line)))
    }

    fn can_parse(&self, _sample: &str) -> bool {
        true
    }

    fn supports_multiline(&self) -> bool {
        true
    }

    fn is_continuation_line(&self, line: &str) -> bool {
        line.starts_with("    ") || line.starts_with("\t")
    }
}

#[test]
fn test_multiline_parser_supports_multiline() {
    let parser = MultilineParser;
    assert!(parser.supports_multiline());
}

#[test]
fn test_multiline_parser_continuation_detection() {
    let parser = MultilineParser;
    assert!(parser.is_continuation_line("    indented line"));
    assert!(parser.is_continuation_line("\ttab indented"));
    assert!(!parser.is_continuation_line("regular line"));
    assert!(!parser.is_continuation_line(""));
}

#[test]
fn test_multiline_parser_parse_lines() {
    let parser = MultilineParser;
    let lines = vec![
        "Main log line".to_string(),
        "    continuation 1".to_string(),
        "    continuation 2".to_string(),
        "Another main line".to_string(),
    ];

    let result = parser.parse_lines(&lines);
    assert!(result.is_ok());

    let entries = result.unwrap();
    assert_eq!(entries.len(), 4); // Default implementation parses each line separately
}

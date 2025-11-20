//! Tests for analyzer module
//!
//! Tests cover:
//! - Analyzer creation and default implementation
//! - analyze method with various inputs
//! - Integration with ErrorGrouper

use chrono::Utc;
use logai::analyzer::Analyzer;
use logai::types::{LogEntry, LogMetadata, Severity};

/// Helper function to create a test LogEntry
fn create_log_entry(message: &str, severity: Severity) -> LogEntry {
    LogEntry {
        timestamp: Some(Utc::now()),
        severity,
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

#[test]
fn test_analyzer_new() {
    let analyzer = Analyzer::new();
    // Should create successfully
    let _ = analyzer;
}

#[test]
fn test_analyzer_default() {
    let analyzer = Analyzer;
    // Should create successfully via Default trait
    let _ = analyzer;
}

#[test]
fn test_analyze_empty_entries() {
    let analyzer = Analyzer::new();
    let entries = vec![];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(groups.is_empty());
}

#[test]
fn test_analyze_single_entry() {
    let analyzer = Analyzer::new();
    let entries = vec![create_log_entry(
        "Error: Connection failed",
        Severity::Error,
    )];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert_eq!(groups.len(), 1);
}

#[test]
fn test_analyze_multiple_similar_entries() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Error: Connection failed to server1", Severity::Error),
        create_log_entry("Error: Connection failed to server2", Severity::Error),
        create_log_entry("Error: Connection failed to server3", Severity::Error),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    // Similar errors should be grouped together
    assert!(!groups.is_empty());
}

#[test]
fn test_analyze_different_entries() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Error: Connection failed", Severity::Error),
        create_log_entry("Warning: Disk space low", Severity::Warning),
        create_log_entry("Error: Database timeout", Severity::Error),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    // Different errors should create multiple groups
    assert!(groups.len() >= 2);
}

#[test]
fn test_analyze_mixed_severities() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Error occurred", Severity::Error),
        create_log_entry("Warning message", Severity::Warning),
        create_log_entry("Info message", Severity::Info),
        create_log_entry("Debug message", Severity::Debug),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(!groups.is_empty());
}

#[test]
fn test_analyze_only_errors() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Error: Failed to connect", Severity::Error),
        create_log_entry("Error: Timeout occurred", Severity::Error),
        create_log_entry("Error: Invalid response", Severity::Error),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(!groups.is_empty());
}

#[test]
fn test_analyze_only_warnings() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Warning: Memory usage high", Severity::Warning),
        create_log_entry("Warning: CPU usage high", Severity::Warning),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(!groups.is_empty());
}

#[test]
fn test_analyze_large_number_of_entries() {
    let analyzer = Analyzer::new();
    let mut entries = Vec::new();

    for i in 0..1000 {
        entries.push(create_log_entry(
            &format!("Error: Connection failed attempt {}", i),
            Severity::Error,
        ));
    }

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(!groups.is_empty());
}

#[test]
fn test_analyze_with_special_characters() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Error: Failed with <special> chars", Severity::Error),
        create_log_entry("Error: Failed with \"quotes\"", Severity::Error),
        create_log_entry("Error: Failed with 🚀 emoji", Severity::Error),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(!groups.is_empty());
}

#[test]
fn test_analyze_with_long_messages() {
    let analyzer = Analyzer::new();
    let long_message = "Error: ".to_string() + &"x".repeat(10000);
    let entries = vec![create_log_entry(&long_message, Severity::Error)];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert_eq!(groups.len(), 1);
}

#[test]
fn test_analyze_with_empty_messages() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("", Severity::Error),
        create_log_entry("", Severity::Warning),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    // Empty messages should still be processed
    let _ = groups;
}

#[test]
fn test_analyze_with_whitespace_only() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("   ", Severity::Error),
        create_log_entry("\t\t", Severity::Error),
        create_log_entry("\n\n", Severity::Error),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    let _ = groups;
}

#[test]
fn test_analyze_preserves_entry_information() {
    let analyzer = Analyzer::new();
    let entries = vec![create_log_entry(
        "Error: Test error message",
        Severity::Error,
    )];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(!groups.is_empty());

    // Check that the group contains the error information
    let group = &groups[0];
    assert!(!group.pattern.is_empty());
    assert!(group.count > 0);
}

#[test]
fn test_analyzer_can_be_reused() {
    let analyzer = Analyzer::new();

    // First analysis
    let entries1 = vec![create_log_entry("Error: First batch", Severity::Error)];
    let result1 = analyzer.analyze(entries1);
    assert!(result1.is_ok());

    // Second analysis with same analyzer
    let entries2 = vec![create_log_entry("Error: Second batch", Severity::Error)];
    let result2 = analyzer.analyze(entries2);
    assert!(result2.is_ok());

    // Both should succeed independently
    assert!(!result1.unwrap().is_empty());
    assert!(!result2.unwrap().is_empty());
}

#[test]
fn test_analyze_with_multiline_messages() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Error: Line 1\nLine 2\nLine 3", Severity::Error),
        create_log_entry("Error: Another\nmultiline\nerror", Severity::Error),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(!groups.is_empty());
}

#[test]
fn test_analyze_with_unicode() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Error: 你好世界", Severity::Error),
        create_log_entry("Error: Привет мир", Severity::Error),
        create_log_entry("Error: مرحبا بالعالم", Severity::Error),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    assert!(!groups.is_empty());
}

#[test]
fn test_analyze_groups_have_valid_ids() {
    let analyzer = Analyzer::new();
    let entries = vec![
        create_log_entry("Error: Test error", Severity::Error),
        create_log_entry("Warning: Test warning", Severity::Warning),
    ];

    let result = analyzer.analyze(entries);
    assert!(result.is_ok());

    let groups = result.unwrap();
    for group in groups {
        assert!(!group.id.is_empty());
        assert!(!group.pattern.is_empty());
    }
}

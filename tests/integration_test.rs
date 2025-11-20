mod common;

use logai::{
    analyzer::Analyzer,
    parser::detector::FormatDetector,
    types::{LogEntry, Severity},
};
use std::fs;

#[test]
fn test_json_log_parsing_and_grouping() {
    let content =
        fs::read_to_string("tests/fixtures/sample.log").expect("Failed to read sample.log");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    assert!(!lines.is_empty(), "Sample log should not be empty");

    let parser = FormatDetector::detect(&lines[0]);
    let mut entries = Vec::new();

    for line in lines {
        if let Ok(Some(entry)) = parser.parse_line(&line) {
            entries.push(entry);
        }
    }

    assert!(!entries.is_empty(), "Should parse at least one entry");

    // Verify we have errors
    let error_count = entries
        .iter()
        .filter(|e| matches!(e.severity, Severity::Error))
        .count();
    assert!(error_count > 0, "Should have at least one error");

    // Test grouping
    let analyzer = Analyzer::new();
    let groups = analyzer.analyze(entries).expect("Analysis should succeed");

    assert!(!groups.is_empty(), "Should have at least one error group");

    // Verify grouping worked (should have fewer groups than total errors)
    assert!(
        groups.len() <= error_count,
        "Groups should be <= total errors"
    );

    // Verify group has correct structure
    let first_group = &groups[0];
    assert!(
        !first_group.pattern.is_empty(),
        "Pattern should not be empty"
    );
    assert!(first_group.count > 0, "Count should be positive");
    assert!(!first_group.entries.is_empty(), "Should have entries");
}

#[test]
fn test_cloudwatch_log_parsing() {
    let content = fs::read_to_string("tests/fixtures/cloudwatch-sample.log")
        .expect("Failed to read cloudwatch-sample.log");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    let parser = FormatDetector::detect(&lines[0]);
    let mut entries = Vec::new();

    for line in lines {
        if let Ok(Some(entry)) = parser.parse_line(&line) {
            entries.push(entry);
        }
    }

    assert!(!entries.is_empty(), "Should parse CloudWatch logs");

    // Verify severity detection
    let has_errors = entries
        .iter()
        .any(|e| matches!(e.severity, Severity::Error));
    assert!(has_errors, "Should detect ERROR severity");
}

#[test]
fn test_spring_boot_log_grouping() {
    let content = fs::read_to_string("tests/fixtures/spring-boot-sample.log")
        .expect("Failed to read spring-boot-sample.log");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    let parser = FormatDetector::detect(&lines[0]);
    let mut entries = Vec::new();

    for line in lines {
        if let Ok(Some(entry)) = parser.parse_line(&line) {
            entries.push(entry);
        }
    }

    let analyzer = Analyzer::new();
    let groups = analyzer.analyze(entries).expect("Analysis should succeed");

    // Should group the two "Connection timeout" errors together
    let connection_errors = groups
        .iter()
        .find(|g| g.pattern.contains("Connection timeout"));

    assert!(
        connection_errors.is_some(),
        "Should find connection timeout errors"
    );

    if let Some(group) = connection_errors {
        assert_eq!(
            group.count, 2,
            "Should group 2 connection timeout errors together"
        );
    }
}

#[test]
fn test_nginx_log_grouping() {
    let content = fs::read_to_string("tests/fixtures/nginx-sample.log")
        .expect("Failed to read nginx-sample.log");
    let lines: Vec<String> = content.lines().map(String::from).collect();

    let parser = FormatDetector::detect(&lines[0]);
    let mut entries = Vec::new();

    for line in lines {
        if let Ok(Some(entry)) = parser.parse_line(&line) {
            entries.push(entry);
        }
    }

    let analyzer = Analyzer::new();
    let groups = analyzer.analyze(entries).expect("Analysis should succeed");

    // Should group the two "Connection refused" errors together
    let connection_refused = groups
        .iter()
        .find(|g| g.pattern.contains("Connection refused"));

    assert!(
        connection_refused.is_some(),
        "Should find connection refused errors"
    );

    if let Some(group) = connection_refused {
        assert_eq!(
            group.count, 2,
            "Should group 2 connection refused errors together"
        );
    }
}

#[test]
fn test_empty_log_file() {
    let entries: Vec<LogEntry> = vec![];
    let analyzer = Analyzer::new();
    let groups = analyzer
        .analyze(entries)
        .expect("Should handle empty input");

    assert!(groups.is_empty(), "Empty input should produce no groups");
}

#[test]
fn test_no_errors_in_logs() {
    let content = r#"{"level":"info","message":"Request processed successfully","timestamp":"2025-11-17T10:30:00Z"}"#;
    let parser = FormatDetector::detect(content);

    let entry = parser
        .parse_line(content)
        .expect("Should parse")
        .expect("Should have entry");

    let analyzer = Analyzer::new();
    let groups = analyzer
        .analyze(vec![entry])
        .expect("Analysis should succeed");

    assert!(
        groups.is_empty(),
        "Info logs should not create error groups"
    );
}

#[test]
fn test_dynamic_value_normalization() {
    let entries = vec![
        create_test_entry("User 12345 not found"),
        create_test_entry("User 67890 not found"),
        create_test_entry("User 99999 not found"),
    ];

    let analyzer = Analyzer::new();
    let groups = analyzer.analyze(entries).expect("Analysis should succeed");

    assert_eq!(
        groups.len(),
        1,
        "Should group all 'User not found' errors together"
    );
    assert_eq!(groups[0].count, 3, "Should have 3 occurrences");
}

// Helper function
fn create_test_entry(message: &str) -> LogEntry {
    use std::collections::HashMap;

    LogEntry {
        timestamp: Some(chrono::Utc::now()),
        severity: Severity::Error,
        message: message.to_string(),
        metadata: logai::types::LogMetadata {
            file: None,
            line: None,
            function: None,
            thread: None,
            extra: HashMap::new(),
        },
        raw: message.to_string(),
    }
}

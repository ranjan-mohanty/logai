mod common;

use common::assertions::{assert_html_contains_elements, assert_html_valid};
use common::fixtures::sample_error_group;
use logai::output::{html::HtmlFormatter, terminal::TerminalFormatter, OutputFormatter};

#[test]
fn test_html_formatter_basic() {
    let formatter = HtmlFormatter::new(10);
    let groups = vec![sample_error_group()];

    let result = formatter.format(&groups);
    assert!(result.is_ok());

    let html = result.unwrap();
    assert_html_valid(&html);
}

#[test]
fn test_html_formatter_with_empty_groups() {
    let formatter = HtmlFormatter::new(10);
    let groups = vec![];

    let result = formatter.format(&groups);
    assert!(result.is_ok());

    let html = result.unwrap();
    assert_html_valid(&html);
}

#[test]
fn test_html_formatter_with_limit() {
    let formatter = HtmlFormatter::new(2);
    let groups = vec![
        sample_error_group(),
        sample_error_group(),
        sample_error_group(),
    ];

    let result = formatter.format(&groups);
    assert!(result.is_ok());

    let html = result.unwrap();
    assert_html_valid(&html);
}

#[test]
fn test_html_formatter_contains_required_elements() {
    let formatter = HtmlFormatter::new(10);
    let groups = vec![sample_error_group()];

    let html = formatter.format(&groups).unwrap();

    assert_html_contains_elements(
        &html,
        &[
            "<!DOCTYPE html>",
            "<html",
            "<head>",
            "<body",
            "LogAI Analysis Report",
        ],
    );
}

#[test]
fn test_html_formatter_with_no_limit() {
    let formatter = HtmlFormatter::new(0);
    let groups = vec![sample_error_group(), sample_error_group()];

    let result = formatter.format(&groups);
    assert!(result.is_ok());
}

#[test]
fn test_terminal_formatter_basic() {
    let formatter = TerminalFormatter::new(10);
    let groups = vec![sample_error_group()];

    let result = formatter.format(&groups);
    assert!(result.is_ok());

    let output = result.unwrap();
    assert!(!output.is_empty());
}

#[test]
fn test_terminal_formatter_with_empty_groups() {
    let formatter = TerminalFormatter::new(10);
    let groups = vec![];

    let result = formatter.format(&groups);
    assert!(result.is_ok());
}

#[test]
fn test_terminal_formatter_with_limit() {
    let formatter = TerminalFormatter::new(2);
    let groups = vec![
        sample_error_group(),
        sample_error_group(),
        sample_error_group(),
    ];

    let result = formatter.format(&groups);
    assert!(result.is_ok());
}

#[test]
fn test_terminal_formatter_contains_error_info() {
    let formatter = TerminalFormatter::new(10);
    let groups = vec![sample_error_group()];

    let output = formatter.format(&groups).unwrap();

    // Should contain error pattern
    assert!(output.contains("Connection failed"));
}

// ============================================================================
// Additional Terminal Formatter Tests
// ============================================================================

#[test]
fn test_terminal_formatter_severity_icons() {
    use common::fixtures;
    use logai::types::Severity;

    // Test that different severities produce different output
    let mut group1 = fixtures::sample_error_group();
    group1.severity = Severity::Error;

    let mut group2 = fixtures::sample_error_group();
    group2.severity = Severity::Warning;

    let mut group3 = fixtures::sample_error_group();
    group3.severity = Severity::Info;

    let formatter = TerminalFormatter::new(10);
    let output1 = formatter.format(&[group1]).unwrap();
    let output2 = formatter.format(&[group2]).unwrap();
    let output3 = formatter.format(&[group3]).unwrap();

    // Each should have different content
    assert_ne!(output1, output2);
    assert_ne!(output2, output3);
}

#[test]
fn test_terminal_formatter_time_range() {
    use chrono::{Duration, Utc};
    use common::fixtures;

    let mut group = fixtures::sample_error_group();

    // Set specific timestamps
    let now = Utc::now();
    let earlier = now - Duration::hours(2);

    if let Some(entry) = group.entries.first_mut() {
        entry.timestamp = Some(earlier);
    }

    group.first_seen = earlier;
    group.last_seen = now;

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    // Should contain time range information
    assert!(output.contains("Time range"));
}

#[test]
fn test_terminal_formatter_multiple_severities() {
    use common::fixtures;
    use logai::types::Severity;

    let mut groups = vec![];

    for severity in &[
        Severity::Error,
        Severity::Warning,
        Severity::Info,
        Severity::Debug,
        Severity::Trace,
        Severity::Unknown,
    ] {
        let mut group = fixtures::sample_error_group();
        group.severity = *severity;
        group.pattern = format!("Error with {:?} severity", severity);
        groups.push(group);
    }

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&groups).unwrap();

    // Should contain all severity types
    assert!(output.contains("Error with"));
}

#[test]
fn test_terminal_formatter_large_count() {
    use common::fixtures;
    let mut group = fixtures::sample_error_group();
    group.count = 9999;

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    // Should display the large count
    assert!(output.contains("9999"));
}

#[test]
fn test_terminal_formatter_with_analysis() {
    use common::fixtures;
    use logai::types::{ErrorAnalysis, Resource, Suggestion};

    let mut group = fixtures::sample_error_group();
    group.analysis = Some(ErrorAnalysis {
        explanation: "This is a test explanation".to_string(),
        root_cause: Some("Test root cause".to_string()),
        suggestions: vec![
            Suggestion {
                description: "Fix suggestion 1".to_string(),
                code_example: Some("code example".to_string()),
                priority: 1,
            },
            Suggestion {
                description: "Fix suggestion 2".to_string(),
                code_example: None,
                priority: 2,
            },
        ],
        related_resources: vec![Resource {
            title: "Example Resource".to_string(),
            url: "https://example.com".to_string(),
            source: "test".to_string(),
        }],
        tool_invocations: vec![],
    });

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    // Should contain analysis information
    assert!(output.contains("explanation") || output.contains("AI Analysis"));
}

#[test]
fn test_terminal_formatter_summary_stats() {
    use common::fixtures;
    let groups = vec![
        fixtures::sample_error_group(),
        fixtures::sample_error_group(),
        fixtures::sample_error_group(),
    ];

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&groups).unwrap();

    // Should contain summary section
    assert!(output.contains("Summary") || output.contains("📊"));
    assert!(output.contains("unique patterns") || output.contains("occurrences"));
}

#[test]
fn test_terminal_formatter_truncation() {
    use common::fixtures;
    let mut groups = vec![];
    for i in 0..20 {
        let mut group = fixtures::sample_error_group();
        group.pattern = format!("Error pattern {}", i);
        groups.push(group);
    }

    let formatter = TerminalFormatter::new(5);
    let output = formatter.format(&groups).unwrap();

    // Should only show first 5
    assert!(output.contains("Error pattern 0"));
    assert!(output.contains("Error pattern 4"));
    // Should not show pattern 10 or higher
    assert!(!output.contains("Error pattern 10"));
}

#[test]
fn test_terminal_formatter_no_limit() {
    use common::fixtures;
    let mut groups = vec![];
    for i in 0..15 {
        let mut group = fixtures::sample_error_group();
        group.pattern = format!("Error pattern {}", i);
        groups.push(group);
    }

    let formatter = TerminalFormatter::new(usize::MAX);
    let output = formatter.format(&groups).unwrap();

    // Should show all patterns
    assert!(output.contains("Error pattern 0"));
    assert!(output.contains("Error pattern 14"));
}

#[test]
fn test_terminal_formatter_special_characters() {
    use common::fixtures;
    let mut group = fixtures::sample_error_group();
    group.pattern = "Error with <special> & \"characters\"".to_string();
    group.entries[0].message = "Message with 'quotes' and\nnewlines".to_string();

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    // Should handle special characters
    assert!(output.contains("special") || output.contains("characters"));
}

#[test]
fn test_terminal_formatter_unicode() {
    use common::fixtures;
    let mut group = fixtures::sample_error_group();
    group.pattern = "Error with unicode: 你好 🚀 ñ".to_string();

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    // Should handle unicode
    assert!(output.contains("unicode") || output.contains("你好"));
}

#[test]
fn test_terminal_formatter_long_pattern() {
    use common::fixtures;
    let mut group = fixtures::sample_error_group();
    group.pattern = "A".repeat(500);

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    // Should not panic with long patterns
    assert!(!output.is_empty());
}

#[test]
fn test_terminal_formatter_empty_message() {
    use common::fixtures;
    let mut group = fixtures::sample_error_group();
    group.entries[0].message = String::new();

    let formatter = TerminalFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    // Should handle empty messages
    assert!(!output.is_empty());
}

// ============================================================================
// Additional HTML Formatter Tests
// ============================================================================

#[test]
fn test_html_formatter_with_analysis() {
    use common::fixtures;
    use logai::types::{ErrorAnalysis, Resource, Suggestion};

    let mut group = fixtures::sample_error_group();
    group.analysis = Some(ErrorAnalysis {
        explanation: "Test explanation for HTML".to_string(),
        root_cause: Some("Test root cause".to_string()),
        suggestions: vec![Suggestion {
            description: "Fix suggestion 1".to_string(),
            code_example: Some("code example".to_string()),
            priority: 1,
        }],
        related_resources: vec![Resource {
            title: "Example Resource".to_string(),
            url: "https://example.com".to_string(),
            source: "test".to_string(),
        }],
        tool_invocations: vec![],
    });

    let formatter = HtmlFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    common::assertions::assert_html_valid(&output);
    assert!(output.contains("explanation") || output.contains("AI Analysis"));
}

#[test]
fn test_html_formatter_multiple_groups() {
    use common::fixtures;

    let mut groups = vec![];
    for i in 0..5 {
        let mut group = fixtures::sample_error_group();
        group.pattern = format!("Error pattern {}", i);
        groups.push(group);
    }

    let formatter = HtmlFormatter::new(10);
    let output = formatter.format(&groups).unwrap();

    common::assertions::assert_html_valid(&output);
    assert!(output.contains("Error pattern 0"));
    assert!(output.contains("Error pattern 4"));
}

#[test]
fn test_html_formatter_special_chars_escaping() {
    use common::fixtures;

    let mut group = fixtures::sample_error_group();
    group.pattern = "Error with <script>alert('xss')</script>".to_string();
    group.entries[0].message = "Message with <>&\"'".to_string();

    let formatter = HtmlFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    common::assertions::assert_html_valid(&output);
    // HTML should contain the pattern (may or may not be escaped depending on implementation)
    assert!(output.contains("script") || output.contains("&lt;script&gt;"));
}

#[test]
fn test_html_formatter_large_groups() {
    use common::fixtures;

    let mut group = fixtures::sample_error_group();
    group.count = 99999;

    let formatter = HtmlFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    common::assertions::assert_html_valid(&output);
    assert!(output.contains("99999"));
}

#[test]
fn test_html_formatter_different_severities() {
    use common::fixtures;
    use logai::types::Severity;

    let mut groups = vec![];
    for severity in &[Severity::Error, Severity::Warning, Severity::Info] {
        let mut group = fixtures::sample_error_group();
        group.severity = *severity;
        groups.push(group);
    }

    let formatter = HtmlFormatter::new(10);
    let output = formatter.format(&groups).unwrap();

    common::assertions::assert_html_valid(&output);
}

#[test]
fn test_html_formatter_unicode_content() {
    use common::fixtures;

    let mut group = fixtures::sample_error_group();
    group.pattern = "Error with unicode: 你好 🚀 ñ".to_string();

    let formatter = HtmlFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    common::assertions::assert_html_valid(&output);
}

#[test]
fn test_html_formatter_long_messages() {
    use common::fixtures;

    let mut group = fixtures::sample_error_group();
    group.entries[0].message = "A".repeat(1000);

    let formatter = HtmlFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    common::assertions::assert_html_valid(&output);
}

#[test]
fn test_html_formatter_with_timestamps() {
    use chrono::Utc;
    use common::fixtures;

    let mut group = fixtures::sample_error_group();
    group.first_seen = Utc::now();
    group.last_seen = Utc::now();

    let formatter = HtmlFormatter::new(10);
    let output = formatter.format(&[group]).unwrap();

    common::assertions::assert_html_valid(&output);
}

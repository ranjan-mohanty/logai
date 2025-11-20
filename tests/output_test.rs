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

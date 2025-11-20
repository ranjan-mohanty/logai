//! Custom assertions for testing
//!
//! This module provides custom assertion helpers for common test scenarios.

#![allow(dead_code)]

use logai::types::ErrorGroup;

/// Assert that HTML is valid and well-formed
pub fn assert_html_valid(html: &str) {
    assert!(html.contains("<!DOCTYPE html>"), "HTML missing DOCTYPE");
    assert!(html.contains("<html"), "HTML missing html tag");
    assert!(html.contains("</html>"), "HTML missing closing html tag");
    assert!(html.contains("<head>"), "HTML missing head tag");
    assert!(html.contains("</head>"), "HTML missing closing head tag");
    assert!(html.contains("<body"), "HTML missing body tag");
    assert!(html.contains("</body>"), "HTML missing closing body tag");
}

/// Assert that HTML contains required elements
pub fn assert_html_contains_elements(html: &str, elements: &[&str]) {
    for element in elements {
        assert!(
            html.contains(element),
            "HTML missing required element: {}",
            element
        );
    }
}

/// Assert that error groups are properly sorted
pub fn assert_groups_sorted_by_count(groups: &[ErrorGroup]) {
    for i in 1..groups.len() {
        assert!(
            groups[i - 1].count >= groups[i].count,
            "Groups not sorted by count: {} < {}",
            groups[i - 1].count,
            groups[i].count
        );
    }
}

/// Assert that error groups have unique patterns
pub fn assert_groups_have_unique_patterns(groups: &[ErrorGroup]) {
    let mut patterns = std::collections::HashSet::new();
    for group in groups {
        assert!(
            patterns.insert(&group.pattern),
            "Duplicate pattern found: {}",
            group.pattern
        );
    }
}

/// Assert that a string is valid JSON
pub fn assert_valid_json(s: &str) {
    serde_json::from_str::<serde_json::Value>(s).expect("String is not valid JSON");
}

/// Assert that a file exists and is readable
pub fn assert_file_exists_and_readable(path: &std::path::Path) {
    assert!(path.exists(), "File does not exist: {:?}", path);
    assert!(path.is_file(), "Path is not a file: {:?}", path);
    std::fs::read(path).expect("File is not readable");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::fixtures;

    #[test]
    fn test_assert_html_valid() {
        let valid_html = r#"<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body><p>Content</p></body>
</html>"#;
        assert_html_valid(valid_html);
    }

    #[test]
    #[should_panic(expected = "HTML missing DOCTYPE")]
    fn test_assert_html_valid_fails() {
        let invalid_html = "<html><body></body></html>";
        assert_html_valid(invalid_html);
    }

    #[test]
    fn test_assert_html_contains_elements() {
        let html = "<div><span>test</span></div>";
        assert_html_contains_elements(html, &["<div>", "<span>", "test"]);
    }

    #[test]
    fn test_assert_groups_sorted_by_count() {
        let groups = vec![
            fixtures::sample_error_group(),
            fixtures::sample_error_group(),
        ];
        assert_groups_sorted_by_count(&groups);
    }

    #[test]
    fn test_assert_valid_json() {
        assert_valid_json(r#"{"key": "value"}"#);
        assert_valid_json(r#"[1, 2, 3]"#);
    }

    #[test]
    #[should_panic(expected = "not valid JSON")]
    fn test_assert_valid_json_fails() {
        assert_valid_json("not json");
    }
}

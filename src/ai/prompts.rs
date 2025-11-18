use crate::types::ErrorGroup;

/// Build the analysis prompt for an error group
pub fn build_analysis_prompt(group: &ErrorGroup) -> String {
    build_enhanced_analysis_prompt(group, 2000)
}

/// Build enhanced analysis prompt with explicit JSON-only instructions
pub fn build_enhanced_analysis_prompt(group: &ErrorGroup, max_length: usize) -> String {
    // Get example messages (up to 3)
    let examples: Vec<String> = group
        .entries
        .iter()
        .take(3)
        .map(|e| truncate_message(&e.message, max_length))
        .collect();

    let examples_text = if examples.is_empty() {
        truncate_message(&group.pattern, max_length)
    } else {
        examples.join("\n")
    };

    format!(
        r#"Analyze this error pattern and provide a JSON response.

IMPORTANT: Return ONLY valid JSON. Do not include:
- Markdown code blocks (```json or ```)
- Explanations outside the JSON structure
- Code examples outside the JSON structure
- Any text before or after the JSON object

Error Pattern:
{}

Occurrences: {}
Severity: {:?}

Example Messages:
{}

Required JSON format:
{{
  "explanation": "Clear explanation of what this error means",
  "root_cause": "The underlying cause of this error",
  "suggestions": [
    {{
      "description": "How to fix this issue",
      "code_example": "Optional code example as a string",
      "priority": 1
    }}
  ]
}}

Priority levels: 1 (critical) to 5 (minor)

Return ONLY the JSON object, nothing else."#,
        group.pattern, group.count, group.severity, examples_text
    )
}

/// Truncate message to maximum length
fn truncate_message(message: &str, max_length: usize) -> String {
    if message.len() > max_length {
        format!("{}... (truncated)", &message[..max_length])
    } else {
        message.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LogEntry, LogMetadata, Severity};
    use chrono::Utc;

    fn create_test_group() -> ErrorGroup {
        ErrorGroup {
            id: "test-1".to_string(),
            pattern: "NullPointerException".to_string(),
            count: 5,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            severity: Severity::Error,
            entries: vec![LogEntry {
                timestamp: Some(Utc::now()),
                severity: Severity::Error,
                message: "NullPointerException at line 42".to_string(),
                metadata: LogMetadata {
                    file: None,
                    line: None,
                    function: None,
                    thread: None,
                    extra: std::collections::HashMap::new(),
                },
                raw: "".to_string(),
            }],
            analysis: None,
        }
    }

    #[test]
    fn test_build_analysis_prompt() {
        let group = create_test_group();
        let prompt = build_analysis_prompt(&group);

        assert!(prompt.contains("NullPointerException"));
        assert!(prompt.contains("Occurrences: 5"));
        assert!(prompt.contains("ONLY valid JSON"));
        assert!(prompt.contains("Required JSON format"));
    }

    #[test]
    fn test_build_enhanced_analysis_prompt() {
        let group = create_test_group();
        let prompt = build_enhanced_analysis_prompt(&group, 2000);

        assert!(prompt.contains("NullPointerException"));
        assert!(prompt.contains("Return ONLY the JSON object"));
        assert!(prompt.contains("Do not include"));
        assert!(prompt.contains("Markdown code blocks"));
    }

    #[test]
    fn test_truncate_message_short() {
        let message = "Short message";
        let truncated = truncate_message(message, 100);
        assert_eq!(truncated, "Short message");
    }

    #[test]
    fn test_truncate_message_long() {
        let message = "A".repeat(3000);
        let truncated = truncate_message(&message, 2000);
        assert_eq!(truncated.len(), 2015); // 2000 + "... (truncated)"
        assert!(truncated.ends_with("... (truncated)"));
    }

    #[test]
    fn test_prompt_includes_multiple_examples() {
        let mut group = create_test_group();
        group.entries.push(LogEntry {
            timestamp: Some(Utc::now()),
            severity: Severity::Error,
            message: "NullPointerException at line 43".to_string(),
            metadata: LogMetadata {
                file: None,
                line: None,
                function: None,
                thread: None,
                extra: std::collections::HashMap::new(),
            },
            raw: "".to_string(),
        });

        let prompt = build_enhanced_analysis_prompt(&group, 2000);
        assert!(prompt.contains("line 42"));
        assert!(prompt.contains("line 43"));
    }

    #[test]
    fn test_prompt_limits_to_three_examples() {
        let mut group = create_test_group();
        for i in 1..=5 {
            group.entries.push(LogEntry {
                timestamp: Some(Utc::now()),
                severity: Severity::Error,
                message: format!("Error {}", i),
                metadata: LogMetadata {
                    file: None,
                    line: None,
                    function: None,
                    thread: None,
                    extra: std::collections::HashMap::new(),
                },
                raw: "".to_string(),
            });
        }

        let prompt = build_enhanced_analysis_prompt(&group, 2000);
        // Should only include first 3 examples from entries
        assert!(prompt.contains("line 42")); // Original entry
        assert!(prompt.contains("Error 1"));
        assert!(prompt.contains("Error 2"));
        assert!(!prompt.contains("Error 3")); // 4th entry, should be excluded
        assert!(!prompt.contains("Error 4"));
        assert!(!prompt.contains("Error 5"));
    }
}

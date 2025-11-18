use crate::types::ErrorGroup;

/// Build the analysis prompt for an error group
pub fn build_analysis_prompt(group: &ErrorGroup) -> String {
    let example = group
        .entries
        .first()
        .map(|e| &e.message)
        .unwrap_or(&group.pattern);

    format!(
        r#"You are a debugging assistant analyzing application errors. Analyze this error and provide actionable insights.

Error Pattern: {}
Occurrences: {}
Severity: {:?}
Example: {}

Provide your analysis in the following JSON format:
{{
  "explanation": "Clear explanation of what this error means in plain English",
  "root_cause": "The likely root cause of this error",
  "suggestions": [
    {{
      "description": "First suggestion to fix the issue",
      "code_example": "Optional code example",
      "priority": 1
    }}
  ]
}}

Be concise and practical. Focus on actionable fixes."#,
        group.pattern, group.count, group.severity, example
    )
}

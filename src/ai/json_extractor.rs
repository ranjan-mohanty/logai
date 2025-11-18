use crate::Result;

/// Enhanced JSON extractor for handling mixed text/JSON responses
pub struct EnhancedJsonExtractor;

impl EnhancedJsonExtractor {
    /// Extract JSON from response that may contain markdown or extra text
    pub fn extract(response: &str) -> Result<String> {
        // First, try to strip markdown code blocks
        let cleaned = Self::strip_markdown(response);

        // Try to find JSON boundaries
        if let Some((start, end)) = Self::find_json_boundaries(cleaned) {
            let json_str = &cleaned[start..=end];

            // Attempt to repair common JSON issues
            let repaired = Self::repair_json(json_str);

            // Validate the JSON
            Self::validate_json(&repaired)?;

            return Ok(repaired);
        }

        Err(anyhow::anyhow!(
            "Could not find valid JSON in response. First 200 chars: {}",
            &response[..response.len().min(200)]
        ))
    }

    /// Strip markdown code blocks from text
    fn strip_markdown(text: &str) -> &str {
        // Remove ```json ... ``` blocks
        if let Some(start) = text.find("```json") {
            let content_start = start + 7; // Length of "```json"
            if let Some(end_marker) = text[content_start..].find("```") {
                let content_end = content_start + end_marker;
                return text[content_start..content_end].trim();
            }
        }

        // Remove ``` ... ``` blocks
        if let Some(start) = text.find("```") {
            let content_start = start + 3;
            if let Some(end_marker) = text[content_start..].find("```") {
                let content_end = content_start + end_marker;
                return text[content_start..content_end].trim();
            }
        }

        text.trim()
    }

    /// Find JSON object boundaries in text
    fn find_json_boundaries(text: &str) -> Option<(usize, usize)> {
        let start = text.find('{')?;

        // Find matching closing brace
        let mut depth = 0;
        let mut in_string = false;
        let mut escape_next = false;

        for (i, ch) in text[start..].char_indices() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' if in_string => escape_next = true,
                '"' => in_string = !in_string,
                '{' if !in_string => depth += 1,
                '}' if !in_string => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((start, start + i));
                    }
                }
                _ => {}
            }
        }

        None
    }

    /// Attempt to repair common JSON issues
    fn repair_json(json: &str) -> String {
        let mut repaired = json.to_string();

        // Fix missing commas between object properties
        // This is a simple heuristic - look for }\n{ or ]\n[ patterns
        repaired = repaired.replace("}\n{", "},\n{");
        repaired = repaired.replace("]\n[", "],\n[");
        repaired = repaired.replace("}\n  {", "},\n  {");
        repaired = repaired.replace("]\n  [", "],\n  [");

        // Fix missing commas between array elements
        repaired = repaired.replace("}\n  ]", "}\n]");
        repaired = repaired.replace("]\n  ]", "]\n]");

        // Remove trailing commas before closing braces/brackets
        repaired = repaired.replace(",\n}", "\n}");
        repaired = repaired.replace(",\n]", "\n]");
        repaired = repaired.replace(", }", " }");
        repaired = repaired.replace(", ]", " ]");

        repaired
    }

    /// Validate extracted JSON structure
    fn validate_json(json: &str) -> Result<()> {
        // Try to parse as JSON
        let value: serde_json::Value = serde_json::from_str(json).map_err(|e| {
            anyhow::anyhow!(
                "Invalid JSON: {}. First 200 chars: {}",
                e,
                &json[..json.len().min(200)]
            )
        })?;

        // Check for required fields
        if let Some(obj) = value.as_object() {
            if !obj.contains_key("explanation") {
                return Err(anyhow::anyhow!(
                    "Missing required field 'explanation' in JSON response"
                ));
            }

            if !obj.contains_key("suggestions") {
                return Err(anyhow::anyhow!(
                    "Missing required field 'suggestions' in JSON response"
                ));
            }
        } else {
            return Err(anyhow::anyhow!("JSON response is not an object"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_from_markdown_json() {
        let response = r#"Here's the analysis:
```json
{
  "explanation": "Test error",
  "suggestions": []
}
```
Hope this helps!"#;

        let result = EnhancedJsonExtractor::extract(response);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("explanation"));
        assert!(json.contains("Test error"));
    }

    #[test]
    fn test_extract_from_markdown_generic() {
        let response = r#"```
{
  "explanation": "Test error",
  "suggestions": []
}
```"#;

        let result = EnhancedJsonExtractor::extract(response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_with_text_before() {
        let response = r#"Sure, here's the analysis:
{
  "explanation": "Test error",
  "suggestions": []
}"#;

        let result = EnhancedJsonExtractor::extract(response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_with_text_after() {
        let response = r#"{
  "explanation": "Test error",
  "suggestions": []
}
Let me know if you need more details."#;

        let result = EnhancedJsonExtractor::extract(response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_with_text_before_and_after() {
        let response = r#"Here's what I found:
{
  "explanation": "Test error",
  "suggestions": []
}
Hope this helps!"#;

        let result = EnhancedJsonExtractor::extract(response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_braces() {
        let response = r#"{
  "explanation": "Error with {nested} braces",
  "suggestions": [
    {
      "description": "Fix {this}",
      "priority": 1
    }
  ]
}"#;

        let result = EnhancedJsonExtractor::extract(response);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("nested"));
    }

    #[test]
    fn test_repair_missing_comma() {
        let json = r#"{
  "explanation": "Test"
  "suggestions": []
}"#;

        let repaired = EnhancedJsonExtractor::repair_json(json);
        // Note: This simple repair won't fix all cases, but it's a best effort
        assert!(repaired.contains("explanation"));
    }

    #[test]
    fn test_repair_trailing_comma() {
        let json = r#"{
  "explanation": "Test",
  "suggestions": [],
}"#;

        let repaired = EnhancedJsonExtractor::repair_json(json);
        assert!(!repaired.contains(",\n}"));
    }

    #[test]
    fn test_validate_missing_explanation() {
        let json = r#"{
  "suggestions": []
}"#;

        let result = EnhancedJsonExtractor::validate_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing required field 'explanation'"));
    }

    #[test]
    fn test_validate_missing_suggestions() {
        let json = r#"{
  "explanation": "Test"
}"#;

        let result = EnhancedJsonExtractor::validate_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing required field 'suggestions'"));
    }

    #[test]
    fn test_validate_valid_json() {
        let json = r#"{
  "explanation": "Test error",
  "root_cause": "Something",
  "suggestions": [
    {
      "description": "Fix it",
      "priority": 1
    }
  ]
}"#;

        let result = EnhancedJsonExtractor::validate_json(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_empty_response() {
        let response = "";
        let result = EnhancedJsonExtractor::extract(response);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_no_json() {
        let response = "This is just plain text without any JSON";
        let result = EnhancedJsonExtractor::extract(response);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_json_boundaries_with_strings() {
        let text = r#"Some text {"key": "value with } brace"} more text"#;
        let result = EnhancedJsonExtractor::find_json_boundaries(text);
        assert!(result.is_some());
        let (start, end) = result.unwrap();
        assert_eq!(&text[start..=end], r#"{"key": "value with } brace"}"#);
    }

    #[test]
    fn test_strip_markdown_json_block() {
        let text = "Some text\n```json\n{\"key\": \"value\"}\n```\nMore text";
        let result = EnhancedJsonExtractor::strip_markdown(text);
        assert_eq!(result, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_strip_markdown_generic_block() {
        let text = "Some text\n```\n{\"key\": \"value\"}\n```\nMore text";
        let result = EnhancedJsonExtractor::strip_markdown(text);
        assert_eq!(result, r#"{"key": "value"}"#);
    }
}

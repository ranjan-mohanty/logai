use regex::Regex;
use std::collections::HashMap;

/// Extractor for metadata fields from log entries
pub struct MetadataExtractor {
    file_pattern: Regex,
    thread_pattern: Regex,
    request_id_pattern: Regex,
    function_pattern: Regex,
    custom_patterns: HashMap<String, Regex>,
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataExtractor {
    /// Create a new metadata extractor with default patterns
    pub fn new() -> Self {
        Self {
            // Match file paths with line numbers: File.java:123, script.py:45, "script.py", line 45
            file_pattern: Regex::new(r#"(?:"([^"]+\.(?:java|py|js|ts|cs|go|rs|cpp|c|h))",\s*line\s*(\d+)|(\w+\.(?:java|py|js|ts|cs|go|rs|cpp|c|h)):(\d+))"#).unwrap(),
            // Match thread information: [thread-1], thread_id=worker-123
            thread_pattern: Regex::new(r"(?:\[([^\]]+)\]|thread[_-]?(?:id)?[=:]?\s*([a-zA-Z0-9-]+))").unwrap(),
            // Match request/trace IDs
            request_id_pattern: Regex::new(
                r"(?:request[_-]?id|req[_-]?id|trace[_-]?id|correlation[_-]?id)[=:]?\s*([a-zA-Z0-9-]+)",
            )
            .unwrap(),
            // Match function names: at com.example.Class.method(
            function_pattern: Regex::new(r"at\s+([a-zA-Z0-9_.]+)\(").unwrap(),
            custom_patterns: HashMap::new(),
        }
    }

    /// Create a metadata extractor with custom patterns
    pub fn with_custom_patterns(patterns: HashMap<String, String>) -> Self {
        let custom_patterns = patterns
            .into_iter()
            .filter_map(|(name, pattern)| Regex::new(&pattern).ok().map(|regex| (name, regex)))
            .collect();

        Self {
            file_pattern: Regex::new(r#"(?:"([^"]+\.(?:java|py|js|ts|cs|go|rs|cpp|c|h))",\s*line\s*(\d+)|(\w+\.(?:java|py|js|ts|cs|go|rs|cpp|c|h)):(\d+))"#).unwrap(),
            thread_pattern: Regex::new(r"(?:\[([^\]]+)\]|thread[_-]?(?:id)?[=:]?\s*([a-zA-Z0-9-]+))").unwrap(),
            request_id_pattern: Regex::new(
                r"(?:request[_-]?id|req[_-]?id|trace[_-]?id|correlation[_-]?id)[=:]?\s*([a-zA-Z0-9-]+)",
            )
            .unwrap(),
            function_pattern: Regex::new(r"at\s+([a-zA-Z0-9_.]+)\(").unwrap(),
            custom_patterns,
        }
    }

    /// Extract metadata from a log line
    pub fn extract(&self, text: &str) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        // Extract file and line number
        if let Some(caps) = self.file_pattern.captures(text) {
            // Python format: "script.py", line 45
            if let Some(file) = caps.get(1) {
                metadata.insert("file".to_string(), file.as_str().to_string());
                if let Some(line) = caps.get(2) {
                    metadata.insert("line".to_string(), line.as_str().to_string());
                }
            }
            // Java/other format: Service.java:123
            else if let Some(file) = caps.get(3) {
                metadata.insert("file".to_string(), file.as_str().to_string());
                if let Some(line) = caps.get(4) {
                    metadata.insert("line".to_string(), line.as_str().to_string());
                }
            }
        }

        // Extract thread information
        if let Some(caps) = self.thread_pattern.captures(text) {
            // Try first capture group (bracket notation)
            if let Some(thread) = caps.get(1) {
                metadata.insert("thread".to_string(), thread.as_str().to_string());
            }
            // Try second capture group (key=value notation)
            else if let Some(thread) = caps.get(2) {
                metadata.insert("thread".to_string(), thread.as_str().to_string());
            }
        }

        // Extract request ID
        if let Some(caps) = self.request_id_pattern.captures(text) {
            if let Some(req_id) = caps.get(1) {
                metadata.insert("request_id".to_string(), req_id.as_str().to_string());
            }
        }

        // Extract function name
        if let Some(caps) = self.function_pattern.captures(text) {
            if let Some(func) = caps.get(1) {
                metadata.insert("function".to_string(), func.as_str().to_string());
            }
        }

        // Extract custom fields
        for (name, pattern) in &self.custom_patterns {
            if let Some(caps) = pattern.captures(text) {
                if let Some(value) = caps.get(1) {
                    metadata.insert(name.clone(), value.as_str().to_string());
                }
            }
        }

        metadata
    }

    /// Extract metadata from a stack trace (uses top frame)
    pub fn extract_from_stack_trace(&self, stack_trace: &str) -> HashMap<String, String> {
        let lines: Vec<&str> = stack_trace.lines().collect();

        // Try to find the first frame with file information
        for line in lines.iter().skip(1) {
            // Skip the error message line
            let metadata = self.extract(line);
            if metadata.contains_key("file") {
                return metadata;
            }
        }

        // If no file found, extract from first line
        if !lines.is_empty() {
            return self.extract(lines[0]);
        }

        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_file_and_line() {
        let extractor = MetadataExtractor::new();

        let text = "at com.example.Service.method(Service.java:123)";
        let metadata = extractor.extract(text);

        assert_eq!(metadata.get("file"), Some(&"Service.java".to_string()));
        assert_eq!(metadata.get("line"), Some(&"123".to_string()));
    }

    #[test]
    fn test_extract_python_file() {
        let extractor = MetadataExtractor::new();

        let text = r#"File "script.py", line 45, in main"#;
        let metadata = extractor.extract(text);

        assert_eq!(metadata.get("file"), Some(&"script.py".to_string()));
        assert_eq!(metadata.get("line"), Some(&"45".to_string()));
    }

    #[test]
    fn test_extract_thread_bracket_notation() {
        let extractor = MetadataExtractor::new();

        let text = "[thread-1] ERROR Something went wrong";
        let metadata = extractor.extract(text);

        assert_eq!(metadata.get("thread"), Some(&"thread-1".to_string()));
    }

    #[test]
    fn test_extract_thread_key_value() {
        let extractor = MetadataExtractor::new();

        let text = "thread_id=worker-123 ERROR Connection failed";
        let metadata = extractor.extract(text);

        assert_eq!(metadata.get("thread"), Some(&"worker-123".to_string()));
    }

    #[test]
    fn test_extract_request_id() {
        let extractor = MetadataExtractor::new();

        let text = "request_id=abc-123-def ERROR Request failed";
        let metadata = extractor.extract(text);

        assert_eq!(metadata.get("request_id"), Some(&"abc-123-def".to_string()));
    }

    #[test]
    fn test_extract_trace_id() {
        let extractor = MetadataExtractor::new();

        let text = "trace-id: xyz-789 ERROR Timeout";
        let metadata = extractor.extract(text);

        assert_eq!(metadata.get("request_id"), Some(&"xyz-789".to_string()));
    }

    #[test]
    fn test_extract_function_name() {
        let extractor = MetadataExtractor::new();

        let text = "at com.example.Service.processRequest(Service.java:123)";
        let metadata = extractor.extract(text);

        assert_eq!(
            metadata.get("function"),
            Some(&"com.example.Service.processRequest".to_string())
        );
    }

    #[test]
    fn test_extract_multiple_fields() {
        let extractor = MetadataExtractor::new();

        let text = "[thread-1] request_id=abc-123 at Service.method(Service.java:45)";
        let metadata = extractor.extract(text);

        assert_eq!(metadata.get("thread"), Some(&"thread-1".to_string()));
        assert_eq!(metadata.get("request_id"), Some(&"abc-123".to_string()));
        assert_eq!(metadata.get("file"), Some(&"Service.java".to_string()));
        assert_eq!(metadata.get("line"), Some(&"45".to_string()));
        assert_eq!(
            metadata.get("function"),
            Some(&"Service.method".to_string())
        );
    }

    #[test]
    fn test_extract_custom_pattern() {
        let mut patterns = HashMap::new();
        patterns.insert(
            "session_id".to_string(),
            r"session[_-]?id[=:]?\s*([a-zA-Z0-9-]+)".to_string(),
        );

        let extractor = MetadataExtractor::with_custom_patterns(patterns);

        let text = "session_id=sess-456 ERROR Login failed";
        let metadata = extractor.extract(text);

        assert_eq!(metadata.get("session_id"), Some(&"sess-456".to_string()));
    }

    #[test]
    fn test_extract_from_stack_trace() {
        let extractor = MetadataExtractor::new();

        let stack_trace = r#"java.lang.NullPointerException: Cannot invoke method
    at com.example.Service.process(Service.java:123)
    at com.example.Controller.handle(Controller.java:45)
    at com.example.Main.main(Main.java:10)"#;

        let metadata = extractor.extract_from_stack_trace(stack_trace);

        // Should extract from the first frame with file info
        assert_eq!(metadata.get("file"), Some(&"Service.java".to_string()));
        assert_eq!(metadata.get("line"), Some(&"123".to_string()));
    }

    #[test]
    fn test_extract_from_empty_text() {
        let extractor = MetadataExtractor::new();

        let metadata = extractor.extract("");
        assert!(metadata.is_empty());
    }

    #[test]
    fn test_extract_no_matches() {
        let extractor = MetadataExtractor::new();

        let text = "Simple log message with no metadata";
        let metadata = extractor.extract(text);

        assert!(metadata.is_empty());
    }
}

use super::LogParser;
use crate::types::LogEntry;
use crate::Result;
use regex::Regex;
use std::sync::Arc;

/// Wrapper parser that handles multi-line stack traces
pub struct StackTraceParser {
    inner: Arc<dyn LogParser>,
    java_pattern: Regex,
    python_pattern: Regex,
    javascript_pattern: Regex,
    csharp_pattern: Regex,
    go_pattern: Regex,
    error_start_pattern: Regex,
}

impl StackTraceParser {
    /// Create a new stack trace parser wrapping another parser
    pub fn new(inner: Arc<dyn LogParser>) -> Self {
        Self {
            inner,
            // Java: "at com.example.Class.method(File.java:123)"
            java_pattern: Regex::new(r"^\s*at\s+[\w.$]+\(").unwrap(),
            // Python: "  File "script.py", line 123, in function"
            python_pattern: Regex::new(r#"^\s+File\s+"[^"]+",\s+line\s+\d+"#).unwrap(),
            // JavaScript: "    at Object.<anonymous> (/path/file.js:123:45)"
            javascript_pattern: Regex::new(r"^\s+at\s+.*\.js:\d+").unwrap(),
            // C#: "   at Namespace.Class.Method() in File.cs:line 123"
            csharp_pattern: Regex::new(r"^\s+at\s+.*\sin\s+.*\.cs:line\s+\d+").unwrap(),
            // Go: "goroutine 1 [running]:" or "\tmain.function()"
            go_pattern: Regex::new(r"^(?:goroutine\s+\d+|\t)").unwrap(),
            // Error start patterns: "Exception", "Error", "Traceback"
            error_start_pattern: Regex::new(
                r"(?i)(?:Exception|Error|Traceback|Panic|Fatal|Caused by:)",
            )
            .unwrap(),
        }
    }

    /// Check if a line is a stack trace continuation
    fn is_stack_trace_line(&self, line: &str) -> bool {
        self.java_pattern.is_match(line)
            || self.python_pattern.is_match(line)
            || self.javascript_pattern.is_match(line)
            || self.csharp_pattern.is_match(line)
            || self.go_pattern.is_match(line)
            || line.trim_start().starts_with("at ")
            || (line.starts_with('\t') || line.starts_with("  ")) && !line.trim().is_empty()
    }

    /// Check if a line starts a new error/exception
    fn is_error_start(&self, line: &str) -> bool {
        self.error_start_pattern.is_match(line)
    }
}

impl LogParser for StackTraceParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        self.inner.parse_line(line)
    }

    fn parse_lines(&self, lines: &[String]) -> Result<Vec<LogEntry>> {
        let mut entries = Vec::new();
        let mut buffer = Vec::new();
        let mut in_stack_trace = false;

        for line in lines {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                if !buffer.is_empty() && in_stack_trace {
                    buffer.push(line.clone());
                }
                continue;
            }

            // Check if this is a stack trace continuation
            if in_stack_trace && self.is_stack_trace_line(line) {
                buffer.push(line.clone());
                continue;
            }

            // If we were in a stack trace and this isn't a continuation, flush the buffer
            if in_stack_trace && !buffer.is_empty() {
                let combined = buffer.join("\n");
                if let Some(entry) = self.inner.parse_line(&combined)? {
                    entries.push(entry);
                }
                buffer.clear();
                in_stack_trace = false;
            }

            // Check if this line starts a new error/stack trace
            if self.is_error_start(line) {
                in_stack_trace = true;
                buffer.push(line.clone());
                continue;
            }

            // Regular line - try to parse it
            if let Some(entry) = self.inner.parse_line(line)? {
                entries.push(entry);
            }
        }

        // Flush any remaining buffered stack trace
        if !buffer.is_empty() {
            let combined = buffer.join("\n");
            if let Some(entry) = self.inner.parse_line(&combined)? {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    fn can_parse(&self, sample: &str) -> bool {
        self.inner.can_parse(sample)
    }

    fn supports_multiline(&self) -> bool {
        true
    }

    fn is_continuation_line(&self, line: &str) -> bool {
        self.is_stack_trace_line(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::formats::PlainTextParser;

    #[test]
    fn test_java_stack_trace() {
        let parser = StackTraceParser::new(Arc::new(PlainTextParser::new()));

        let lines = vec![
            "java.lang.NullPointerException: Cannot invoke method".to_string(),
            "    at com.example.Service.process(Service.java:123)".to_string(),
            "    at com.example.Controller.handle(Controller.java:45)".to_string(),
            "    at com.example.Main.main(Main.java:10)".to_string(),
        ];

        let result = parser.parse_lines(&lines).unwrap();
        assert_eq!(result.len(), 1);

        let entry = &result[0];
        assert!(entry.message.contains("NullPointerException"));
        assert!(entry.message.contains("Service.java:123"));
    }

    #[test]
    fn test_python_traceback() {
        let parser = StackTraceParser::new(Arc::new(PlainTextParser::new()));

        let lines = vec![
            "Traceback (most recent call last):".to_string(),
            r#"  File "script.py", line 45, in main"#.to_string(),
            r#"  File "helper.py", line 12, in process"#.to_string(),
        ];

        let result = parser.parse_lines(&lines).unwrap();
        assert_eq!(result.len(), 1);

        let entry = &result[0];
        assert!(entry.message.contains("Traceback"));
        assert!(entry.message.contains("script.py"));
    }

    #[test]
    fn test_javascript_stack() {
        let parser = StackTraceParser::new(Arc::new(PlainTextParser::new()));

        let lines = vec![
            "Error: Connection failed".to_string(),
            "    at Object.<anonymous> (/app/server.js:123:45)".to_string(),
            "    at Module._compile (internal/modules/cjs/loader.js:999:30)".to_string(),
        ];

        let result = parser.parse_lines(&lines).unwrap();
        assert_eq!(result.len(), 1);

        let entry = &result[0];
        assert!(entry.message.contains("Error: Connection failed"));
        assert!(entry.message.contains("server.js:123"));
    }

    #[test]
    fn test_mixed_logs_and_stack_traces() {
        let parser = StackTraceParser::new(Arc::new(PlainTextParser::new()));

        let lines = vec![
            "INFO Starting application".to_string(),
            "ERROR Exception occurred:".to_string(),
            "    at com.example.Service.method(Service.java:100)".to_string(),
            "INFO Application recovered".to_string(),
        ];

        let result = parser.parse_lines(&lines).unwrap();
        assert_eq!(result.len(), 3);

        assert!(result[0].message.contains("Starting application"));
        assert!(result[1].message.contains("Exception occurred"));
        assert!(result[1].message.contains("Service.java:100"));
        assert!(result[2].message.contains("Application recovered"));
    }

    #[test]
    fn test_is_stack_trace_line() {
        let parser = StackTraceParser::new(Arc::new(PlainTextParser::new()));

        // Java
        assert!(parser.is_stack_trace_line("    at com.example.Class.method(File.java:123)"));

        // Python
        assert!(parser.is_stack_trace_line(r#"  File "script.py", line 45, in main"#));

        // JavaScript
        assert!(parser.is_stack_trace_line("    at Object.<anonymous> (/app/file.js:123:45)"));

        // C#
        assert!(parser.is_stack_trace_line("   at Namespace.Class.Method() in File.cs:line 123"));

        // Go
        assert!(parser.is_stack_trace_line("goroutine 1 [running]:"));
        assert!(parser.is_stack_trace_line("\tmain.function()"));

        // Not stack trace
        assert!(!parser.is_stack_trace_line("Regular log message"));
    }

    #[test]
    fn test_is_error_start() {
        let parser = StackTraceParser::new(Arc::new(PlainTextParser::new()));

        assert!(parser.is_error_start("java.lang.NullPointerException: message"));
        assert!(parser.is_error_start("Error: Connection failed"));
        assert!(parser.is_error_start("Traceback (most recent call last):"));
        assert!(parser.is_error_start("Caused by: java.io.IOException"));
        assert!(parser.is_error_start("FATAL: System panic"));

        assert!(!parser.is_error_start("Regular log message"));
        assert!(!parser.is_error_start("INFO Starting application"));
    }

    #[test]
    fn test_empty_lines_in_stack_trace() {
        let parser = StackTraceParser::new(Arc::new(PlainTextParser::new()));

        let lines = vec![
            "Error: Test error".to_string(),
            "    at function1()".to_string(),
            "".to_string(),
            "    at function2()".to_string(),
        ];

        let result = parser.parse_lines(&lines).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_supports_multiline() {
        let parser = StackTraceParser::new(Arc::new(PlainTextParser::new()));
        assert!(parser.supports_multiline());
    }
}

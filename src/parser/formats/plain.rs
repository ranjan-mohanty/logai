use crate::parser::LogParser;
use crate::types::{LogEntry, LogMetadata, Severity};
use crate::Result;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::collections::HashMap;

pub struct PlainTextParser {
    timestamp_regex: Regex,
    severity_regex: Regex,
}

impl Default for PlainTextParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PlainTextParser {
    pub fn new() -> Self {
        Self {
            // Match common timestamp formats
            timestamp_regex: Regex::new(
                r"(\d{4}-\d{2}-\d{2}[T\s]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?)",
            )
            .unwrap(),
            // Match severity levels
            severity_regex: Regex::new(
                r"(?i)\b(ERROR|ERR|FATAL|CRITICAL|WARN|WARNING|INFO|DEBUG|TRACE)\b",
            )
            .unwrap(),
        }
    }

    fn extract_timestamp(&self, line: &str) -> Option<DateTime<Utc>> {
        self.timestamp_regex
            .captures(line)
            .and_then(|cap| cap.get(1))
            .and_then(|m| {
                let ts_str = m.as_str();
                DateTime::parse_from_rfc3339(ts_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            })
    }

    fn extract_severity(&self, line: &str) -> Severity {
        self.severity_regex
            .captures(line)
            .and_then(|cap| cap.get(1))
            .map(|m| {
                let s = m.as_str().to_lowercase();
                match s.as_str() {
                    "error" | "err" | "fatal" | "critical" => Severity::Error,
                    "warn" | "warning" => Severity::Warning,
                    "info" => Severity::Info,
                    "debug" => Severity::Debug,
                    "trace" => Severity::Trace,
                    _ => Severity::Unknown,
                }
            })
            .unwrap_or(Severity::Unknown)
    }
}

impl LogParser for PlainTextParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        let timestamp = self.extract_timestamp(line);
        let severity = self.extract_severity(line);

        // For plain text, the entire line is the message
        let message = line.to_string();

        let metadata = LogMetadata {
            file: None,
            line: None,
            function: None,
            thread: None,
            extra: HashMap::new(),
        };

        Ok(Some(LogEntry {
            timestamp,
            severity,
            message,
            metadata,
            raw: line.to_string(),
        }))
    }

    fn can_parse(&self, _sample: &str) -> bool {
        // Plain text parser is the fallback, always returns true
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plain_log() {
        let parser = PlainTextParser::new();
        let line = "2025-11-17T10:30:00Z ERROR Connection failed to database";

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Error);
        assert!(entry.timestamp.is_some());
    }
}

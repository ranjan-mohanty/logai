use super::LogParser;
use crate::types::{LogEntry, LogMetadata, Severity};
use crate::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

pub struct JsonParser;

impl Default for JsonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_severity(value: &Value) -> Severity {
        let s = value.as_str().unwrap_or("").to_lowercase();
        match s.as_str() {
            "error" | "err" | "fatal" | "critical" => Severity::Error,
            "warn" | "warning" => Severity::Warning,
            "info" | "information" => Severity::Info,
            "debug" => Severity::Debug,
            "trace" => Severity::Trace,
            _ => Severity::Unknown,
        }
    }

    fn parse_timestamp(value: &Value) -> Option<DateTime<Utc>> {
        if let Some(ts_str) = value.as_str() {
            DateTime::parse_from_rfc3339(ts_str)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .or_else(|| {
                    DateTime::parse_from_rfc2822(ts_str)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                })
        } else if let Some(ts_num) = value.as_i64() {
            DateTime::from_timestamp(ts_num, 0)
        } else {
            None
        }
    }
}

impl LogParser for JsonParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        let json: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        let obj = match json.as_object() {
            Some(o) => o,
            None => return Ok(None),
        };

        // Extract common fields
        let message = obj
            .get("message")
            .or_else(|| obj.get("msg"))
            .or_else(|| obj.get("text"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let severity = obj
            .get("level")
            .or_else(|| obj.get("severity"))
            .or_else(|| obj.get("priority"))
            .map(Self::parse_severity)
            .unwrap_or(Severity::Unknown);

        let timestamp = obj
            .get("timestamp")
            .or_else(|| obj.get("time"))
            .or_else(|| obj.get("@timestamp"))
            .and_then(Self::parse_timestamp);

        // Extract metadata
        let mut extra = HashMap::new();
        for (key, value) in obj.iter() {
            if !matches!(
                key.as_str(),
                "message" | "msg" | "text" | "level" | "severity" | "timestamp" | "time"
            ) {
                if let Some(s) = value.as_str() {
                    extra.insert(key.clone(), s.to_string());
                }
            }
        }

        let metadata = LogMetadata {
            file: obj.get("file").and_then(|v| v.as_str()).map(String::from),
            line: obj.get("line").and_then(|v| v.as_u64()).map(|n| n as u32),
            function: obj
                .get("function")
                .or_else(|| obj.get("func"))
                .and_then(|v| v.as_str())
                .map(String::from),
            thread: obj
                .get("thread")
                .or_else(|| obj.get("thread_id"))
                .and_then(|v| v.as_str())
                .map(String::from),
            extra,
        };

        Ok(Some(LogEntry {
            timestamp,
            severity,
            message,
            metadata,
            raw: line.to_string(),
        }))
    }

    fn can_parse(&self, sample: &str) -> bool {
        sample.trim_start().starts_with('{') && serde_json::from_str::<Value>(sample).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_log() {
        let parser = JsonParser::new();
        let line =
            r#"{"level":"error","message":"Connection failed","timestamp":"2025-11-17T10:30:00Z"}"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.message, "Connection failed");
        assert_eq!(entry.severity, Severity::Error);
    }
}

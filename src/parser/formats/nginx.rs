use crate::parser::LogParser;
use crate::types::{LogEntry, LogMetadata, Severity};
use crate::Result;
use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;
use std::collections::HashMap;

/// Parser for Nginx HTTP server logs
pub struct NginxParser {
    access_pattern: Regex,
    error_pattern: Regex,
}

impl Default for NginxParser {
    fn default() -> Self {
        Self::new()
    }
}

impl NginxParser {
    /// Create a new Nginx log parser
    pub fn new() -> Self {
        // Access log format: IP - - [timestamp] "request" status size response_time
        let access_pattern =
            Regex::new(r#"^(\S+) - - \[([^\]]+)\] "([^"]*)" (\d{3}) (\d+) ([\d.]+)"#).unwrap();

        // Error log format: timestamp [level] pid#tid: *cid message
        let error_pattern =
            Regex::new(r"^(\d{4}/\d{2}/\d{2} \d{2}:\d{2}:\d{2}) \[(\w+)\] (\d+)#(\d+): (.+)")
                .unwrap();

        Self {
            access_pattern,
            error_pattern,
        }
    }

    /// Parse Nginx timestamp format: 17/Nov/2025:10:30:00 +0000
    fn parse_nginx_timestamp(&self, timestamp_str: &str) -> Option<DateTime<Utc>> {
        // Nginx access log format: 17/Nov/2025:10:30:00 +0000
        let parts: Vec<&str> = timestamp_str.split(':').collect();
        if parts.len() < 2 {
            return None;
        }

        let date_part = parts[0];
        let time_part = parts[1..].join(":");

        // Parse date: 17/Nov/2025
        let date_parts: Vec<&str> = date_part.split('/').collect();
        if date_parts.len() != 3 {
            return None;
        }

        let day = date_parts[0];
        let month = date_parts[1];
        let year = date_parts[2];

        // Construct RFC 2822 format: 17 Nov 2025 10:30:00 +0000
        let rfc2822 = format!("{} {} {} {}", day, month, year, time_part);

        DateTime::parse_from_str(&rfc2822, "%d %b %Y %H:%M:%S %z")
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }

    /// Parse Nginx error log timestamp: 2025/11/17 10:30:00
    fn parse_nginx_error_timestamp(&self, timestamp_str: &str) -> Option<DateTime<Utc>> {
        use chrono::NaiveDateTime;

        NaiveDateTime::parse_from_str(timestamp_str, "%Y/%m/%d %H:%M:%S")
            .ok()
            .map(|dt| Utc.from_utc_datetime(&dt))
    }

    /// Map HTTP status code to severity
    fn status_to_severity(status: u16) -> Severity {
        match status {
            500..=599 => Severity::Error,
            400..=499 => Severity::Warning,
            _ => Severity::Info,
        }
    }

    /// Map Nginx error level to severity
    fn error_level_to_severity(level: &str) -> Severity {
        match level.to_lowercase().as_str() {
            "emerg" | "alert" | "crit" | "error" => Severity::Error,
            "warn" => Severity::Warning,
            "notice" | "info" => Severity::Info,
            "debug" => Severity::Debug,
            _ => Severity::Unknown,
        }
    }
}

impl LogParser for NginxParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        // Try access log format first
        if let Some(caps) = self.access_pattern.captures(line) {
            let client_ip = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let timestamp_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let request = caps.get(3).map(|m| m.as_str()).unwrap_or("");
            let status_str = caps.get(4).map(|m| m.as_str()).unwrap_or("0");
            let size_str = caps.get(5).map(|m| m.as_str()).unwrap_or("0");
            let response_time_str = caps.get(6).map(|m| m.as_str()).unwrap_or("0");

            let status: u16 = status_str.parse().unwrap_or(0);
            let severity = Self::status_to_severity(status);
            let timestamp = self.parse_nginx_timestamp(timestamp_str);

            let mut extra = HashMap::new();
            extra.insert("client_ip".to_string(), client_ip.to_string());
            extra.insert("status".to_string(), status.to_string());
            extra.insert("response_size".to_string(), size_str.to_string());
            extra.insert("response_time".to_string(), response_time_str.to_string());

            let metadata = LogMetadata {
                file: None,
                line: None,
                function: None,
                thread: None,
                extra,
            };

            return Ok(Some(LogEntry {
                timestamp,
                severity,
                message: format!("{} {}", status, request),
                metadata,
                raw: line.to_string(),
            }));
        }

        // Try error log format
        if let Some(caps) = self.error_pattern.captures(line) {
            let timestamp_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let level = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let pid = caps.get(3).map(|m| m.as_str()).unwrap_or("");
            let tid = caps.get(4).map(|m| m.as_str()).unwrap_or("");
            let message = caps.get(5).map(|m| m.as_str()).unwrap_or("");

            let severity = Self::error_level_to_severity(level);
            let timestamp = self.parse_nginx_error_timestamp(timestamp_str);

            let mut extra = HashMap::new();
            extra.insert("pid".to_string(), pid.to_string());
            extra.insert("tid".to_string(), tid.to_string());
            extra.insert("level".to_string(), level.to_string());

            let metadata = LogMetadata {
                file: None,
                line: None,
                function: None,
                thread: None,
                extra,
            };

            return Ok(Some(LogEntry {
                timestamp,
                severity,
                message: message.to_string(),
                metadata,
                raw: line.to_string(),
            }));
        }

        // Not an Nginx log format
        Ok(None)
    }

    fn can_parse(&self, sample: &str) -> bool {
        self.access_pattern.is_match(sample) || self.error_pattern.is_match(sample)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_access_log() {
        let parser = NginxParser::new();
        let line = r#"192.168.1.1 - - [17/Nov/2025:10:30:00 +0000] "GET /api/users HTTP/1.1" 200 1234 0.123"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Info);
        assert!(entry.message.contains("200"));
        assert!(entry.message.contains("GET /api/users"));

        let metadata = &entry.metadata.extra;
        assert_eq!(metadata.get("client_ip"), Some(&"192.168.1.1".to_string()));
        assert_eq!(metadata.get("status"), Some(&"200".to_string()));
        assert_eq!(metadata.get("response_size"), Some(&"1234".to_string()));
        assert_eq!(metadata.get("response_time"), Some(&"0.123".to_string()));
    }

    #[test]
    fn test_parse_access_log_4xx() {
        let parser = NginxParser::new();
        let line =
            r#"192.168.1.1 - - [17/Nov/2025:10:30:00 +0000] "GET /missing HTTP/1.1" 404 0 0.001"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Warning);
        assert!(entry.message.contains("404"));
    }

    #[test]
    fn test_parse_access_log_5xx() {
        let parser = NginxParser::new();
        let line = r#"192.168.1.1 - - [17/Nov/2025:10:30:00 +0000] "POST /api/data HTTP/1.1" 500 1234 0.500"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Error);
        assert!(entry.message.contains("500"));
    }

    #[test]
    fn test_parse_error_log() {
        let parser = NginxParser::new();
        let line =
            r#"2025/11/17 10:30:00 [error] 12345#0: *1 connect() failed (111: Connection refused)"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Error);
        assert!(entry
            .message
            .contains("connect() failed (111: Connection refused)"));

        let metadata = &entry.metadata.extra;
        assert_eq!(metadata.get("pid"), Some(&"12345".to_string()));
        assert_eq!(metadata.get("tid"), Some(&"0".to_string()));
        assert_eq!(metadata.get("level"), Some(&"error".to_string()));
    }

    #[test]
    fn test_parse_error_log_warn() {
        let parser = NginxParser::new();
        let line = r#"2025/11/17 10:30:00 [warn] 12345#0: *1 upstream server temporarily disabled"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Warning);
    }

    #[test]
    fn test_parse_error_log_crit() {
        let parser = NginxParser::new();
        let line = r#"2025/11/17 10:30:00 [crit] 12345#0: *1 SSL handshake failed"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Error);
    }

    #[test]
    fn test_can_parse_nginx_format() {
        let parser = NginxParser::new();

        let access_line =
            r#"192.168.1.1 - - [17/Nov/2025:10:30:00 +0000] "GET /test HTTP/1.1" 200 123 0.001"#;
        assert!(parser.can_parse(access_line));

        let error_line = r#"2025/11/17 10:30:00 [error] 12345#0: *1 test error"#;
        assert!(parser.can_parse(error_line));

        let not_nginx = "This is not an Nginx log";
        assert!(!parser.can_parse(not_nginx));
    }

    #[test]
    fn test_parse_empty_line() {
        let parser = NginxParser::new();
        let result = parser.parse_line("").unwrap();
        assert!(result.is_none());
    }
}

use super::LogParser;
use crate::types::{LogEntry, LogMetadata, Severity};
use crate::Result;
use chrono::{DateTime, Utc};
use regex::Regex;
use std::collections::HashMap;

/// Parser for Apache HTTP server logs
pub struct ApacheParser {
    common_pattern: Regex,
    combined_pattern: Regex,
}

impl Default for ApacheParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ApacheParser {
    /// Create a new Apache log parser
    pub fn new() -> Self {
        // Common Log Format: IP - user [timestamp] "request" status size
        let common_pattern =
            Regex::new(r#"^(\S+) \S+ (\S+) \[([^\]]+)\] "([^"]*)" (\d{3}) (\d+|-)"#).unwrap();

        // Combined Log Format: Common + "referrer" "user-agent"
        let combined_pattern = Regex::new(
            r#"^(\S+) \S+ (\S+) \[([^\]]+)\] "([^"]*)" (\d{3}) (\d+|-) "([^"]*)" "([^"]*)""#,
        )
        .unwrap();

        Self {
            common_pattern,
            combined_pattern,
        }
    }

    /// Parse Apache timestamp format: 10/Oct/2000:13:55:36 -0700
    fn parse_apache_timestamp(&self, timestamp_str: &str) -> Option<DateTime<Utc>> {
        // Apache format: 10/Oct/2000:13:55:36 -0700
        // Convert to RFC 2822 format for parsing
        let parts: Vec<&str> = timestamp_str.split(':').collect();
        if parts.len() < 2 {
            return None;
        }

        let date_part = parts[0];
        let time_part = parts[1..].join(":");

        // Parse date: 10/Oct/2000
        let date_parts: Vec<&str> = date_part.split('/').collect();
        if date_parts.len() != 3 {
            return None;
        }

        let day = date_parts[0];
        let month = date_parts[1];
        let year = date_parts[2];

        // Construct RFC 2822 format: Mon, 10 Oct 2000 13:55:36 -0700
        let rfc2822 = format!("{} {} {} {}", day, month, year, time_part);

        DateTime::parse_from_str(&rfc2822, "%d %b %Y %H:%M:%S %z")
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }

    /// Map HTTP status code to severity
    fn status_to_severity(status: u16) -> Severity {
        match status {
            500..=599 => Severity::Error,
            400..=499 => Severity::Warning,
            _ => Severity::Info,
        }
    }
}

impl LogParser for ApacheParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        // Try combined format first (more specific)
        if let Some(caps) = self.combined_pattern.captures(line) {
            let client_ip = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let user = caps.get(2).map(|m| m.as_str()).unwrap_or("-");
            let timestamp_str = caps.get(3).map(|m| m.as_str()).unwrap_or("");
            let request = caps.get(4).map(|m| m.as_str()).unwrap_or("");
            let status_str = caps.get(5).map(|m| m.as_str()).unwrap_or("0");
            let size_str = caps.get(6).map(|m| m.as_str()).unwrap_or("-");
            let referrer = caps.get(7).map(|m| m.as_str()).unwrap_or("-");
            let user_agent = caps.get(8).map(|m| m.as_str()).unwrap_or("-");

            let status: u16 = status_str.parse().unwrap_or(0);
            let severity = Self::status_to_severity(status);
            let timestamp = self.parse_apache_timestamp(timestamp_str);

            let mut extra = HashMap::new();
            extra.insert("client_ip".to_string(), client_ip.to_string());
            if user != "-" {
                extra.insert("user".to_string(), user.to_string());
            }
            extra.insert("status".to_string(), status.to_string());
            if size_str != "-" {
                extra.insert("response_size".to_string(), size_str.to_string());
            }
            if referrer != "-" {
                extra.insert("referrer".to_string(), referrer.to_string());
            }
            if user_agent != "-" {
                extra.insert("user_agent".to_string(), user_agent.to_string());
            }

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

        // Try common format
        if let Some(caps) = self.common_pattern.captures(line) {
            let client_ip = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let user = caps.get(2).map(|m| m.as_str()).unwrap_or("-");
            let timestamp_str = caps.get(3).map(|m| m.as_str()).unwrap_or("");
            let request = caps.get(4).map(|m| m.as_str()).unwrap_or("");
            let status_str = caps.get(5).map(|m| m.as_str()).unwrap_or("0");
            let size_str = caps.get(6).map(|m| m.as_str()).unwrap_or("-");

            let status: u16 = status_str.parse().unwrap_or(0);
            let severity = Self::status_to_severity(status);
            let timestamp = self.parse_apache_timestamp(timestamp_str);

            let mut extra = HashMap::new();
            extra.insert("client_ip".to_string(), client_ip.to_string());
            if user != "-" {
                extra.insert("user".to_string(), user.to_string());
            }
            extra.insert("status".to_string(), status.to_string());
            if size_str != "-" {
                extra.insert("response_size".to_string(), size_str.to_string());
            }

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

        // Not an Apache log format
        Ok(None)
    }

    fn can_parse(&self, sample: &str) -> bool {
        self.common_pattern.is_match(sample) || self.combined_pattern.is_match(sample)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_common_log_format() {
        let parser = ApacheParser::new();
        let line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Info);
        assert!(entry.message.contains("200"));
        assert!(entry.message.contains("GET /apache_pb.gif"));

        let metadata = &entry.metadata.extra;
        assert_eq!(metadata.get("client_ip"), Some(&"127.0.0.1".to_string()));
        assert_eq!(metadata.get("user"), Some(&"frank".to_string()));
        assert_eq!(metadata.get("status"), Some(&"200".to_string()));
        assert_eq!(metadata.get("response_size"), Some(&"2326".to_string()));
    }

    #[test]
    fn test_parse_combined_log_format() {
        let parser = ApacheParser::new();
        let line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08""#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Info);

        let metadata = &entry.metadata.extra;
        assert_eq!(metadata.get("client_ip"), Some(&"127.0.0.1".to_string()));
        assert_eq!(
            metadata.get("referrer"),
            Some(&"http://www.example.com/start.html".to_string())
        );
        assert_eq!(
            metadata.get("user_agent"),
            Some(&"Mozilla/4.08".to_string())
        );
    }

    #[test]
    fn test_parse_4xx_status() {
        let parser = ApacheParser::new();
        let line =
            r#"192.168.1.1 - - [10/Oct/2000:13:55:36 -0700] "GET /missing.html HTTP/1.0" 404 0"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Warning);
        assert!(entry.message.contains("404"));
    }

    #[test]
    fn test_parse_5xx_status() {
        let parser = ApacheParser::new();
        let line =
            r#"192.168.1.1 - - [10/Oct/2000:13:55:36 -0700] "GET /error.php HTTP/1.0" 500 1234"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Error);
        assert!(entry.message.contains("500"));
    }

    #[test]
    fn test_can_parse_apache_format() {
        let parser = ApacheParser::new();

        let common_line =
            r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /test HTTP/1.0" 200 123"#;
        assert!(parser.can_parse(common_line));

        let combined_line = r#"127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /test HTTP/1.0" 200 123 "http://example.com" "Mozilla/5.0""#;
        assert!(parser.can_parse(combined_line));

        let not_apache = "This is not an Apache log";
        assert!(!parser.can_parse(not_apache));
    }

    #[test]
    fn test_parse_empty_line() {
        let parser = ApacheParser::new();
        let result = parser.parse_line("").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_with_dash_user() {
        let parser = ApacheParser::new();
        let line =
            r#"192.168.1.1 - - [10/Oct/2000:13:55:36 -0700] "GET /index.html HTTP/1.0" 200 1234"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        let metadata = &entry.metadata.extra;
        // User should not be in metadata when it's "-"
        assert!(!metadata.contains_key("user"));
    }

    #[test]
    fn test_parse_with_dash_size() {
        let parser = ApacheParser::new();
        let line =
            r#"192.168.1.1 - - [10/Oct/2000:13:55:36 -0700] "GET /index.html HTTP/1.0" 304 -"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        let metadata = &entry.metadata.extra;
        // Response size should not be in metadata when it's "-"
        assert!(!metadata.contains_key("response_size"));
    }
}

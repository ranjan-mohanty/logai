use super::LogParser;
use crate::types::{LogEntry, LogMetadata, Severity};
use crate::Result;
use chrono::{DateTime, Datelike, TimeZone, Utc};
use regex::Regex;
use std::collections::HashMap;

/// Parser for Syslog format logs (RFC 3164 and RFC 5424)
pub struct SyslogParser {
    rfc3164_pattern: Regex,
    rfc5424_pattern: Regex,
}

impl Default for SyslogParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SyslogParser {
    /// Create a new Syslog parser
    pub fn new() -> Self {
        // RFC 3164 (BSD Syslog): <priority>timestamp hostname tag: message
        let rfc3164_pattern =
            Regex::new(r"^<(\d+)>(\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+(\S+)\s+([^:]+):\s*(.+)")
                .unwrap();

        // RFC 5424: <priority>version timestamp hostname app-name procid msgid [structured-data] message
        let rfc5424_pattern = Regex::new(
            r"^<(\d+)>(\d+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(?:\[([^\]]+)\]\s+)?(.*)$",
        )
        .unwrap();

        Self {
            rfc3164_pattern,
            rfc5424_pattern,
        }
    }

    /// Calculate severity from syslog priority
    /// Priority = Facility * 8 + Severity
    fn priority_to_severity(priority: u8) -> Severity {
        let sev = priority & 0x07; // Last 3 bits
        match sev {
            0..=3 => Severity::Error, // Emergency, Alert, Critical, Error
            4 => Severity::Warning,   // Warning
            5 | 6 => Severity::Info,  // Notice, Informational
            7 => Severity::Debug,     // Debug
            _ => Severity::Unknown,
        }
    }

    /// Parse RFC 3164 timestamp: Oct 11 22:14:15
    fn parse_rfc3164_timestamp(&self, timestamp_str: &str) -> Option<DateTime<Utc>> {
        use chrono::NaiveDateTime;

        // RFC 3164 doesn't include year, use current year
        let current_year = Utc::now().year();
        let with_year = format!("{} {}", timestamp_str, current_year);

        NaiveDateTime::parse_from_str(&with_year, "%b %d %H:%M:%S %Y")
            .ok()
            .map(|dt| Utc.from_utc_datetime(&dt))
    }

    /// Parse RFC 5424 timestamp: 2025-11-17T10:30:00.123Z
    fn parse_rfc5424_timestamp(&self, timestamp_str: &str) -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(timestamp_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
}

impl LogParser for SyslogParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        // Try RFC 5424 format first (more specific)
        if let Some(caps) = self.rfc5424_pattern.captures(line) {
            let priority_str = caps.get(1).map(|m| m.as_str()).unwrap_or("0");
            let _version = caps.get(2).map(|m| m.as_str()).unwrap_or("1");
            let timestamp_str = caps.get(3).map(|m| m.as_str()).unwrap_or("");
            let hostname = caps.get(4).map(|m| m.as_str()).unwrap_or("-");
            let app_name = caps.get(5).map(|m| m.as_str()).unwrap_or("-");
            let procid = caps.get(6).map(|m| m.as_str()).unwrap_or("-");
            let _msgid = caps.get(7).map(|m| m.as_str()).unwrap_or("-");
            let _structured_data = caps.get(8).map(|m| m.as_str());
            let message = caps.get(9).map(|m| m.as_str()).unwrap_or("");

            let priority: u8 = priority_str.parse().unwrap_or(0);
            let severity = Self::priority_to_severity(priority);
            let timestamp = self.parse_rfc5424_timestamp(timestamp_str);

            let mut extra = HashMap::new();
            extra.insert("priority".to_string(), priority.to_string());
            if hostname != "-" {
                extra.insert("hostname".to_string(), hostname.to_string());
            }
            if app_name != "-" {
                extra.insert("app_name".to_string(), app_name.to_string());
            }
            if procid != "-" {
                extra.insert("pid".to_string(), procid.to_string());
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
                message: message.to_string(),
                metadata,
                raw: line.to_string(),
            }));
        }

        // Try RFC 3164 format
        if let Some(caps) = self.rfc3164_pattern.captures(line) {
            let priority_str = caps.get(1).map(|m| m.as_str()).unwrap_or("0");
            let timestamp_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let hostname = caps.get(3).map(|m| m.as_str()).unwrap_or("");
            let tag = caps.get(4).map(|m| m.as_str()).unwrap_or("");
            let message = caps.get(5).map(|m| m.as_str()).unwrap_or("");

            let priority: u8 = priority_str.parse().unwrap_or(0);
            let severity = Self::priority_to_severity(priority);
            let timestamp = self.parse_rfc3164_timestamp(timestamp_str);

            // Extract PID from tag if present: su[1234]
            let mut app_name = tag.to_string();
            let mut pid = None;
            if let Some(bracket_pos) = tag.find('[') {
                app_name = tag[..bracket_pos].to_string();
                if let Some(end_bracket) = tag.find(']') {
                    pid = Some(tag[bracket_pos + 1..end_bracket].to_string());
                }
            }

            let mut extra = HashMap::new();
            extra.insert("priority".to_string(), priority.to_string());
            extra.insert("hostname".to_string(), hostname.to_string());
            extra.insert("app_name".to_string(), app_name);
            if let Some(p) = pid {
                extra.insert("pid".to_string(), p);
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
                message: message.to_string(),
                metadata,
                raw: line.to_string(),
            }));
        }

        // Not a Syslog format
        Ok(None)
    }

    fn can_parse(&self, sample: &str) -> bool {
        self.rfc3164_pattern.is_match(sample) || self.rfc5424_pattern.is_match(sample)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rfc3164() {
        let parser = SyslogParser::new();
        let line =
            r#"<34>Oct 11 22:14:15 mymachine su: 'su root' failed for lonvick on /dev/pts/8"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Error);
        assert!(entry
            .message
            .contains("'su root' failed for lonvick on /dev/pts/8"));

        let metadata = &entry.metadata.extra;
        assert_eq!(metadata.get("hostname"), Some(&"mymachine".to_string()));
        assert_eq!(metadata.get("app_name"), Some(&"su".to_string()));
        assert_eq!(metadata.get("priority"), Some(&"34".to_string()));
    }

    #[test]
    fn test_parse_rfc3164_with_pid() {
        let parser = SyslogParser::new();
        let line = r#"<34>Oct 11 22:14:15 mymachine sshd[1234]: Connection from 192.168.1.1"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        let metadata = &entry.metadata.extra;
        assert_eq!(metadata.get("app_name"), Some(&"sshd".to_string()));
        assert_eq!(metadata.get("pid"), Some(&"1234".to_string()));
    }

    #[test]
    fn test_parse_rfc5424() {
        let parser = SyslogParser::new();
        let line = r#"<165>1 2025-11-17T10:30:00.123Z mymachine.example.com evntslog - ID47 [exampleSDID@32473 iut="3"] An application event log entry"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.severity, Severity::Info);
        assert_eq!(entry.message, "An application event log entry");

        let metadata = &entry.metadata.extra;
        assert_eq!(
            metadata.get("hostname"),
            Some(&"mymachine.example.com".to_string())
        );
        assert_eq!(metadata.get("app_name"), Some(&"evntslog".to_string()));
        assert_eq!(metadata.get("priority"), Some(&"165".to_string()));
    }

    #[test]
    fn test_parse_rfc5424_minimal() {
        let parser = SyslogParser::new();
        let line = r#"<34>1 2025-11-17T10:30:00Z hostname app - - Test message"#;

        let result = parser.parse_line(line).unwrap();
        assert!(result.is_some());

        let entry = result.unwrap();
        assert_eq!(entry.message, "Test message");
    }

    #[test]
    fn test_priority_to_severity() {
        // Emergency (0)
        assert_eq!(SyslogParser::priority_to_severity(0), Severity::Error);
        // Alert (1)
        assert_eq!(SyslogParser::priority_to_severity(1), Severity::Error);
        // Critical (2)
        assert_eq!(SyslogParser::priority_to_severity(2), Severity::Error);
        // Error (3)
        assert_eq!(SyslogParser::priority_to_severity(3), Severity::Error);
        // Warning (4)
        assert_eq!(SyslogParser::priority_to_severity(4), Severity::Warning);
        // Notice (5)
        assert_eq!(SyslogParser::priority_to_severity(5), Severity::Info);
        // Informational (6)
        assert_eq!(SyslogParser::priority_to_severity(6), Severity::Info);
        // Debug (7)
        assert_eq!(SyslogParser::priority_to_severity(7), Severity::Debug);

        // Test with facility bits (priority = facility * 8 + severity)
        // Local0 (16) + Error (3) = 131
        assert_eq!(SyslogParser::priority_to_severity(131), Severity::Error);
    }

    #[test]
    fn test_can_parse_syslog_format() {
        let parser = SyslogParser::new();

        let rfc3164_line = r#"<34>Oct 11 22:14:15 mymachine su: test"#;
        assert!(parser.can_parse(rfc3164_line));

        let rfc5424_line = r#"<34>1 2025-11-17T10:30:00Z hostname app - - - test"#;
        assert!(parser.can_parse(rfc5424_line));

        let not_syslog = "This is not a syslog message";
        assert!(!parser.can_parse(not_syslog));
    }

    #[test]
    fn test_parse_empty_line() {
        let parser = SyslogParser::new();
        let result = parser.parse_line("").unwrap();
        assert!(result.is_none());
    }
}

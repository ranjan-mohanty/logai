use crate::Result;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use regex::Regex;

/// Parser for various timestamp formats
pub struct TimestampParser {
    // Pre-compiled patterns for common formats
    unix_epoch_pattern: Regex,
    custom_patterns: Vec<(Regex, String)>,
}

impl Default for TimestampParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TimestampParser {
    /// Create a new timestamp parser with default patterns
    pub fn new() -> Self {
        Self {
            unix_epoch_pattern: Regex::new(r"^\d{10,13}$").unwrap(),
            custom_patterns: vec![],
        }
    }

    /// Create a timestamp parser with custom format patterns
    pub fn with_custom_formats(formats: Vec<(String, String)>) -> Self {
        let custom_patterns = formats
            .into_iter()
            .filter_map(|(pattern, format)| Regex::new(&pattern).ok().map(|regex| (regex, format)))
            .collect();

        Self {
            unix_epoch_pattern: Regex::new(r"^\d{10,13}$").unwrap(),
            custom_patterns,
        }
    }

    /// Parse a timestamp string into a DateTime<Utc>
    pub fn parse(&self, text: &str) -> Result<DateTime<Utc>> {
        let text = text.trim();

        // Try formats in order of specificity
        self.try_iso8601(text)
            .or_else(|_| self.try_rfc3339(text))
            .or_else(|_| self.try_rfc2822(text))
            .or_else(|_| self.try_unix_epoch(text))
            .or_else(|_| self.try_custom_patterns(text))
            .or_else(|_| {
                log::warn!("Failed to parse timestamp: {}", text);
                Ok(Utc::now()) // Fallback to current time
            })
    }

    /// Try parsing as ISO 8601 format
    fn try_iso8601(&self, text: &str) -> Result<DateTime<Utc>> {
        // ISO 8601: 2025-11-17T10:30:00Z or 2025-11-17T10:30:00.123Z
        DateTime::parse_from_rfc3339(text)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| anyhow::anyhow!("Not ISO 8601: {}", e))
    }

    /// Try parsing as RFC 3339 format
    fn try_rfc3339(&self, text: &str) -> Result<DateTime<Utc>> {
        // RFC 3339: 2025-11-17T10:30:00+00:00 or 2025-11-17T10:30:00-07:00
        DateTime::parse_from_rfc3339(text)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| anyhow::anyhow!("Not RFC 3339: {}", e))
    }

    /// Try parsing as RFC 2822 format
    fn try_rfc2822(&self, text: &str) -> Result<DateTime<Utc>> {
        // RFC 2822: Mon, 17 Nov 2025 10:30:00 +0000
        DateTime::parse_from_rfc2822(text)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| anyhow::anyhow!("Not RFC 2822: {}", e))
    }

    /// Try parsing as Unix epoch timestamp
    fn try_unix_epoch(&self, text: &str) -> Result<DateTime<Utc>> {
        if !self.unix_epoch_pattern.is_match(text) {
            return Err(anyhow::anyhow!("Not a Unix timestamp"));
        }

        if let Ok(timestamp) = text.parse::<i64>() {
            // Check if it's milliseconds (13 digits) or seconds (10 digits)
            if timestamp > 1_000_000_000_000 {
                // Milliseconds
                return Utc
                    .timestamp_millis_opt(timestamp)
                    .single()
                    .ok_or_else(|| anyhow::anyhow!("Invalid millisecond timestamp"));
            } else {
                // Seconds
                return Utc
                    .timestamp_opt(timestamp, 0)
                    .single()
                    .ok_or_else(|| anyhow::anyhow!("Invalid second timestamp"));
            }
        }

        Err(anyhow::anyhow!("Failed to parse Unix timestamp"))
    }

    /// Try parsing with custom patterns
    fn try_custom_patterns(&self, text: &str) -> Result<DateTime<Utc>> {
        for (pattern, format) in &self.custom_patterns {
            if pattern.is_match(text) {
                return NaiveDateTime::parse_from_str(text, format)
                    .map(|dt| Utc.from_utc_datetime(&dt))
                    .map_err(|e| anyhow::anyhow!("Custom pattern failed: {}", e));
            }
        }
        Err(anyhow::anyhow!("No custom pattern matched"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_iso8601() {
        let parser = TimestampParser::new();

        // With Z timezone
        let result = parser.parse("2025-11-17T10:30:00Z");
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 11);
        assert_eq!(dt.day(), 17);

        // With milliseconds
        let result = parser.parse("2025-11-17T10:30:00.123Z");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_rfc3339() {
        let parser = TimestampParser::new();

        // With timezone offset
        let result = parser.parse("2025-11-17T10:30:00+00:00");
        assert!(result.is_ok());

        let result = parser.parse("2025-11-17T10:30:00-07:00");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_rfc2822() {
        let parser = TimestampParser::new();

        let result = parser.parse("Mon, 17 Nov 2025 10:30:00 +0000");
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 11);
    }

    #[test]
    fn test_parse_unix_epoch_seconds() {
        let parser = TimestampParser::new();

        // 1700220600 = 2023-11-17 10:30:00 UTC
        let result = parser.parse("1700220600");
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2023);
        assert_eq!(dt.month(), 11);
    }

    #[test]
    fn test_parse_unix_epoch_milliseconds() {
        let parser = TimestampParser::new();

        // 1700220600000 = 2023-11-17 10:30:00 UTC
        let result = parser.parse("1700220600000");
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2023);
        assert_eq!(dt.month(), 11);
    }

    #[test]
    fn test_parse_custom_format() {
        let parser = TimestampParser::with_custom_formats(vec![(
            r"^\w{3} \d{2} \d{2}:\d{2}:\d{2}$".to_string(),
            "%b %d %H:%M:%S".to_string(),
        )]);

        let result = parser.parse("Nov 17 10:30:00");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_fallback_to_current_time() {
        let parser = TimestampParser::new();

        // Invalid timestamp should fallback to current time
        let result = parser.parse("not a timestamp");
        assert!(result.is_ok());

        // Should be close to current time (within 1 second)
        let dt = result.unwrap();
        let now = Utc::now();
        let diff = (now - dt).num_seconds().abs();
        assert!(diff < 2);
    }

    #[test]
    fn test_parse_empty_string() {
        let parser = TimestampParser::new();

        let result = parser.parse("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_whitespace() {
        let parser = TimestampParser::new();

        let result = parser.parse("  2025-11-17T10:30:00Z  ");
        assert!(result.is_ok());
    }
}

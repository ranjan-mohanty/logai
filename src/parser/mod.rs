pub mod detector;
pub mod json;
pub mod plain;

use crate::types::LogEntry;
use crate::Result;

/// Trait for log parsers
pub trait LogParser: Send + Sync {
    /// Parse a single line of log
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>>;

    /// Parse multiple lines (for multi-line errors like stack traces)
    fn parse_lines(&self, lines: &[String]) -> Result<Vec<LogEntry>> {
        let mut entries = Vec::new();
        for line in lines {
            if let Some(entry) = self.parse_line(line)? {
                entries.push(entry);
            }
        }
        Ok(entries)
    }

    /// Check if this parser can handle the given content
    fn can_parse(&self, sample: &str) -> bool;
}

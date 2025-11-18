pub mod config;
pub mod detector;
pub mod formats;
pub mod metadata;
pub mod parallel;
pub mod stack_trace;
pub mod statistics;
pub mod timestamp;

use crate::types::LogEntry;
use crate::Result;

pub use config::ParserConfig;
pub use formats::{ApacheParser, JsonParser, NginxParser, PlainTextParser, SyslogParser};
pub use metadata::MetadataExtractor;
pub use parallel::ParallelParser;
pub use stack_trace::StackTraceParser;
pub use statistics::{ParsingContext, ParsingStatistics};
pub use timestamp::TimestampParser;

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

    /// Check if this parser supports multi-line log entries
    fn supports_multiline(&self) -> bool {
        false
    }

    /// Check if a line is a continuation of a previous entry (e.g., stack trace line)
    fn is_continuation_line(&self, _line: &str) -> bool {
        false
    }
}

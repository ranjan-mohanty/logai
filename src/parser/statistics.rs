use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

/// Statistics collected during log parsing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParsingStatistics {
    /// Total number of lines processed
    pub total_lines: usize,
    /// Number of successfully parsed log entries
    pub parsed_entries: usize,
    /// Number of lines that failed to parse
    pub parse_errors: usize,
    /// Number of multi-line entries (e.g., stack traces)
    pub multiline_entries: usize,
    /// Duration of parsing in milliseconds
    pub duration_ms: u64,
}

impl ParsingStatistics {
    /// Calculate parsing throughput in lines per second
    pub fn throughput(&self) -> f64 {
        if self.duration_ms == 0 {
            return 0.0;
        }
        self.total_lines as f64 / (self.duration_ms as f64 / 1000.0)
    }

    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_lines == 0 {
            return 0.0;
        }
        (self.parsed_entries as f64 / self.total_lines as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throughput_calculation() {
        let stats = ParsingStatistics {
            total_lines: 1000,
            parsed_entries: 950,
            parse_errors: 50,
            multiline_entries: 10,
            duration_ms: 1000,
        };

        assert_eq!(stats.throughput(), 1000.0);
    }

    #[test]
    fn test_success_rate_calculation() {
        let stats = ParsingStatistics {
            total_lines: 1000,
            parsed_entries: 950,
            parse_errors: 50,
            multiline_entries: 10,
            duration_ms: 1000,
        };

        assert_eq!(stats.success_rate(), 95.0);
    }

    #[test]
    fn test_zero_duration() {
        let stats = ParsingStatistics {
            total_lines: 1000,
            parsed_entries: 1000,
            parse_errors: 0,
            multiline_entries: 0,
            duration_ms: 0,
        };

        assert_eq!(stats.throughput(), 0.0);
    }
}

/// Context for tracking parsing statistics during parsing operations
pub struct ParsingContext {
    start_time: Instant,
    total_lines: AtomicUsize,
    parsed_entries: AtomicUsize,
    parse_errors: AtomicUsize,
    multiline_entries: AtomicUsize,
}

impl ParsingContext {
    /// Create a new parsing context
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_lines: AtomicUsize::new(0),
            parsed_entries: AtomicUsize::new(0),
            parse_errors: AtomicUsize::new(0),
            multiline_entries: AtomicUsize::new(0),
        }
    }

    /// Record that a line was processed
    pub fn record_line(&self) {
        self.total_lines.fetch_add(1, Ordering::Relaxed);
    }

    /// Record that an entry was successfully parsed
    pub fn record_entry(&self, is_multiline: bool) {
        self.parsed_entries.fetch_add(1, Ordering::Relaxed);
        if is_multiline {
            self.multiline_entries.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Record that a parse error occurred
    pub fn record_error(&self) {
        self.parse_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Get the current statistics snapshot
    pub fn get_statistics(&self) -> ParsingStatistics {
        ParsingStatistics {
            total_lines: self.total_lines.load(Ordering::Relaxed),
            parsed_entries: self.parsed_entries.load(Ordering::Relaxed),
            parse_errors: self.parse_errors.load(Ordering::Relaxed),
            multiline_entries: self.multiline_entries.load(Ordering::Relaxed),
            duration_ms: self.start_time.elapsed().as_millis() as u64,
        }
    }
}

impl Default for ParsingContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn test_parsing_context_new() {
        let context = ParsingContext::new();
        let stats = context.get_statistics();

        assert_eq!(stats.total_lines, 0);
        assert_eq!(stats.parsed_entries, 0);
        assert_eq!(stats.parse_errors, 0);
        assert_eq!(stats.multiline_entries, 0);
    }

    #[test]
    fn test_record_line() {
        let context = ParsingContext::new();
        context.record_line();
        context.record_line();
        context.record_line();

        let stats = context.get_statistics();
        assert_eq!(stats.total_lines, 3);
    }

    #[test]
    fn test_record_entry() {
        let context = ParsingContext::new();
        context.record_entry(false);
        context.record_entry(true);
        context.record_entry(false);

        let stats = context.get_statistics();
        assert_eq!(stats.parsed_entries, 3);
        assert_eq!(stats.multiline_entries, 1);
    }

    #[test]
    fn test_record_error() {
        let context = ParsingContext::new();
        context.record_error();
        context.record_error();

        let stats = context.get_statistics();
        assert_eq!(stats.parse_errors, 2);
    }

    #[test]
    fn test_duration_tracking() {
        let context = ParsingContext::new();
        std::thread::sleep(std::time::Duration::from_millis(10));

        let stats = context.get_statistics();
        assert!(stats.duration_ms >= 10);
    }
}

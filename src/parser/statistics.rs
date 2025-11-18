use serde::{Deserialize, Serialize};

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

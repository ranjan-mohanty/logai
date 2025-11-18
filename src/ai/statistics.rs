use std::collections::HashMap;
use std::time::Duration;

/// Comprehensive statistics for AI analysis
#[derive(Debug, Clone)]
pub struct AnalysisStatistics {
    pub total_groups: usize,
    pub successful: usize,
    pub failed: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub total_duration: Duration,
    pub average_duration: Duration,
    pub throughput: f64,
    pub retry_counts: HashMap<usize, usize>,
    pub failure_reasons: HashMap<String, usize>,
}

impl Default for AnalysisStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisStatistics {
    /// Create new statistics tracker
    pub fn new() -> Self {
        Self {
            total_groups: 0,
            successful: 0,
            failed: 0,
            cache_hits: 0,
            cache_misses: 0,
            total_duration: Duration::from_secs(0),
            average_duration: Duration::from_secs(0),
            throughput: 0.0,
            retry_counts: HashMap::new(),
            failure_reasons: HashMap::new(),
        }
    }

    /// Record a successful analysis
    pub fn record_success(&mut self, duration: Duration) {
        self.successful += 1;
        self.total_groups += 1;
        self.update_duration(duration);
    }

    /// Record a failed analysis
    pub fn record_failure(&mut self, reason: String, attempts: usize) {
        self.failed += 1;
        self.total_groups += 1;

        // Track retry attempts
        *self.retry_counts.entry(attempts).or_insert(0) += 1;

        // Track failure reason
        *self.failure_reasons.entry(reason).or_insert(0) += 1;
    }

    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
        self.successful += 1;
        self.total_groups += 1;
    }

    /// Record a cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }

    /// Update duration statistics
    fn update_duration(&mut self, duration: Duration) {
        self.total_duration += duration;
        if self.successful > 0 {
            self.average_duration = self.total_duration / self.successful as u32;
        }
    }

    /// Calculate throughput
    pub fn calculate_throughput(&mut self, elapsed: Duration) {
        if elapsed.as_secs_f64() > 0.0 {
            self.throughput = self.total_groups as f64 / elapsed.as_secs_f64();
        }
    }

    /// Format statistics summary for terminal display
    pub fn format_summary(&self) -> String {
        let mut output = String::new();

        output.push_str("\n📊 Analysis Statistics:\n");
        output.push_str(&format!("  Total groups: {}\n", self.total_groups));
        output.push_str(&format!("  Successful: {}\n", self.successful));
        output.push_str(&format!("  Failed: {}\n", self.failed));

        if self.cache_hits > 0 || self.cache_misses > 0 {
            output.push_str(&format!(
                "  Cache: {} hits, {} misses\n",
                self.cache_hits, self.cache_misses
            ));
        }

        output.push_str(&format!(
            "  Duration: {}\n",
            Self::format_duration(self.total_duration)
        ));

        if self.successful > 0 {
            output.push_str(&format!(
                "  Average: {}\n",
                Self::format_duration(self.average_duration)
            ));
        }

        output.push_str(&format!(
            "  Throughput: {:.2} groups/sec\n",
            self.throughput
        ));

        // Show retry statistics if any
        if !self.retry_counts.is_empty() {
            output.push_str("\n  Retry attempts:\n");
            let mut sorted_retries: Vec<_> = self.retry_counts.iter().collect();
            sorted_retries.sort_by_key(|(attempts, _)| *attempts);
            for (attempts, count) in sorted_retries {
                output.push_str(&format!("    {} attempts: {} times\n", attempts, count));
            }
        }

        // Show failure reasons if any
        if !self.failure_reasons.is_empty() {
            output.push_str("\n  Failure reasons:\n");
            let mut sorted_failures: Vec<_> = self.failure_reasons.iter().collect();
            sorted_failures.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
            for (reason, count) in sorted_failures.iter().take(5) {
                let truncated = if reason.len() > 60 {
                    format!("{}...", &reason[..60])
                } else {
                    reason.to_string()
                };
                output.push_str(&format!("    {}: {} times\n", truncated, count));
            }
        }

        output
    }

    /// Format duration as human-readable string
    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_groups > 0 {
            (self.successful as f64 / self.total_groups as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Get cache hit rate as percentage
    pub fn cache_hit_rate(&self) -> f64 {
        let total_cache_ops = self.cache_hits + self.cache_misses;
        if total_cache_ops > 0 {
            (self.cache_hits as f64 / total_cache_ops as f64) * 100.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_statistics() {
        let stats = AnalysisStatistics::new();
        assert_eq!(stats.total_groups, 0);
        assert_eq!(stats.successful, 0);
        assert_eq!(stats.failed, 0);
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 0);
    }

    #[test]
    fn test_record_success() {
        let mut stats = AnalysisStatistics::new();
        stats.record_success(Duration::from_secs(5));

        assert_eq!(stats.successful, 1);
        assert_eq!(stats.total_groups, 1);
        assert_eq!(stats.total_duration, Duration::from_secs(5));
        assert_eq!(stats.average_duration, Duration::from_secs(5));
    }

    #[test]
    fn test_record_multiple_successes() {
        let mut stats = AnalysisStatistics::new();
        stats.record_success(Duration::from_secs(4));
        stats.record_success(Duration::from_secs(6));

        assert_eq!(stats.successful, 2);
        assert_eq!(stats.total_groups, 2);
        assert_eq!(stats.total_duration, Duration::from_secs(10));
        assert_eq!(stats.average_duration, Duration::from_secs(5));
    }

    #[test]
    fn test_record_failure() {
        let mut stats = AnalysisStatistics::new();
        stats.record_failure("timeout".to_string(), 3);

        assert_eq!(stats.failed, 1);
        assert_eq!(stats.total_groups, 1);
        assert_eq!(*stats.retry_counts.get(&3).unwrap(), 1);
        assert_eq!(*stats.failure_reasons.get("timeout").unwrap(), 1);
    }

    #[test]
    fn test_record_multiple_failures() {
        let mut stats = AnalysisStatistics::new();
        stats.record_failure("timeout".to_string(), 3);
        stats.record_failure("timeout".to_string(), 2);
        stats.record_failure("network".to_string(), 3);

        assert_eq!(stats.failed, 3);
        assert_eq!(*stats.retry_counts.get(&3).unwrap(), 2);
        assert_eq!(*stats.retry_counts.get(&2).unwrap(), 1);
        assert_eq!(*stats.failure_reasons.get("timeout").unwrap(), 2);
        assert_eq!(*stats.failure_reasons.get("network").unwrap(), 1);
    }

    #[test]
    fn test_record_cache_hit() {
        let mut stats = AnalysisStatistics::new();
        stats.record_cache_hit();

        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.successful, 1);
        assert_eq!(stats.total_groups, 1);
    }

    #[test]
    fn test_record_cache_miss() {
        let mut stats = AnalysisStatistics::new();
        stats.record_cache_miss();

        assert_eq!(stats.cache_misses, 1);
        assert_eq!(stats.successful, 0);
        assert_eq!(stats.total_groups, 0);
    }

    #[test]
    fn test_calculate_throughput() {
        let mut stats = AnalysisStatistics::new();
        stats.record_success(Duration::from_secs(1));
        stats.record_success(Duration::from_secs(1));
        stats.calculate_throughput(Duration::from_secs(2));

        assert_eq!(stats.throughput, 1.0);
    }

    #[test]
    fn test_calculate_throughput_zero_duration() {
        let mut stats = AnalysisStatistics::new();
        stats.record_success(Duration::from_secs(1));
        stats.calculate_throughput(Duration::from_secs(0));

        assert_eq!(stats.throughput, 0.0);
    }

    #[test]
    fn test_success_rate() {
        let mut stats = AnalysisStatistics::new();
        stats.record_success(Duration::from_secs(1));
        stats.record_success(Duration::from_secs(1));
        stats.record_failure("error".to_string(), 1);

        assert_eq!(stats.success_rate(), 66.66666666666666);
    }

    #[test]
    fn test_success_rate_zero_total() {
        let stats = AnalysisStatistics::new();
        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn test_cache_hit_rate() {
        let mut stats = AnalysisStatistics::new();
        stats.record_cache_hit();
        stats.record_cache_hit();
        stats.record_cache_miss();

        assert_eq!(stats.cache_hit_rate(), 66.66666666666666);
    }

    #[test]
    fn test_cache_hit_rate_zero_ops() {
        let stats = AnalysisStatistics::new();
        assert_eq!(stats.cache_hit_rate(), 0.0);
    }

    #[test]
    fn test_format_summary() {
        let mut stats = AnalysisStatistics::new();
        stats.record_success(Duration::from_secs(5));
        stats.record_failure("timeout".to_string(), 3);
        stats.record_cache_hit();
        stats.calculate_throughput(Duration::from_secs(10));

        let summary = stats.format_summary();
        assert!(summary.contains("Total groups: 3"));
        assert!(summary.contains("Successful: 2"));
        assert!(summary.contains("Failed: 1"));
        assert!(summary.contains("Cache:"));
        assert!(summary.contains("Throughput:"));
    }

    #[test]
    fn test_format_duration_seconds() {
        let duration = Duration::from_secs(45);
        assert_eq!(AnalysisStatistics::format_duration(duration), "45s");
    }

    #[test]
    fn test_format_duration_minutes() {
        let duration = Duration::from_secs(125);
        assert_eq!(AnalysisStatistics::format_duration(duration), "2m 5s");
    }

    #[test]
    fn test_format_duration_hours() {
        let duration = Duration::from_secs(3665);
        assert_eq!(AnalysisStatistics::format_duration(duration), "1h 1m 5s");
    }
}

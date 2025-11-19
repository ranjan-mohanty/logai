use std::time::Duration;

/// Progress update information for AI analysis
#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    pub current: usize,
    pub total: usize,
    pub pattern: String,
    pub elapsed: Duration,
    pub throughput: f64,
}

impl ProgressUpdate {
    /// Create a new progress update
    pub fn new(current: usize, total: usize, pattern: String, elapsed: Duration) -> Self {
        let throughput = if elapsed.as_secs_f64() > 0.0 {
            current as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        Self {
            current,
            total,
            pattern,
            elapsed,
            throughput,
        }
    }

    /// Format progress for terminal display with progress bar
    pub fn format_terminal(&self) -> String {
        let percentage = if self.total > 0 {
            (self.current as f64 / self.total as f64 * 100.0) as usize
        } else {
            0
        };

        let bar_width = 30;
        let filled = if self.total > 0 {
            (bar_width * self.current) / self.total
        } else {
            0
        };
        let empty = bar_width.saturating_sub(filled);

        let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

        // Calculate ETA
        let eta = if self.throughput > 0.0 && self.current < self.total {
            let remaining = self.total - self.current;
            let eta_secs = remaining as f64 / self.throughput;
            let minutes = (eta_secs / 60.0) as usize;
            let seconds = (eta_secs % 60.0) as usize;
            if minutes > 0 {
                format!(" - ETA {}m{}s", minutes, seconds)
            } else {
                format!(" - ETA {}s", seconds)
            }
        } else {
            String::new()
        };

        format!(
            "{} {}/{} ({}%){}",
            bar, self.current, self.total, percentage, eta
        )
    }

    /// Get completion percentage
    pub fn percentage(&self) -> f64 {
        if self.total > 0 {
            (self.current as f64 / self.total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Check if analysis is complete
    pub fn is_complete(&self) -> bool {
        self.current >= self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_progress_update() {
        let update =
            ProgressUpdate::new(50, 100, "Test error".to_string(), Duration::from_secs(60));

        assert_eq!(update.current, 50);
        assert_eq!(update.total, 100);
        assert_eq!(update.pattern, "Test error");
        assert_eq!(update.elapsed, Duration::from_secs(60));
        assert!((update.throughput - 0.833).abs() < 0.01);
    }

    #[test]
    fn test_throughput_calculation() {
        let update = ProgressUpdate::new(100, 200, "Test".to_string(), Duration::from_secs(50));

        assert_eq!(update.throughput, 2.0);
    }

    #[test]
    fn test_throughput_zero_elapsed() {
        let update = ProgressUpdate::new(10, 100, "Test".to_string(), Duration::from_secs(0));

        assert_eq!(update.throughput, 0.0);
    }

    #[test]
    fn test_percentage() {
        let update = ProgressUpdate::new(25, 100, "Test".to_string(), Duration::from_secs(10));

        assert_eq!(update.percentage(), 25.0);
    }

    #[test]
    fn test_percentage_zero_total() {
        let update = ProgressUpdate::new(10, 0, "Test".to_string(), Duration::from_secs(10));

        assert_eq!(update.percentage(), 0.0);
    }

    #[test]
    fn test_is_complete() {
        let complete = ProgressUpdate::new(100, 100, "Test".to_string(), Duration::from_secs(10));
        assert!(complete.is_complete());

        let incomplete = ProgressUpdate::new(50, 100, "Test".to_string(), Duration::from_secs(10));
        assert!(!incomplete.is_complete());
    }

    #[test]
    fn test_format_terminal() {
        let update = ProgressUpdate::new(
            50,
            100,
            "Test error pattern".to_string(),
            Duration::from_secs(60),
        );

        let formatted = update.format_terminal();
        assert!(formatted.contains("50/100"));
        assert!(formatted.contains("50%"));
        assert!(formatted.contains("ETA"));
    }

    #[test]
    fn test_format_terminal_long_pattern() {
        let long_pattern = "A".repeat(100);
        let update = ProgressUpdate::new(10, 100, long_pattern, Duration::from_secs(10));

        let formatted = update.format_terminal();
        // Pattern is no longer shown in the compact format
        assert!(formatted.contains("10/100"));
        assert!(formatted.contains("10%"));
    }

    #[test]
    fn test_progress_bar_rendering() {
        let update = ProgressUpdate::new(50, 100, "Test".to_string(), Duration::from_secs(10));

        let formatted = update.format_terminal();
        // Should have filled and empty blocks
        assert!(formatted.contains("█"));
        assert!(formatted.contains("░"));
    }
}

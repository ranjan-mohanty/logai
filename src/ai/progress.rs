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

        let bar_width = 40;
        let filled = if self.total > 0 {
            (bar_width * self.current) / self.total
        } else {
            0
        };
        let empty = bar_width.saturating_sub(filled);

        let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

        let eta = self.calculate_eta();

        let truncated_pattern = if self.pattern.len() > 60 {
            format!("{}...", &self.pattern[..60])
        } else {
            self.pattern.clone()
        };

        format!(
            "{} {} / {} ({}%)\nCurrent: \"{}\"\nElapsed: {} | Throughput: {:.1} groups/sec | {}",
            bar,
            self.current,
            self.total,
            percentage,
            truncated_pattern,
            self.format_duration(self.elapsed),
            self.throughput,
            eta
        )
    }

    /// Calculate estimated time to completion
    fn calculate_eta(&self) -> String {
        if self.throughput > 0.0 && self.current < self.total {
            let remaining = self.total - self.current;
            let eta_secs = remaining as f64 / self.throughput;
            let eta_duration = Duration::from_secs_f64(eta_secs);
            format!("ETA: {}", self.format_duration(eta_duration))
        } else if self.current >= self.total {
            "Complete".to_string()
        } else {
            "ETA: calculating...".to_string()
        }
    }

    /// Format duration as human-readable string
    fn format_duration(&self, duration: Duration) -> String {
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
    fn test_format_duration_seconds() {
        let update = ProgressUpdate::new(1, 100, "Test".to_string(), Duration::from_secs(45));

        assert_eq!(update.format_duration(Duration::from_secs(45)), "45s");
    }

    #[test]
    fn test_format_duration_minutes() {
        let update = ProgressUpdate::new(1, 100, "Test".to_string(), Duration::from_secs(125));

        assert_eq!(update.format_duration(Duration::from_secs(125)), "2m 5s");
    }

    #[test]
    fn test_format_duration_hours() {
        let update = ProgressUpdate::new(1, 100, "Test".to_string(), Duration::from_secs(3665));

        assert_eq!(
            update.format_duration(Duration::from_secs(3665)),
            "1h 1m 5s"
        );
    }

    #[test]
    fn test_calculate_eta() {
        let update = ProgressUpdate::new(50, 100, "Test".to_string(), Duration::from_secs(60));

        let eta = update.calculate_eta();
        assert!(eta.contains("ETA:"));
        assert!(eta.contains("m") || eta.contains("s"));
    }

    #[test]
    fn test_calculate_eta_complete() {
        let update = ProgressUpdate::new(100, 100, "Test".to_string(), Duration::from_secs(60));

        assert_eq!(update.calculate_eta(), "Complete");
    }

    #[test]
    fn test_calculate_eta_zero_throughput() {
        let update = ProgressUpdate::new(0, 100, "Test".to_string(), Duration::from_secs(0));

        assert_eq!(update.calculate_eta(), "ETA: calculating...");
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
        assert!(formatted.contains("50 / 100"));
        assert!(formatted.contains("50%"));
        assert!(formatted.contains("Test error pattern"));
        assert!(formatted.contains("Throughput"));
        assert!(formatted.contains("ETA"));
    }

    #[test]
    fn test_format_terminal_long_pattern() {
        let long_pattern = "A".repeat(100);
        let update = ProgressUpdate::new(10, 100, long_pattern, Duration::from_secs(10));

        let formatted = update.format_terminal();
        // Should be truncated to 60 chars + "..."
        assert!(formatted.contains("..."));
        assert!(!formatted.contains(&"A".repeat(100)));
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

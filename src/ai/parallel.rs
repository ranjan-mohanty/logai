use crate::ai::provider::AIProvider;
use crate::types::ErrorGroup;
use crate::Result;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

/// Configuration for parallel analysis
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Maximum number of concurrent analysis requests (1-20)
    pub max_concurrency: usize,
    /// Enable retry logic for failed requests
    pub enable_retry: bool,
    /// Maximum number of retry attempts
    pub max_retries: usize,
    /// Initial backoff duration in milliseconds
    pub initial_backoff_ms: u64,
    /// Maximum backoff duration in milliseconds
    pub max_backoff_ms: u64,
    /// Enable response caching
    pub enable_cache: bool,
    /// Maximum message length before truncation
    pub truncate_length: usize,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            max_concurrency: 5,
            enable_retry: true,
            max_retries: 3,
            initial_backoff_ms: 1000,
            max_backoff_ms: 30000,
            enable_cache: true,
            truncate_length: 2000,
        }
    }
}

impl AnalysisConfig {
    /// Create a new configuration with validation
    pub fn new(max_concurrency: usize) -> Result<Self> {
        if !(1..=20).contains(&max_concurrency) {
            return Err(anyhow::anyhow!(
                "Concurrency must be between 1 and 20, got {}",
                max_concurrency
            ));
        }

        Ok(Self {
            max_concurrency,
            ..Default::default()
        })
    }
}

/// Progress update information
#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    pub current: usize,
    pub total: usize,
    pub pattern: String,
    pub elapsed: std::time::Duration,
    pub throughput: f64,
}

impl ProgressUpdate {
    /// Format progress for terminal display
    pub fn format_terminal(&self) -> String {
        let percentage = (self.current as f64 / self.total as f64 * 100.0) as usize;
        let bar_width = 40;
        let filled = (bar_width * self.current) / self.total;
        let empty = bar_width - filled;

        let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));

        let eta = if self.throughput > 0.0 {
            let remaining = self.total - self.current;
            let eta_secs = remaining as f64 / self.throughput;
            format!(
                "ETA: {}m {}s",
                (eta_secs / 60.0) as usize,
                (eta_secs % 60.0) as usize
            )
        } else {
            "ETA: calculating...".to_string()
        };

        format!(
            "{} {} / {} ({}%)\nCurrent: \"{}\"\nElapsed: {}m {}s | Throughput: {:.1} groups/sec | {}",
            bar,
            self.current,
            self.total,
            percentage,
            if self.pattern.len() > 60 {
                format!("{}...", &self.pattern[..60])
            } else {
                self.pattern.clone()
            },
            self.elapsed.as_secs() / 60,
            self.elapsed.as_secs() % 60,
            self.throughput,
            eta
        )
    }
}

/// Parallel analyzer for processing error groups concurrently
pub struct ParallelAnalyzer {
    provider: Arc<dyn AIProvider>,
    semaphore: Arc<Semaphore>,
    #[allow(dead_code)]
    config: AnalysisConfig,
}

impl ParallelAnalyzer {
    /// Create a new parallel analyzer
    pub fn new(provider: Arc<dyn AIProvider>, config: AnalysisConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrency));

        Self {
            provider,
            semaphore,
            config,
        }
    }

    /// Analyze multiple error groups in parallel
    pub async fn analyze_groups<F>(
        &self,
        groups: &mut [ErrorGroup],
        progress_callback: F,
    ) -> Result<()>
    where
        F: Fn(ProgressUpdate) + Send + Sync + 'static,
    {
        let total = groups.len();
        let start_time = std::time::Instant::now();
        let completed = Arc::new(Mutex::new(0usize));
        let callback = Arc::new(progress_callback);

        // Create tasks for each group with index to maintain ordering
        let mut tasks = Vec::new();

        for (index, group) in groups.iter().enumerate() {
            let provider = Arc::clone(&self.provider);
            let semaphore = Arc::clone(&self.semaphore);
            let completed = Arc::clone(&completed);
            let callback = Arc::clone(&callback);
            let group_clone = group.clone();
            let pattern = group.pattern.clone();

            let task = tokio::spawn(async move {
                // Acquire semaphore permit to limit concurrency
                let _permit = semaphore.acquire().await.unwrap();

                // Analyze the group
                let result = provider.analyze(&group_clone).await;

                // Update progress
                let mut count = completed.lock().await;
                *count += 1;
                let current = *count;
                drop(count);

                let elapsed = start_time.elapsed();
                let throughput = current as f64 / elapsed.as_secs_f64();

                callback(ProgressUpdate {
                    current,
                    total,
                    pattern,
                    elapsed,
                    throughput,
                });

                (index, result)
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete and collect results
        let mut results = Vec::new();
        for task in tasks {
            match task.await {
                Ok((index, result)) => results.push((index, result)),
                Err(e) => {
                    log::error!("Task failed: {}", e);
                }
            }
        }

        // Sort results by index to maintain original ordering
        results.sort_by_key(|(index, _)| *index);

        // Apply results back to groups
        for (index, result) in results {
            if let Ok(analysis) = result {
                groups[index].analysis = Some(analysis);
            } else if let Err(e) = result {
                log::warn!("Failed to analyze group {}: {}", groups[index].id, e);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        assert!(AnalysisConfig::new(5).is_ok());
        assert!(AnalysisConfig::new(1).is_ok());
        assert!(AnalysisConfig::new(20).is_ok());
        assert!(AnalysisConfig::new(0).is_err());
        assert!(AnalysisConfig::new(21).is_err());
    }

    #[test]
    fn test_progress_format() {
        let update = ProgressUpdate {
            current: 50,
            total: 100,
            pattern: "Test error pattern".to_string(),
            elapsed: std::time::Duration::from_secs(60),
            throughput: 0.83,
        };

        let formatted = update.format_terminal();
        assert!(formatted.contains("50 / 100"));
        assert!(formatted.contains("50%"));
        assert!(formatted.contains("Test error pattern"));
    }
}

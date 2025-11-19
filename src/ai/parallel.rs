//! Parallel AI analysis infrastructure for processing multiple error groups concurrently.
//!
//! This module provides the core parallel processing capabilities for AI analysis,
//! including concurrency control, result ordering, and progress tracking.
//!
//! # Example
//!
//! ```no_run
//! use logai::ai::{ParallelAnalyzer, AnalysisConfig, ProgressUpdate};
//! use std::sync::Arc;
//!
//! # async fn example() -> anyhow::Result<()> {
//! # let provider = Arc::new(logai::ai::NoAI);
//! # let mut groups = vec![];
//! let config = AnalysisConfig::default();
//! let analyzer = ParallelAnalyzer::new(provider, config);
//!
//! let progress_callback = |update: ProgressUpdate| {
//!     println!("{}", update.format_terminal());
//! };
//!
//! analyzer.analyze_groups(&mut groups, progress_callback).await?;
//! # Ok(())
//! # }
//! ```

use crate::ai::progress::ProgressUpdate;
use crate::ai::provider::AIProvider;
use crate::types::ErrorGroup;
use crate::Result;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

/// Configuration for parallel analysis
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Maximum number of concurrent analysis requests (1-100)
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
        if !(1..=100).contains(&max_concurrency) {
            return Err(anyhow::anyhow!(
                "Concurrency must be between 1 and 100, got {}",
                max_concurrency
            ));
        }

        Ok(Self {
            max_concurrency,
            ..Default::default()
        })
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

                callback(ProgressUpdate::new(current, total, pattern, elapsed));

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
                let group = &groups[index];
                log::debug!(
                    "Failed to analyze group {} (pattern: {}, count: {}): {}",
                    group.id,
                    group.pattern,
                    group.count,
                    e
                );
                // Log the full error chain for debugging
                log::debug!("Error chain for group {}: {:?}", group.id, e);
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
        assert!(AnalysisConfig::new(100).is_ok());
        assert!(AnalysisConfig::new(0).is_err());
        assert!(AnalysisConfig::new(101).is_err());
    }
}

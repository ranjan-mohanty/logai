use crate::ai::provider::AIProvider;
use crate::types::{ErrorAnalysis, ErrorGroup};
use crate::Result;
use std::sync::Arc;
use std::time::Duration;

/// Analyzer that wraps an AI provider with retry logic
pub struct RetryableAnalyzer {
    provider: Arc<dyn AIProvider>,
    max_retries: usize,
    initial_backoff_ms: u64,
    max_backoff_ms: u64,
}

impl RetryableAnalyzer {
    /// Create a new retryable analyzer
    pub fn new(
        provider: Arc<dyn AIProvider>,
        max_retries: usize,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
    ) -> Self {
        Self {
            provider,
            max_retries,
            initial_backoff_ms,
            max_backoff_ms,
        }
    }

    /// Analyze an error group with retry logic
    pub async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            match self.provider.analyze(group).await {
                Ok(analysis) => {
                    if attempt > 0 {
                        log::info!(
                            "Successfully analyzed group {} after {} retries",
                            group.id,
                            attempt
                        );
                    }
                    return Ok(analysis);
                }
                Err(e) => {
                    last_error = Some(e);

                    // Check if error is retryable
                    if !self.should_retry(last_error.as_ref().unwrap()) {
                        log::warn!(
                            "Non-retryable error for group {}: {}",
                            group.id,
                            last_error.as_ref().unwrap()
                        );
                        break;
                    }

                    // Retry with backoff if attempts remain
                    if attempt < self.max_retries {
                        let backoff = self.calculate_backoff(attempt);
                        log::warn!(
                            "Attempt {} failed for group {}, retrying in {:?}: {}",
                            attempt + 1,
                            group.id,
                            backoff,
                            last_error.as_ref().unwrap()
                        );
                        tokio::time::sleep(backoff).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Analysis failed after retries")))
    }

    /// Determine if an error should be retried
    fn should_retry(&self, error: &anyhow::Error) -> bool {
        let error_str = error.to_string().to_lowercase();

        // Retryable errors: network, timeout, rate limit, server errors
        if error_str.contains("timeout")
            || error_str.contains("timed out")
            || error_str.contains("connection")
            || error_str.contains("network")
            || error_str.contains("429")
            || error_str.contains("rate limit")
            || error_str.contains("500")
            || error_str.contains("502")
            || error_str.contains("503")
            || error_str.contains("empty response")
        {
            return true;
        }

        // Partially retryable: JSON parsing errors
        if error_str.contains("json") || error_str.contains("parse") {
            return true;
        }

        // Non-retryable: authentication, authorization, bad request
        if error_str.contains("401")
            || error_str.contains("403")
            || error_str.contains("400")
            || error_str.contains("unauthorized")
            || error_str.contains("forbidden")
            || error_str.contains("authentication")
        {
            return false;
        }

        // Default to retryable for unknown errors
        true
    }

    /// Calculate exponential backoff with jitter
    fn calculate_backoff(&self, attempt: usize) -> Duration {
        use rand::Rng;

        // Calculate base backoff: initial * 2^attempt
        let backoff_ms = self
            .initial_backoff_ms
            .saturating_mul(2_u64.saturating_pow(attempt as u32));
        let backoff_ms = backoff_ms.min(self.max_backoff_ms);

        // Add jitter (±20%)
        let jitter_range = (backoff_ms as f64 * 0.2) as u64;
        let mut rng = rand::rng();
        let jitter = rng.random_range(0..=jitter_range);

        let final_backoff = if rng.random_bool(0.5) {
            backoff_ms.saturating_add(jitter)
        } else {
            backoff_ms.saturating_sub(jitter)
        };

        Duration::from_millis(final_backoff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ErrorGroup, Severity};
    use async_trait::async_trait;
    use chrono::Utc;

    struct MockProvider {
        fail_count: Arc<std::sync::Mutex<usize>>,
        error_type: String,
    }

    impl MockProvider {
        fn new(fail_count: usize, error_type: &str) -> Self {
            Self {
                fail_count: Arc::new(std::sync::Mutex::new(fail_count)),
                error_type: error_type.to_string(),
            }
        }
    }

    #[async_trait]
    impl AIProvider for MockProvider {
        async fn analyze(&self, _group: &ErrorGroup) -> Result<ErrorAnalysis> {
            let mut count = self.fail_count.lock().unwrap();
            if *count > 0 {
                *count -= 1;
                return Err(anyhow::anyhow!("{}", self.error_type));
            }

            Ok(ErrorAnalysis {
                explanation: "Test analysis".to_string(),
                root_cause: None,
                suggestions: vec![],
                related_resources: vec![],
                tool_invocations: vec![],
            })
        }

        fn name(&self) -> &str {
            "mock"
        }
    }

    fn create_test_group() -> ErrorGroup {
        ErrorGroup {
            id: "test-1".to_string(),
            pattern: "Test error".to_string(),
            count: 1,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            severity: Severity::Error,
            entries: vec![],
            analysis: None,
        }
    }

    #[tokio::test]
    async fn test_retry_on_timeout() {
        let provider = Arc::new(MockProvider::new(2, "timeout error"));
        let analyzer = RetryableAnalyzer::new(provider, 3, 10, 1000);

        let group = create_test_group();
        let result = analyzer.analyze(&group).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_retry_on_network_error() {
        let provider = Arc::new(MockProvider::new(1, "network connection failed"));
        let analyzer = RetryableAnalyzer::new(provider, 3, 10, 1000);

        let group = create_test_group();
        let result = analyzer.analyze(&group).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_no_retry_on_auth_error() {
        let provider = Arc::new(MockProvider::new(5, "401 unauthorized"));
        let fail_count = Arc::clone(&provider.fail_count);
        let analyzer = RetryableAnalyzer::new(provider, 3, 10, 1000);

        let group = create_test_group();
        let result = analyzer.analyze(&group).await;

        // Should fail immediately without retries
        assert!(result.is_err());
        assert_eq!(*fail_count.lock().unwrap(), 4); // Only 1 attempt
    }

    #[tokio::test]
    async fn test_max_retries_exhausted() {
        let provider = Arc::new(MockProvider::new(10, "timeout"));
        let fail_count = Arc::clone(&provider.fail_count);
        let analyzer = RetryableAnalyzer::new(provider, 2, 10, 1000);

        let group = create_test_group();
        let result = analyzer.analyze(&group).await;

        // Should fail after max retries
        assert!(result.is_err());
        assert_eq!(*fail_count.lock().unwrap(), 7); // 3 attempts (0, 1, 2)
    }

    #[test]
    fn test_backoff_calculation() {
        let provider = Arc::new(MockProvider::new(0, ""));
        let analyzer = RetryableAnalyzer::new(provider, 3, 1000, 30000);

        // Test exponential growth
        let backoff0 = analyzer.calculate_backoff(0);
        let backoff1 = analyzer.calculate_backoff(1);
        let backoff2 = analyzer.calculate_backoff(2);

        // Backoff should grow exponentially (with jitter)
        assert!(backoff0.as_millis() >= 800); // ~1000ms ±20%
        assert!(backoff0.as_millis() <= 1200);

        assert!(backoff1.as_millis() >= 1600); // ~2000ms ±20%
        assert!(backoff1.as_millis() <= 2400);

        assert!(backoff2.as_millis() >= 3200); // ~4000ms ±20%
        assert!(backoff2.as_millis() <= 4800);
    }

    #[test]
    fn test_backoff_max_limit() {
        let provider = Arc::new(MockProvider::new(0, ""));
        let analyzer = RetryableAnalyzer::new(provider, 10, 1000, 5000);

        // Test that backoff doesn't exceed max
        let backoff10 = analyzer.calculate_backoff(10);
        assert!(backoff10.as_millis() <= 6000); // Max 5000ms + 20% jitter
    }

    #[test]
    fn test_should_retry_logic() {
        let provider = Arc::new(MockProvider::new(0, ""));
        let analyzer = RetryableAnalyzer::new(provider, 3, 1000, 30000);

        // Retryable errors
        assert!(analyzer.should_retry(&anyhow::anyhow!("timeout error")));
        assert!(analyzer.should_retry(&anyhow::anyhow!("connection refused")));
        assert!(analyzer.should_retry(&anyhow::anyhow!("429 rate limit")));
        assert!(analyzer.should_retry(&anyhow::anyhow!("500 server error")));
        assert!(analyzer.should_retry(&anyhow::anyhow!("json parse error")));

        // Non-retryable errors
        assert!(!analyzer.should_retry(&anyhow::anyhow!("401 unauthorized")));
        assert!(!analyzer.should_retry(&anyhow::anyhow!("403 forbidden")));
        assert!(!analyzer.should_retry(&anyhow::anyhow!("400 bad request")));
    }
}

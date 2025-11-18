use chrono::Utc;
use logai::ai::{AnalysisConfig, ParallelAnalyzer, ProgressUpdate};
use logai::types::{ErrorAnalysis, ErrorGroup, LogEntry, LogMetadata, Severity, Suggestion};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Mock AI provider for testing
struct MockProvider {
    delay_ms: u64,
    failure_rate: f64,
    call_count: Arc<Mutex<usize>>,
}

impl MockProvider {
    fn new(delay_ms: u64, failure_rate: f64) -> Self {
        Self {
            delay_ms,
            failure_rate,
            call_count: Arc::new(Mutex::new(0)),
        }
    }

    fn get_call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }
}

#[async_trait::async_trait]
impl logai::ai::AIProvider for MockProvider {
    async fn analyze(&self, group: &ErrorGroup) -> anyhow::Result<ErrorAnalysis> {
        // Increment call count
        {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;
        }

        // Simulate processing delay
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;

        // Simulate random failures
        if rand::random::<f64>() < self.failure_rate {
            return Err(anyhow::anyhow!("Simulated failure"));
        }

        // Return mock analysis
        Ok(ErrorAnalysis {
            explanation: format!("Analysis for group {}", group.id),
            root_cause: Some("Mock root cause".to_string()),
            suggestions: vec![Suggestion {
                description: "Mock suggestion".to_string(),
                code_example: None,
                priority: 1,
            }],
            related_resources: vec![],
            tool_invocations: vec![],
        })
    }

    fn name(&self) -> &str {
        "mock"
    }
}

fn create_test_groups(count: usize) -> Vec<ErrorGroup> {
    (0..count)
        .map(|i| ErrorGroup {
            id: i.to_string(),
            pattern: format!("Error pattern {}", i),
            count: 1,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            severity: Severity::Error,
            entries: vec![LogEntry {
                timestamp: Some(Utc::now()),
                severity: Severity::Error,
                message: format!("Error message {}", i),
                metadata: LogMetadata {
                    file: None,
                    line: None,
                    function: None,
                    thread: None,
                    extra: std::collections::HashMap::new(),
                },
                raw: format!("Error message {}", i),
            }],
            analysis: None,
        })
        .collect()
}

#[tokio::test]
async fn test_parallel_processing_with_mock_provider() {
    let provider = Arc::new(MockProvider::new(10, 0.0));
    let config = AnalysisConfig {
        max_concurrency: 3,
        enable_retry: false,
        max_retries: 0,
        initial_backoff_ms: 100,
        max_backoff_ms: 1000,
        enable_cache: false,
        truncate_length: 2000,
    };

    let analyzer = ParallelAnalyzer::new(provider.clone(), config);
    let mut groups = create_test_groups(10);

    let progress_updates = Arc::new(Mutex::new(Vec::new()));
    let progress_updates_clone = progress_updates.clone();

    let callback = move |update: ProgressUpdate| {
        progress_updates_clone.lock().unwrap().push(update);
    };

    let result = analyzer.analyze_groups(&mut groups, callback).await;
    assert!(result.is_ok());

    // Verify all groups were analyzed
    assert_eq!(groups.iter().filter(|g| g.analysis.is_some()).count(), 10);

    // Verify provider was called for each group
    assert_eq!(provider.get_call_count(), 10);

    // Verify progress updates were sent
    let updates = progress_updates.lock().unwrap();
    assert!(!updates.is_empty());
}

#[tokio::test]
async fn test_result_ordering_is_maintained() {
    let provider = Arc::new(MockProvider::new(5, 0.0));
    let config = AnalysisConfig::default();

    let analyzer = ParallelAnalyzer::new(provider, config);
    let mut groups = create_test_groups(20);

    let callback = |_: ProgressUpdate| {};

    analyzer
        .analyze_groups(&mut groups, callback)
        .await
        .unwrap();

    // Verify results are in the same order as input
    for (i, group) in groups.iter().enumerate() {
        assert_eq!(group.id, i.to_string());
        assert!(group.analysis.is_some());
        let analysis = group.analysis.as_ref().unwrap();
        assert!(analysis.explanation.contains(&i.to_string()));
    }
}

#[tokio::test]
async fn test_concurrency_limits_are_respected() {
    let provider = Arc::new(MockProvider::new(100, 0.0));
    let config = AnalysisConfig {
        max_concurrency: 2,
        enable_retry: false,
        max_retries: 0,
        initial_backoff_ms: 100,
        max_backoff_ms: 1000,
        enable_cache: false,
        truncate_length: 2000,
    };

    let analyzer = ParallelAnalyzer::new(provider.clone(), config);
    let mut groups = create_test_groups(10);

    let start = std::time::Instant::now();
    let callback = |_: ProgressUpdate| {};

    analyzer
        .analyze_groups(&mut groups, callback)
        .await
        .unwrap();

    let duration = start.elapsed();

    // With concurrency of 2 and 10 groups taking 100ms each,
    // minimum time should be around 500ms (10 groups / 2 concurrent = 5 batches * 100ms)
    // Allow some overhead
    assert!(
        duration >= Duration::from_millis(400),
        "Duration too short: {:?}",
        duration
    );
}

#[tokio::test]
async fn test_mixed_success_failure_scenarios() {
    // 50% failure rate
    let provider = Arc::new(MockProvider::new(10, 0.5));
    let config = AnalysisConfig {
        max_concurrency: 5,
        enable_retry: false,
        max_retries: 0,
        initial_backoff_ms: 100,
        max_backoff_ms: 1000,
        enable_cache: false,
        truncate_length: 2000,
    };

    let analyzer = ParallelAnalyzer::new(provider, config);
    let mut groups = create_test_groups(20);

    let callback = |_: ProgressUpdate| {};

    let result = analyzer.analyze_groups(&mut groups, callback).await;
    assert!(result.is_ok());

    // Some groups should have analysis, some should not
    let analyzed_count = groups.iter().filter(|g| g.analysis.is_some()).count();
    let failed_count = groups.iter().filter(|g| g.analysis.is_none()).count();

    assert!(analyzed_count > 0, "Expected some successful analyses");
    assert!(failed_count > 0, "Expected some failures");
    assert_eq!(analyzed_count + failed_count, 20);
}

#[tokio::test]
async fn test_progress_callback_invocation() {
    let provider = Arc::new(MockProvider::new(10, 0.0));
    let config = AnalysisConfig::default();

    let analyzer = ParallelAnalyzer::new(provider, config);
    let mut groups = create_test_groups(5);

    let progress_count = Arc::new(Mutex::new(0));
    let progress_count_clone = progress_count.clone();

    let callback = move |_: ProgressUpdate| {
        *progress_count_clone.lock().unwrap() += 1;
    };

    analyzer
        .analyze_groups(&mut groups, callback)
        .await
        .unwrap();

    // Should have received progress updates (at least one per group)
    let count = *progress_count.lock().unwrap();
    assert!(
        count >= 5,
        "Expected at least 5 progress updates, got {}",
        count
    );
}

#[tokio::test]
async fn test_empty_groups_list() {
    let provider = Arc::new(MockProvider::new(10, 0.0));
    let config = AnalysisConfig::default();

    let analyzer = ParallelAnalyzer::new(provider, config);
    let mut groups: Vec<ErrorGroup> = vec![];

    let callback = |_: ProgressUpdate| {};

    let result = analyzer.analyze_groups(&mut groups, callback).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_single_group() {
    let provider = Arc::new(MockProvider::new(10, 0.0));
    let config = AnalysisConfig::default();

    let analyzer = ParallelAnalyzer::new(provider, config);
    let mut groups = create_test_groups(1);

    let callback = |_: ProgressUpdate| {};

    analyzer
        .analyze_groups(&mut groups, callback)
        .await
        .unwrap();

    assert_eq!(groups.len(), 1);
    assert!(groups[0].analysis.is_some());
}

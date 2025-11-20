//! Mock implementations for testing
//!
//! This module provides mock implementations of external dependencies
//! for isolated testing.

use async_trait::async_trait;
use logai::ai::AIProvider;
use logai::types::{ErrorAnalysis, ErrorGroup};
use logai::Result;

/// Mock AI provider for testing
pub struct MockAIProvider {
    pub responses: Vec<ErrorAnalysis>,
    pub call_count: std::sync::Arc<std::sync::Mutex<usize>>,
}

impl MockAIProvider {
    pub fn new() -> Self {
        Self {
            responses: vec![Self::default_response()],
            call_count: std::sync::Arc::new(std::sync::Mutex::new(0)),
        }
    }

    pub fn with_responses(responses: Vec<ErrorAnalysis>) -> Self {
        Self {
            responses,
            call_count: std::sync::Arc::new(std::sync::Mutex::new(0)),
        }
    }

    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    fn default_response() -> ErrorAnalysis {
        use logai::types::Suggestion;

        ErrorAnalysis {
            explanation: "Mock explanation".to_string(),
            root_cause: Some("Mock root cause".to_string()),
            suggestions: vec![Suggestion {
                description: "Mock suggestion 1".to_string(),
                code_example: None,
                priority: 1,
            }],
            related_resources: vec![],
            tool_invocations: vec![],
        }
    }
}

impl Default for MockAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AIProvider for MockAIProvider {
    fn name(&self) -> &str {
        "mock"
    }

    async fn analyze(&self, _group: &ErrorGroup) -> Result<ErrorAnalysis> {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        let index = (*count - 1) % self.responses.len();
        Ok(self.responses[index].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::fixtures;

    #[tokio::test]
    async fn test_mock_provider() {
        let provider = MockAIProvider::new();
        assert_eq!(provider.name(), "mock");
        assert_eq!(provider.call_count(), 0);
    }

    #[tokio::test]
    async fn test_mock_provider_analyze() {
        let provider = MockAIProvider::new();
        let group = fixtures::sample_error_group();

        let result = provider.analyze(&group).await;
        assert!(result.is_ok());
        assert_eq!(provider.call_count(), 1);

        let analysis = result.unwrap();
        assert_eq!(analysis.explanation, "Mock explanation");
    }

    #[tokio::test]
    async fn test_mock_provider_multiple_responses() {
        let responses = vec![
            ErrorAnalysis {
                explanation: "First".to_string(),
                root_cause: None,
                suggestions: vec![],
                related_resources: vec![],
                tool_invocations: vec![],
            },
            ErrorAnalysis {
                explanation: "Second".to_string(),
                root_cause: None,
                suggestions: vec![],
                related_resources: vec![],
                tool_invocations: vec![],
            },
        ];

        let provider = MockAIProvider::with_responses(responses);
        let group = fixtures::sample_error_group();

        let result1 = provider.analyze(&group).await.unwrap();
        assert_eq!(result1.explanation, "First");

        let result2 = provider.analyze(&group).await.unwrap();
        assert_eq!(result2.explanation, "Second");

        // Should cycle back to first
        let result3 = provider.analyze(&group).await.unwrap();
        assert_eq!(result3.explanation, "First");
    }
}

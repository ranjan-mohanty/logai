use logai::ai::provider::AIProvider;
use logai::ai::providers::BedrockProvider;
use logai::types::{ErrorGroup, LogEntry, LogMetadata, Severity};

#[tokio::test]
async fn test_bedrock_provider_creation() {
    // Test that BedrockProvider can be created with valid parameters
    // This will fail without AWS credentials, but tests the validation logic
    let result = BedrockProvider::new(
        Some("anthropic.claude-3-5-sonnet-20241022-v2:0".to_string()),
        Some("us-east-1".to_string()),
        Some(4096),
        Some(0.7),
    )
    .await;

    // Without real AWS credentials, this should fail with credentials error
    if let Err(e) = result {
        let error_msg = e.to_string();
        // Should be a credentials error, not a validation error
        assert!(
            error_msg.contains("credentials")
                || error_msg.contains("region")
                || error_msg.contains("provider")
        );
    }
}

#[tokio::test]
async fn test_bedrock_invalid_model() {
    let result = BedrockProvider::new(Some("invalid-model-id".to_string()), None, None, None).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Invalid model ID"));
}

#[tokio::test]
async fn test_bedrock_invalid_region() {
    let result = BedrockProvider::new(None, Some("invalid".to_string()), None, None).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Invalid region"));
}

#[test]
fn test_bedrock_provider_name() {
    let provider = BedrockProvider::default();
    assert_eq!(provider.name(), "bedrock");
}

#[test]
fn test_error_group_creation_for_bedrock() {
    // Test that we can create error groups that would be sent to Bedrock
    let entry = LogEntry {
        timestamp: Some(chrono::Utc::now()),
        severity: Severity::Error,
        message: "Database connection failed".to_string(),
        metadata: LogMetadata {
            file: None,
            line: None,
            function: None,
            thread: None,
            extra: std::collections::HashMap::new(),
        },
        raw: "Database connection failed".to_string(),
    };

    let group = ErrorGroup {
        id: "test-1".to_string(),
        pattern: "Database connection failed".to_string(),
        count: 5,
        entries: vec![entry],
        first_seen: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        severity: Severity::Error,
        analysis: None,
    };

    assert_eq!(group.pattern, "Database connection failed");
    assert_eq!(group.count, 5);
    assert_eq!(group.severity, Severity::Error);
}

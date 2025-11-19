use crate::ai::json_extractor::EnhancedJsonExtractor;
use crate::ai::prompts::build_enhanced_analysis_prompt;
use crate::ai::provider::AIProvider;
use crate::types::{ErrorAnalysis, ErrorGroup};
use crate::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use aws_sdk_bedrockruntime::config::ProvideCredentials;
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use aws_sdk_bedrockruntime::Client as BedrockClient;
use thiserror::Error;

/// Bedrock-specific errors
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum BedrockError {
    #[error("AWS credentials not found: {0}")]
    CredentialsNotFound(String),

    #[error("Invalid model ID: {0}. Supported models: anthropic.claude-3-5-sonnet-20241022-v2:0, anthropic.claude-3-haiku-20240307-v1:0, meta.llama3-2-90b-instruct-v1:0, amazon.titan-text-premier-v1:0")]
    InvalidModel(String),

    #[error("Bedrock API error: {0}")]
    ApiError(String),

    #[error("Throttling error: {0}. Try reducing concurrency or adding delays")]
    Throttling(String),

    #[error("Invalid region: {0}. Use a valid AWS region like us-east-1, us-west-2, eu-west-1")]
    InvalidRegion(String),

    #[error("Authentication error: {0}. Check your AWS credentials")]
    AuthenticationError(String),

    #[error("Model not available in region: {0}")]
    ModelNotFound(String),
}

/// AWS Bedrock provider for AI-powered log analysis
#[derive(Debug)]
pub struct BedrockProvider {
    client: BedrockClient,
    model_id: String,
    max_tokens: i32,
    temperature: f32,
}

impl Default for BedrockProvider {
    fn default() -> Self {
        Self {
            client: BedrockClient::from_conf(
                aws_sdk_bedrockruntime::Config::builder()
                    .behavior_version(aws_sdk_bedrockruntime::config::BehaviorVersion::latest())
                    .build(),
            ),
            model_id: "anthropic.claude-3-5-sonnet-20241022-v2:0".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

impl BedrockProvider {
    /// Supported Bedrock models
    const SUPPORTED_MODELS: &'static [&'static str] = &[
        "anthropic.claude-3-5-sonnet-20241022-v2:0",
        "anthropic.claude-3-haiku-20240307-v1:0",
        "meta.llama3-2-90b-instruct-v1:0",
        "amazon.titan-text-premier-v1:0",
    ];

    /// Create a new Bedrock provider with custom configuration
    pub async fn new(
        model_id: Option<String>,
        region: Option<String>,
        max_tokens: Option<i32>,
        temperature: Option<f32>,
    ) -> Result<Self> {
        let model_id =
            model_id.unwrap_or_else(|| "anthropic.claude-3-5-sonnet-20241022-v2:0".to_string());

        log::debug!("Creating BedrockProvider with model: {}", model_id);

        // Validate model ID
        if !Self::SUPPORTED_MODELS.contains(&model_id.as_str()) {
            log::error!("Invalid model ID: {}", model_id);
            return Err(BedrockError::InvalidModel(model_id).into());
        }

        // Load AWS configuration - this will check env vars and ~/.aws/credentials
        log::debug!("Loading AWS configuration...");
        let mut config_loader = aws_config::defaults(aws_config::BehaviorVersion::latest());

        if let Some(ref region_str) = region {
            log::debug!("Using region: {}", region_str);
            // Basic region validation
            if region_str.is_empty() || !region_str.contains('-') {
                log::error!("Invalid region format: {}", region_str);
                return Err(BedrockError::InvalidRegion(region_str.clone()).into());
            }
            config_loader = config_loader.region(aws_config::Region::new(region_str.clone()));
        } else {
            log::debug!("No region specified, will use default from AWS config or environment");
        }

        let config = config_loader.load().await;

        // Check region
        let region_name = config.region().map(|r| r.as_ref().to_string());
        if region_name.is_none() {
            log::warn!("No AWS region configured. Bedrock requires a region. Set AWS_REGION or AWS_DEFAULT_REGION environment variable, or configure ~/.aws/config");
            return Err(anyhow!("No AWS region configured. Set AWS_REGION or AWS_DEFAULT_REGION environment variable, or add 'region = us-east-1' to ~/.aws/config"));
        }
        log::debug!("Using AWS region: {}", region_name.as_ref().unwrap());

        // Log credential source
        if let Some(provider) = config.credentials_provider() {
            log::debug!("AWS credentials provider found");
            // Try to load credentials to verify they work
            match provider.provide_credentials().await {
                Ok(creds) => {
                    log::debug!(
                        "Successfully loaded AWS credentials (access key: {}...)",
                        &creds.access_key_id()[..std::cmp::min(8, creds.access_key_id().len())]
                    );
                }
                Err(e) => {
                    log::error!("Failed to load AWS credentials: {}", e);
                    return Err(BedrockError::CredentialsNotFound(
                        format!("Failed to load AWS credentials: {}. Check your AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables, or ~/.aws/credentials file", e)
                    ).into());
                }
            }
        } else {
            log::error!("No AWS credentials provider found");
            return Err(BedrockError::CredentialsNotFound(
                "No AWS credentials found. Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables, or configure ~/.aws/credentials".to_string()
            ).into());
        }

        let client = BedrockClient::new(&config);
        log::debug!("BedrockClient created successfully");

        Ok(Self {
            client,
            model_id,
            max_tokens: max_tokens.unwrap_or(4096),
            temperature: temperature.unwrap_or(0.7),
        })
    }

    /// Call the Bedrock Converse API
    async fn call_api(&self, prompt: String) -> Result<String> {
        log::debug!("Building Bedrock API request for model: {}", self.model_id);

        let message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(prompt))
            .build()
            .map_err(|e| {
                log::error!("Failed to build message: {}", e);
                anyhow!("Failed to build message: {}", e)
            })?;

        log::debug!(
            "Sending request to Bedrock API (max_tokens: {}, temperature: {})",
            self.max_tokens,
            self.temperature
        );

        let response = self
            .client
            .converse()
            .model_id(&self.model_id)
            .messages(message)
            .inference_config(
                aws_sdk_bedrockruntime::types::InferenceConfiguration::builder()
                    .max_tokens(self.max_tokens)
                    .temperature(self.temperature)
                    .build(),
            )
            .send()
            .await
            .map_err(|e| {
                let error_msg = e.to_string();
                log::error!("Bedrock API call failed: {}", error_msg);
                anyhow!("Failed to call Bedrock API: {}", error_msg)
            })?;

        log::debug!("Received response from Bedrock API");

        // Extract text from response
        let output = response
            .output()
            .ok_or_else(|| anyhow!("No output in Bedrock response"))?;

        let message = match output.as_message() {
            Ok(msg) => msg,
            Err(_) => return Err(anyhow!("Response is not a message")),
        };

        let content = message
            .content()
            .first()
            .ok_or_else(|| anyhow!("No content in message"))?;

        let text = match content.as_text() {
            Ok(t) => t,
            Err(_) => return Err(anyhow!("Content is not text")),
        };

        Ok(text.to_string())
    }
}

#[async_trait]
impl AIProvider for BedrockProvider {
    async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis> {
        log::debug!(
            "Analyzing error group: {} (count: {})",
            group.pattern,
            group.count
        );

        // Build the analysis prompt
        let prompt = build_enhanced_analysis_prompt(group, 2000);
        log::debug!("Built analysis prompt ({} chars)", prompt.len());

        // Call Bedrock API
        let response_text = self.call_api(prompt).await?;
        log::debug!("Received response ({} chars)", response_text.len());

        // Extract and parse JSON from response
        let json_str = match EnhancedJsonExtractor::extract(&response_text) {
            Ok(json) => {
                log::debug!("Extracted JSON ({} chars)", json.len());
                json
            }
            Err(e) => {
                log::debug!(
                    "Failed to extract JSON from Bedrock response for group {}: {}",
                    group.id,
                    e
                );
                log::debug!("Full Bedrock response text:\n{}", response_text);
                return Err(anyhow!(
                    "Failed to extract JSON from Bedrock response: {}",
                    e
                ));
            }
        };

        let analysis: ErrorAnalysis = match serde_json::from_str(&json_str) {
            Ok(analysis) => {
                log::debug!("Successfully parsed ErrorAnalysis");
                analysis
            }
            Err(e) => {
                log::debug!(
                    "Failed to parse JSON into ErrorAnalysis for group {}: {}",
                    group.id,
                    e
                );
                log::debug!("Extracted JSON string:\n{}", json_str);
                return Err(anyhow!("Failed to parse JSON into ErrorAnalysis: {}", e));
            }
        };

        Ok(analysis)
    }

    fn name(&self) -> &str {
        "bedrock"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bedrock_provider() {
        let provider = BedrockProvider::default();
        assert_eq!(provider.name(), "bedrock");
        assert_eq!(
            provider.model_id,
            "anthropic.claude-3-5-sonnet-20241022-v2:0"
        );
        assert_eq!(provider.max_tokens, 4096);
        assert_eq!(provider.temperature, 0.7);
    }

    #[test]
    fn test_provider_name() {
        let provider = BedrockProvider::default();
        assert_eq!(provider.name(), "bedrock");
    }

    #[test]
    fn test_supported_models() {
        assert!(BedrockProvider::SUPPORTED_MODELS
            .contains(&"anthropic.claude-3-5-sonnet-20241022-v2:0"));
        assert!(
            BedrockProvider::SUPPORTED_MODELS.contains(&"anthropic.claude-3-haiku-20240307-v1:0")
        );
        assert!(BedrockProvider::SUPPORTED_MODELS.contains(&"meta.llama3-2-90b-instruct-v1:0"));
        assert!(BedrockProvider::SUPPORTED_MODELS.contains(&"amazon.titan-text-premier-v1:0"));
    }

    #[tokio::test]
    async fn test_invalid_model_id() {
        let result =
            BedrockProvider::new(Some("invalid-model".to_string()), None, None, None).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Invalid model ID"));
    }

    #[tokio::test]
    async fn test_invalid_region() {
        let result = BedrockProvider::new(None, Some("invalid".to_string()), None, None).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Invalid region"));
    }

    #[test]
    fn test_bedrock_error_display() {
        let error = BedrockError::InvalidModel("test-model".to_string());
        assert!(error.to_string().contains("Invalid model ID: test-model"));

        let error = BedrockError::Throttling("rate limit".to_string());
        assert!(error.to_string().contains("Throttling error"));
        assert!(error.to_string().contains("Try reducing concurrency"));

        let error = BedrockError::CredentialsNotFound("no creds".to_string());
        assert!(error.to_string().contains("AWS credentials not found"));
    }

    #[test]
    fn test_configuration_validation() {
        // Test valid model IDs
        for model in BedrockProvider::SUPPORTED_MODELS {
            assert!(BedrockProvider::SUPPORTED_MODELS.contains(model));
        }
    }

    #[tokio::test]
    async fn test_custom_configuration() {
        // This test will fail without AWS credentials, but tests the validation logic
        let result = BedrockProvider::new(
            Some("anthropic.claude-3-haiku-20240307-v1:0".to_string()),
            Some("us-west-2".to_string()),
            Some(2048),
            Some(0.5),
        )
        .await;

        // Will fail due to missing credentials, but model/region validation should pass
        if let Err(e) = result {
            // Should be credentials error, not validation error
            assert!(e.to_string().contains("credentials") || e.to_string().contains("region"));
        }
    }
}

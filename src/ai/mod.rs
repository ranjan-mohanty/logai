pub mod cache;
pub mod config;
pub mod mcp_helper;
pub mod parallel;
pub mod prompts;
pub mod provider;
pub mod providers;
pub mod retry;

pub use cache::AnalysisCache;
pub use config::AIConfig;
pub use parallel::{AnalysisConfig, ParallelAnalyzer, ProgressUpdate};
pub use provider::{AIProvider, NoAI};
pub use providers::{ClaudeProvider, GeminiProvider, OllamaProvider, OpenAIProvider};
pub use retry::RetryableAnalyzer;

use crate::Result;
use std::sync::Arc;

pub fn create_provider(
    provider_name: &str,
    api_key: Option<String>,
    model: Option<String>,
    host: Option<String>,
) -> Result<Arc<dyn AIProvider>> {
    // Load config file
    let config = AIConfig::load().unwrap_or_default();
    let provider_config = config.get_provider(provider_name);

    match provider_name.to_lowercase().as_str() {
        "openai" => {
            let api_key = api_key
                .or_else(|| provider_config.and_then(|c| c.api_key.clone()))
                .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                .ok_or_else(|| anyhow::anyhow!("OpenAI API key not provided. Set OPENAI_API_KEY environment variable, use --api-key, or configure in ~/.logai/config.toml"))?;
            let model = model.or_else(|| provider_config.and_then(|c| c.model.clone()));
            Ok(Arc::new(OpenAIProvider::new(api_key, model)))
        }
        "claude" => {
            let api_key = api_key
                .or_else(|| provider_config.and_then(|c| c.api_key.clone()))
                .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
                .ok_or_else(|| anyhow::anyhow!("Claude API key not provided. Set ANTHROPIC_API_KEY environment variable, use --api-key, or configure in ~/.logai/config.toml"))?;
            let model = model.or_else(|| provider_config.and_then(|c| c.model.clone()));
            Ok(Arc::new(ClaudeProvider::new(api_key, model)))
        }
        "gemini" => {
            let api_key = api_key
                .or_else(|| provider_config.and_then(|c| c.api_key.clone()))
                .or_else(|| std::env::var("GEMINI_API_KEY").ok())
                .ok_or_else(|| anyhow::anyhow!("Gemini API key not provided. Set GEMINI_API_KEY environment variable, use --api-key, or configure in ~/.logai/config.toml"))?;
            let model = model.or_else(|| provider_config.and_then(|c| c.model.clone()));
            Ok(Arc::new(GeminiProvider::new(api_key, model)))
        }
        "ollama" => {
            let host = host.or_else(|| provider_config.and_then(|c| c.host.clone()));
            let model = model.or_else(|| provider_config.and_then(|c| c.model.clone()));
            Ok(Arc::new(OllamaProvider::new(host, model)))
        }
        "none" => Ok(Arc::new(NoAI)),
        _ => Err(anyhow::anyhow!(
            "Unknown AI provider: {}. Supported: openai, claude, gemini, ollama, none",
            provider_name
        )),
    }
}

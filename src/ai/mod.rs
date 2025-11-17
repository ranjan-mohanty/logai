pub mod cache;
pub mod claude;
pub mod gemini;
pub mod ollama;
pub mod openai;
pub mod provider;

pub use cache::AnalysisCache;
pub use claude::ClaudeProvider;
pub use gemini::GeminiProvider;
pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;
pub use provider::{AIProvider, NoAI};

use crate::Result;
use std::sync::Arc;

pub fn create_provider(
    provider_name: &str,
    api_key: Option<String>,
    model: Option<String>,
    host: Option<String>,
) -> Result<Arc<dyn AIProvider>> {
    match provider_name.to_lowercase().as_str() {
        "openai" => {
            let api_key = api_key
                .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                .ok_or_else(|| anyhow::anyhow!("OpenAI API key not provided. Set OPENAI_API_KEY environment variable or use --api-key"))?;
            Ok(Arc::new(OpenAIProvider::new(api_key, model)))
        }
        "claude" => {
            let api_key = api_key
                .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
                .ok_or_else(|| anyhow::anyhow!("Claude API key not provided. Set ANTHROPIC_API_KEY environment variable or use --api-key"))?;
            Ok(Arc::new(ClaudeProvider::new(api_key, model)))
        }
        "gemini" => {
            let api_key = api_key
                .or_else(|| std::env::var("GEMINI_API_KEY").ok())
                .ok_or_else(|| anyhow::anyhow!("Gemini API key not provided. Set GEMINI_API_KEY environment variable or use --api-key"))?;
            Ok(Arc::new(GeminiProvider::new(api_key, model)))
        }
        "ollama" => Ok(Arc::new(OllamaProvider::new(host, model))),
        "none" => Ok(Arc::new(NoAI)),
        _ => Err(anyhow::anyhow!(
            "Unknown AI provider: {}. Supported: openai, claude, gemini, ollama, none",
            provider_name
        )),
    }
}

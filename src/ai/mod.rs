pub mod provider;
pub mod openai;
pub mod ollama;

pub use provider::{AIProvider, NoAI};
pub use openai::OpenAIProvider;
pub use ollama::OllamaProvider;

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
        "ollama" => Ok(Arc::new(OllamaProvider::new(host, model))),
        "none" => Ok(Arc::new(NoAI)),
        _ => Err(anyhow::anyhow!("Unknown AI provider: {}", provider_name)),
    }
}

//! AI-powered error analysis with parallel processing and retry logic.
//!
//! This module provides the infrastructure for analyzing error groups using various
//! AI providers (OpenAI, Claude, Gemini, Ollama) with support for:
//!
//! - **Parallel Processing**: Analyze multiple error groups concurrently
//! - **Retry Logic**: Automatic retry with exponential backoff for transient failures
//! - **Enhanced JSON Extraction**: Robust parsing of AI responses
//! - **Progress Tracking**: Real-time progress updates and statistics
//! - **Configuration**: File-based configuration with CLI overrides
//!
//! # Quick Start
//!
//! ```no_run
//! use logai::ai::{self, ParallelAnalyzer, AnalysisConfig, ProgressUpdate};
//!
//! # async fn example() -> anyhow::Result<()> {
//! # let mut groups = vec![];
//! // Create AI provider
//! let provider = ai::create_provider("ollama", None, Some("llama3.2".to_string()), None)?;
//!
//! // Configure parallel analysis
//! let config = AnalysisConfig::default();
//! let analyzer = ParallelAnalyzer::new(provider, config);
//!
//! // Analyze with progress tracking
//! analyzer.analyze_groups(&mut groups, |update: ProgressUpdate| {
//!     println!("{}", update.format_terminal());
//! }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration
//!
//! Configuration can be loaded from `~/.logai/config.toml`:
//!
//! ```toml
//! [analysis]
//! max_concurrency = 5
//! enable_retry = true
//! max_retries = 3
//! initial_backoff_ms = 1000
//! max_backoff_ms = 30000
//! ```

pub mod cache;
pub mod config;
pub mod json_extractor;
pub mod mcp_helper;
pub mod parallel;
pub mod progress;
pub mod prompts;
pub mod provider;
pub mod providers;
pub mod retry;
pub mod statistics;

pub use cache::AnalysisCache;
pub use config::AIConfig;
pub use json_extractor::EnhancedJsonExtractor;
pub use parallel::{AnalysisConfig, ParallelAnalyzer};
pub use progress::ProgressUpdate;
pub use provider::{AIProvider, NoAI};
pub use providers::{ClaudeProvider, GeminiProvider, OllamaProvider, OpenAIProvider};
pub use retry::RetryableAnalyzer;
pub use statistics::AnalysisStatistics;

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

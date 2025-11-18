//! AI provider implementations
//!
//! This module contains integrations with various AI providers:
//! - OpenAI (GPT models)
//! - Claude (Anthropic)
//! - Gemini (Google)
//! - Ollama (Local models)

mod claude;
mod gemini;
mod ollama;
mod openai;

pub use claude::ClaudeProvider;
pub use gemini::GeminiProvider;
pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;

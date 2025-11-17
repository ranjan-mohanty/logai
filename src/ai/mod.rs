// AI module - to be implemented in Phase 2
// This will contain AI provider trait and implementations for OpenAI, Claude, Ollama, etc.

use crate::types::{ErrorAnalysis, ErrorGroup};
use crate::Result;

pub trait AIProvider: Send + Sync {
    fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis>;
}

// Placeholder for future implementation
pub struct NoAI;

impl AIProvider for NoAI {
    fn analyze(&self, _group: &ErrorGroup) -> Result<ErrorAnalysis> {
        Ok(ErrorAnalysis {
            explanation: "AI analysis not enabled".to_string(),
            root_cause: None,
            suggestions: vec![],
            related_resources: vec![],
        })
    }
}

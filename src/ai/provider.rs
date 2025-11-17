use crate::types::{ErrorAnalysis, ErrorGroup};
use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis>;
    fn name(&self) -> &str;
}

pub struct NoAI;

#[async_trait]
impl AIProvider for NoAI {
    async fn analyze(&self, _group: &ErrorGroup) -> Result<ErrorAnalysis> {
        Ok(ErrorAnalysis {
            explanation: "AI analysis not enabled. Use --ai flag to enable.".to_string(),
            root_cause: None,
            suggestions: vec![],
            related_resources: vec![],
        })
    }

    fn name(&self) -> &str {
        "none"
    }
}

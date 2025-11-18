use crate::mcp::MCPClient;
use crate::types::{ErrorAnalysis, ErrorGroup};
use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis>;

    /// Analyze with MCP tools support
    async fn analyze_with_tools(
        &self,
        group: &ErrorGroup,
        _mcp_client: Option<&MCPClient>,
    ) -> Result<ErrorAnalysis> {
        // Default implementation: just call analyze without tools
        self.analyze(group).await
    }

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
            tool_invocations: vec![],
        })
    }

    fn name(&self) -> &str {
        "none"
    }
}

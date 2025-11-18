use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Information about an MCP tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    /// Name of the tool
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// JSON Schema for tool parameters
    pub parameters: serde_json::Value,
    /// Name of the server providing this tool
    pub server: String,
}

/// A request to invoke an MCP tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvocation {
    /// Name of the tool to invoke
    pub tool: String,
    /// Parameters to pass to the tool (must match tool's schema)
    pub parameters: serde_json::Value,
}

/// Result of a tool invocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Whether the invocation succeeded
    pub success: bool,
    /// Result data (if successful)
    pub data: Option<serde_json::Value>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Metadata about the invocation
    pub metadata: ToolMetadata,
}

/// Metadata about a tool invocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// When the tool was invoked
    pub invoked_at: DateTime<Utc>,
    /// How long the invocation took (milliseconds)
    pub duration_ms: u64,
    /// Which server handled the invocation
    pub server: String,
}

/// Record of a tool invocation for analysis context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvocationRecord {
    /// Name of the tool that was invoked
    pub tool_name: String,
    /// When it was invoked
    pub invoked_at: DateTime<Utc>,
    /// Duration in milliseconds
    pub duration_ms: u64,
    /// The result of the invocation
    pub result: ToolResult,
}

/// Summary of tool invocation for output display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvocationSummary {
    /// Tool name
    pub tool: String,
    /// Status: "success", "failure", or "timeout"
    pub status: String,
    /// Duration in milliseconds
    pub duration_ms: u64,
    /// Which suggestions/resources came from this tool
    pub contributed_to: Vec<String>,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MCPError {
    #[error("Failed to connect to MCP server '{server}': {source}")]
    ConnectionFailed {
        server: String,
        #[source]
        source: anyhow::Error,
    },

    #[error("Tool '{tool}' not found")]
    ToolNotFound { tool: String },

    #[error("Tool invocation failed: {0}")]
    InvocationFailed(String),

    #[error("Tool '{tool}' timed out after {timeout_ms}ms")]
    ToolTimeout { tool: String, timeout_ms: u64 },

    #[error("Invalid tool parameters: {0}")]
    InvalidParameters(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Transport error: {0}")]
    TransportError(String),
}

pub type Result<T> = std::result::Result<T, MCPError>;

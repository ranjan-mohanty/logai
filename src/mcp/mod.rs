pub mod client;
pub mod config;
pub mod error;
pub mod protocol;
pub mod transport;

pub use client::MCPClient;
pub use config::{AuthConfig, ConnectionConfig, MCPConfig, ServerConfig};
pub use error::{MCPError, Result};
pub use protocol::{
    ToolInfo, ToolInvocation, ToolInvocationRecord, ToolInvocationSummary, ToolMetadata, ToolResult,
};
pub use transport::{StdioTransport, Transport};

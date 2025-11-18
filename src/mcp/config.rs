use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Configuration for MCP integration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MCPConfig {
    /// List of MCP servers to connect to
    #[serde(default)]
    pub servers: Vec<ServerConfig>,
    /// Default timeout for tool invocations (seconds)
    #[serde(default = "default_timeout")]
    pub default_timeout: u64,
    /// Per-tool timeout overrides (seconds)
    #[serde(default)]
    pub tool_timeouts: HashMap<String, u64>,
}

fn default_timeout() -> u64 {
    30
}

impl MCPConfig {
    /// Get timeout for a specific tool (falls back to default)
    pub fn get_tool_timeout(&self, tool_name: &str) -> Duration {
        let seconds = self
            .tool_timeouts
            .get(tool_name)
            .copied()
            .unwrap_or(self.default_timeout);
        Duration::from_secs(seconds)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        for server in &self.servers {
            if server.name.is_empty() {
                return Err("Server name cannot be empty".to_string());
            }
        }
        Ok(())
    }
}

/// Configuration for a single MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Unique name for this server
    pub name: String,
    /// Whether this server is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Connection configuration
    pub connection: ConnectionConfig,
    /// Optional authentication configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthConfig>,
}

fn default_enabled() -> bool {
    true
}

/// Connection configuration for an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionConfig {
    /// Connect via stdio (spawn a process)
    Stdio {
        /// Command to execute
        command: String,
        /// Arguments to pass to the command
        #[serde(default)]
        args: Vec<String>,
    },
    /// Connect via HTTP
    Http {
        /// HTTP URL
        url: String,
    },
    /// Connect via WebSocket
    WebSocket {
        /// WebSocket URL
        url: String,
    },
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication scheme (e.g., "Bearer", "Basic")
    pub scheme: String,
    /// Credentials (key-value pairs)
    pub credentials: HashMap<String, String>,
}

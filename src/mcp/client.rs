use crate::mcp::{MCPConfig, MCPError, Result, StdioTransport, ToolInfo, Transport};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Connection to an MCP server
pub struct ServerConnection {
    pub name: String,
    pub connected: bool,
    pub transport: Arc<Mutex<Box<dyn Transport>>>,
}

/// MCP Client for managing connections and tool invocations
pub struct MCPClient {
    /// Connected servers
    servers: HashMap<String, ServerConnection>,
    /// Available tools from all servers
    tools: HashMap<String, ToolInfo>,
    /// Configuration
    config: MCPConfig,
}

impl MCPClient {
    /// Create a new MCP client from configuration
    pub fn new(config: MCPConfig) -> Result<Self> {
        // Validate configuration
        config.validate().map_err(MCPError::ConfigError)?;

        Ok(Self {
            servers: HashMap::new(),
            tools: HashMap::new(),
            config,
        })
    }

    /// Connect to all configured MCP servers
    pub async fn connect(&mut self) -> Result<()> {
        let mut connection_errors = Vec::new();

        // Clone server configs to avoid borrow checker issues
        let server_configs = self.config.servers.clone();

        for server_config in &server_configs {
            if !server_config.enabled {
                continue;
            }

            match self.connect_server(server_config).await {
                Ok(()) => {
                    log::info!("Connected to MCP server: {}", server_config.name);
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to MCP server '{}': {}",
                        server_config.name,
                        e
                    );
                    connection_errors.push((server_config.name.clone(), e));
                }
            }
        }

        // Return Ok even if some connections failed (graceful degradation)
        Ok(())
    }

    /// Connect to a single server
    async fn connect_server(&mut self, server_config: &crate::mcp::ServerConfig) -> Result<()> {
        use crate::mcp::ConnectionConfig;

        let transport: Box<dyn Transport> = match &server_config.connection {
            ConnectionConfig::Stdio { command, args } => {
                let transport = StdioTransport::new(command.clone(), args.clone());
                transport.start().await?;
                Box::new(transport)
            }
            ConnectionConfig::Http { url: _ } => {
                return Err(MCPError::TransportError(
                    "HTTP transport not yet implemented".to_string(),
                ));
            }
            ConnectionConfig::WebSocket { url: _ } => {
                return Err(MCPError::TransportError(
                    "WebSocket transport not yet implemented".to_string(),
                ));
            }
        };

        let transport = Arc::new(Mutex::new(transport));

        // Perform MCP initialization handshake
        self.initialize_connection(&server_config.name, &transport)
            .await?;

        let connection = ServerConnection {
            name: server_config.name.clone(),
            connected: true,
            transport,
        };

        self.servers.insert(server_config.name.clone(), connection);
        Ok(())
    }

    /// Initialize MCP connection with handshake
    async fn initialize_connection(
        &self,
        server_name: &str,
        transport: &Arc<Mutex<Box<dyn Transport>>>,
    ) -> Result<()> {
        // Send initialize request
        let init_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "logai",
                    "version": env!("CARGO_PKG_VERSION")
                }
            }
        });

        let transport_guard = transport.lock().await;
        transport_guard.send(init_request).await?;

        // Receive initialize response
        let response = transport_guard.receive().await?;

        // Check for errors
        if let Some(error) = response.get("error") {
            let error_msg = error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown initialization error")
                .to_string();
            return Err(MCPError::ConnectionFailed {
                server: server_name.to_string(),
                source: anyhow::anyhow!(error_msg),
            });
        }

        // Verify we got a valid response
        if response.get("result").is_none() {
            return Err(MCPError::ProtocolError(
                "Invalid initialize response".to_string(),
            ));
        }

        log::debug!("Initialized connection to server '{}'", server_name);
        Ok(())
    }

    /// Discover tools from all connected servers
    pub async fn discover_tools(&mut self) -> Result<Vec<ToolInfo>> {
        let mut all_tools = Vec::new();

        for (server_name, connection) in &self.servers {
            if !connection.connected {
                continue;
            }

            match self
                .discover_tools_from_server(server_name, connection)
                .await
            {
                Ok(tools) => {
                    log::info!(
                        "Discovered {} tools from server '{}'",
                        tools.len(),
                        server_name
                    );
                    all_tools.extend(tools);
                }
                Err(e) => {
                    log::error!(
                        "Failed to discover tools from server '{}': {}",
                        server_name,
                        e
                    );
                }
            }
        }

        // Store tools in registry
        for tool in &all_tools {
            self.tools.insert(tool.name.clone(), tool.clone());
        }

        Ok(all_tools)
    }

    /// Discover tools from a single server
    async fn discover_tools_from_server(
        &self,
        server_name: &str,
        connection: &ServerConnection,
    ) -> Result<Vec<ToolInfo>> {
        // Send list_tools request
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list",
            "params": {}
        });

        let transport = connection.transport.lock().await;
        transport.send(request).await?;

        // Receive response
        let response = transport.receive().await?;

        // Parse tools from response
        log::debug!(
            "Tools response: {}",
            serde_json::to_string_pretty(&response).unwrap_or_default()
        );

        let tools = response
            .get("result")
            .and_then(|r| r.get("tools"))
            .and_then(|t| t.as_array())
            .ok_or_else(|| {
                MCPError::ProtocolError(format!(
                    "Invalid tools response: {}",
                    serde_json::to_string(&response).unwrap_or_default()
                ))
            })?;

        let mut tool_infos = Vec::new();
        for tool in tools {
            let name = tool
                .get("name")
                .and_then(|n| n.as_str())
                .ok_or_else(|| MCPError::ProtocolError("Tool missing name".to_string()))?
                .to_string();

            let description = tool
                .get("description")
                .and_then(|d| d.as_str())
                .unwrap_or("")
                .to_string();

            let parameters = tool
                .get("inputSchema")
                .cloned()
                .unwrap_or(serde_json::json!({}));

            tool_infos.push(ToolInfo {
                name,
                description,
                parameters,
                server: server_name.to_string(),
            });
        }

        Ok(tool_infos)
    }

    /// Close all server connections
    pub async fn disconnect(&mut self) -> Result<()> {
        for (name, connection) in self.servers.iter_mut() {
            let mut transport = connection.transport.lock().await;
            if let Err(e) = transport.close().await {
                log::error!("Failed to close connection to '{}': {}", name, e);
            }
            connection.connected = false;
        }
        Ok(())
    }

    /// Get the configuration
    pub fn config(&self) -> &MCPConfig {
        &self.config
    }

    /// Get list of available tools
    pub fn available_tools(&self) -> Vec<&ToolInfo> {
        self.tools.values().collect()
    }

    /// Get a specific tool by name
    pub fn get_tool(&self, name: &str) -> Option<&ToolInfo> {
        self.tools.get(name)
    }

    /// Check if a tool is available
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Get list of connected servers
    pub fn connected_servers(&self) -> Vec<&str> {
        self.servers
            .values()
            .filter(|s| s.connected)
            .map(|s| s.name.as_str())
            .collect()
    }

    /// Check if any servers are connected
    pub fn is_connected(&self) -> bool {
        self.servers.values().any(|s| s.connected)
    }

    /// Invoke a tool by name with parameters
    pub async fn invoke_tool(
        &self,
        tool_name: &str,
        params: serde_json::Value,
    ) -> Result<crate::mcp::ToolResult> {
        use chrono::Utc;
        use std::time::Instant;

        let start = Instant::now();
        let invoked_at = Utc::now();

        // Find the tool
        let tool = self
            .get_tool(tool_name)
            .ok_or_else(|| MCPError::ToolNotFound {
                tool: tool_name.to_string(),
            })?;

        // Validate parameters against schema (basic validation)
        self.validate_parameters(tool_name, &params, &tool.parameters)?;

        // Get the server connection
        let connection = self.servers.get(&tool.server).ok_or_else(|| {
            MCPError::InvocationFailed(format!("Server '{}' not found", tool.server))
        })?;

        if !connection.connected {
            return Err(MCPError::InvocationFailed(format!(
                "Server '{}' is not connected",
                tool.server
            )));
        }

        // Get timeout for this tool
        let timeout = self.config.get_tool_timeout(tool_name);

        // Invoke with timeout
        let result = tokio::time::timeout(
            timeout,
            self.invoke_tool_on_server(tool_name, params, connection),
        )
        .await;

        let duration_ms = start.elapsed().as_millis() as u64;

        match result {
            Ok(Ok(data)) => Ok(crate::mcp::ToolResult {
                success: true,
                data: Some(data),
                error: None,
                metadata: crate::mcp::ToolMetadata {
                    invoked_at,
                    duration_ms,
                    server: tool.server.clone(),
                },
            }),
            Ok(Err(e)) => {
                log::error!("Tool '{}' invocation failed: {}", tool_name, e);
                Ok(crate::mcp::ToolResult {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    metadata: crate::mcp::ToolMetadata {
                        invoked_at,
                        duration_ms,
                        server: tool.server.clone(),
                    },
                })
            }
            Err(_) => {
                log::error!("Tool '{}' timed out after {}ms", tool_name, duration_ms);
                Err(MCPError::ToolTimeout {
                    tool: tool_name.to_string(),
                    timeout_ms: duration_ms,
                })
            }
        }
    }

    /// Invoke a tool on a specific server
    async fn invoke_tool_on_server(
        &self,
        tool_name: &str,
        params: serde_json::Value,
        connection: &ServerConnection,
    ) -> Result<serde_json::Value> {
        // Send tool invocation request
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": params
            }
        });

        let transport = connection.transport.lock().await;
        transport.send(request).await?;

        // Receive response
        let response = transport.receive().await?;

        // Check for errors
        if let Some(error) = response.get("error") {
            let error_msg = error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            return Err(MCPError::InvocationFailed(error_msg.to_string()));
        }

        // Extract result
        let result = response
            .get("result")
            .cloned()
            .ok_or_else(|| MCPError::ProtocolError("Missing result in response".to_string()))?;

        Ok(result)
    }

    /// Validate parameters against tool schema
    fn validate_parameters(
        &self,
        tool_name: &str,
        params: &serde_json::Value,
        schema: &serde_json::Value,
    ) -> Result<()> {
        // Basic validation: check if params is an object when schema expects one
        if let Some("object") = schema.get("type").and_then(|t| t.as_str()) {
            if !params.is_object() {
                return Err(MCPError::InvalidParameters(format!(
                    "Tool '{}' expects an object, got {}",
                    tool_name, params
                )));
            }

            // Check required fields
            if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
                let params_obj = params.as_object().unwrap();
                for req_field in required {
                    if let Some(field_name) = req_field.as_str() {
                        if !params_obj.contains_key(field_name) {
                            return Err(MCPError::InvalidParameters(format!(
                                "Tool '{}' missing required parameter: {}",
                                tool_name, field_name
                            )));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcp::ServerConfig;

    #[test]
    fn test_new_client_with_valid_config() {
        let config = MCPConfig::default();
        let client = MCPClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_new_client_validates_config() {
        let mut config = MCPConfig::default();
        config.servers.push(ServerConfig {
            name: String::new(), // Invalid: empty name
            enabled: true,
            connection: crate::mcp::ConnectionConfig::Stdio {
                command: "test".to_string(),
                args: vec![],
            },
            auth: None,
        });

        let client = MCPClient::new(config);
        assert!(client.is_err());
    }

    #[test]
    fn test_available_tools_empty() {
        let config = MCPConfig::default();
        let client = MCPClient::new(config).unwrap();
        assert_eq!(client.available_tools().len(), 0);
    }

    #[test]
    fn test_has_tool() {
        let config = MCPConfig::default();
        let client = MCPClient::new(config).unwrap();
        assert!(!client.has_tool("nonexistent"));
    }

    #[test]
    fn test_is_connected_initially_false() {
        let config = MCPConfig::default();
        let client = MCPClient::new(config).unwrap();
        assert!(!client.is_connected());
    }
}

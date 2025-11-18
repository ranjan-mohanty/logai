use logai::mcp::{ConnectionConfig, MCPClient, MCPConfig, ServerConfig};

#[tokio::test]
#[ignore] // Ignore by default since it requires external MCP server
async fn test_mcp_client_with_echo_server() {
    // Create a simple config with an echo-like MCP server
    let config = MCPConfig {
        servers: vec![ServerConfig {
            name: "test-server".to_string(),
            enabled: true,
            connection: ConnectionConfig::Stdio {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-everything".to_string(),
                    "stdio".to_string(),
                ],
            },
            auth: None,
        }],
        default_timeout: 30,
        tool_timeouts: std::collections::HashMap::new(),
    };

    // Create client
    let mut client = MCPClient::new(config).expect("Failed to create client");

    // Connect to servers
    client.connect().await.expect("Failed to connect");

    // Check if connected
    assert!(client.is_connected(), "Client should be connected");

    // Discover tools
    let tools = client
        .discover_tools()
        .await
        .expect("Failed to discover tools");

    println!("Discovered {} tools:", tools.len());
    for tool in &tools {
        println!("  - {} ({})", tool.name, tool.description);
    }

    assert!(!tools.is_empty(), "Should discover at least one tool");

    // Try to invoke a tool if available
    if let Some(tool) = tools.first() {
        println!("\nTesting tool: {}", tool.name);

        // Create simple test parameters
        let params = serde_json::json!({});

        match client.invoke_tool(&tool.name, params).await {
            Ok(result) => {
                println!("Tool invocation result:");
                println!("  Success: {}", result.success);
                println!("  Duration: {}ms", result.metadata.duration_ms);
                if let Some(data) = result.data {
                    println!("  Data: {}", serde_json::to_string_pretty(&data).unwrap());
                }
            }
            Err(e) => {
                println!("Tool invocation failed (expected for some tools): {}", e);
            }
        }
    }

    // Disconnect
    client.disconnect().await.expect("Failed to disconnect");
}

#[tokio::test]
async fn test_mcp_client_basic_operations() {
    // Test with empty config
    let config = MCPConfig::default();
    let client = MCPClient::new(config).expect("Failed to create client");

    // Should not be connected initially
    assert!(!client.is_connected());

    // Should have no tools
    assert_eq!(client.available_tools().len(), 0);

    // Should have no connected servers
    assert_eq!(client.connected_servers().len(), 0);
}

#[tokio::test]
async fn test_mcp_client_tool_not_found() {
    let config = MCPConfig::default();
    let client = MCPClient::new(config).expect("Failed to create client");

    // Try to invoke non-existent tool
    let result = client
        .invoke_tool("nonexistent", serde_json::json!({}))
        .await;

    assert!(result.is_err());
    match result {
        Err(logai::mcp::MCPError::ToolNotFound { tool }) => {
            assert_eq!(tool, "nonexistent");
        }
        _ => panic!("Expected ToolNotFound error"),
    }
}

use logai::mcp::{ConnectionConfig, MCPClient, MCPConfig, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    println!("Testing MCP Client Integration\n");

    // Create config
    let config = MCPConfig {
        servers: vec![ServerConfig {
            name: "everything-server".to_string(),
            enabled: true,
            connection: ConnectionConfig::Stdio {
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-filesystem".to_string(),
                    "/tmp".to_string(),
                ],
            },
            auth: None,
        }],
        default_timeout: 30,
        tool_timeouts: std::collections::HashMap::new(),
    };

    // Create client
    println!("Creating MCP client...");
    let mut client = MCPClient::new(config)?;

    // Connect
    println!("Connecting to MCP servers...");
    client.connect().await?;

    if client.is_connected() {
        println!("✓ Connected successfully");
        println!("  Connected servers: {:?}", client.connected_servers());
    } else {
        println!("✗ Failed to connect");
        return Ok(());
    }

    // Discover tools
    println!("\nDiscovering tools...");
    match client.discover_tools().await {
        Ok(tools) => {
            println!("✓ Discovered {} tools:", tools.len());
            for tool in &tools {
                println!("  - {} ({})", tool.name, tool.description);
                println!("    Server: {}", tool.server);
            }

            // Try invoking list_allowed_directories (no params needed)
            if let Some(tool) = tools.iter().find(|t| t.name == "list_allowed_directories") {
                println!("\nTesting tool invocation: {}", tool.name);
                let params = serde_json::json!({});

                match client.invoke_tool(&tool.name, params).await {
                    Ok(result) => {
                        println!("✓ Tool invocation successful");
                        println!("  Duration: {}ms", result.metadata.duration_ms);
                        if let Some(data) = result.data {
                            println!(
                                "  Data: {}",
                                serde_json::to_string_pretty(&data).unwrap_or_default()
                            );
                        }
                    }
                    Err(e) => {
                        println!("✗ Tool invocation failed: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to discover tools: {}", e);
        }
    }

    // Disconnect
    println!("\nDisconnecting...");
    client.disconnect().await?;
    println!("✓ Disconnected");

    Ok(())
}

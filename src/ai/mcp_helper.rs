use crate::mcp::{MCPClient, ToolInvocationRecord};
use crate::types::ErrorGroup;
use chrono::Utc;

/// Invoke relevant MCP tools based on error content
pub async fn invoke_relevant_tools(
    client: &MCPClient,
    _group: &ErrorGroup,
) -> Vec<ToolInvocationRecord> {
    let mut results = Vec::new();

    // Get available tools
    let available_tools = client.available_tools();
    log::debug!("MCP: Found {} available tools", available_tools.len());

    // For now, try to invoke simple tools that don't require parameters
    // In the future, we'll have smarter logic to determine which tools to invoke based on:
    // - Error patterns (stack traces -> search_code)
    // - Timestamps (-> check_metrics)
    // - Error messages (-> search_docs)
    // - Recurring patterns (-> query_logs)

    for tool in available_tools {
        // Only invoke simple tools that don't require parameters for now
        if tool.name == "list_allowed_directories" {
            log::debug!("MCP: Invoking tool: {}", tool.name);
            let params = serde_json::json!({});

            match client.invoke_tool(&tool.name, params).await {
                Ok(result) => {
                    log::debug!(
                        "MCP: Tool {} succeeded in {}ms",
                        tool.name,
                        result.metadata.duration_ms
                    );
                    results.push(ToolInvocationRecord {
                        tool_name: tool.name.clone(),
                        invoked_at: Utc::now(),
                        duration_ms: result.metadata.duration_ms,
                        result,
                    });
                }
                Err(e) => {
                    log::warn!("MCP: Failed to invoke tool {}: {}", tool.name, e);
                }
            }
        }
    }

    log::debug!("MCP: Invoked {} tools total", results.len());
    results
}

/// Build enhanced prompt with tool results
pub fn augment_prompt_with_tools(
    base_prompt: String,
    tool_results: &[ToolInvocationRecord],
) -> String {
    let mut prompt = base_prompt;

    if !tool_results.is_empty() {
        prompt.push_str("\n\n## Additional Context from MCP Tools:\n");
        for result in tool_results {
            if result.result.success {
                if let Some(data) = &result.result.data {
                    prompt.push_str(&format!(
                        "\n### Tool: {} ({}ms)\n{}\n",
                        result.tool_name,
                        result.duration_ms,
                        serde_json::to_string_pretty(data).unwrap_or_default()
                    ));
                }
            } else if let Some(error) = &result.result.error {
                prompt.push_str(&format!(
                    "\n### Tool: {} (failed)\nError: {}\n",
                    result.tool_name, error
                ));
            }
        }
    }

    prompt
}

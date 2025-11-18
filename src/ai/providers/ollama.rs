use crate::ai::prompts::build_analysis_prompt;
use crate::ai::provider::AIProvider;
use crate::types::{ErrorAnalysis, ErrorGroup, Suggestion};
use crate::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaProvider {
    client: Client,
    host: String,
    model: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaProvider {
    pub fn new(host: Option<String>, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            host: host.unwrap_or_else(|| "http://localhost:11434".to_string()),
            model: model.unwrap_or_else(|| "llama3.2".to_string()),
        }
    }

    async fn call_api(&self, prompt: String) -> Result<String> {
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
        };

        let url = format!("{}/api/generate", self.host);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| anyhow!("Ollama API request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow!("Ollama API error {}: {}", status, error_text));
        }

        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse Ollama API response: {}", e))?;

        if ollama_response.response.trim().is_empty() {
            return Err(anyhow!("Ollama returned an empty response"));
        }

        Ok(ollama_response.response)
    }

    fn parse_response(&self, response: &str) -> Result<ErrorAnalysis> {
        // Check if response is empty
        if response.trim().is_empty() {
            return Err(anyhow!("Empty response from Ollama"));
        }

        // Try to extract JSON from markdown code blocks or find JSON object
        let json_str = if response.contains("```json") {
            response
                .split("```json")
                .nth(1)
                .and_then(|s| s.split("```").next())
                .unwrap_or(response)
                .trim()
        } else if response.contains("```") {
            response
                .split("```")
                .nth(1)
                .and_then(|s| s.split("```").next())
                .unwrap_or(response)
                .trim()
        } else if let Some(start) = response.find('{') {
            // Find the JSON object by looking for the first { and last }
            if let Some(end) = response.rfind('}') {
                if end > start {
                    &response[start..=end]
                } else {
                    response.trim()
                }
            } else {
                response.trim()
            }
        } else {
            response.trim()
        };

        // Log the JSON string for debugging
        log::debug!("Attempting to parse JSON: {}", json_str);

        #[derive(Deserialize)]
        struct ApiResponse {
            explanation: String,
            root_cause: Option<String>,
            suggestions: Vec<ApiSuggestion>,
        }

        #[derive(Deserialize)]
        struct ApiSuggestion {
            description: String,
            code_example: Option<String>,
            priority: u8,
        }

        // Try to parse the JSON
        match serde_json::from_str::<ApiResponse>(json_str) {
            Ok(parsed) => Ok(ErrorAnalysis {
                explanation: parsed.explanation,
                root_cause: parsed.root_cause,
                suggestions: parsed
                    .suggestions
                    .into_iter()
                    .map(|s| Suggestion {
                        description: s.description,
                        code_example: s.code_example,
                        priority: s.priority,
                    })
                    .collect(),
                related_resources: vec![],
                tool_invocations: vec![],
            }),
            Err(e) => {
                // If JSON parsing fails, try to extract partial information
                log::warn!(
                    "Failed to parse complete JSON, attempting partial extraction: {}",
                    e
                );

                // Try to extract explanation at least
                let explanation = if let Some(exp_start) = json_str.find("\"explanation\"") {
                    json_str[exp_start..]
                        .split("\"explanation\"")
                        .nth(1)
                        .and_then(|s| s.split(':').nth(1))
                        .and_then(|s| s.split('"').nth(1))
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| {
                            "Unable to parse error analysis from AI response".to_string()
                        })
                } else {
                    "Unable to parse error analysis from AI response".to_string()
                };

                // Return a basic analysis with whatever we could extract
                Ok(ErrorAnalysis {
                    explanation,
                    root_cause: None,
                    suggestions: vec![],
                    related_resources: vec![],
                    tool_invocations: vec![],
                })
            }
        }
    }
}

#[async_trait]
impl AIProvider for OllamaProvider {
    async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis> {
        let prompt = build_analysis_prompt(group);
        let response = self.call_api(prompt).await?;
        self.parse_response(&response)
    }

    async fn analyze_with_tools(
        &self,
        group: &ErrorGroup,
        mcp_client: Option<&crate::mcp::MCPClient>,
    ) -> Result<ErrorAnalysis> {
        // If no MCP client, fall back to regular analysis
        let Some(client) = mcp_client else {
            return self.analyze(group).await;
        };

        // Build base prompt
        let base_prompt = build_analysis_prompt(group);

        // Invoke relevant MCP tools and collect results
        let tool_results = crate::ai::mcp_helper::invoke_relevant_tools(client, group).await;

        // Augment prompt with tool results
        let prompt = crate::ai::mcp_helper::augment_prompt_with_tools(base_prompt, &tool_results);

        // Call AI with enhanced prompt
        let response = self.call_api(prompt).await?;
        let mut analysis = self.parse_response(&response)?;

        // Add tool invocation summaries
        analysis.tool_invocations = tool_results
            .into_iter()
            .map(|r| crate::mcp::ToolInvocationSummary {
                tool: r.tool_name,
                status: if r.result.success {
                    "success".to_string()
                } else {
                    "failure".to_string()
                },
                duration_ms: r.duration_ms,
                contributed_to: vec![],
            })
            .collect();

        Ok(analysis)
    }

    fn name(&self) -> &str {
        "ollama"
    }
}

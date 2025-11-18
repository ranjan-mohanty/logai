use crate::ai::prompts::build_analysis_prompt;
use crate::ai::provider::AIProvider;
use crate::types::{ErrorAnalysis, ErrorGroup, Suggestion};
use crate::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: model.unwrap_or_else(|| "gpt-4o-mini".to_string()),
        }
    }

    async fn call_api(&self, prompt: String) -> Result<String> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.3,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow!("OpenAI API error {}: {}", status, error_text));
        }

        let openai_response: OpenAIResponse = response.json().await?;

        openai_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow!("No response from OpenAI"))
    }

    fn parse_response(&self, response: &str) -> Result<ErrorAnalysis> {
        // Try to extract JSON from markdown code blocks
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
        } else {
            response.trim()
        };

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

        let parsed: ApiResponse = serde_json::from_str(json_str)?;

        Ok(ErrorAnalysis {
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
        })
    }
}

#[async_trait]
impl AIProvider for OpenAIProvider {
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
                contributed_to: vec![], // TODO: Track which suggestions came from which tools
            })
            .collect();

        Ok(analysis)
    }

    fn name(&self) -> &str {
        "openai"
    }
}

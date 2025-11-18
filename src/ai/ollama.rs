use super::prompts::build_analysis_prompt;
use super::provider::AIProvider;
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

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow!("Ollama API error {}: {}", status, error_text));
        }

        let ollama_response: OllamaResponse = response.json().await?;
        Ok(ollama_response.response)
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
        })
    }
}

#[async_trait]
impl AIProvider for OllamaProvider {
    async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis> {
        let prompt = build_analysis_prompt(group);
        let response = self.call_api(prompt).await?;
        self.parse_response(&response)
    }

    fn name(&self) -> &str {
        "ollama"
    }
}

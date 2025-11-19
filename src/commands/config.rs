//! Config command implementation.
//!
//! This module contains the business logic for the `config` command,
//! which manages LogAI configuration.

use crate::{ai::AIConfig, cli::ConfigAction, Result};

/// Config command implementation
pub struct ConfigCommand;

impl ConfigCommand {
    /// Execute the config command
    pub fn execute(action: ConfigAction) -> Result<()> {
        match action {
            ConfigAction::Show => Self::show_config(),
            ConfigAction::Set { key, value } => Self::set_config(&key, &value),
        }
    }

    fn show_config() -> Result<()> {
        let config = AIConfig::load()?;
        let config_path = AIConfig::config_path()?;

        println!("📝 LogAI Configuration");
        println!("   Location: {}\n", config_path.display());

        // Display MCP settings
        println!("MCP Settings:");
        println!("  enabled: {}", config.mcp.enabled);
        if let Some(path) = &config.mcp.config_path {
            println!("  config_path: {}", path);
        }
        println!("  default_timeout: {}s\n", config.mcp.default_timeout);

        if let Some(provider) = &config.ai.provider {
            println!("Default Provider: {}", provider);
        }

        if config.providers.is_empty() {
            Self::show_example_config();
        } else {
            Self::show_providers(&config);
        }

        Ok(())
    }

    fn show_example_config() {
        println!("\nNo providers configured.");
        println!("\nExample configuration:");
        println!("  logai config set ai.provider bedrock");
        println!("  logai config set ollama.model llama3.1:8b");
        println!("  logai config set openai.api_key sk-...");
        println!("  logai config set bedrock.region us-east-1");
        println!("  logai config set output.path ./reports");
        println!("  logai config set output.format html");
        println!("  logai config set mcp.enabled true");
        println!("  logai config set mcp.config_path ~/.logai/mcp.toml");
    }

    fn show_providers(config: &AIConfig) {
        println!("\nProviders:");
        for (name, provider) in &config.providers {
            println!("\n  [{}]", name);
            if let Some(model) = &provider.model {
                println!("    model: {}", model);
            }
            if let Some(host) = &provider.host {
                println!("    host: {}", host);
            }
            if let Some(region) = &provider.region {
                println!("    region: {}", region);
            }
            if let Some(max_tokens) = provider.max_tokens {
                println!("    max_tokens: {}", max_tokens);
            }
            if let Some(temperature) = provider.temperature {
                println!("    temperature: {}", temperature);
            }
            if provider.api_key.is_some() {
                println!("    api_key: ***");
            }
            println!("    enabled: {}", provider.enabled);
        }
    }

    fn set_config(key: &str, value: &str) -> Result<()> {
        let mut config = AIConfig::load().unwrap_or_default();
        config.set_value(key, value)?;
        config.save()?;

        println!("✅ Configuration updated: {} = {}", key, value);
        println!("   Saved to: {}", AIConfig::config_path()?.display());

        Ok(())
    }
}

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

        // Serialize config to TOML for display
        let toml_str = toml::to_string_pretty(&config)
            .map_err(|e| anyhow::anyhow!("Failed to serialize config: {}", e))?;

        // Mask API keys in the output
        let masked_toml = Self::mask_api_keys(&toml_str);

        println!("{}", masked_toml);

        if config.providers.is_empty() {
            println!("\n💡 Example configuration:");
            println!("  logai config set ai.provider bedrock");
            println!("  logai config set ollama.model llama3.1:8b");
            println!("  logai config set openai.api_key sk-...");
            println!("  logai config set bedrock.region us-east-1");
            println!("  logai config set output.path ./reports");
            println!("  logai config set output.format html");
            println!("  logai config set output.logs_dir ./logs");
            println!("  logai config set analysis.max_concurrency 10");
        }

        Ok(())
    }

    fn mask_api_keys(toml: &str) -> String {
        // Replace API key values with ***
        let re = regex::Regex::new(r#"api_key = "([^"]+)""#).unwrap();
        re.replace_all(toml, r#"api_key = "***""#).to_string()
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

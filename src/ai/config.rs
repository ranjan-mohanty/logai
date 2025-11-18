use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AIConfig {
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
    #[serde(default)]
    pub default_provider: Option<String>,
    #[serde(default)]
    pub mcp: MCPSettings,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MCPSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub config_path: Option<String>,
    #[serde(default = "default_timeout")]
    pub default_timeout: u64,
}

fn default_timeout() -> u64 {
    30
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub host: Option<String>,
    #[serde(default)]
    pub enabled: bool,
}

impl AIConfig {
    /// Load config from ~/.logai/config.toml
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: AIConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save config to ~/.logai/config.toml
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    /// Get config file path
    pub fn config_path() -> Result<PathBuf> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;
        Ok(PathBuf::from(home).join(".logai").join("config.toml"))
    }

    /// Get provider config by name
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.providers.get(name)
    }

    /// Set provider config
    pub fn set_provider(&mut self, name: String, config: ProviderConfig) {
        self.providers.insert(name, config);
    }

    /// Set a configuration value by key path (e.g., "mcp.enabled", "openai.model")
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        let parts: Vec<&str> = key.split('.').collect();

        match parts.as_slice() {
            ["mcp", "enabled"] => {
                self.mcp.enabled = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value: {}", value))?;
            }
            ["mcp", "config_path"] => {
                self.mcp.config_path = Some(value.to_string());
            }
            ["mcp", "default_timeout"] => {
                self.mcp.default_timeout = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid timeout value: {}", value))?;
            }
            [provider, "api_key"] => {
                let config = self
                    .providers
                    .entry(provider.to_string())
                    .or_insert_with(|| ProviderConfig {
                        api_key: None,
                        model: None,
                        host: None,
                        enabled: false,
                    });
                config.api_key = Some(value.to_string());
            }
            [provider, "model"] => {
                let config = self
                    .providers
                    .entry(provider.to_string())
                    .or_insert_with(|| ProviderConfig {
                        api_key: None,
                        model: None,
                        host: None,
                        enabled: false,
                    });
                config.model = Some(value.to_string());
            }
            [provider, "host"] => {
                let config = self
                    .providers
                    .entry(provider.to_string())
                    .or_insert_with(|| ProviderConfig {
                        api_key: None,
                        model: None,
                        host: None,
                        enabled: false,
                    });
                config.host = Some(value.to_string());
            }
            [provider, "enabled"] => {
                let config = self
                    .providers
                    .entry(provider.to_string())
                    .or_insert_with(|| ProviderConfig {
                        api_key: None,
                        model: None,
                        host: None,
                        enabled: false,
                    });
                config.enabled = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value: {}", value))?;
            }
            ["default_provider"] => {
                self.default_provider = Some(value.to_string());
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown configuration key: {}", key));
            }
        }

        Ok(())
    }

    /// Display configuration in a readable format
    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str("LogAI Configuration\n");
        output.push_str("===================\n\n");

        // Default provider
        if let Some(provider) = &self.default_provider {
            output.push_str(&format!("Default Provider: {}\n\n", provider));
        }

        // MCP settings
        output.push_str("MCP Settings:\n");
        output.push_str(&format!("  enabled: {}\n", self.mcp.enabled));
        if let Some(path) = &self.mcp.config_path {
            output.push_str(&format!("  config_path: {}\n", path));
        }
        output.push_str(&format!(
            "  default_timeout: {}s\n\n",
            self.mcp.default_timeout
        ));

        // Providers
        if !self.providers.is_empty() {
            output.push_str("AI Providers:\n");
            for (name, config) in &self.providers {
                output.push_str(&format!("  {}:\n", name));
                output.push_str(&format!("    enabled: {}\n", config.enabled));
                if let Some(model) = &config.model {
                    output.push_str(&format!("    model: {}\n", model));
                }
                if let Some(host) = &config.host {
                    output.push_str(&format!("    host: {}\n", host));
                }
                if config.api_key.is_some() {
                    output.push_str("    api_key: [set]\n");
                }
                output.push('\n');
            }
        }

        output
    }
}

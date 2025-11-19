//! Configuration management for AI providers and analysis settings.
//!
//! This module handles loading, saving, and managing configuration for AI providers,
//! MCP settings, and analysis parameters. Configuration is stored in `~/.logai/config.toml`.
//!
//! # Example
//!
//! ```no_run
//! use logai::ai::AIConfig;
//!
//! # fn example() -> anyhow::Result<()> {
//! // Load configuration from file
//! let mut config = AIConfig::load()?;
//!
//! // Set a configuration value
//! config.set_value("analysis.max_concurrency", "10")?;
//!
//! // Save configuration
//! config.save()?;
//!
//! // Get analysis configuration
//! let analysis_config = config.get_analysis_config();
//! # Ok(())
//! # }
//! ```

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
    pub ai: AISettings,
    #[serde(default)]
    pub mcp: MCPSettings,
    #[serde(default)]
    pub analysis: AnalysisSettings,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AISettings {
    #[serde(default)]
    pub provider: Option<String>,
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
pub struct AnalysisSettings {
    #[serde(default = "default_max_concurrency")]
    pub max_concurrency: usize,
    #[serde(default = "default_enable_retry")]
    pub enable_retry: bool,
    #[serde(default = "default_max_retries")]
    pub max_retries: usize,
    #[serde(default = "default_initial_backoff_ms")]
    pub initial_backoff_ms: u64,
    #[serde(default = "default_max_backoff_ms")]
    pub max_backoff_ms: u64,
    #[serde(default = "default_enable_cache")]
    pub enable_cache: bool,
    #[serde(default = "default_truncate_length")]
    pub truncate_length: usize,
}

impl Default for AnalysisSettings {
    fn default() -> Self {
        Self {
            max_concurrency: default_max_concurrency(),
            enable_retry: default_enable_retry(),
            max_retries: default_max_retries(),
            initial_backoff_ms: default_initial_backoff_ms(),
            max_backoff_ms: default_max_backoff_ms(),
            enable_cache: default_enable_cache(),
            truncate_length: default_truncate_length(),
        }
    }
}

fn default_max_concurrency() -> usize {
    5
}

fn default_enable_retry() -> bool {
    true
}

fn default_max_retries() -> usize {
    3
}

fn default_initial_backoff_ms() -> u64 {
    1000
}

fn default_max_backoff_ms() -> u64 {
    30000
}

fn default_enable_cache() -> bool {
    true
}

fn default_truncate_length() -> usize {
    2000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub host: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    // Bedrock-specific fields
    pub region: Option<String>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
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

    /// Set a configuration value by key path (e.g., "mcp.enabled", "openai.model", "analysis.max_concurrency")
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
            ["analysis", "max_concurrency"] => {
                self.analysis.max_concurrency = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid concurrency value: {}", value))?;
            }
            ["analysis", "enable_retry"] => {
                self.analysis.enable_retry = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value: {}", value))?;
            }
            ["analysis", "max_retries"] => {
                self.analysis.max_retries = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid retries value: {}", value))?;
            }
            ["analysis", "initial_backoff_ms"] => {
                self.analysis.initial_backoff_ms = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid backoff value: {}", value))?;
            }
            ["analysis", "max_backoff_ms"] => {
                self.analysis.max_backoff_ms = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid max backoff value: {}", value))?;
            }
            ["analysis", "enable_cache"] => {
                self.analysis.enable_cache = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value: {}", value))?;
            }
            ["analysis", "truncate_length"] => {
                self.analysis.truncate_length = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid truncate length value: {}", value))?;
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
                        region: None,
                        max_tokens: None,
                        temperature: None,
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
                        region: None,
                        max_tokens: None,
                        temperature: None,
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
                        region: None,
                        max_tokens: None,
                        temperature: None,
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
                        region: None,
                        max_tokens: None,
                        temperature: None,
                    });
                config.enabled = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid boolean value: {}", value))?;
            }
            [provider, "region"] => {
                let config = self
                    .providers
                    .entry(provider.to_string())
                    .or_insert_with(|| ProviderConfig {
                        api_key: None,
                        model: None,
                        host: None,
                        enabled: false,
                        region: None,
                        max_tokens: None,
                        temperature: None,
                    });
                config.region = Some(value.to_string());
            }
            [provider, "max_tokens"] => {
                let config = self
                    .providers
                    .entry(provider.to_string())
                    .or_insert_with(|| ProviderConfig {
                        api_key: None,
                        model: None,
                        host: None,
                        enabled: false,
                        region: None,
                        max_tokens: None,
                        temperature: None,
                    });
                config.max_tokens = Some(
                    value
                        .parse()
                        .map_err(|_| anyhow::anyhow!("Invalid max_tokens value: {}", value))?,
                );
            }
            [provider, "temperature"] => {
                let config = self
                    .providers
                    .entry(provider.to_string())
                    .or_insert_with(|| ProviderConfig {
                        api_key: None,
                        model: None,
                        host: None,
                        enabled: false,
                        region: None,
                        max_tokens: None,
                        temperature: None,
                    });
                config.temperature = Some(
                    value
                        .parse()
                        .map_err(|_| anyhow::anyhow!("Invalid temperature value: {}", value))?,
                );
            }
            ["ai", "provider"] => {
                self.ai.provider = Some(value.to_string());
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown configuration key: {}", key));
            }
        }

        Ok(())
    }

    /// Get analysis configuration
    pub fn get_analysis_config(&self) -> crate::ai::parallel::AnalysisConfig {
        crate::ai::parallel::AnalysisConfig {
            max_concurrency: self.analysis.max_concurrency,
            enable_retry: self.analysis.enable_retry,
            max_retries: self.analysis.max_retries,
            initial_backoff_ms: self.analysis.initial_backoff_ms,
            max_backoff_ms: self.analysis.max_backoff_ms,
            enable_cache: self.analysis.enable_cache,
            truncate_length: self.analysis.truncate_length,
        }
    }

    /// Display configuration in a readable format
    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str("LogAI Configuration\n");
        output.push_str("===================\n\n");

        // AI Settings
        output.push_str("AI Settings:\n");
        if let Some(provider) = &self.ai.provider {
            output.push_str(&format!("  provider: {}\n", provider));
        }
        output.push('\n');

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

        // Analysis settings
        output.push_str("Analysis Settings:\n");
        output.push_str(&format!(
            "  max_concurrency: {}\n",
            self.analysis.max_concurrency
        ));
        output.push_str(&format!("  enable_retry: {}\n", self.analysis.enable_retry));
        output.push_str(&format!("  max_retries: {}\n", self.analysis.max_retries));
        output.push_str(&format!(
            "  initial_backoff_ms: {}\n",
            self.analysis.initial_backoff_ms
        ));
        output.push_str(&format!(
            "  max_backoff_ms: {}\n",
            self.analysis.max_backoff_ms
        ));
        output.push_str(&format!("  enable_cache: {}\n", self.analysis.enable_cache));
        output.push_str(&format!(
            "  truncate_length: {}\n\n",
            self.analysis.truncate_length
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_analysis_settings() {
        let settings = AnalysisSettings::default();
        assert_eq!(settings.max_concurrency, 5);
        assert!(settings.enable_retry);
        assert_eq!(settings.max_retries, 3);
        assert_eq!(settings.initial_backoff_ms, 1000);
        assert_eq!(settings.max_backoff_ms, 30000);
        assert!(settings.enable_cache);
        assert_eq!(settings.truncate_length, 2000);
    }

    #[test]
    fn test_set_analysis_values() {
        let mut config = AIConfig::default();

        config.set_value("analysis.max_concurrency", "10").unwrap();
        assert_eq!(config.analysis.max_concurrency, 10);

        config.set_value("analysis.enable_retry", "false").unwrap();
        assert!(!config.analysis.enable_retry);

        config.set_value("analysis.max_retries", "5").unwrap();
        assert_eq!(config.analysis.max_retries, 5);

        config
            .set_value("analysis.initial_backoff_ms", "2000")
            .unwrap();
        assert_eq!(config.analysis.initial_backoff_ms, 2000);

        config
            .set_value("analysis.max_backoff_ms", "60000")
            .unwrap();
        assert_eq!(config.analysis.max_backoff_ms, 60000);

        config.set_value("analysis.enable_cache", "false").unwrap();
        assert!(!config.analysis.enable_cache);

        config
            .set_value("analysis.truncate_length", "1000")
            .unwrap();
        assert_eq!(config.analysis.truncate_length, 1000);
    }

    #[test]
    fn test_invalid_analysis_values() {
        let mut config = AIConfig::default();

        assert!(config
            .set_value("analysis.max_concurrency", "invalid")
            .is_err());
        assert!(config
            .set_value("analysis.enable_retry", "invalid")
            .is_err());
        assert!(config.set_value("analysis.max_retries", "invalid").is_err());
    }

    #[test]
    fn test_get_analysis_config() {
        let mut config = AIConfig::default();
        config.analysis.max_concurrency = 10;
        config.analysis.enable_retry = false;
        config.analysis.max_retries = 5;

        let analysis_config = config.get_analysis_config();
        assert_eq!(analysis_config.max_concurrency, 10);
        assert!(!analysis_config.enable_retry);
        assert_eq!(analysis_config.max_retries, 5);
    }

    #[test]
    fn test_set_ai_provider() {
        let mut config = AIConfig::default();

        // Test setting via ai.provider
        config.set_value("ai.provider", "bedrock").unwrap();
        assert_eq!(config.ai.provider, Some("bedrock".to_string()));

        // Test changing provider
        config.set_value("ai.provider", "ollama").unwrap();
        assert_eq!(config.ai.provider, Some("ollama".to_string()));
    }

    #[test]
    fn test_display_includes_analysis_settings() {
        let config = AIConfig::default();
        let display = config.display();

        assert!(display.contains("Analysis Settings:"));
        assert!(display.contains("max_concurrency: 5"));
        assert!(display.contains("enable_retry: true"));
        assert!(display.contains("max_retries: 3"));
    }
}

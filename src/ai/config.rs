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
}

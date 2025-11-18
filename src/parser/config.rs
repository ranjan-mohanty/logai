use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for log parser behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    // Format detection
    /// Automatically detect log format from content
    pub auto_detect: bool,
    /// Default format to use if auto-detection fails
    pub default_format: Option<String>,

    // Multi-line handling
    /// Enable multi-line log entry handling (e.g., stack traces)
    pub enable_multiline: bool,
    /// Timeout in milliseconds for multi-line entry completion
    pub multiline_timeout_ms: u64,
    /// Maximum number of lines in a single multi-line entry
    pub max_multiline_lines: usize,

    // Performance
    /// Enable parallel parsing for large files
    pub parallel_parsing: bool,
    /// Number of lines to process in each chunk
    pub chunk_size: usize,
    /// Number of threads to use (None = use system default)
    pub num_threads: Option<usize>,
    /// File size threshold in MB for streaming mode
    pub streaming_threshold_mb: usize,

    // Timestamp parsing
    /// Custom timestamp format patterns
    pub custom_timestamp_formats: Vec<String>,
    /// Use current time as fallback when timestamp parsing fails
    pub fallback_to_current_time: bool,

    // Metadata extraction
    /// Enable metadata extraction from log entries
    pub extract_metadata: bool,
    /// Custom regex patterns for metadata extraction (field_name -> pattern)
    pub custom_metadata_patterns: HashMap<String, String>,

    // Error handling
    /// Skip invalid lines instead of failing
    pub skip_invalid_lines: bool,
    /// Maximum number of parse errors before stopping (None = unlimited)
    pub max_parse_errors: Option<usize>,
    /// Attempt UTF-8 recovery for encoding errors
    pub utf8_recovery: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            auto_detect: true,
            default_format: None,
            enable_multiline: true,
            multiline_timeout_ms: 5000,
            max_multiline_lines: 100,
            parallel_parsing: true,
            chunk_size: 1000,
            num_threads: None,
            streaming_threshold_mb: 100,
            custom_timestamp_formats: vec![],
            fallback_to_current_time: true,
            extract_metadata: true,
            custom_metadata_patterns: HashMap::new(),
            skip_invalid_lines: true,
            max_parse_errors: None,
            utf8_recovery: true,
        }
    }
}

impl ParserConfig {
    /// Create a new parser configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from a TOML file
    pub fn from_file(path: &std::path::Path) -> crate::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn to_file(&self, path: &std::path::Path) -> crate::Result<()> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Validate configuration values
    pub fn validate(&self) -> crate::Result<()> {
        if self.chunk_size == 0 {
            return Err(anyhow::anyhow!("chunk_size must be greater than 0"));
        }
        if self.max_multiline_lines == 0 {
            return Err(anyhow::anyhow!(
                "max_multiline_lines must be greater than 0"
            ));
        }
        if self.streaming_threshold_mb == 0 {
            return Err(anyhow::anyhow!(
                "streaming_threshold_mb must be greater than 0"
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ParserConfig::default();
        assert!(config.auto_detect);
        assert!(config.enable_multiline);
        assert!(config.parallel_parsing);
        assert_eq!(config.chunk_size, 1000);
    }

    #[test]
    fn test_validate_config() {
        let mut config = ParserConfig::default();
        assert!(config.validate().is_ok());

        config.chunk_size = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_custom_config() {
        let config = ParserConfig {
            chunk_size: 500,
            parallel_parsing: false,
            enable_multiline: false,
            ..Default::default()
        };

        assert_eq!(config.chunk_size, 500);
        assert!(!config.parallel_parsing);
        assert!(!config.enable_multiline);
    }
}

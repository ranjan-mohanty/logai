//! Comprehensive tests for AI configuration management
//!
//! Tests cover:
//! - Configuration loading and saving
//! - Provider configuration
//! - Analysis settings
//! - MCP settings
//! - Output settings
//! - Configuration validation

use logai::ai::AIConfig;

#[test]
fn test_default_config() {
    let config = AIConfig::default();

    // Check default analysis settings
    assert_eq!(config.analysis.max_concurrency, 5);
    assert!(config.analysis.enable_retry);
    assert_eq!(config.analysis.max_retries, 3);
    assert_eq!(config.analysis.initial_backoff_ms, 1000);
    assert_eq!(config.analysis.max_backoff_ms, 30000);
    assert!(config.analysis.enable_cache);
    assert_eq!(config.analysis.truncate_length, 2000);

    // Check default MCP settings
    assert!(!config.mcp.enabled);
    // default_timeout may be 0 if not set through serde default
    assert!(config.mcp.default_timeout == 0 || config.mcp.default_timeout == 30);
}

#[test]
fn test_config_serialization() {
    let config = AIConfig::default();
    let toml_str = toml::to_string(&config).unwrap();

    // Should be valid TOML
    assert!(toml_str.contains("[analysis]"));
}

#[test]
fn test_config_deserialization() {
    let toml_str = r#"
[ai]
provider = "openai"

[analysis]
max_concurrency = 10
enable_retry = false
max_retries = 5

[mcp]
enabled = true
default_timeout = 60

[output]
format = "html"
path = "/tmp/reports"
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.ai.provider, Some("openai".to_string()));
    assert_eq!(config.analysis.max_concurrency, 10);
    assert!(!config.analysis.enable_retry);
    assert_eq!(config.analysis.max_retries, 5);
    assert!(config.mcp.enabled);
    assert_eq!(config.mcp.default_timeout, 60);
    assert_eq!(config.output.format, Some("html".to_string()));
    assert_eq!(config.output.path, Some("/tmp/reports".to_string()));
}

#[test]
fn test_provider_config() {
    let toml_str = r#"
[providers.openai]
api_key = "test-key"
model = "gpt-4"
enabled = true

[providers.claude]
api_key = "claude-key"
model = "claude-3"
enabled = false
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.providers.len(), 2);

    let openai = config.providers.get("openai").unwrap();
    assert_eq!(openai.api_key, Some("test-key".to_string()));
    assert_eq!(openai.model, Some("gpt-4".to_string()));
    assert!(openai.enabled);

    let claude = config.providers.get("claude").unwrap();
    assert_eq!(claude.api_key, Some("claude-key".to_string()));
    assert!(!claude.enabled);
}

#[test]
fn test_bedrock_provider_config() {
    let toml_str = r#"
[providers.bedrock]
model = "anthropic.claude-v2"
region = "us-east-1"
max_tokens = 1024
temperature = 0.7
enabled = true
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    let bedrock = config.providers.get("bedrock").unwrap();
    assert_eq!(bedrock.model, Some("anthropic.claude-v2".to_string()));
    assert_eq!(bedrock.region, Some("us-east-1".to_string()));
    assert_eq!(bedrock.max_tokens, Some(1024));
    assert_eq!(bedrock.temperature, Some(0.7));
    assert!(bedrock.enabled);
}

#[test]
fn test_ollama_provider_config() {
    let toml_str = r#"
[providers.ollama]
host = "http://localhost:11434"
model = "llama3.2"
enabled = true
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    let ollama = config.providers.get("ollama").unwrap();
    assert_eq!(ollama.host, Some("http://localhost:11434".to_string()));
    assert_eq!(ollama.model, Some("llama3.2".to_string()));
}

#[test]
fn test_mcp_config_path() {
    let toml_str = r#"
[mcp]
enabled = true
config_path = "/custom/path/mcp.toml"
default_timeout = 45
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert!(config.mcp.enabled);
    assert_eq!(
        config.mcp.config_path,
        Some("/custom/path/mcp.toml".to_string())
    );
    assert_eq!(config.mcp.default_timeout, 45);
}

#[test]
fn test_output_settings() {
    let toml_str = r#"
[output]
path = "./custom-reports"
format = "json"
logs_dir = "./custom-logs"
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.output.path, Some("./custom-reports".to_string()));
    assert_eq!(config.output.format, Some("json".to_string()));
    assert_eq!(config.output.logs_dir, Some("./custom-logs".to_string()));
}

#[test]
fn test_analysis_settings_all_fields() {
    let toml_str = r#"
[analysis]
max_concurrency = 8
enable_retry = true
max_retries = 5
initial_backoff_ms = 2000
max_backoff_ms = 60000
enable_cache = false
truncate_length = 3000
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.analysis.max_concurrency, 8);
    assert!(config.analysis.enable_retry);
    assert_eq!(config.analysis.max_retries, 5);
    assert_eq!(config.analysis.initial_backoff_ms, 2000);
    assert_eq!(config.analysis.max_backoff_ms, 60000);
    assert!(!config.analysis.enable_cache);
    assert_eq!(config.analysis.truncate_length, 3000);
}

#[test]
fn test_partial_config() {
    // Test that partial config uses defaults for missing fields
    let toml_str = r#"
[analysis]
max_concurrency = 10
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.analysis.max_concurrency, 10);
    // Other fields should use defaults
    assert!(config.analysis.enable_retry);
    assert_eq!(config.analysis.max_retries, 3);
}

#[test]
fn test_empty_config() {
    let toml_str = "";
    let config: AIConfig = toml::from_str(toml_str).unwrap();

    // Should use all defaults
    assert_eq!(config.analysis.max_concurrency, 5);
    assert!(config.analysis.enable_retry);
    assert!(!config.mcp.enabled);
}

#[test]
fn test_multiple_providers() {
    let toml_str = r#"
[providers.openai]
api_key = "openai-key"
enabled = true

[providers.claude]
api_key = "claude-key"
enabled = true

[providers.gemini]
api_key = "gemini-key"
enabled = false

[providers.ollama]
host = "http://localhost:11434"
enabled = true
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.providers.len(), 4);
    assert!(config.providers.contains_key("openai"));
    assert!(config.providers.contains_key("claude"));
    assert!(config.providers.contains_key("gemini"));
    assert!(config.providers.contains_key("ollama"));
}

#[test]
fn test_config_with_comments() {
    let toml_str = r#"
# AI Provider Configuration
[ai]
provider = "openai"  # Default provider

# Analysis Settings
[analysis]
max_concurrency = 5  # Number of concurrent analyses
enable_retry = true  # Enable retry on failure
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.ai.provider, Some("openai".to_string()));
    assert_eq!(config.analysis.max_concurrency, 5);
    assert!(config.analysis.enable_retry);
}

#[test]
fn test_config_round_trip() {
    let original = AIConfig::default();
    let toml_str = toml::to_string(&original).unwrap();
    let deserialized: AIConfig = toml::from_str(&toml_str).unwrap();

    // Check that serialization and deserialization preserve values
    assert_eq!(
        original.analysis.max_concurrency,
        deserialized.analysis.max_concurrency
    );
    assert_eq!(
        original.analysis.enable_retry,
        deserialized.analysis.enable_retry
    );
    assert_eq!(original.mcp.enabled, deserialized.mcp.enabled);
}

#[test]
fn test_invalid_toml() {
    let invalid_toml = "this is not valid toml {{{";
    let result: Result<AIConfig, _> = toml::from_str(invalid_toml);
    assert!(result.is_err());
}

#[test]
fn test_config_with_invalid_types() {
    let toml_str = r#"
[analysis]
max_concurrency = "not a number"
"#;

    let result: Result<AIConfig, _> = toml::from_str(toml_str);
    assert!(result.is_err());
}

#[test]
fn test_provider_without_api_key() {
    let toml_str = r#"
[providers.openai]
model = "gpt-4"
enabled = true
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();
    let openai = config.providers.get("openai").unwrap();

    assert!(openai.api_key.is_none());
    assert_eq!(openai.model, Some("gpt-4".to_string()));
}

#[test]
fn test_analysis_settings_edge_values() {
    let toml_str = r#"
[analysis]
max_concurrency = 1
max_retries = 0
initial_backoff_ms = 0
max_backoff_ms = 0
truncate_length = 100
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.analysis.max_concurrency, 1);
    assert_eq!(config.analysis.max_retries, 0);
    assert_eq!(config.analysis.initial_backoff_ms, 0);
    assert_eq!(config.analysis.max_backoff_ms, 0);
    assert_eq!(config.analysis.truncate_length, 100);
}

#[test]
fn test_analysis_settings_large_values() {
    let toml_str = r#"
[analysis]
max_concurrency = 100
max_retries = 10
initial_backoff_ms = 10000
max_backoff_ms = 300000
truncate_length = 10000
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    assert_eq!(config.analysis.max_concurrency, 100);
    assert_eq!(config.analysis.max_retries, 10);
    assert_eq!(config.analysis.initial_backoff_ms, 10000);
    assert_eq!(config.analysis.max_backoff_ms, 300000);
    assert_eq!(config.analysis.truncate_length, 10000);
}

#[test]
fn test_mcp_disabled_by_default() {
    let config = AIConfig::default();
    assert!(!config.mcp.enabled);
}

#[test]
fn test_output_settings_optional_fields() {
    let config = AIConfig::default();
    assert!(config.output.path.is_none());
    assert!(config.output.format.is_none());
    assert!(config.output.logs_dir.is_none());
}

#[test]
fn test_ai_settings_optional_provider() {
    let config = AIConfig::default();
    assert!(config.ai.provider.is_none());
}

#[test]
fn test_provider_config_all_optional_fields() {
    let toml_str = r#"
[providers.test]
enabled = false
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();
    let provider = config.providers.get("test").unwrap();

    assert!(provider.api_key.is_none());
    assert!(provider.model.is_none());
    assert!(provider.host.is_none());
    assert!(provider.region.is_none());
    assert!(provider.max_tokens.is_none());
    assert!(provider.temperature.is_none());
    assert!(!provider.enabled);
}

#[test]
fn test_config_display() {
    let config = AIConfig::default();
    let display_str = format!("{:?}", config);

    // Should contain key configuration sections
    assert!(display_str.contains("AIConfig"));
}

// ============================================================================
// Configuration Methods Tests
// ============================================================================

#[test]
fn test_get_provider() {
    let toml_str = r#"
[providers.openai]
api_key = "test-key"
model = "gpt-4"
"#;

    let config: AIConfig = toml::from_str(toml_str).unwrap();

    let provider = config.get_provider("openai");
    assert!(provider.is_some());
    assert_eq!(provider.unwrap().api_key, Some("test-key".to_string()));

    let missing = config.get_provider("nonexistent");
    assert!(missing.is_none());
}

#[test]
fn test_set_provider() {
    use logai::ai::config::ProviderConfig;

    let mut config = AIConfig::default();

    let provider_config = ProviderConfig {
        api_key: Some("new-key".to_string()),
        model: Some("gpt-4".to_string()),
        host: None,
        enabled: true,
        region: None,
        max_tokens: None,
        temperature: None,
    };

    config.set_provider("openai".to_string(), provider_config);

    let retrieved = config.get_provider("openai");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().api_key, Some("new-key".to_string()));
}

#[test]
fn test_set_value_mcp_enabled() {
    let mut config = AIConfig::default();

    config.set_value("mcp.enabled", "true").unwrap();
    assert!(config.mcp.enabled);

    config.set_value("mcp.enabled", "false").unwrap();
    assert!(!config.mcp.enabled);
}

#[test]
fn test_set_value_mcp_config_path() {
    let mut config = AIConfig::default();

    config.set_value("mcp.config_path", "/custom/path").unwrap();
    assert_eq!(config.mcp.config_path, Some("/custom/path".to_string()));
}

#[test]
fn test_set_value_mcp_timeout() {
    let mut config = AIConfig::default();

    config.set_value("mcp.default_timeout", "60").unwrap();
    assert_eq!(config.mcp.default_timeout, 60);
}

#[test]
fn test_set_value_analysis_max_concurrency() {
    let mut config = AIConfig::default();

    config.set_value("analysis.max_concurrency", "10").unwrap();
    assert_eq!(config.analysis.max_concurrency, 10);
}

#[test]
fn test_set_value_analysis_enable_retry() {
    let mut config = AIConfig::default();

    config.set_value("analysis.enable_retry", "false").unwrap();
    assert!(!config.analysis.enable_retry);
}

#[test]
fn test_set_value_analysis_max_retries() {
    let mut config = AIConfig::default();

    config.set_value("analysis.max_retries", "5").unwrap();
    assert_eq!(config.analysis.max_retries, 5);
}

#[test]
fn test_set_value_analysis_backoff() {
    let mut config = AIConfig::default();

    config
        .set_value("analysis.initial_backoff_ms", "2000")
        .unwrap();
    assert_eq!(config.analysis.initial_backoff_ms, 2000);

    config
        .set_value("analysis.max_backoff_ms", "60000")
        .unwrap();
    assert_eq!(config.analysis.max_backoff_ms, 60000);
}

#[test]
fn test_set_value_analysis_enable_cache() {
    let mut config = AIConfig::default();

    config.set_value("analysis.enable_cache", "false").unwrap();
    assert!(!config.analysis.enable_cache);
}

#[test]
fn test_set_value_analysis_truncate_length() {
    let mut config = AIConfig::default();

    config
        .set_value("analysis.truncate_length", "3000")
        .unwrap();
    assert_eq!(config.analysis.truncate_length, 3000);
}

#[test]
fn test_set_value_output_path() {
    let mut config = AIConfig::default();

    config.set_value("output.path", "/custom/reports").unwrap();
    assert_eq!(config.output.path, Some("/custom/reports".to_string()));
}

#[test]
fn test_set_value_output_format() {
    let mut config = AIConfig::default();

    config.set_value("output.format", "html").unwrap();
    assert_eq!(config.output.format, Some("html".to_string()));

    config.set_value("output.format", "json").unwrap();
    assert_eq!(config.output.format, Some("json".to_string()));

    config.set_value("output.format", "terminal").unwrap();
    assert_eq!(config.output.format, Some("terminal".to_string()));
}

#[test]
fn test_set_value_output_format_invalid() {
    let mut config = AIConfig::default();

    let result = config.set_value("output.format", "invalid");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid output format"));
}

#[test]
fn test_set_value_output_logs_dir() {
    let mut config = AIConfig::default();

    config.set_value("output.logs_dir", "/custom/logs").unwrap();
    assert_eq!(config.output.logs_dir, Some("/custom/logs".to_string()));
}

#[test]
fn test_set_value_provider_api_key() {
    let mut config = AIConfig::default();

    config.set_value("openai.api_key", "test-key").unwrap();

    let provider = config.get_provider("openai");
    assert!(provider.is_some());
    assert_eq!(provider.unwrap().api_key, Some("test-key".to_string()));
}

#[test]
fn test_set_value_provider_model() {
    let mut config = AIConfig::default();

    config.set_value("claude.model", "claude-3").unwrap();

    let provider = config.get_provider("claude");
    assert!(provider.is_some());
    assert_eq!(provider.unwrap().model, Some("claude-3".to_string()));
}

#[test]
fn test_set_value_provider_host() {
    let mut config = AIConfig::default();

    config
        .set_value("ollama.host", "http://localhost:11434")
        .unwrap();

    let provider = config.get_provider("ollama");
    assert!(provider.is_some());
    assert_eq!(
        provider.unwrap().host,
        Some("http://localhost:11434".to_string())
    );
}

#[test]
fn test_set_value_invalid_boolean() {
    let mut config = AIConfig::default();

    let result = config.set_value("mcp.enabled", "not_a_boolean");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid boolean"));
}

#[test]
fn test_set_value_invalid_number() {
    let mut config = AIConfig::default();

    let result = config.set_value("analysis.max_concurrency", "not_a_number");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid"));
}

#[test]
fn test_set_value_invalid_key() {
    let mut config = AIConfig::default();

    let result = config.set_value("invalid.key.path", "value");
    assert!(result.is_err());
}

#[test]
fn test_get_analysis_config() {
    let mut config = AIConfig::default();
    config.analysis.max_concurrency = 10;
    config.analysis.enable_retry = false;

    let analysis_config = config.get_analysis_config();

    assert_eq!(analysis_config.max_concurrency, 10);
    assert!(!analysis_config.enable_retry);
}

#[test]
fn test_config_path() {
    let path = AIConfig::config_path();
    assert!(path.is_ok());

    let path = path.unwrap();
    assert!(path.to_string_lossy().contains(".logai"));
    assert!(path.to_string_lossy().contains("config.toml"));
}

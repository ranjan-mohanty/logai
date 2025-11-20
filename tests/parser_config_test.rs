//! Tests for parser configuration

use logai::parser::ParserConfig;
use std::collections::HashMap;

#[test]
fn test_parser_config_new() {
    let config = ParserConfig::new();
    assert!(config.auto_detect);
    assert!(config.enable_multiline);
}

#[test]
fn test_parser_config_auto_detect() {
    let config = ParserConfig {
        auto_detect: false,
        ..Default::default()
    };
    assert!(!config.auto_detect);
}

#[test]
fn test_parser_config_default_format() {
    let config = ParserConfig {
        default_format: Some("json".to_string()),
        ..Default::default()
    };
    assert_eq!(config.default_format, Some("json".to_string()));
}

#[test]
fn test_parser_config_multiline_settings() {
    let config = ParserConfig {
        enable_multiline: false,
        multiline_timeout_ms: 10000,
        max_multiline_lines: 200,
        ..Default::default()
    };

    assert!(!config.enable_multiline);
    assert_eq!(config.multiline_timeout_ms, 10000);
    assert_eq!(config.max_multiline_lines, 200);
}

#[test]
fn test_parser_config_parallel_parsing() {
    let config = ParserConfig {
        parallel_parsing: false,
        chunk_size: 500,
        num_threads: Some(4),
        ..Default::default()
    };

    assert!(!config.parallel_parsing);
    assert_eq!(config.chunk_size, 500);
    assert_eq!(config.num_threads, Some(4));
}

#[test]
fn test_parser_config_streaming_threshold() {
    let config = ParserConfig {
        streaming_threshold_mb: 50,
        ..Default::default()
    };

    assert_eq!(config.streaming_threshold_mb, 50);
    assert_eq!(config.streaming_threshold_bytes(), 50 * 1024 * 1024);
}

#[test]
fn test_parser_config_custom_timestamp_formats() {
    let config = ParserConfig {
        custom_timestamp_formats: vec!["%Y-%m-%d".to_string(), "%H:%M:%S".to_string()],
        ..Default::default()
    };

    assert_eq!(config.custom_timestamp_formats.len(), 2);
}

#[test]
fn test_parser_config_fallback_to_current_time() {
    let config = ParserConfig {
        fallback_to_current_time: false,
        ..Default::default()
    };

    assert!(!config.fallback_to_current_time);
}

#[test]
fn test_parser_config_metadata_extraction() {
    let mut patterns = HashMap::new();
    patterns.insert("request_id".to_string(), r"\[(\w+)\]".to_string());

    let config = ParserConfig {
        extract_metadata: false,
        custom_metadata_patterns: patterns,
        ..Default::default()
    };

    assert!(!config.extract_metadata);
    assert_eq!(config.custom_metadata_patterns.len(), 1);
}

#[test]
fn test_parser_config_error_handling() {
    let config = ParserConfig {
        skip_invalid_lines: false,
        max_parse_errors: Some(100),
        utf8_recovery: false,
        ..Default::default()
    };

    assert!(!config.skip_invalid_lines);
    assert_eq!(config.max_parse_errors, Some(100));
    assert!(!config.utf8_recovery);
}

#[test]
fn test_parser_config_validation_zero_chunk_size() {
    let config = ParserConfig {
        chunk_size: 0,
        ..Default::default()
    };

    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_parser_config_validation_zero_max_multiline() {
    let config = ParserConfig {
        max_multiline_lines: 0,
        ..Default::default()
    };

    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_parser_config_validation_zero_streaming_threshold() {
    let config = ParserConfig {
        streaming_threshold_mb: 0,
        ..Default::default()
    };

    let result = config.validate();
    assert!(result.is_err());
}

#[test]
fn test_parser_config_validation_valid() {
    let config = ParserConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_parser_config_clone() {
    let config1 = ParserConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.enable_multiline, config2.enable_multiline);
    assert_eq!(config1.chunk_size, config2.chunk_size);
    assert_eq!(config1.parallel_parsing, config2.parallel_parsing);
}

#[test]
fn test_parser_config_debug() {
    let config = ParserConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("ParserConfig"));
}

#[test]
fn test_parser_config_serialization() {
    let config = ParserConfig::default();
    let toml_str = toml::to_string(&config).unwrap();

    assert!(toml_str.contains("auto_detect"));
    assert!(toml_str.contains("enable_multiline"));
}

#[test]
fn test_parser_config_deserialization() {
    let config_default = ParserConfig::default();
    let toml_str = toml::to_string(&config_default).unwrap();

    let config: ParserConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(config.auto_detect, config_default.auto_detect);
    assert_eq!(config.enable_multiline, config_default.enable_multiline);
    assert_eq!(config.chunk_size, config_default.chunk_size);
}

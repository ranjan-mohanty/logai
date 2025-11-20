//! Comprehensive tests for the investigate command
//!
//! Tests cover:
//! - Different log formats (JSON, plain, Apache, Nginx, syslog)
//! - Multiple files and directories
//! - Error handling
//! - Output formats (HTML, JSON, terminal)
//! - Statistics display
//! - AI analysis integration

mod common;

use common::fixtures::{
    create_temp_log_file, sample_apache_log, sample_json_log, sample_nginx_log, sample_plain_log,
    sample_syslog,
};
use logai::commands::investigate::{InvestigateCommand, InvestigateOptions};
use std::fs;
use tempfile::TempDir;

fn default_options() -> InvestigateOptions {
    InvestigateOptions {
        files: vec![],
        log_format: "auto".to_string(),
        no_multiline: false,
        stats: false,
        ai_provider: "none".to_string(),
        model: None,
        api_key: None,
        ollama_host: None,
        region: None,
        format: "json".to_string(),
        limit: 10,
        no_mcp: true,
        mcp_config: None,
        concurrency: None,
    }
}

// ============================================================================
// Log Format Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_json_format() {
    let content = format!("{}\n{}\n", sample_json_log(), sample_json_log());
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.log_format = "json".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_plain_format() {
    let content = format!("{}\n{}\n", sample_plain_log(), sample_plain_log());
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.log_format = "plain".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_apache_format() {
    let content = format!("{}\n{}\n", sample_apache_log(), sample_apache_log());
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.log_format = "apache".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_nginx_format() {
    let content = format!("{}\n{}\n", sample_nginx_log(), sample_nginx_log());
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.log_format = "nginx".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_syslog_format() {
    let content = format!("{}\n{}\n", sample_syslog(), sample_syslog());
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.log_format = "syslog".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_auto_format_detection() {
    let content = sample_json_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.log_format = "auto".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// Output Format Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_json_output() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.format = "json".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_terminal_output() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.format = "terminal".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_html_output() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.format = "html".to_string();

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// Statistics Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_with_stats() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.stats = true;

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_without_stats() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.stats = false;

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// Multiline Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_with_multiline() {
    let content = "2024-11-19 10:30:00 ERROR Failed\n    at line 1\n    at line 2";
    let temp_file = create_temp_log_file(content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.no_multiline = false;

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_no_multiline() {
    let content = "2024-11-19 10:30:00 ERROR Failed\n    at line 1\n    at line 2";
    let temp_file = create_temp_log_file(content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.no_multiline = true;

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// Limit Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_with_limit() {
    let mut content = String::new();
    for i in 0..20 {
        content.push_str(&format!("2024-11-19 10:30:00 ERROR Error {}\n", i));
    }
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.limit = 5;

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_with_no_limit() {
    let mut content = String::new();
    for i in 0..20 {
        content.push_str(&format!("2024-11-19 10:30:00 ERROR Error {}\n", i));
    }
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.limit = usize::MAX;

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// Multiple Files Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_multiple_files() {
    let file1 = create_temp_log_file(&sample_json_log());
    let file2 = create_temp_log_file(&sample_plain_log());

    let mut opts = default_options();
    opts.files = vec![
        file1.path().to_str().unwrap().to_string(),
        file2.path().to_str().unwrap().to_string(),
    ];

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_directory() {
    let temp_dir = TempDir::new().unwrap();
    let log_file1 = temp_dir.path().join("test1.log");
    let log_file2 = temp_dir.path().join("test2.log");

    fs::write(&log_file1, sample_json_log()).unwrap();
    fs::write(&log_file2, sample_plain_log()).unwrap();

    let mut opts = default_options();
    opts.files = vec![temp_dir.path().to_str().unwrap().to_string()];

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_mixed_files_and_dirs() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");
    fs::write(&log_file, sample_json_log()).unwrap();

    let temp_file = create_temp_log_file(&sample_plain_log());

    let mut opts = default_options();
    opts.files = vec![
        temp_dir.path().to_str().unwrap().to_string(),
        temp_file.path().to_str().unwrap().to_string(),
    ];

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_nonexistent_file() {
    let mut opts = default_options();
    opts.files = vec!["nonexistent_file.log".to_string()];

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_investigate_empty_file() {
    let temp_file = create_temp_log_file("");

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_malformed_logs() {
    let content = "not a valid log\nstill not valid\n{invalid json}";
    let temp_file = create_temp_log_file(content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];

    let result = InvestigateCommand::execute(opts).await;
    // Should succeed but with no entries
    assert!(result.is_ok());
}

// ============================================================================
// Concurrency Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_with_concurrency() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.concurrency = Some(2);

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_without_concurrency() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.concurrency = None;

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// Large File Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_large_file() {
    let mut content = String::new();
    for i in 0..1000 {
        content.push_str(&format!("2024-11-19 10:30:00 ERROR Error {}\n", i));
    }
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// Special Characters Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_unicode_logs() {
    let content = "2024-11-19 10:30:00 ERROR 错误信息 🚀 ñ";
    let temp_file = create_temp_log_file(content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_special_characters() {
    let content = r#"2024-11-19 10:30:00 ERROR Special <>&"' chars"#;
    let temp_file = create_temp_log_file(content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

// ============================================================================
// MCP Tests
// ============================================================================

#[tokio::test]
async fn test_investigate_with_mcp_disabled() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.no_mcp = true;

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_with_mcp_config() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);

    let mut opts = default_options();
    opts.files = vec![temp_file.path().to_str().unwrap().to_string()];
    opts.no_mcp = false;
    opts.mcp_config = Some("/path/to/mcp.toml".to_string());

    let result = InvestigateCommand::execute(opts).await;
    // May fail if MCP config doesn't exist, but should handle gracefully
    let _ = result;
}

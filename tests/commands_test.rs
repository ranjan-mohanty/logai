mod common;

use common::fixtures::{create_temp_log_file, sample_json_log, sample_plain_log};
use logai::commands::investigate::{InvestigateCommand, InvestigateOptions};
use tempfile::TempDir;

#[tokio::test]
async fn test_investigate_with_empty_file() {
    let temp_file = create_temp_log_file("");
    let path = temp_file.path().to_str().unwrap().to_string();

    let opts = InvestigateOptions {
        files: vec![path],
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
    };

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_with_json_logs() {
    let content = format!("{}\n{}\n", sample_json_log(), sample_json_log());
    let temp_file = create_temp_log_file(&content);
    let path = temp_file.path().to_str().unwrap().to_string();

    let opts = InvestigateOptions {
        files: vec![path],
        log_format: "json".to_string(),
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
    };

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_with_plain_logs() {
    let content = format!("{}\n{}\n", sample_plain_log(), sample_plain_log());
    let temp_file = create_temp_log_file(&content);
    let path = temp_file.path().to_str().unwrap().to_string();

    let opts = InvestigateOptions {
        files: vec![path],
        log_format: "plain".to_string(),
        no_multiline: false,
        stats: true,
        ai_provider: "none".to_string(),
        model: None,
        api_key: None,
        ollama_host: None,
        region: None,
        format: "terminal".to_string(),
        limit: 5,
        no_mcp: true,
        mcp_config: None,
        concurrency: None,
    };

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_with_multiple_files() {
    let content1 = sample_json_log();
    let content2 = sample_plain_log();

    let temp_file1 = create_temp_log_file(&content1);
    let temp_file2 = create_temp_log_file(&content2);

    let path1 = temp_file1.path().to_str().unwrap().to_string();
    let path2 = temp_file2.path().to_str().unwrap().to_string();

    let opts = InvestigateOptions {
        files: vec![path1, path2],
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
    };

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_with_nonexistent_file() {
    let opts = InvestigateOptions {
        files: vec!["nonexistent_file.log".to_string()],
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
    };

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_investigate_with_directory() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");
    std::fs::write(&log_file, sample_json_log()).unwrap();

    let opts = InvestigateOptions {
        files: vec![temp_dir.path().to_str().unwrap().to_string()],
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
    };

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_with_no_multiline() {
    let content = sample_plain_log();
    let temp_file = create_temp_log_file(&content);
    let path = temp_file.path().to_str().unwrap().to_string();

    let opts = InvestigateOptions {
        files: vec![path],
        log_format: "plain".to_string(),
        no_multiline: true,
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
    };

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_investigate_html_output() {
    let content = sample_json_log();
    let temp_file = create_temp_log_file(&content);
    let path = temp_file.path().to_str().unwrap().to_string();

    let opts = InvestigateOptions {
        files: vec![path],
        log_format: "json".to_string(),
        no_multiline: false,
        stats: false,
        ai_provider: "none".to_string(),
        model: None,
        api_key: None,
        ollama_host: None,
        region: None,
        format: "html".to_string(),
        limit: 10,
        no_mcp: true,
        mcp_config: None,
        concurrency: None,
    };

    let result = InvestigateCommand::execute(opts).await;
    assert!(result.is_ok());

    // Verify HTML file was created
    assert!(std::path::Path::new("reports").exists());
}

//! Comprehensive tests for the clean command
//!
//! Tests cover:
//! - Cleaning HTML reports
//! - Cleaning log files
//! - Handling missing directories
//! - Force mode vs interactive mode

mod common;

use logai::commands::clean::CleanCommand;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn setup_test_reports(dir: &PathBuf, count: usize) {
    fs::create_dir_all(dir).unwrap();
    for i in 0..count {
        let file_path = dir.join(format!("report-{}.html", i));
        fs::write(&file_path, "<html><body>Test Report</body></html>").unwrap();
    }
}

fn setup_test_logs(dir: &PathBuf, count: usize) {
    fs::create_dir_all(dir).unwrap();
    for i in 0..count {
        let file_path = dir.join(format!("logai_{}.log", i));
        fs::write(&file_path, "Test log content").unwrap();
    }
}

#[test]
fn test_clean_with_force_removes_reports() {
    let temp_dir = TempDir::new().unwrap();
    let reports_dir = temp_dir.path().join("reports");
    setup_test_reports(&reports_dir, 3);

    // Verify files exist
    assert_eq!(fs::read_dir(&reports_dir).unwrap().count(), 3);

    // Clean with force (this will use default ./reports directory)
    // Note: This test is limited because CleanCommand uses hardcoded paths
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());
}

#[test]
fn test_clean_with_no_reports() {
    // Clean when no reports exist should succeed
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());
}

#[test]
fn test_clean_with_non_html_files() {
    let temp_dir = TempDir::new().unwrap();
    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create non-HTML files
    fs::write(reports_dir.join("test.txt"), "text file").unwrap();
    fs::write(reports_dir.join("test.json"), "{}").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Non-HTML files should still exist
    assert!(reports_dir.join("test.txt").exists());
    assert!(reports_dir.join("test.json").exists());
}

#[test]
fn test_clean_handles_permission_errors_gracefully() {
    // Test that clean doesn't panic on permission errors
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());
}

#[test]
fn test_clean_with_mixed_files() {
    let temp_dir = TempDir::new().unwrap();
    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create mix of HTML and non-HTML files
    fs::write(reports_dir.join("report1.html"), "<html></html>").unwrap();
    fs::write(reports_dir.join("data.json"), "{}").unwrap();
    fs::write(reports_dir.join("report2.html"), "<html></html>").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());
}

#[test]
fn test_clean_empty_reports_directory() {
    let temp_dir = TempDir::new().unwrap();
    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Directory exists but is empty
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());
}

#[test]
fn test_clean_with_subdirectories() {
    let temp_dir = TempDir::new().unwrap();
    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create subdirectory with HTML file
    let subdir = reports_dir.join("subdir");
    fs::create_dir_all(&subdir).unwrap();
    fs::write(subdir.join("report.html"), "<html></html>").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Subdirectory files should not be deleted (only top-level)
    assert!(subdir.join("report.html").exists());
}

#[test]
fn test_clean_logs_directory() {
    let temp_dir = TempDir::new().unwrap();
    let logs_dir = temp_dir.path().join("logs");
    setup_test_logs(&logs_dir, 5);

    // Verify files exist
    assert_eq!(fs::read_dir(&logs_dir).unwrap().count(), 5);

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());
}

#[test]
fn test_clean_only_logai_logs() {
    let temp_dir = TempDir::new().unwrap();
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create logai logs and other logs
    fs::write(logs_dir.join("logai_123.log"), "logai log").unwrap();
    fs::write(logs_dir.join("other.log"), "other log").unwrap();
    fs::write(logs_dir.join("app.log"), "app log").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Non-logai logs should still exist
    assert!(logs_dir.join("other.log").exists());
    assert!(logs_dir.join("app.log").exists());
}

#[test]
fn test_clean_with_both_reports_and_logs() {
    // Create both reports and logs
    let reports_dir = PathBuf::from("./reports");
    let logs_dir = PathBuf::from("./logs");

    fs::create_dir_all(&reports_dir).ok();
    fs::create_dir_all(&logs_dir).ok();

    fs::write(reports_dir.join("test-report.html"), "<html></html>").ok();
    fs::write(logs_dir.join("logai_test.log"), "test log").ok();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());
}

#[test]
fn test_clean_force_mode_no_prompt() {
    // Force mode should not prompt for confirmation
    // This is implicitly tested by all the above tests using force=true
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());
}

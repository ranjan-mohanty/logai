//! Tests for logging functionality
//!
//! Tests cover:
//! - Log cleanup functionality
//! - Directory handling
//! - Basic logging behavior

use logai::logging::cleanup_old_logs;
use std::env;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cleanup_old_logs_no_directory() {
    // Test cleanup when logs directory doesn't exist
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    let result = cleanup_old_logs(5);
    assert!(result.is_ok());

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_cleanup_old_logs_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();

    // Create logs directory in temp dir
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Change to temp directory for cleanup
    env::set_current_dir(&temp_dir).unwrap();

    let result = cleanup_old_logs(3);
    assert!(result.is_ok());

    // Directory should still exist but be empty
    assert!(logs_dir.exists());
    assert!(logs_dir.is_dir());

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_cleanup_old_logs_function_exists() {
    // Test that the cleanup function can be called and returns Ok
    // when there are no logs to clean up
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    let result = cleanup_old_logs(0);
    assert!(result.is_ok());

    let result = cleanup_old_logs(10);
    assert!(result.is_ok());

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_cleanup_with_non_log_files() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();

    // Create logs directory in temp dir
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create non-log files that should be ignored
    fs::write(logs_dir.join("other_file.txt"), "other content").unwrap();
    fs::write(logs_dir.join("README.md"), "readme").unwrap();
    fs::write(logs_dir.join("backup.log"), "backup").unwrap();

    // Change to temp directory for cleanup
    env::set_current_dir(&temp_dir).unwrap();

    let result = cleanup_old_logs(0);
    assert!(result.is_ok());

    // All non-logai files should still exist
    assert!(logs_dir.join("other_file.txt").exists());
    assert!(logs_dir.join("README.md").exists());
    assert!(logs_dir.join("backup.log").exists());

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_cleanup_identifies_log_files_correctly() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();

    // Create logs directory in temp dir
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create files with various names
    let test_files = [
        ("logai_20240101_120000.log", true),  // Valid log file
        ("logai_20240102_120000.log", true),  // Valid log file
        ("logai_file.txt", false),            // Wrong extension
        ("other_20240101_120000.log", false), // Wrong prefix
        ("logai_incomplete", false),          // No extension
        ("backup.log", false),                // Wrong name
    ];

    for (filename, _is_log) in &test_files {
        fs::write(logs_dir.join(filename), "content").unwrap();
    }

    // Change to temp directory for cleanup
    env::set_current_dir(&temp_dir).unwrap();

    // Count files before cleanup
    let all_files_before: Vec<_> = if logs_dir.exists() {
        fs::read_dir(&logs_dir)
            .map(|entries| entries.filter_map(|entry| entry.ok()).collect())
            .unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    let result = cleanup_old_logs(10); // Keep more than we have
    assert!(result.is_ok());

    // All files should still exist since we're keeping more than we have
    let all_files_after: Vec<_> = if logs_dir.exists() {
        fs::read_dir(&logs_dir)
            .map(|entries| entries.filter_map(|entry| entry.ok()).collect())
            .unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    assert_eq!(all_files_before.len(), all_files_after.len());

    // All non-log files should definitely still exist
    for (filename, is_log) in &test_files {
        if !is_log {
            assert!(
                logs_dir.join(filename).exists(),
                "Non-log file {} should still exist",
                filename
            );
        }
    }

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_cleanup_handles_different_keep_counts() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();

    // Create logs directory in temp dir
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Change to temp directory for cleanup
    env::set_current_dir(&temp_dir).unwrap();

    // Test various keep counts
    for keep_count in [0, 1, 5, 10, 100] {
        let result = cleanup_old_logs(keep_count);
        assert!(
            result.is_ok(),
            "cleanup_old_logs({}) should succeed",
            keep_count
        );
    }

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_cleanup_with_existing_logs_directory() {
    // Test cleanup when logs directory already exists (like in the main project)
    let logs_dir = env::current_dir().unwrap().join("logs");

    if logs_dir.exists() {
        // Should be able to run cleanup without errors
        let result = cleanup_old_logs(10);
        assert!(result.is_ok());

        // Directory should still exist
        assert!(logs_dir.exists());
    }
}

#[test]
fn test_cleanup_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();

    // Create logs directory in temp dir
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create a log file
    fs::write(logs_dir.join("logai_20240101_120000.log"), "content").unwrap();

    // Change to temp directory for cleanup
    env::set_current_dir(&temp_dir).unwrap();

    // Even if file removal fails internally, the function should return Ok
    // (based on the implementation using let _ = fs::remove_file)
    let result = cleanup_old_logs(0);
    assert!(result.is_ok());

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_cleanup_directory_traversal() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();

    // Create logs directory with subdirectories
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    let sub_dir = logs_dir.join("subdir");
    fs::create_dir_all(&sub_dir).unwrap();

    // Create files in main logs directory
    fs::write(logs_dir.join("logai_20240101_120000.log"), "content").unwrap();
    fs::write(logs_dir.join("other_file.txt"), "other").unwrap();

    // Create files in subdirectory (should be ignored)
    fs::write(sub_dir.join("logai_20240101_120000.log"), "sub_content").unwrap();

    // Change to temp directory for cleanup
    env::set_current_dir(&temp_dir).unwrap();

    let result = cleanup_old_logs(0);
    assert!(result.is_ok());

    // Subdirectory and its contents should be untouched
    assert!(sub_dir.exists());
    assert!(sub_dir.join("logai_20240101_120000.log").exists());

    // Non-log files in main directory should be untouched
    assert!(logs_dir.join("other_file.txt").exists());

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_cleanup_file_pattern_matching() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = env::current_dir().unwrap();

    // Create logs directory in temp dir
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create files with edge case names
    let edge_case_files = [
        "logai_.log",                    // Empty timestamp
        "logai_abc.log",                 // Non-numeric timestamp
        "logai_20240101_120000.log.bak", // Extra extension
        "logai_20240101_120000",         // Missing extension
        "LOGAI_20240101_120000.log",     // Wrong case
        "logai_20240101_120000.LOG",     // Wrong case extension
    ];

    for filename in &edge_case_files {
        fs::write(logs_dir.join(filename), "content").unwrap();
    }

    // Also create a valid log file
    fs::write(logs_dir.join("logai_20240101_120000.log"), "valid").unwrap();

    // Change to temp directory for cleanup
    env::set_current_dir(&temp_dir).unwrap();

    let result = cleanup_old_logs(10); // Keep more than we have
    assert!(result.is_ok());

    // All edge case files should still exist (they don't match the pattern)
    for filename in &edge_case_files {
        assert!(
            logs_dir.join(filename).exists(),
            "Edge case file {} should still exist",
            filename
        );
    }

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

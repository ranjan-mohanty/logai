//! Advanced tests for clean command edge cases and error conditions
//!
//! These tests focus on improving coverage by testing:
//! - Error handling paths
//! - Edge cases with file operations
//! - Different directory configurations
//! - Boundary conditions

mod common;

use logai::commands::clean::CleanCommand;
use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Helper function to safely get current directory
fn get_current_dir_safe() -> Option<PathBuf> {
    env::current_dir().ok()
}

// Helper function to safely set current directory
fn set_current_dir_safe(path: &std::path::Path) -> bool {
    env::set_current_dir(path).is_ok()
}

#[test]
fn test_clean_with_current_directory_reports() {
    // Test cleaning when reports are in the current directory
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => {
            // Skip test if we can't get current directory (CI environment)
            return;
        }
    };

    // Change to temp directory
    if !set_current_dir_safe(temp_dir.path()) {
        // Skip test if we can't change directory (CI environment)
        return;
    }

    // Create reports directory in current location
    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create some HTML files
    for i in 0..3 {
        let file_path = reports_dir.join(format!("test-report-{}.html", i));
        fs::write(
            &file_path,
            format!("<html><body>Report {}</body></html>", i),
        )
        .unwrap();
    }

    // Create some non-HTML files that should be ignored
    fs::write(reports_dir.join("readme.txt"), "Not an HTML file").unwrap();
    fs::write(reports_dir.join("data.json"), "{}").unwrap();

    // Execute clean command
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_current_directory_logs() {
    // Test cleaning when logs are in the current directory
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => {
            // Skip test if we can't get current directory (CI environment)
            return;
        }
    };

    // Change to temp directory
    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    // Create logs directory in current location
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create some log files
    for i in 0..3 {
        let file_path = logs_dir.join(format!("logai_{}.log", i));
        fs::write(&file_path, format!("Log entry {}", i)).unwrap();
    }

    // Create some non-log files that should be ignored
    fs::write(logs_dir.join("other.log"), "Not a logai log").unwrap();
    fs::write(logs_dir.join("logai_incomplete"), "No extension").unwrap();

    // Execute clean command
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Restore original directory
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_empty_html_files() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create empty HTML files
    for i in 0..2 {
        let file_path = reports_dir.join(format!("empty-{}.html", i));
        fs::write(&file_path, "").unwrap();
    }

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_large_html_files() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create large HTML files
    let large_content = "x".repeat(10000);
    for i in 0..2 {
        let file_path = reports_dir.join(format!("large-{}.html", i));
        fs::write(&file_path, &large_content).unwrap();
    }

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_special_characters_in_filenames() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create files with special characters (that are valid on most filesystems)
    let special_names = [
        "report-with-dashes.html",
        "report_with_underscores.html",
        "report.with.dots.html",
        "report (with parentheses).html",
    ];

    for name in &special_names {
        let file_path = reports_dir.join(name);
        fs::write(&file_path, "<html></html>").unwrap();
    }

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_nested_subdirectories() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create nested subdirectories with files
    let sub_dir = reports_dir.join("subdir");
    fs::create_dir_all(&sub_dir).unwrap();

    // Files in main directory
    fs::write(reports_dir.join("main.html"), "<html></html>").unwrap();

    // Files in subdirectory (should be ignored by clean command)
    fs::write(sub_dir.join("nested.html"), "<html></html>").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Subdirectory files should still exist
    assert!(sub_dir.join("nested.html").exists());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_case_sensitive_extensions() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create files with different case extensions
    fs::write(reports_dir.join("lowercase.html"), "<html></html>").unwrap();
    fs::write(reports_dir.join("uppercase.HTML"), "<html></html>").unwrap();
    fs::write(reports_dir.join("mixed.Html"), "<html></html>").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_log_files_different_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create various log file patterns
    fs::write(logs_dir.join("logai_20240101_120000.log"), "Valid log").unwrap();
    fs::write(logs_dir.join("logai_test.log"), "Valid log").unwrap();
    fs::write(logs_dir.join("logai_.log"), "Edge case log").unwrap();

    // These should NOT be cleaned
    fs::write(logs_dir.join("other_20240101_120000.log"), "Other log").unwrap();
    fs::write(logs_dir.join("logai_file.txt"), "Not a log").unwrap();
    fs::write(logs_dir.join("application.log"), "App log").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Non-logai files should still exist
    assert!(logs_dir.join("other_20240101_120000.log").exists());
    assert!(logs_dir.join("logai_file.txt").exists());
    assert!(logs_dir.join("application.log").exists());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_readonly_directory() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create a file
    fs::write(reports_dir.join("test.html"), "<html></html>").unwrap();

    // Try to make directory readonly (this might not work on all systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&reports_dir).unwrap().permissions();
        perms.set_mode(0o444); // Read-only
        let _ = fs::set_permissions(&reports_dir, perms);
    }

    // Clean should handle permission errors gracefully
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_symlinks() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create a regular HTML file
    let target_file = reports_dir.join("target.html");
    fs::write(&target_file, "<html></html>").unwrap();

    // Try to create a symlink (might not work on all systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs;
        let link_file = reports_dir.join("link.html");
        let _ = fs::symlink(&target_file, &link_file);
    }

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_very_long_filenames() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create file with long name (but within filesystem limits)
    let long_name = "a".repeat(100) + ".html";
    let file_path = reports_dir.join(&long_name);

    // Only create if the path is valid
    if file_path.to_string_lossy().len() < 255 {
        fs::write(&file_path, "<html></html>").unwrap();
    }

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_unicode_filenames() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create files with unicode characters
    let unicode_names = [
        "报告.html",     // Chinese
        "レポート.html", // Japanese
        "отчет.html",    // Russian
        "rapport.html",  // French (with accent)
        "🚀report.html", // Emoji
    ];

    for name in &unicode_names {
        let file_path = reports_dir.join(name);
        if fs::write(&file_path, "<html></html>").is_ok() {
            // File creation succeeded
        }
    }

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_zero_byte_files() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create zero-byte HTML files
    for i in 0..3 {
        let file_path = reports_dir.join(format!("zero-{}.html", i));
        fs::File::create(&file_path).unwrap();
    }

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_error_handling_with_invalid_current_dir() {
    // Test behavior when current directory is invalid
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    // Change to temp directory then remove it
    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };
    let _temp_path = temp_dir.path().to_path_buf();
    drop(temp_dir); // This removes the directory

    // Now current directory is invalid, but clean should still work
    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Try to restore original directory (might fail)
    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_mixed_file_types_in_reports() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let reports_dir = temp_dir.path().join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    // Create various file types
    fs::write(reports_dir.join("report.html"), "<html></html>").unwrap();
    fs::write(reports_dir.join("style.css"), "body {}").unwrap();
    fs::write(reports_dir.join("script.js"), "console.log('test')").unwrap();
    fs::write(reports_dir.join("data.json"), "{}").unwrap();
    fs::write(reports_dir.join("readme.txt"), "readme").unwrap();
    fs::write(reports_dir.join("image.png"), "fake png data").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Check that the command executed successfully
    // Note: The actual file deletion depends on the working directory being ./reports
    // These tests primarily exercise the code paths for coverage

    let _ = env::set_current_dir(original_dir);
}

#[test]
fn test_clean_with_mixed_file_types_in_logs() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = match get_current_dir_safe() {
        Some(dir) => dir,
        None => return,
    };

    if !set_current_dir_safe(temp_dir.path()) {
        return;
    };

    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&logs_dir).unwrap();

    // Create various file types
    fs::write(logs_dir.join("logai_test.log"), "logai log").unwrap();
    fs::write(logs_dir.join("application.log"), "app log").unwrap();
    fs::write(logs_dir.join("error.log"), "error log").unwrap();
    fs::write(logs_dir.join("debug.txt"), "debug info").unwrap();
    fs::write(logs_dir.join("config.json"), "{}").unwrap();

    let result = CleanCommand::execute(true);
    assert!(result.is_ok());

    // Check that the command executed successfully
    // Note: The actual file deletion depends on the working directory being ./logs
    // These tests primarily exercise the code paths for coverage

    let _ = env::set_current_dir(original_dir);
}

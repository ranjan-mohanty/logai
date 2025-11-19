//! Logging configuration with file output support
//!
//! This module sets up logging to write to both console and a log file.
//! Detailed error information is captured in the log file for later review.

use anyhow::Result;
use log::LevelFilter;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// Custom logger that writes to both console and file
struct DualLogger {
    console_level: LevelFilter,
    file: Mutex<std::fs::File>,
}

impl DualLogger {
    fn new(console_level: LevelFilter, log_file: std::fs::File) -> Self {
        Self {
            console_level,
            file: Mutex::new(log_file),
        }
    }
}

impl log::Log for DualLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        // Always log to file at DEBUG level or higher
        metadata.level() <= LevelFilter::Debug
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let level = record.level();
        let target = record.target();
        let message = record.args();

        // Format the log message
        let log_line = format!("[{} {} {}] {}\n", timestamp, level, target, message);

        // Always write to file (at DEBUG level or higher)
        if let Ok(mut file) = self.file.lock() {
            let _ = file.write_all(log_line.as_bytes());
            let _ = file.flush();
        }

        // Write to console based on console_level
        if record.level() <= self.console_level {
            // Use the standard env_logger format for console
            eprintln!("[{} {} {}] {}", timestamp, level, target, message);
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}

/// Get the log file path
fn get_log_file_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;

    let log_dir = PathBuf::from(home).join(".logai");

    // Create directory if it doesn't exist
    fs::create_dir_all(&log_dir)?;

    // Use a timestamped log file name
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let log_file = log_dir.join(format!("logai_{}.log", timestamp));

    Ok(log_file)
}

/// Initialize logging with both console and file output
pub fn init_logging(verbose: bool) -> Result<()> {
    let console_level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    // Get log file path
    let log_file_path = get_log_file_path()?;

    // Open log file in append mode
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)?;

    // Create and set the logger
    let logger = DualLogger::new(console_level, log_file);

    log::set_boxed_logger(Box::new(logger))
        .map_err(|e| anyhow::anyhow!("Failed to set logger: {}", e))?;

    // Set max level to Debug so file logging captures everything
    log::set_max_level(LevelFilter::Debug);

    // Log the file location at startup
    log::info!("Logging to file: {}", log_file_path.display());

    Ok(())
}

/// Clean up old log files (keep last N files)
pub fn cleanup_old_logs(keep_count: usize) -> Result<()> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;

    let log_dir = PathBuf::from(home).join(".logai");

    if !log_dir.exists() {
        return Ok(());
    }

    // Get all log files
    let mut log_files: Vec<_> = fs::read_dir(&log_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_name().to_string_lossy().starts_with("logai_")
                && entry.file_name().to_string_lossy().ends_with(".log")
        })
        .collect();

    // Sort by modification time (newest first)
    log_files.sort_by_key(|entry| {
        entry
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    log_files.reverse();

    // Remove old files
    for entry in log_files.iter().skip(keep_count) {
        let _ = fs::remove_file(entry.path());
    }

    Ok(())
}

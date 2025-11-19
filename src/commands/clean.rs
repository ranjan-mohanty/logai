//! Clean command implementation.
//!
//! This module contains the business logic for the `clean` command,
//! which removes generated HTML reports.

use crate::Result;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// Clean command implementation
pub struct CleanCommand;

impl CleanCommand {
    /// Execute the clean command
    pub fn execute(force: bool) -> Result<()> {
        let report_dir = Self::get_report_directory();
        let logs_dir = Self::get_logs_directory();

        let mut total_deleted = 0;

        // Clean reports
        if report_dir.exists() {
            let html_files: Vec<_> = fs::read_dir(&report_dir)?
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext == "html")
                        .unwrap_or(false)
                })
                .collect();

            if !html_files.is_empty() {
                let count = html_files.len();
                println!("Found {} report(s) in: {}", count, report_dir.display());

                // Confirm deletion
                if !force {
                    print!("Delete all reports? [y/N]: ");
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if !input.trim().eq_ignore_ascii_case("y") {
                        println!("Cancelled.");
                        return Ok(());
                    }
                }

                // Delete files
                for entry in html_files {
                    if fs::remove_file(entry.path()).is_ok() {
                        total_deleted += 1;
                    }
                }

                println!("✅ Deleted {} report(s)", total_deleted);
            }
        }

        // Clean logs
        if logs_dir.exists() {
            let log_files: Vec<_> = fs::read_dir(&logs_dir)?
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    let name = entry.file_name().to_string_lossy().to_string();
                    name.starts_with("logai_") && name.ends_with(".log")
                })
                .collect();

            if !log_files.is_empty() {
                let count = log_files.len();
                println!("Found {} log file(s) in: {}", count, logs_dir.display());

                // Confirm deletion
                if !force {
                    print!("Delete all logs? [y/N]: ");
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if !input.trim().eq_ignore_ascii_case("y") {
                        println!("Cancelled.");
                        return Ok(());
                    }
                }

                // Delete files
                let mut logs_deleted = 0;
                for entry in log_files {
                    if fs::remove_file(entry.path()).is_ok() {
                        logs_deleted += 1;
                    }
                }

                println!("✅ Deleted {} log file(s)", logs_deleted);
            }
        }

        if total_deleted == 0 && !logs_dir.exists() && !report_dir.exists() {
            println!("No reports or logs found to clean.");
        }

        Ok(())
    }

    fn get_report_directory() -> PathBuf {
        // Check config for custom path
        if let Ok(config) = crate::ai::AIConfig::load() {
            if let Some(path) = config.output.path {
                return PathBuf::from(path);
            }
        }

        // Default to ./reports
        PathBuf::from("./reports")
    }

    fn get_logs_directory() -> PathBuf {
        // Check config for custom path
        if let Ok(config) = crate::ai::AIConfig::load() {
            if let Some(logs_dir) = config.output.logs_dir {
                return PathBuf::from(logs_dir);
            }
        }

        // Default to ./logs
        if let Ok(current_dir) = std::env::current_dir() {
            current_dir.join("logs")
        } else {
            PathBuf::from("./logs")
        }
    }
}

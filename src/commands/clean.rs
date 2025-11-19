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

        if !report_dir.exists() {
            println!("No reports directory found at: {}", report_dir.display());
            return Ok(());
        }

        // Count HTML files
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

        if html_files.is_empty() {
            println!("No reports found in: {}", report_dir.display());
            return Ok(());
        }

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
        let mut deleted = 0;
        for entry in html_files {
            if fs::remove_file(entry.path()).is_ok() {
                deleted += 1;
            }
        }

        println!("✅ Deleted {} report(s)", deleted);

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
}

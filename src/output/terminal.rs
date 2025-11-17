use super::OutputFormatter;
use crate::types::{ErrorGroup, Severity};
use crate::Result;
use colored::*;

pub struct TerminalFormatter {
    show_limit: usize,
}

impl TerminalFormatter {
    pub fn new(show_limit: usize) -> Self {
        Self { show_limit }
    }

    fn severity_icon(severity: &Severity) -> ColoredString {
        match severity {
            Severity::Error => "🔴".red(),
            Severity::Warning => "🟡".yellow(),
            Severity::Info => "🔵".blue(),
            Severity::Debug => "⚪".white(),
            Severity::Trace => "⚫".bright_black(),
            Severity::Unknown => "❓".white(),
        }
    }

    fn severity_label(severity: &Severity) -> ColoredString {
        match severity {
            Severity::Error => "Critical".red().bold(),
            Severity::Warning => "Warning".yellow().bold(),
            Severity::Info => "Info".blue(),
            Severity::Debug => "Debug".white(),
            Severity::Trace => "Trace".bright_black(),
            Severity::Unknown => "Unknown".white(),
        }
    }

    fn format_time_ago(timestamp: &chrono::DateTime<chrono::Utc>) -> String {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(*timestamp);

        if duration.num_seconds() < 60 {
            format!("{} seconds ago", duration.num_seconds())
        } else if duration.num_minutes() < 60 {
            format!("{} minutes ago", duration.num_minutes())
        } else if duration.num_hours() < 24 {
            format!("{} hours ago", duration.num_hours())
        } else {
            format!("{} days ago", duration.num_days())
        }
    }
}

impl OutputFormatter for TerminalFormatter {
    fn format(&self, groups: &[ErrorGroup]) -> Result<String> {
        let mut output = String::new();

        // Header
        output.push_str(&format!(
            "\n{} {}\n",
            "🔍".bold(),
            "Sherlog Investigation Report".bold().bright_cyan()
        ));
        output.push_str(&"━".repeat(80).bright_black().to_string());
        output.push('\n');

        // Summary
        let total_errors: usize = groups.iter().map(|g| g.count).sum();
        output.push_str(&format!("\n{}\n", "📊 Summary".bold()));
        output.push_str(&format!(
            "   Errors found: {} unique patterns ({} occurrences)\n",
            groups.len().to_string().cyan().bold(),
            total_errors.to_string().cyan().bold()
        ));

        if let (Some(first), Some(last)) = (
            groups
                .iter()
                .flat_map(|g| g.entries.first())
                .min_by_key(|e| e.timestamp),
            groups
                .iter()
                .flat_map(|g| g.entries.last())
                .max_by_key(|e| e.timestamp),
        ) {
            if let (Some(first_ts), Some(last_ts)) = (first.timestamp, last.timestamp) {
                output.push_str(&format!(
                    "   Time range: {} - {}\n",
                    first_ts.format("%Y-%m-%d %H:%M:%S"),
                    last_ts.format("%Y-%m-%d %H:%M:%S")
                ));
            }
        }

        output.push('\n');
        output.push_str(&"━".repeat(80).bright_black().to_string());
        output.push('\n');

        // Error groups
        for (idx, group) in groups.iter().take(self.show_limit).enumerate() {
            if idx > 0 {
                output.push('\n');
                output.push_str(&"━".repeat(80).bright_black().to_string());
                output.push('\n');
            }

            output.push('\n');
            output.push_str(&format!(
                "{} {}: {} ({} occurrences)\n",
                Self::severity_icon(&group.severity),
                Self::severity_label(&group.severity),
                group.pattern.bright_white().bold(),
                group.count.to_string().cyan()
            ));

            output.push_str(&format!(
                "   First seen: {} | Last seen: {}\n",
                Self::format_time_ago(&group.first_seen).bright_black(),
                Self::format_time_ago(&group.last_seen).bright_black()
            ));

            // Show first entry as example
            if let Some(entry) = group.entries.first() {
                output.push_str(&format!("\n   {}\n", "📋 Example:".bold()));
                output.push_str(&format!("   {}\n", entry.message.bright_white()));

                if let Some(file) = &entry.metadata.file {
                    output.push_str(&format!(
                        "   {} {}\n",
                        "📍 Location:".bold(),
                        format!(
                            "{}{}",
                            file,
                            entry
                                .metadata
                                .line
                                .map(|l| format!(":{}", l))
                                .unwrap_or_default()
                        )
                        .cyan()
                    ));
                }
            }

            // AI analysis (if available)
            if let Some(analysis) = &group.analysis {
                output.push_str(&format!("\n   {}\n", "🎯 Explanation:".bold()));
                output.push_str(&format!("   {}\n", analysis.explanation.bright_white()));

                if let Some(root_cause) = &analysis.root_cause {
                    output.push_str(&format!("\n   {}\n", "🔍 Root Cause:".bold()));
                    output.push_str(&format!("   {}\n", root_cause.yellow()));
                }

                if !analysis.suggestions.is_empty() {
                    output.push_str(&format!("\n   {}\n", "💡 Suggested Fixes:".bold()));
                    for (i, suggestion) in analysis.suggestions.iter().enumerate() {
                        output.push_str(&format!(
                            "\n   {}. {}\n",
                            i + 1,
                            suggestion.description.green()
                        ));
                        if let Some(code) = &suggestion.code_example {
                            output.push_str(&format!("      {}\n", code.bright_black()));
                        }
                    }
                }

                if !analysis.related_resources.is_empty() {
                    output.push_str(&format!("\n   {}\n", "📚 Related Resources:".bold()));
                    for resource in &analysis.related_resources {
                        output.push_str(&format!(
                            "   • {} - {}\n",
                            resource.title,
                            resource.url.cyan()
                        ));
                    }
                }
            }
        }

        if groups.len() > self.show_limit {
            output.push('\n');
            output.push_str(&"━".repeat(80).bright_black().to_string());
            output.push('\n');
            output.push_str(&format!(
                "\n{} Showing {} of {} error groups. Use --limit to see more.\n",
                "ℹ️".blue(),
                self.show_limit,
                groups.len()
            ));
        }

        output.push('\n');
        Ok(output)
    }
}

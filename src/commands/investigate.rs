//! Investigate command implementation.
//!
//! This module contains the business logic for the `investigate` command,
//! which analyzes log files and groups similar errors.

use crate::{
    ai,
    analyzer::Analyzer,
    output::{html::HtmlFormatter, terminal::TerminalFormatter, OutputFormatter},
    parser::detector::FormatDetector,
    types::LogEntry,
    Result,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

/// Options for the investigate command
pub struct InvestigateOptions {
    pub files: Vec<String>,
    pub log_format: String,
    pub no_multiline: bool,
    pub stats: bool,
    pub ai_provider: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub ollama_host: Option<String>,
    pub region: Option<String>,
    pub format: String,
    pub limit: usize,
    pub no_mcp: bool,
    pub mcp_config: Option<String>,
    pub concurrency: usize,
}

/// Investigate command implementation
pub struct InvestigateCommand;

impl InvestigateCommand {
    /// Execute the investigate command
    pub async fn execute(opts: InvestigateOptions) -> Result<()> {
        let InvestigateOptions {
            files,
            log_format,
            no_multiline,
            stats,
            ai_provider,
            model,
            api_key,
            ollama_host,
            region,
            format,
            limit,
            no_mcp,
            mcp_config,
            concurrency,
        } = opts;

        let mut all_entries = Vec::new();
        let mut total_lines = 0;
        let mut parse_errors = 0;
        let parse_start = std::time::Instant::now();

        eprintln!("📖 Parsing logs...");

        // Read logs from all files
        for file_path in files {
            let (entries, file_stats) = if file_path == "-" {
                Self::read_logs_from_stdin(&log_format, no_multiline)?
            } else {
                let path = std::path::Path::new(&file_path);
                if path.is_dir() {
                    Self::read_logs_from_directory(&file_path, &log_format, no_multiline)?
                } else {
                    Self::read_logs_from_file(&file_path, &log_format, no_multiline)?
                }
            };
            total_lines += file_stats.0;
            parse_errors += file_stats.1;
            all_entries.extend(entries);
        }

        let parse_duration = parse_start.elapsed();

        if all_entries.is_empty() {
            eprintln!("✅ Parsed {} lines, found 0 log entries\n", total_lines);
            println!("No log entries found.");
            return Ok(());
        }

        // Display parsing summary
        if stats {
            Self::display_parsing_stats(
                total_lines,
                all_entries.len(),
                parse_errors,
                parse_duration,
            );
        } else {
            eprintln!("✅ Parsed {} log entries\n", all_entries.len());
        }

        // Analyze logs
        let analyzer = Analyzer::new();
        let mut groups = analyzer.analyze(all_entries)?;

        if groups.is_empty() {
            println!("No errors or warnings found in logs.");
            return Ok(());
        }

        // Initialize MCP client if enabled
        let _mcp_client = if !no_mcp && ai_provider != "none" {
            Self::initialize_mcp_client(mcp_config.as_deref())
                .await
                .ok()
        } else {
            None
        };

        // AI analysis if enabled
        if ai_provider != "none" {
            Self::run_ai_analysis(
                &mut groups,
                &ai_provider,
                model,
                api_key,
                ollama_host,
                region,
                concurrency,
                stats,
            )
            .await?;
        }

        // Priority: CLI flag > config > fallback to html
        let config = crate::ai::AIConfig::load().unwrap_or_default();
        let output_format = if format != "html" {
            // CLI flag was explicitly set to something other than default
            format
        } else {
            // Use config or fallback to html
            config.output.format.unwrap_or_else(|| "html".to_string())
        };

        // Format and display output
        Self::display_output(&groups, &output_format, limit)?;

        Ok(())
    }

    fn display_parsing_stats(
        total_lines: usize,
        parsed_entries: usize,
        parse_errors: usize,
        duration: std::time::Duration,
    ) {
        eprintln!("✅ Parsed {} log entries\n", parsed_entries);
        eprintln!("📊 Parsing Statistics:");
        eprintln!("  Total lines: {}", total_lines);
        eprintln!("  Parsed entries: {}", parsed_entries);
        eprintln!("  Parse errors: {}", parse_errors);
        eprintln!("  Duration: {:.2}s", duration.as_secs_f64());
        eprintln!(
            "  Throughput: {:.2} lines/sec",
            total_lines as f64 / duration.as_secs_f64()
        );
        eprintln!();
    }

    #[allow(clippy::too_many_arguments)]
    async fn run_ai_analysis(
        groups: &mut [crate::types::ErrorGroup],
        ai_provider: &str,
        model: Option<String>,
        api_key: Option<String>,
        ollama_host: Option<String>,
        region: Option<String>,
        concurrency: usize,
        stats: bool,
    ) -> Result<()> {
        let provider_name = ai_provider.to_string();
        eprintln!("🤖 Analyzing error groups...");

        // Load configuration from file and merge with CLI flags
        let ai_config = ai::AIConfig::load().unwrap_or_default();

        // Get the actual model being used (from CLI or config)
        let model_display = if let Some(ref m) = model {
            m.clone()
        } else {
            // Try to get model from config for the provider
            ai_config
                .providers
                .get(ai_provider)
                .and_then(|p| p.model.clone())
                .unwrap_or_else(|| "default".to_string())
        };

        let provider = ai::create_provider(ai_provider, api_key, model, ollama_host, region)?;

        let mut config = ai_config.get_analysis_config();
        config.max_concurrency = concurrency;

        let parallel_analyzer = ai::ParallelAnalyzer::new(provider, config);

        // Create progress callback
        let analysis_start = std::time::Instant::now();
        let progress_callback = move |update: ai::ProgressUpdate| {
            eprint!("\r\x1b[K{}", update.format_terminal());
            std::io::Write::flush(&mut std::io::stderr()).unwrap();
        };

        // Run parallel analysis
        parallel_analyzer
            .analyze_groups(groups, progress_callback)
            .await?;

        // Clear progress line and show completion
        eprint!("\r\x1b[K");
        let analysis_duration = analysis_start.elapsed();
        let throughput = groups.len() as f64 / analysis_duration.as_secs_f64();

        if stats {
            eprintln!("✅ Analyzed {} error groups\n", groups.len());
            eprintln!("📊 Analysis Statistics:");
            eprintln!("  AI Provider: {}", provider_name);
            eprintln!("  Model: {}", model_display);
            eprintln!("  Error groups: {}", groups.len());
            eprintln!("  Duration: {:.2}s", analysis_duration.as_secs_f64());
            eprintln!("  Throughput: {:.2} groups/sec", throughput);
            eprintln!("  Concurrency: {}", concurrency);
            eprintln!();
        } else {
            eprintln!("✅ Analyzed {} error groups\n", groups.len());
        }

        Ok(())
    }

    fn display_output(
        groups: &[crate::types::ErrorGroup],
        format: &str,
        limit: usize,
    ) -> Result<()> {
        match format {
            "terminal" => {
                let formatter = TerminalFormatter::new(limit);
                let output = formatter.format(groups)?;
                print!("{}", output);
            }
            "json" => {
                let json = serde_json::to_string_pretty(groups)?;
                println!("{}", json);
            }
            "html" => {
                let formatter = HtmlFormatter::new(limit);
                let output = formatter.format(groups)?;

                // Generate filename based on timestamp
                let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
                let filename = format!("logai-report-{}.html", timestamp);

                // Get report path from config or use default
                let config = crate::ai::AIConfig::load().unwrap_or_default();
                let report_dir = config.output.path.unwrap_or_else(|| "reports".to_string());

                // Create directory if it doesn't exist
                std::fs::create_dir_all(&report_dir)?;

                let filepath = std::path::Path::new(&report_dir).join(&filename);
                std::fs::write(&filepath, output)?;

                let canonical_path = filepath.canonicalize()?;
                let file_url = format!("file://{}", canonical_path.display());

                // Use OSC 8 hyperlink escape sequence for clickable link
                println!("📊 Report: {}", filepath.display());
                println!(
                    "   \x1b]8;;{}\x1b\\Click to open in browser\x1b]8;;\x1b\\",
                    file_url
                );
            }
            _ => {
                eprintln!("Unknown format: {}", format);
            }
        }
        Ok(())
    }

    fn read_logs_from_file(
        path: &str,
        log_format: &str,
        no_multiline: bool,
    ) -> Result<(Vec<LogEntry>, (usize, usize))> {
        let file = File::open(path)
            .map_err(|e| anyhow::anyhow!("Failed to open file '{}': {}", path, e))?;

        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .collect::<std::io::Result<_>>()
            .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", path, e))?;

        if lines.is_empty() {
            eprintln!("⚠️  Warning: File '{}' is empty", path);
            return Ok((Vec::new(), (0, 0)));
        }

        let total_lines = lines.len();
        let parser = Self::create_parser(log_format, no_multiline, &lines[0]);
        let entries = Self::parse_with_parser(&parser, &lines, no_multiline)?;
        let parse_errors = total_lines.saturating_sub(entries.len());

        if parse_errors > 0 {
            eprintln!(
                "⚠️  Warning: Failed to parse {} lines in '{}'",
                parse_errors, path
            );
        }

        Ok((entries, (total_lines, parse_errors)))
    }

    fn read_logs_from_stdin(
        log_format: &str,
        no_multiline: bool,
    ) -> Result<(Vec<LogEntry>, (usize, usize))> {
        let stdin = std::io::stdin();
        let lines: Vec<String> = stdin.lock().lines().collect::<std::io::Result<_>>()?;

        if lines.is_empty() {
            return Ok((Vec::new(), (0, 0)));
        }

        let total_lines = lines.len();
        let parser = Self::create_parser(log_format, no_multiline, &lines[0]);
        let entries = Self::parse_with_parser(&parser, &lines, no_multiline)?;
        let parse_errors = total_lines.saturating_sub(entries.len());

        Ok((entries, (total_lines, parse_errors)))
    }

    fn read_logs_from_directory(
        dir_path: &str,
        log_format: &str,
        no_multiline: bool,
    ) -> Result<(Vec<LogEntry>, (usize, usize))> {
        use std::fs;

        let mut all_entries = Vec::new();
        let mut file_count = 0;
        let mut total_lines = 0;
        let mut total_errors = 0;

        let entries = fs::read_dir(dir_path)
            .map_err(|e| anyhow::anyhow!("Failed to read directory '{}': {}", dir_path, e))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if let Some(ext) = path.extension() {
                if ext == "log" {
                    if let Some(path_str) = path.to_str() {
                        match Self::read_logs_from_file(path_str, log_format, no_multiline) {
                            Ok((entries, (lines, errors))) => {
                                all_entries.extend(entries);
                                total_lines += lines;
                                total_errors += errors;
                                file_count += 1;
                            }
                            Err(e) => {
                                eprintln!("⚠️  Warning: Failed to read '{}': {}", path_str, e);
                            }
                        }
                    }
                }
            }
        }

        if file_count > 0 {
            eprintln!(
                "📂 Processed {} log file(s) from '{}'",
                file_count, dir_path
            );
        } else {
            eprintln!("⚠️  Warning: No .log files found in '{}'", dir_path);
        }

        Ok((all_entries, (total_lines, total_errors)))
    }

    fn create_parser(
        log_format: &str,
        no_multiline: bool,
        first_line: &str,
    ) -> Arc<dyn crate::parser::LogParser> {
        use crate::parser::formats::{
            ApacheParser, JsonParser, NginxParser, PlainTextParser, SyslogParser,
        };
        use crate::parser::StackTraceParser;

        match log_format {
            "auto" => FormatDetector::detect(first_line),
            "json" => {
                if no_multiline {
                    Arc::new(JsonParser::new())
                } else {
                    Arc::new(StackTraceParser::new(Arc::new(JsonParser::new())))
                }
            }
            "apache" => Arc::new(ApacheParser::new()),
            "nginx" => Arc::new(NginxParser::new()),
            "syslog" => Arc::new(SyslogParser::new()),
            "plain" => {
                if no_multiline {
                    Arc::new(PlainTextParser::new())
                } else {
                    Arc::new(StackTraceParser::new(Arc::new(PlainTextParser::new())))
                }
            }
            _ => {
                eprintln!("⚠️  Unknown format '{}', using auto-detection", log_format);
                FormatDetector::detect(first_line)
            }
        }
    }

    fn parse_with_parser(
        parser: &Arc<dyn crate::parser::LogParser>,
        lines: &[String],
        no_multiline: bool,
    ) -> Result<Vec<LogEntry>> {
        if parser.supports_multiline() && !no_multiline {
            parser.parse_lines(lines)
        } else {
            let mut entries = Vec::new();
            for line in lines {
                if let Some(entry) = parser.parse_line(line)? {
                    entries.push(entry);
                }
            }
            Ok(entries)
        }
    }

    async fn initialize_mcp_client(config_path: Option<&str>) -> Result<crate::mcp::MCPClient> {
        use crate::mcp::{MCPClient, MCPConfig};

        let config = if let Some(path) = config_path {
            let content = std::fs::read_to_string(path)?;
            toml::from_str(&content)?
        } else {
            let config_dir = dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
                .join(".logai");

            let mcp_config_path = config_dir.join("mcp.toml");

            if mcp_config_path.exists() {
                let content = std::fs::read_to_string(&mcp_config_path)?;
                toml::from_str(&content)?
            } else {
                MCPConfig::default()
            }
        };

        let mut client = MCPClient::new(config)?;
        client.connect().await?;

        if client.is_connected() {
            client.discover_tools().await?;
            println!(
                "🔧 MCP tools enabled ({} servers connected)\n",
                client.connected_servers().len()
            );
        }

        Ok(client)
    }
}

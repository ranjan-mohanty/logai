use clap::Parser;
use logai::{
    ai,
    analyzer::Analyzer,
    cli::{Cli, Commands, ConfigAction},
    output::{terminal::TerminalFormatter, OutputFormatter},
    parser::detector::FormatDetector,
    types::LogEntry,
    Result,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Investigate {
            files,
            log_format,
            no_multiline,
            stats,
            ai: ai_provider,
            model,
            api_key,
            ollama_host,
            no_cache: _,
            format,
            output: _,
            limit,
            severity: _,
            no_mcp,
            mcp_config,
            concurrency,
        } => {
            investigate_logs(InvestigateOptions {
                files,
                log_format,
                no_multiline,
                stats,
                ai_provider,
                model,
                api_key,
                ollama_host,
                format,
                limit,
                no_mcp,
                mcp_config,
                concurrency,
            })
            .await?;
        }
        Commands::Watch { file: _ } => {
            println!("Watch mode coming soon!");
        }
        Commands::Config { action } => {
            handle_config(action)?;
        }
    }

    Ok(())
}

struct InvestigateOptions {
    files: Vec<String>,
    log_format: String,
    no_multiline: bool,
    stats: bool,
    ai_provider: String,
    model: Option<String>,
    api_key: Option<String>,
    ollama_host: Option<String>,
    format: String,
    limit: usize,
    no_mcp: bool,
    mcp_config: Option<String>,
    concurrency: usize,
}

async fn investigate_logs(opts: InvestigateOptions) -> Result<()> {
    let InvestigateOptions {
        files,
        log_format,
        no_multiline,
        stats,
        ai_provider,
        model,
        api_key,
        ollama_host,
        format,
        limit,
        no_mcp,
        mcp_config,
        concurrency,
    } = opts;
    let mut all_entries = Vec::new();

    let mut total_lines = 0;
    let mut parse_errors = 0;
    let start_time = std::time::Instant::now();

    for file_path in files {
        let (entries, file_stats) = if file_path == "-" {
            read_logs_from_stdin(&log_format, no_multiline)?
        } else {
            // Check if it's a directory
            let path = std::path::Path::new(&file_path);
            if path.is_dir() {
                read_logs_from_directory(&file_path, &log_format, no_multiline)?
            } else {
                read_logs_from_file(&file_path, &log_format, no_multiline)?
            }
        };
        total_lines += file_stats.0;
        parse_errors += file_stats.1;
        all_entries.extend(entries);
    }

    if stats {
        let duration = start_time.elapsed();
        println!("\n📊 Parsing Statistics:");
        println!("  Total lines: {}", total_lines);
        println!("  Parsed entries: {}", all_entries.len());
        println!("  Parse errors: {}", parse_errors);
        println!("  Duration: {:?}", duration);
        println!(
            "  Throughput: {:.2} lines/sec\n",
            total_lines as f64 / duration.as_secs_f64()
        );
    }

    if all_entries.is_empty() {
        println!("No log entries found.");
        return Ok(());
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
        match initialize_mcp_client(mcp_config.as_deref()).await {
            Ok(client) => {
                if client.is_connected() {
                    println!(
                        "🔧 MCP tools enabled ({} servers connected)\n",
                        client.connected_servers().len()
                    );
                }
                Some(client)
            }
            Err(e) => {
                eprintln!("⚠️  MCP initialization failed: {}", e);
                eprintln!("   Continuing without MCP tools...\n");
                None
            }
        }
    } else {
        None
    };

    // AI analysis if enabled
    if ai_provider != "none" {
        let model_display = model.as_deref().unwrap_or("default");
        println!(
            "🤖 Analyzing {} error groups with {} ({})...\n",
            groups.len(),
            ai_provider,
            model_display
        );

        let provider = ai::create_provider(&ai_provider, api_key, model.clone(), ollama_host)?;

        // Create parallel analyzer with configuration
        let config = ai::AnalysisConfig::new(concurrency)?;
        let parallel_analyzer = ai::ParallelAnalyzer::new(provider, config);

        // Create progress callback
        let analysis_start = std::time::Instant::now();
        let progress_callback = move |update: ai::ProgressUpdate| {
            print!("\r\x1b[K{}", update.format_terminal());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        };

        // Run parallel analysis
        parallel_analyzer
            .analyze_groups(&mut groups, progress_callback)
            .await?;

        // Clear progress line and show completion
        println!("\r\x1b[K");
        let analysis_duration = analysis_start.elapsed();
        println!(
            "✅ Analysis complete in {:.1}s ({:.1} groups/sec)\n",
            analysis_duration.as_secs_f64(),
            groups.len() as f64 / analysis_duration.as_secs_f64()
        );
    }

    // Format output
    match format.as_str() {
        "terminal" => {
            let formatter = TerminalFormatter::new(limit);
            let output = formatter.format(&groups)?;
            print!("{}", output);
        }
        "json" => {
            let json = serde_json::to_string_pretty(&groups)?;
            println!("{}", json);
        }
        "html" => {
            println!("HTML output coming soon!");
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
    use logai::parser::{
        formats::{ApacheParser, JsonParser, NginxParser, PlainTextParser, SyslogParser},
        StackTraceParser,
    };
    use std::sync::Arc;

    let file =
        File::open(path).map_err(|e| anyhow::anyhow!("Failed to open file '{}': {}", path, e))?;

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

    // Select parser based on format
    let parser: Arc<dyn logai::parser::LogParser> = match log_format {
        "auto" => FormatDetector::detect(&lines[0]),
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
            FormatDetector::detect(&lines[0])
        }
    };

    // Parse lines (use parse_lines for multi-line support)
    let entries = if parser.supports_multiline() && !no_multiline {
        parser.parse_lines(&lines)?
    } else {
        let mut entries = Vec::new();
        for line in &lines {
            if let Some(entry) = parser.parse_line(line)? {
                entries.push(entry);
            }
        }
        entries
    };

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
    use logai::parser::{
        formats::{ApacheParser, JsonParser, NginxParser, PlainTextParser, SyslogParser},
        StackTraceParser,
    };
    use std::sync::Arc;

    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin.lock().lines().collect::<std::io::Result<_>>()?;

    if lines.is_empty() {
        return Ok((Vec::new(), (0, 0)));
    }

    let total_lines = lines.len();

    let parser: Arc<dyn logai::parser::LogParser> = match log_format {
        "auto" => FormatDetector::detect(&lines[0]),
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
        _ => FormatDetector::detect(&lines[0]),
    };

    let entries = if parser.supports_multiline() && !no_multiline {
        parser.parse_lines(&lines)?
    } else {
        let mut entries = Vec::new();
        for line in &lines {
            if let Some(entry) = parser.parse_line(line)? {
                entries.push(entry);
            }
        }
        entries
    };

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

        // Only process files (not subdirectories)
        if !path.is_file() {
            continue;
        }

        // Only process .log files
        if let Some(ext) = path.extension() {
            if ext == "log" {
                if let Some(path_str) = path.to_str() {
                    match read_logs_from_file(path_str, log_format, no_multiline) {
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

fn handle_config(action: ConfigAction) -> Result<()> {
    use ai::AIConfig;

    match action {
        ConfigAction::Show => {
            let config = AIConfig::load()?;
            let config_path = AIConfig::config_path()?;

            println!("📝 LogAI Configuration");
            println!("   Location: {}\n", config_path.display());

            // Display MCP settings
            println!("MCP Settings:");
            println!("  enabled: {}", config.mcp.enabled);
            if let Some(path) = &config.mcp.config_path {
                println!("  config_path: {}", path);
            }
            println!("  default_timeout: {}s\n", config.mcp.default_timeout);

            if let Some(default) = &config.default_provider {
                println!("Default Provider: {}", default);
            }

            if config.providers.is_empty() {
                println!("\nNo providers configured.");
                println!("\nExample configuration:");
                println!("  logai config set ollama.model llama3.1:8b");
                println!("  logai config set openai.api_key sk-...");
                println!("  logai config set mcp.enabled true");
                println!("  logai config set mcp.config_path ~/.logai/mcp.toml");
            } else {
                println!("\nProviders:");
                for (name, provider) in &config.providers {
                    println!("\n  [{}]", name);
                    if let Some(model) = &provider.model {
                        println!("    model: {}", model);
                    }
                    if let Some(host) = &provider.host {
                        println!("    host: {}", host);
                    }
                    if provider.api_key.is_some() {
                        println!("    api_key: ***");
                    }
                    println!("    enabled: {}", provider.enabled);
                }
            }
        }
        ConfigAction::Set { key, value } => {
            let mut config = AIConfig::load().unwrap_or_default();

            // Use the new set_value method that handles both provider and MCP settings
            config.set_value(&key, &value)?;

            config.save()?;
            println!("✅ Configuration updated: {} = {}", key, value);
            println!("   Saved to: {}", AIConfig::config_path()?.display());
        }
    }

    Ok(())
}

/// Initialize MCP client from configuration
async fn initialize_mcp_client(config_path: Option<&str>) -> Result<logai::mcp::MCPClient> {
    use logai::mcp::{MCPClient, MCPConfig};

    // Load MCP configuration
    let config = if let Some(path) = config_path {
        // Load from specified path
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content)?
    } else {
        // Try to load from default location (~/.logai/mcp.toml)
        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".logai");

        let mcp_config_path = config_dir.join("mcp.toml");

        if mcp_config_path.exists() {
            let content = std::fs::read_to_string(&mcp_config_path)?;
            toml::from_str(&content)?
        } else {
            // Return default empty config if no config file exists
            MCPConfig::default()
        }
    };

    // Create and connect client
    let mut client = MCPClient::new(config)?;
    client.connect().await?;

    // Discover tools
    if client.is_connected() {
        client.discover_tools().await?;
    }

    Ok(client)
}

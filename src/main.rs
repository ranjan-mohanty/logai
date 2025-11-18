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
            ai: ai_provider,
            model,
            api_key,
            ollama_host,
            no_cache,
            format,
            output: _,
            limit,
            severity: _,
            no_mcp,
            mcp_config,
        } => {
            investigate_logs(InvestigateOptions {
                files,
                ai_provider,
                model,
                api_key,
                ollama_host,
                no_cache,
                format,
                limit,
                no_mcp,
                mcp_config,
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
    ai_provider: String,
    model: Option<String>,
    api_key: Option<String>,
    ollama_host: Option<String>,
    no_cache: bool,
    format: String,
    limit: usize,
    no_mcp: bool,
    mcp_config: Option<String>,
}

async fn investigate_logs(opts: InvestigateOptions) -> Result<()> {
    let InvestigateOptions {
        files,
        ai_provider,
        model,
        api_key,
        ollama_host,
        no_cache,
        format,
        limit,
        no_mcp,
        mcp_config,
    } = opts;
    let mut all_entries = Vec::new();

    for file_path in files {
        let entries = if file_path == "-" {
            read_logs_from_stdin()?
        } else {
            // Check if it's a directory
            let path = std::path::Path::new(&file_path);
            if path.is_dir() {
                read_logs_from_directory(&file_path)?
            } else {
                read_logs_from_file(&file_path)?
            }
        };
        all_entries.extend(entries);
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
    let mcp_client = if !no_mcp && ai_provider != "none" {
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
        println!("🤖 Analyzing errors with {}...\n", ai_provider);

        let provider = ai::create_provider(&ai_provider, api_key, model.clone(), ollama_host)?;
        let cache = if !no_cache {
            ai::AnalysisCache::new().ok()
        } else {
            None
        };

        let model_name = model.as_deref().unwrap_or("default");
        let mut cache_hits = 0;
        let mut cache_misses = 0;

        for group in groups.iter_mut() {
            // Try cache first
            if let Some(ref cache) = cache {
                if let Ok(Some(cached)) = cache.get(&group.pattern, &ai_provider, model_name) {
                    group.analysis = Some(cached);
                    cache_hits += 1;
                    continue;
                }
            }

            // Call AI provider (with MCP tools if available)
            let analysis_result = if mcp_client.is_some() {
                provider
                    .analyze_with_tools(group, mcp_client.as_ref())
                    .await
            } else {
                provider.analyze(group).await
            };

            match analysis_result {
                Ok(analysis) => {
                    // Cache the result
                    if let Some(ref cache) = cache {
                        let _ = cache.set(&group.pattern, &ai_provider, model_name, &analysis);
                    }
                    group.analysis = Some(analysis);
                    cache_misses += 1;
                }
                Err(e) => {
                    eprintln!("⚠️  Failed to analyze error group {}: {}", group.id, e);
                }
            }
        }

        if cache_hits > 0 {
            println!("💾 Cache: {} hits, {} misses\n", cache_hits, cache_misses);
        }
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

fn read_logs_from_file(path: &str) -> Result<Vec<LogEntry>> {
    let file =
        File::open(path).map_err(|e| anyhow::anyhow!("Failed to open file '{}': {}", path, e))?;

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<std::io::Result<_>>()
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", path, e))?;

    if lines.is_empty() {
        eprintln!("⚠️  Warning: File '{}' is empty", path);
        return Ok(Vec::new());
    }

    // Detect format from first line
    let parser = FormatDetector::detect(&lines[0]);

    // Parse all lines
    let mut entries = Vec::new();
    let mut parse_errors = 0;

    for line in lines {
        match parser.parse_line(&line) {
            Ok(Some(entry)) => entries.push(entry),
            Ok(None) => {} // Empty line or filtered out
            Err(_) => parse_errors += 1,
        }
    }

    if parse_errors > 0 {
        eprintln!(
            "⚠️  Warning: Failed to parse {} lines in '{}'",
            parse_errors, path
        );
    }

    Ok(entries)
}

fn read_logs_from_stdin() -> Result<Vec<LogEntry>> {
    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin.lock().lines().collect::<std::io::Result<_>>()?;

    if lines.is_empty() {
        return Ok(Vec::new());
    }

    let parser = FormatDetector::detect(&lines[0]);

    let mut entries = Vec::new();
    for line in lines {
        if let Some(entry) = parser.parse_line(&line)? {
            entries.push(entry);
        }
    }

    Ok(entries)
}

fn read_logs_from_directory(dir_path: &str) -> Result<Vec<LogEntry>> {
    use std::fs;

    let mut all_entries = Vec::new();
    let mut file_count = 0;

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
                    match read_logs_from_file(path_str) {
                        Ok(entries) => {
                            all_entries.extend(entries);
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

    Ok(all_entries)
}

fn handle_config(action: ConfigAction) -> Result<()> {
    use ai::{config::ProviderConfig, AIConfig};

    match action {
        ConfigAction::Show => {
            let config = AIConfig::load()?;
            let config_path = AIConfig::config_path()?;

            println!("📝 LogAI Configuration");
            println!("   Location: {}\n", config_path.display());

            if let Some(default) = &config.default_provider {
                println!("Default Provider: {}", default);
            }

            if config.providers.is_empty() {
                println!("\nNo providers configured.");
                println!("\nExample configuration:");
                println!("  logai config set ollama.model llama3.1:8b");
                println!("  logai config set openai.api_key sk-...");
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

            // Parse key as provider.field
            let parts: Vec<&str> = key.split('.').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!(
                    "Invalid key format. Use: provider.field (e.g., ollama.model)"
                ));
            }

            let provider_name = parts[0];
            let field = parts[1];

            // Get or create provider config
            let provider_config = config
                .providers
                .entry(provider_name.to_string())
                .or_insert_with(|| ProviderConfig {
                    api_key: None,
                    model: None,
                    host: None,
                    enabled: true,
                });

            // Update field
            match field {
                "model" => provider_config.model = Some(value.clone()),
                "api_key" => provider_config.api_key = Some(value.clone()),
                "host" => provider_config.host = Some(value.clone()),
                "enabled" => {
                    provider_config.enabled = value.parse().unwrap_or(true);
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Unknown field: {}. Valid fields: model, api_key, host, enabled",
                        field
                    ));
                }
            }

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

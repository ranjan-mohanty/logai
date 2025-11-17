use clap::Parser;
use sherlog::{
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
        } => {
            investigate_logs(files, ai_provider, model, api_key, ollama_host, no_cache, format, limit).await?;
        }
        Commands::Watch { file: _ } => {
            println!("Watch mode coming soon!");
        }
        Commands::Config { action } => match action {
            ConfigAction::Set { key: _, value: _ } => {
                println!("Config management coming soon!");
            }
            ConfigAction::Show => {
                println!("Config management coming soon!");
            }
        },
    }

    Ok(())
}

async fn investigate_logs(
    files: Vec<String>,
    ai_provider: String,
    model: Option<String>,
    api_key: Option<String>,
    ollama_host: Option<String>,
    no_cache: bool,
    format: String,
    limit: usize,
) -> Result<()> {
    let mut all_entries = Vec::new();

    for file_path in files {
        let entries = if file_path == "-" {
            read_logs_from_stdin()?
        } else {
            read_logs_from_file(&file_path)?
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
            
            // Call AI provider
            match provider.analyze(group).await {
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
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<std::io::Result<_>>()?;

    if lines.is_empty() {
        return Ok(Vec::new());
    }

    // Detect format from first line
    let parser = FormatDetector::detect(&lines[0]);

    // Parse all lines
    let mut entries = Vec::new();
    for line in lines {
        if let Some(entry) = parser.parse_line(&line)? {
            entries.push(entry);
        }
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

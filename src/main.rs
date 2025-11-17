use clap::Parser;
use sherlog::{
    analyzer::Analyzer,
    cli::{Cli, Commands, ConfigAction},
    output::{terminal::TerminalFormatter, OutputFormatter},
    parser::detector::FormatDetector,
    types::LogEntry,
    Result,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Investigate {
            files,
            ai: _,
            model: _,
            no_ai: _,
            format,
            output: _,
            limit,
            severity: _,
        } => {
            investigate_logs(files, format, limit)?;
        }
        Commands::Watch { file: _ } => {
            println!("Watch mode coming in Phase 2!");
        }
        Commands::Config { action } => match action {
            ConfigAction::Set { key: _, value: _ } => {
                println!("Config management coming in Phase 2!");
            }
            ConfigAction::Show => {
                println!("Config management coming in Phase 2!");
            }
        },
    }

    Ok(())
}

fn investigate_logs(files: Vec<String>, format: String, limit: usize) -> Result<()> {
    let mut all_entries = Vec::new();

    for file_path in files {
        let entries = if file_path == "-" {
            // Read from stdin
            read_logs_from_stdin()?
        } else {
            // Read from file
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
    let groups = analyzer.analyze(all_entries)?;

    if groups.is_empty() {
        println!("No errors or warnings found in logs.");
        return Ok(());
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
            println!("HTML output coming in Phase 3!");
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

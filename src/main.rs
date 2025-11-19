use clap::Parser;
use logai::{
    cli::{Cli, Commands},
    commands::{CleanCommand, ConfigCommand, InvestigateCommand, InvestigateOptions},
    logging, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
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
            region,
            no_cache: _,
            format,
            output: _,
            limit,
            severity: _,
            no_mcp,
            mcp_config,
            concurrency,
        } => {
            // Enable file logging for investigate command
            let log_file_path = logging::init_logging(cli.verbose)?;

            // Clean up old log files (keep last 10)
            let _ = logging::cleanup_old_logs(10);

            // If no AI provider specified via CLI, check config file
            let ai_provider = if ai_provider == "none" {
                logai::ai::AIConfig::load()
                    .ok()
                    .and_then(|config| config.ai.provider)
                    .unwrap_or_else(|| "none".to_string())
            } else {
                ai_provider
            };

            InvestigateCommand::execute(InvestigateOptions {
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
            })
            .await?;

            // Print log file location at the end
            eprintln!("\n📋 Detailed logs: {}", log_file_path.display());
        }
        Commands::Watch { file: _ } => {
            // Initialize basic console logging for other commands
            env_logger::Builder::from_default_env()
                .filter_level(if cli.verbose {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .init();

            println!("Watch mode coming soon!");
        }
        Commands::Config { action } => {
            // Initialize basic console logging for other commands
            env_logger::Builder::from_default_env()
                .filter_level(if cli.verbose {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .init();

            ConfigCommand::execute(action)?;
        }
        Commands::Clean { force } => {
            // Initialize basic console logging for other commands
            env_logger::Builder::from_default_env()
                .filter_level(if cli.verbose {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .init();

            CleanCommand::execute(force)?;
        }
    }

    Ok(())
}

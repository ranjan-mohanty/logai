use clap::Parser;
use logai::{
    cli::{Cli, Commands},
    commands::{CleanCommand, ConfigCommand, InvestigateCommand, InvestigateOptions},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger with appropriate level based on verbose flag
    env_logger::Builder::from_default_env()
        .filter_level(if cli.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

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
        }
        Commands::Watch { file: _ } => {
            println!("Watch mode coming soon!");
        }
        Commands::Config { action } => {
            ConfigCommand::execute(action)?;
        }
        Commands::Clean { force } => {
            CleanCommand::execute(force)?;
        }
    }

    Ok(())
}

use clap::Parser;
use logai::{
    cli::{Cli, Commands},
    commands::{ConfigCommand, InvestigateCommand, InvestigateOptions},
    Result,
};

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
    }

    Ok(())
}

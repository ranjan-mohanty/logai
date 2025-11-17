use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "logai")]
#[command(about = "AI-powered log analysis - Parse, group, and understand your logs", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze log file(s)
    Investigate {
        /// Log file(s) to analyze (use '-' for stdin)
        #[arg(required = true)]
        files: Vec<String>,

        /// AI provider to use (openai, claude, gemini, ollama, none)
        #[arg(long, default_value = "none")]
        ai: String,

        /// AI model to use
        #[arg(long)]
        model: Option<String>,

        /// API key for AI provider (or set OPENAI_API_KEY/ANTHROPIC_API_KEY/GEMINI_API_KEY env var)
        #[arg(long)]
        api_key: Option<String>,

        /// Ollama host (default: http://localhost:11434)
        #[arg(long)]
        ollama_host: Option<String>,

        /// Disable response caching
        #[arg(long)]
        no_cache: bool,

        /// Output format (terminal, json, html)
        #[arg(long, short = 'f', default_value = "terminal")]
        format: String,

        /// Save output to file
        #[arg(long, short = 'o')]
        output: Option<String>,

        /// Maximum number of error groups to show
        #[arg(long, default_value = "20")]
        limit: usize,

        /// Filter by severity (error, warn, info)
        #[arg(long)]
        severity: Option<String>,
    },

    /// Watch and analyze logs in real-time
    Watch {
        /// Log file to watch (use '-' for stdin)
        file: String,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., ai.provider)
        key: String,
        /// Configuration value
        value: String,
    },
    /// Show current configuration
    Show,
}

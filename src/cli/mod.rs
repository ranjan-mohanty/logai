use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "logai")]
#[command(about = "AI-powered log analysis - Parse, group, and understand your logs", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
pub struct Cli {
    /// Enable verbose/debug logging
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    /// Analyze log file(s)
    Investigate {
        /// Log file(s) or directory to analyze (omit or use '-' for stdin)
        #[arg(default_value = "-")]
        files: Vec<String>,

        /// Log format (auto, json, apache, nginx, syslog, plain)
        #[arg(long, default_value = "auto")]
        log_format: String,

        /// Disable multi-line log handling (e.g., stack traces)
        #[arg(long)]
        no_multiline: bool,

        /// Show parsing statistics
        #[arg(long)]
        stats: bool,

        /// AI provider to use (openai, claude, gemini, ollama, bedrock, none)
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

        /// AWS region for Bedrock (e.g., us-east-1, us-west-2)
        #[arg(long)]
        region: Option<String>,

        /// Disable response caching
        #[arg(long)]
        no_cache: bool,

        /// Output format (terminal, json, html)
        #[arg(long, short = 'f', default_value = "html")]
        format: String,

        /// Save output to file
        #[arg(long, short = 'o')]
        output: Option<String>,

        /// Maximum number of error groups to show (0 = unlimited)
        #[arg(long, default_value = "0")]
        limit: usize,

        /// Filter by severity (error, warn, info)
        #[arg(long)]
        severity: Option<String>,

        /// Disable MCP tools integration
        #[arg(long)]
        no_mcp: bool,

        /// Path to MCP configuration file
        #[arg(long)]
        mcp_config: Option<String>,

        /// Maximum concurrent AI analysis requests (1-100)
        #[arg(long)]
        concurrency: Option<usize>,
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

    /// Clean up generated reports
    Clean {
        /// Skip confirmation prompt
        #[arg(long, short = 'f')]
        force: bool,
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

//! Command implementations for LogAI CLI.
//!
//! This module contains the business logic for each CLI command,
//! separated from the CLI parsing layer for better testability.

pub mod config;
pub mod investigate;

pub use config::ConfigCommand;
pub use investigate::{InvestigateCommand, InvestigateOptions};

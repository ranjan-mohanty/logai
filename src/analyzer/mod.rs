pub mod grouper;

use crate::types::{ErrorGroup, LogEntry};
use crate::Result;

pub struct Analyzer;

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, entries: Vec<LogEntry>) -> Result<Vec<ErrorGroup>> {
        let grouper = grouper::ErrorGrouper::new();
        grouper.group(entries)
    }
}

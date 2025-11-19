# LogAI API Documentation

This document describes LogAI's internal API for developers who want to use
LogAI as a library or extend its functionality.

## Table of Contents

- [Using LogAI as a Library](#using-logai-as-a-library)
- [Core Modules](#core-modules)
- [Parser API](#parser-api)
- [Analyzer API](#analyzer-api)
- [AI Provider API](#ai-provider-api)
- [Output Formatter API](#output-formatter-api)
- [Extension Points](#extension-points)

## Using LogAI as a Library

Add LogAI to your `Cargo.toml`:

```toml
[dependencies]
logai = { git = "https://github.com/your-org/logai.git" }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

### Basic Usage Example

```rust
use logai::parser::{LogParser, create_parser};
use logai::analyzer::Analyzer;
use logai::ai::{AIProvider, create_provider};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Parse logs
    let parser = create_parser("json")?;
    let entries = parser.parse_file("app.log")?;

    // 2. Analyze and group errors
    let analyzer = Analyzer::new();
    let groups = analyzer.analyze(&entries)?;

    // 3. Get AI insights
    let provider = create_provider("ollama", None).await?;
    for group in groups {
        let analysis = provider.analyze(&group).await?;
        println!("{}", analysis);
    }

    Ok(())
}
```

## Core Modules

### Parser Module (`logai::parser`)

Handles log file parsing and format detection.

```rust
use logai::parser::{LogParser, LogEntry, create_parser, detect_format};

// Auto-detect format
let format = detect_format("app.log")?;
let parser = create_parser(&format)?;

// Parse file
let entries = parser.parse_file("app.log")?;

// Parse individual lines
for line in lines {
    if let Some(entry) = parser.parse_line(&line)? {
        println!("{:?}", entry);
    }
}
```

### Analyzer Module (`logai::analyzer`)

Groups and analyzes log entries.

```rust
use logai::analyzer::{Analyzer, ErrorGroup};

let analyzer = Analyzer::new();
let groups = analyzer.analyze(&entries)?;

for group in groups {
    println!("Pattern: {}", group.pattern);
    println!("Count: {}", group.count);
    println!("Severity: {:?}", group.severity);
}
```

### AI Module (`logai::ai`)

Provides AI-powered analysis.

```rust
use logai::ai::{create_provider, AIProvider, AIConfig};

// Create provider
let config = AIConfig::default();
let provider = create_provider("ollama", Some(config)).await?;

// Analyze error group
let analysis = provider.analyze(&error_group).await?;
println!("Root cause: {}", analysis.root_cause);
println!("Solution: {}", analysis.solution);
```

## Parser API

### LogParser Trait

Implement this trait to create custom parsers:

```rust
use logai::parser::{LogParser, LogEntry};
use anyhow::Result;

pub struct CustomParser {
    // Your parser state
}

impl LogParser for CustomParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        // Parse logic here
        Ok(Some(LogEntry {
            timestamp: parse_timestamp(line)?,
            level: parse_level(line)?,
            message: parse_message(line)?,
            metadata: Default::default(),
        }))
    }

    fn parse_file(&self, path: &str) -> Result<Vec<LogEntry>> {
        // File parsing logic
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);

        let mut entries = Vec::new();
        for line in reader.lines() {
            if let Some(entry) = self.parse_line(&line?)? {
                entries.push(entry);
            }
        }
        Ok(entries)
    }

    fn name(&self) -> &str {
        "custom"
    }
}
```

### LogEntry Structure

```rust
pub struct LogEntry {
    pub timestamp: Option<DateTime<Utc>>,
    pub level: LogLevel,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
```

### Format Detection

```rust
use logai::parser::detector::{detect_format, FormatDetector};

// Auto-detect from file
let format = detect_format("app.log")?;

// Manual detection
let detector = FormatDetector::new();
let sample_lines = vec![
    r#"{"level":"error","msg":"failed"}"#,
    r#"{"level":"warn","msg":"slow"}"#,
];
let format = detector.detect_from_lines(&sample_lines)?;
```

### Parallel Processing

```rust
use logai::parser::parallel::ParallelParser;

let parser = create_parser("json")?;
let parallel_parser = ParallelParser::new(parser, 4); // 4 threads

let entries = parallel_parser.parse_file("large.log")?;
```

## Analyzer API

### Analyzer Configuration

```rust
use logai::analyzer::{Analyzer, AnalyzerConfig};

let config = AnalyzerConfig {
    min_group_size: 2,
    normalize_ids: true,
    normalize_timestamps: true,
    normalize_paths: true,
    ..Default::default()
};

let analyzer = Analyzer::with_config(config);
```

### Error Grouping

```rust
use logai::analyzer::{Analyzer, ErrorGroup};

let analyzer = Analyzer::new();
let groups = analyzer.analyze(&entries)?;

// Access group information
for group in groups {
    println!("Pattern: {}", group.pattern);
    println!("Occurrences: {}", group.count);
    println!("First seen: {:?}", group.first_seen);
    println!("Last seen: {:?}", group.last_seen);
    println!("Severity: {:?}", group.severity);

    // Access original entries
    for entry in &group.entries {
        println!("  - {}", entry.message);
    }
}
```

### Custom Normalization

```rust
use logai::analyzer::normalizer::Normalizer;

let mut normalizer = Normalizer::new();

// Add custom normalization rules
normalizer.add_pattern(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b", "<IP>");
normalizer.add_pattern(r"\b[A-Z0-9]{8}-[A-Z0-9]{4}\b", "<TOKEN>");

let normalized = normalizer.normalize("Error at 192.168.1.1 with token ABC12345-6789");
// Result: "Error at <IP> with token <TOKEN>"
```

## AI Provider API

### AIProvider Trait

Implement this trait to add new AI providers:

```rust
use logai::ai::{AIProvider, AIAnalysis, ErrorGroup};
use async_trait::async_trait;
use anyhow::Result;

pub struct CustomAIProvider {
    api_key: String,
    model: String,
}

#[async_trait]
impl AIProvider for CustomAIProvider {
    async fn analyze(&self, group: &ErrorGroup) -> Result<AIAnalysis> {
        // Call your AI service
        let response = self.call_api(&group.pattern).await?;

        Ok(AIAnalysis {
            root_cause: response.root_cause,
            impact: response.impact,
            solution: response.solution,
            confidence: response.confidence,
        })
    }

    fn name(&self) -> &str {
        "custom"
    }

    async fn health_check(&self) -> Result<bool> {
        // Check if service is available
        Ok(true)
    }
}
```

### Parallel Analysis

```rust
use logai::ai::parallel::ParallelAnalyzer;

let provider = create_provider("ollama", None).await?;
let analyzer = ParallelAnalyzer::new(provider, 5); // 5 concurrent requests

let results = analyzer.analyze_groups(&error_groups).await?;
```

### Retry Logic

```rust
use logai::ai::retry::RetryConfig;

let retry_config = RetryConfig {
    max_retries: 3,
    initial_backoff_ms: 1000,
    max_backoff_ms: 30000,
    backoff_multiplier: 2.0,
    jitter: true,
};

let provider = create_provider("openai", Some(config)).await?;
let provider_with_retry = provider.with_retry(retry_config);
```

### Caching

```rust
use logai::ai::cache::{Cache, CacheConfig};

let cache_config = CacheConfig {
    enabled: true,
    ttl_seconds: 3600,
    max_size_mb: 100,
};

let cache = Cache::new(cache_config)?;

// Check cache before analysis
if let Some(cached) = cache.get(&group.pattern)? {
    return Ok(cached);
}

// Analyze and cache result
let analysis = provider.analyze(&group).await?;
cache.set(&group.pattern, &analysis)?;
```

## Output Formatter API

### Formatter Trait

```rust
use logai::output::{Formatter, AnalysisResult};
use anyhow::Result;

pub trait Formatter {
    fn format(&self, result: &AnalysisResult) -> Result<String>;
    fn name(&self) -> &str;
}

// Example: JSON formatter
pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String> {
        Ok(serde_json::to_string_pretty(result)?)
    }

    fn name(&self) -> &str {
        "json"
    }
}
```

### Terminal Formatter

```rust
use logai::output::terminal::TerminalFormatter;

let formatter = TerminalFormatter::new();
let output = formatter.format(&analysis_result)?;
println!("{}", output);
```

### Custom Formatter Example

```rust
use logai::output::{Formatter, AnalysisResult};

pub struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String> {
        let mut output = String::new();

        output.push_str("# Log Analysis Report\n\n");
        output.push_str(&format!("**Total Errors**: {}\n\n", result.total_errors));

        for (i, group) in result.groups.iter().enumerate() {
            output.push_str(&format!("## Error Group {}\n\n", i + 1));
            output.push_str(&format!("**Pattern**: `{}`\n\n", group.pattern));
            output.push_str(&format!("**Count**: {}\n\n", group.count));

            if let Some(analysis) = &group.ai_analysis {
                output.push_str("### AI Analysis\n\n");
                output.push_str(&format!("**Root Cause**: {}\n\n", analysis.root_cause));
                output.push_str(&format!("**Solution**: {}\n\n", analysis.solution));
            }
        }

        Ok(output)
    }

    fn name(&self) -> &str {
        "markdown"
    }
}
```

## Extension Points

### Adding a New Log Format

1. **Create parser implementation**:

```rust
// src/parser/formats/myformat.rs
use crate::parser::{LogParser, LogEntry};

pub struct MyFormatParser;

impl LogParser for MyFormatParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        // Your parsing logic
    }

    fn name(&self) -> &str {
        "myformat"
    }
}
```

2. **Register in detector**:

```rust
// src/parser/detector.rs
pub fn detect_format(path: &str) -> Result<String> {
    // Add detection logic
    if line.starts_with("MYFORMAT:") {
        return Ok("myformat".to_string());
    }
    // ...
}
```

3. **Add to factory**:

```rust
// src/parser/mod.rs
pub fn create_parser(format: &str) -> Result<Box<dyn LogParser>> {
    match format {
        "myformat" => Ok(Box::new(MyFormatParser)),
        // ...
    }
}
```

### Adding a New AI Provider

1. **Implement AIProvider trait**:

```rust
// src/ai/providers/myprovider.rs
use crate::ai::{AIProvider, AIAnalysis};

pub struct MyProvider {
    config: MyProviderConfig,
}

#[async_trait]
impl AIProvider for MyProvider {
    async fn analyze(&self, group: &ErrorGroup) -> Result<AIAnalysis> {
        // Implementation
    }

    fn name(&self) -> &str {
        "myprovider"
    }
}
```

2. **Add to factory**:

```rust
// src/ai/mod.rs
pub async fn create_provider(name: &str, config: Option<AIConfig>) -> Result<Box<dyn AIProvider>> {
    match name {
        "myprovider" => Ok(Box::new(MyProvider::new(config)?)),
        // ...
    }
}
```

### Adding a New Command

1. **Create command module**:

```rust
// src/commands/mycommand.rs
use anyhow::Result;

pub async fn execute(/* args */) -> Result<()> {
    // Command implementation
    Ok(())
}
```

2. **Add CLI definition**:

```rust
// src/cli/mod.rs
#[derive(Subcommand)]
pub enum Commands {
    MyCommand {
        #[arg(short, long)]
        option: String,
    },
    // ...
}
```

3. **Add dispatch**:

```rust
// src/main.rs
match cli.command {
    Commands::MyCommand { option } => {
        commands::mycommand::execute(option).await?;
    }
    // ...
}
```

## Testing Your Extensions

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_parser() {
        let parser = MyFormatParser;
        let result = parser.parse_line("MYFORMAT: error message").unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_custom_provider() {
        let provider = MyProvider::new(config).unwrap();
        let analysis = provider.analyze(&test_group).await.unwrap();
        assert!(!analysis.root_cause.is_empty());
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use logai::parser::create_parser;
use logai::analyzer::Analyzer;

#[test]
fn test_end_to_end() {
    let parser = create_parser("myformat").unwrap();
    let entries = parser.parse_file("tests/fixtures/sample.log").unwrap();

    let analyzer = Analyzer::new();
    let groups = analyzer.analyze(&entries).unwrap();

    assert!(!groups.is_empty());
}
```

## Performance Considerations

### Memory Management

```rust
// Use streaming for large files
use std::io::{BufReader, BufRead};

pub fn parse_file_streaming<F>(&self, path: &str, mut callback: F) -> Result<()>
where
    F: FnMut(LogEntry) -> Result<()>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Some(entry) = self.parse_line(&line?)? {
            callback(entry)?;
        }
    }
    Ok(())
}
```

### Parallel Processing

```rust
use rayon::prelude::*;

// Process entries in parallel
let results: Vec<_> = entries
    .par_iter()
    .map(|entry| process_entry(entry))
    .collect();
```

### Caching Strategies

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CachedProvider {
    provider: Box<dyn AIProvider>,
    cache: Arc<RwLock<HashMap<String, AIAnalysis>>>,
}

impl CachedProvider {
    pub async fn analyze(&self, group: &ErrorGroup) -> Result<AIAnalysis> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.get(&group.pattern) {
                return Ok(cached.clone());
            }
        }

        // Analyze and cache
        let analysis = self.provider.analyze(group).await?;

        {
            let mut cache = self.cache.write().await;
            cache.insert(group.pattern.clone(), analysis.clone());
        }

        Ok(analysis)
    }
}
```

## Error Handling

### Custom Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LogAIError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("AI provider error: {0}")]
    AIError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### Error Context

```rust
use anyhow::{Context, Result};

pub fn parse_file(&self, path: &str) -> Result<Vec<LogEntry>> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path))?;

    // ...
}
```

## Documentation

Generate API documentation:

```bash
# Generate docs
cargo doc --no-deps --open

# Generate with private items
cargo doc --no-deps --document-private-items --open
```

## Examples

See the `examples/` directory for complete working examples:

- `examples/basic_usage.rs` - Basic parsing and analysis
- `examples/custom_parser.rs` - Implementing a custom parser
- `examples/custom_provider.rs` - Implementing a custom AI provider
- `examples/parallel_processing.rs` - High-performance processing
- `examples/streaming.rs` - Streaming large files

## Further Reading

- [Architecture Documentation](ARCHITECTURE.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Source Code](../src/)
- [Test Suite](../tests/)

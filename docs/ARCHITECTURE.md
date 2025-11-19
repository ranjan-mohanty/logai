# LogAI Architecture

This document provides a comprehensive overview of LogAI's architecture, design
decisions, and implementation details for contributors and maintainers.

## Table of Contents

- [Overview](#overview)
- [Core Concepts](#core-concepts)
- [Architecture Layers](#architecture-layers)
- [Data Flow](#data-flow)
- [Key Components](#key-components)
- [Design Decisions](#design-decisions)
- [Performance Considerations](#performance-considerations)
- [Extension Points](#extension-points)

## Overview

LogAI is a CLI tool that analyzes application logs using AI to identify, group,
and explain errors. The architecture is designed for:

- **Modularity**: Clear separation of concerns
- **Extensibility**: Easy to add new parsers, AI providers, and output formats
- **Performance**: Parallel processing for both parsing and AI analysis
- **Testability**: Comprehensive unit and integration tests

## Core Concepts

### 1. Log Entry

A parsed log line with structured data:

- Timestamp
- Severity level
- Message
- Metadata (file, line, thread, etc.)

### 2. Error Group

Similar errors grouped together by normalized pattern:

- Pattern (normalized message)
- Count (occurrences)
- Time range (first/last seen)
- Example entries
- AI analysis (optional)

### 3. AI Analysis

AI-generated insights for each error group:

- Explanation of what went wrong
- Root cause analysis
- Suggested fixes with priority
- Related resources

## Architecture Layers

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│  (Command parsing, argument validation, user interaction)   │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                      Commands Layer                          │
│     (Business logic for investigate, config, watch)         │
└──────────────────────────┬──────────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
┌───────▼────────┐ ┌──────▼──────┐ ┌────────▼────────┐
│  Parser Layer  │ │ Analyzer    │ │  AI Layer       │
│  (Log parsing) │ │ (Grouping)  │ │  (Analysis)     │
└───────┬────────┘ └──────┬──────┘ └────────┬────────┘
        │                  │                  │
┌───────▼────────┐ ┌──────▼──────┐ ┌────────▼────────┐
│  Format        │ │  Grouper    │ │  Providers      │
│  Detectors     │ │  Algorithm  │ │  (OpenAI, etc)  │
└────────────────┘ └─────────────┘ └─────────────────┘
```

## Data Flow

### 1. Investigation Flow

```
Input Files
    │
    ▼
┌─────────────────┐
│ Format Detection│ (Auto-detect or use specified format)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Parallel Parse │ (Parse logs in parallel chunks)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Error Grouping │ (Normalize and group similar errors)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  AI Analysis    │ (Parallel analysis with retry logic)
│  (Optional)     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Format Output  │ (Terminal, JSON, or HTML)
└─────────────────┘
```

### 2. AI Analysis Flow

```
Error Groups
    │
    ▼
┌──────────────────────┐
│  ParallelAnalyzer    │ (Manages concurrency)
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│  RetryableAnalyzer   │ (Handles transient failures)
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│  AI Provider         │ (OpenAI, Claude, Ollama, etc)
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│  JSON Extractor      │ (Parse and validate response)
└──────────┬───────────┘
           │
           ▼
    Error Analysis
```

## Key Components

### Parser Layer (`src/parser/`)

**Responsibility**: Convert raw log text into structured `LogEntry` objects

**Components**:

- **Format Detectors**: Auto-detect log format from samples
- **Format Parsers**: Parse specific formats (JSON, Apache, Nginx, Syslog,
  Plain)
- **Stack Trace Parser**: Handle multi-line stack traces
- **Parallel Parser**: Process large files in parallel chunks
- **Metadata Extractor**: Extract file, line, thread, trace IDs

**Key Files**:

- `detector.rs` - Format detection logic
- `formats/` - Format-specific parsers
- `parallel.rs` - Parallel parsing infrastructure
- `metadata.rs` - Metadata extraction patterns

### Analyzer Layer (`src/analyzer/`)

**Responsibility**: Group similar errors together

**Algorithm**:

1. Normalize messages (replace dynamic values with `<DYNAMIC>`)
2. Group by normalized pattern
3. Track count, time range, and examples
4. Sort by severity and count

**Key Files**:

- `grouper.rs` - Error grouping logic

### AI Layer (`src/ai/`)

**Responsibility**: Analyze error groups using AI providers

**Components**:

- **Parallel Analyzer**: Process multiple groups concurrently
- **Retry Logic**: Exponential backoff for transient failures
- **JSON Extractor**: Robust parsing of AI responses
- **Progress Tracker**: Real-time progress updates
- **Statistics**: Comprehensive metrics tracking
- **Providers**: OpenAI, Claude, Gemini, Ollama implementations

**Key Files**:

- `parallel.rs` - Parallel analysis infrastructure
- `retry.rs` - Retry logic with exponential backoff
- `json_extractor.rs` - Enhanced JSON extraction
- `progress.rs` - Progress tracking
- `statistics.rs` - Analysis statistics
- `providers/` - AI provider implementations

### Commands Layer (`src/commands/`)

**Responsibility**: Implement business logic for CLI commands

**Components**:

- **Investigate**: Main analysis command
- **Config**: Configuration management
- **Watch**: Real-time log monitoring (future)

**Key Files**:

- `investigate.rs` - Investigation command logic
- `config.rs` - Configuration command logic

### Output Layer (`src/output/`)

**Responsibility**: Format analysis results for display

**Formats**:

- **Terminal**: Colored, formatted output with progress bars
- **JSON**: Machine-readable output
- **HTML**: Web-based report (future)

**Key Files**:

- `terminal.rs` - Terminal output formatter

### MCP Layer (`src/mcp/`)

**Responsibility**: Model Context Protocol integration for external tools

**Components**:

- **Client**: MCP client implementation
- **Transport**: Communication layer (stdio)
- **Protocol**: MCP protocol types

**Key Files**:

- `client.rs` - MCP client
- `transport.rs` - Transport layer
- `protocol.rs` - Protocol types

## Design Decisions

### 1. Why Rust?

- **Performance**: Fast parsing and analysis
- **Safety**: Memory safety without garbage collection
- **Concurrency**: Excellent async/await support
- **Tooling**: Cargo, clippy, rustfmt

### 2. Why Parallel Processing?

- **Speed**: 5x faster analysis with parallel AI requests
- **Scalability**: Handle thousands of error groups
- **Efficiency**: Maximize resource utilization

**Implementation**:

- Tokio for async runtime
- Semaphore for concurrency control
- Indexed futures for result ordering

### 3. Why Retry Logic?

- **Reliability**: Handle transient API failures
- **User Experience**: Automatic recovery without user intervention
- **Cost**: Reduce failed requests

**Implementation**:

- Exponential backoff with jitter
- Error categorization (retryable vs non-retryable)
- Configurable max retries

### 4. Why Enhanced JSON Extraction?

- **Robustness**: Handle mixed text/JSON responses
- **Flexibility**: Work with various AI providers
- **Reliability**: Repair common JSON issues

**Implementation**:

- Markdown code block stripping
- JSON boundary detection
- Common issue repair (missing commas, etc.)

### 5. Why Modular Architecture?

- **Maintainability**: Easy to understand and modify
- **Testability**: Isolated components
- **Extensibility**: Easy to add new features

## Performance Considerations

### 1. Parallel Parsing

- **Chunk Size**: 10,000 lines per chunk
- **Thread Pool**: Tokio runtime manages threads
- **Memory**: Streaming to avoid loading entire file

### 2. Parallel AI Analysis

- **Concurrency**: Default 5, configurable 1-20
- **Batching**: Process groups in parallel batches
- **Ordering**: Maintain input order using indexed futures

### 3. Caching

- **SQLite**: Local cache for AI responses
- **Key**: Hash of error pattern
- **Expiry**: No expiry (manual clear)

### 4. Memory Management

- **Streaming**: Process logs in chunks
- **Arc**: Shared ownership for parsers
- **Drop**: Explicit cleanup where needed

## Extension Points

### 1. Adding a New Log Format

1. Create parser in `src/parser/formats/`
2. Implement `LogParser` trait
3. Add to format detector
4. Add tests

**Example**:

```rust
pub struct MyFormatParser;

impl LogParser for MyFormatParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        // Parse logic
    }
}
```

### 2. Adding a New AI Provider

1. Create provider in `src/ai/providers/`
2. Implement `AIProvider` trait
3. Add to `create_provider()` function
4. Add configuration support

**Example**:

```rust
pub struct MyAIProvider {
    api_key: String,
}

#[async_trait]
impl AIProvider for MyAIProvider {
    async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis> {
        // Analysis logic
    }
}
```

### 3. Adding a New Output Format

1. Create formatter in `src/output/`
2. Implement `OutputFormatter` trait
3. Add to output format matching
4. Add tests

**Example**:

```rust
pub struct MyFormatter;

impl OutputFormatter for MyFormatter {
    fn format(&self, groups: &[ErrorGroup]) -> Result<String> {
        // Formatting logic
    }
}
```

### 4. Adding a New Command

1. Create command in `src/commands/`
2. Add to CLI definitions
3. Add to main.rs command matching
4. Add tests

**Example**:

```rust
pub struct MyCommand;

impl MyCommand {
    pub async fn execute(opts: MyOptions) -> Result<()> {
        // Command logic
    }
}
```

## Testing Strategy

### Unit Tests

- Located in same file as implementation
- Test individual functions and methods
- Mock external dependencies

### Integration Tests

- Located in `tests/` directory
- Test end-to-end workflows
- Use real parsers and analyzers

### Doc Tests

- Embedded in documentation comments
- Ensure examples compile and run
- Validate public API usage

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on:

- Code style
- Testing requirements
- Pull request process
- Community guidelines

## Resources

- [Project Structure](PROJECT_STRUCTURE.md)
- [Development Guide](DEVELOPMENT.md)
- [Usage Guide](USAGE.md)
- [API Documentation](https://docs.rs/logai)

## Questions?

- Open an issue on GitHub
- Join discussions
- Check existing documentation

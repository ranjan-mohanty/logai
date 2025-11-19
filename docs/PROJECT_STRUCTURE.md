# LogAI Project Structure

```text
logai/
├── .github/              # GitHub configuration
│   ├── ISSUE_TEMPLATE/   # Issue templates
│   ├── workflows/        # CI/CD workflows (release, security, CI)
│   ├── dependabot.yml    # Dependency updates
│   └── PULL_REQUEST_TEMPLATE.md
│
├── docs/                 # Documentation
│   ├── COMPATIBILITY.md  # Log format compatibility guide
│   ├── DEVELOPMENT.md    # Development guide
│   ├── PROJECT_STRUCTURE.md  # Project structure overview
│   └── USAGE.md          # Detailed usage guide
│
├── examples/             # Example usage and sample logs
│   ├── logs/             # Sample log files
│   └── README.md         # Example usage scenarios
│
├── hooks/                # Git hooks
│   ├── commit-msg        # Commit message validation
│   └── pre-commit        # Pre-commit checks (fmt, clippy, tests)
│
├── scripts/              # Installation and utility scripts
│   ├── homebrew/         # Homebrew formula
│   ├── install.sh        # Quick install script
│   └── README.md         # Installation guide
│
├── src/                  # Source code
│   ├── ai/               # AI provider integrations & analysis
│   │   ├── providers/    # AI provider implementations
│   │   │   ├── claude.rs     # Claude provider
│   │   │   ├── gemini.rs     # Gemini provider
│   │   │   ├── ollama.rs     # Ollama provider
│   │   │   ├── openai.rs     # OpenAI provider
│   │   │   └── mod.rs        # Providers module
│   │   ├── cache.rs          # Analysis caching
│   │   ├── config.rs         # Configuration management
│   │   ├── json_extractor.rs # Enhanced JSON extraction
│   │   ├── mcp_helper.rs     # MCP integration helper
│   │   ├── parallel.rs       # Parallel analysis infrastructure
│   │   ├── progress.rs       # Progress tracking
│   │   ├── prompts.rs        # Shared prompts
│   │   ├── provider.rs       # Provider trait
│   │   ├── retry.rs          # Retry logic with backoff
│   │   ├── statistics.rs     # Analysis statistics
│   │   └── mod.rs            # AI module
│   │
│   ├── analyzer/         # Log analysis
│   │   ├── grouper.rs    # Error grouping logic
│   │   └── mod.rs        # Analyzer module
│   │
│   ├── cli/              # Command-line interface
│   │   └── mod.rs        # CLI definitions
│   │
│   ├── mcp/              # Model Context Protocol integration
│   │   ├── client.rs     # MCP client implementation
│   │   ├── config.rs     # MCP configuration
│   │   ├── error.rs      # MCP error types
│   │   ├── protocol.rs   # MCP protocol types
│   │   ├── transport.rs  # MCP transport layer
│   │   └── mod.rs        # MCP module
│   │
│   ├── output/           # Output formatters
│   │   ├── terminal.rs   # Terminal output
│   │   └── mod.rs        # Output module
│   │
│   ├── parser/           # Log parsers
│   │   ├── formats/      # Format-specific parsers
│   │   │   ├── apache.rs     # Apache log parser
│   │   │   ├── json.rs       # JSON log parser
│   │   │   ├── nginx.rs      # Nginx log parser
│   │   │   ├── plain.rs      # Plain text parser
│   │   │   ├── syslog.rs     # Syslog parser
│   │   │   └── mod.rs        # Formats module
│   │   ├── config.rs         # Parser configuration
│   │   ├── detector.rs       # Format detection
│   │   ├── encoding.rs       # Encoding handling
│   │   ├── metadata.rs       # Metadata extraction
│   │   ├── parallel.rs       # Parallel parsing
│   │   ├── stack_trace.rs    # Stack trace parsing
│   │   ├── statistics.rs     # Parsing statistics
│   │   ├── timestamp.rs      # Timestamp parsing
│   │   └── mod.rs            # Parser module
│   │
│   ├── search/           # Search functionality (future)
│   ├── storage/          # Storage layer (future)
│   ├── lib.rs            # Library entry point
│   └── main.rs           # Binary entry point
│
├── tests/                # Integration tests
│   ├── fixtures/         # Test log files
│   └── integration_test.rs
│
├── .gitignore            # Git ignore rules
├── .pre-commit-config.yaml  # Pre-commit framework config
├── .prettierrc.json      # Prettier configuration
├── Cargo.toml            # Rust package manifest
├── CHANGELOG.md          # Version history
├── CONTRIBUTING.md       # Contribution guidelines
├── LICENSE               # MIT License
├── Makefile              # Development commands
└── README.md             # Project overview

```

## Key Directories

### `/src`

Core application code organized by functionality:

- **ai/** - AI provider integrations, parallel analysis, retry logic, and
  progress tracking
  - `providers/` - AI provider implementations (OpenAI, Claude, Gemini, Ollama)
  - `parallel.rs` - Parallel analysis infrastructure with concurrency control
  - `retry.rs` - Retry logic with exponential backoff
  - `progress.rs` - Real-time progress tracking
  - `statistics.rs` - Analysis statistics and metrics
  - `json_extractor.rs` - Enhanced JSON extraction from AI responses
  - `config.rs` - Configuration management for AI and analysis settings
- **analyzer/** - Log analysis and error grouping
- **cli/** - Command-line interface definitions
- **mcp/** - Model Context Protocol integration for external tools
- **output/** - Output formatting (terminal, JSON, HTML)
- **parser/** - Log format detection and parsing
  - `formats/` - Format-specific parsers (Apache, Nginx, Syslog, JSON, Plain)
  - `parallel.rs` - Parallel log parsing
  - `detector.rs` - Automatic format detection
  - `metadata.rs` - Metadata extraction (file, line, thread, etc.)
  - `stack_trace.rs` - Stack trace parsing
  - `timestamp.rs` - Timestamp parsing with multiple format support

### `/docs`

User-facing documentation:

- Compatibility guide for supported log formats
- Development guide for contributors
- Quick start guide for new users
- Detailed usage guide

### `/examples`

Sample log files and usage examples to help users get started quickly.

### `/scripts`

Installation scripts and package definitions:

- Quick install script for curl-based installation
- Homebrew formula for tap installation

### `/tests`

Integration tests with fixture log files to ensure reliability.

### `/.github`

GitHub-specific configuration:

- CI/CD workflows for automated testing and releases
- Issue and PR templates
- Dependabot for dependency updates

## Configuration Files

- **Cargo.toml** - Rust package configuration
- **Makefile** - Development shortcuts
- **.prettierrc.json** - Markdown formatting
- **.gitignore** - Git ignore patterns
- **hooks/pre-commit** - Git pre-commit checks

## Recent Improvements

### AI Analysis Optimization (v0.1.0-beta.1)

- **Parallel Processing** - Process multiple error groups concurrently (5x
  faster)
- **Retry Logic** - Automatic retry with exponential backoff for transient
  failures
- **Progress Tracking** - Real-time progress updates with throughput and ETA
- **Statistics** - Comprehensive analysis metrics and reporting
- **Configuration** - File-based configuration with CLI overrides

### Code Organization

- **Total Lines:** ~8,300
- **Modules:** 8 main modules
- **Submodules:** 2 (ai/providers, parser/formats)
- **Tests:** 203 tests (161 unit + 42 integration)
- **Test Coverage:** Comprehensive coverage of core functionality

## Future Improvements

See [REFACTORING_PLAN.md](REFACTORING_PLAN.md) for planned structural
improvements:

1. **Phase 1:** Extract commands from main.rs (reduce from 527 to ~100 lines)
2. **Phase 2:** Split ai/config.rs into focused modules
3. **Phase 3:** Reorganize AI module with subdirectories
4. **Phase 4:** Reorganize parser module with subdirectories

## Development Workflow

1. Make changes in `/src`
2. Run `make check` (fmt, lint, test)
3. Pre-commit hook runs automatically
4. Create PR with tests
5. CI runs on all platforms
6. Merge triggers release workflow

## Architecture Principles

- **Modularity** - Clear separation of concerns
- **Testability** - Comprehensive unit and integration tests
- **Documentation** - Module-level docs and examples
- **Performance** - Parallel processing where beneficial
- **Maintainability** - Clean code with clear ownership

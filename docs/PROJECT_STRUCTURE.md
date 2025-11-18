# LogAI Project Structure

```
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
│   ├── QUICKSTART.md     # Quick start guide
│   └── USAGE.md          # Detailed usage guide
│
├── examples/             # Example usage and sample logs
│   ├── logs/             # Sample log files
│   └── README.md         # Example usage scenarios
│
├── hooks/                # Git hooks
│   ├── pre-commit        # Pre-commit checks (fmt, clippy, tests)
│   └── README.md         # Hook documentation
│
├── scripts/              # Installation and utility scripts
│   ├── homebrew/         # Homebrew formula
│   ├── install.sh        # Quick install script
│   └── README.md         # Installation guide
│
├── src/                  # Source code
│   ├── ai/               # AI provider integrations
│   │   ├── providers/    # AI provider implementations
│   │   │   ├── claude.rs     # Claude provider
│   │   │   ├── gemini.rs     # Gemini provider
│   │   │   ├── ollama.rs     # Ollama provider
│   │   │   ├── openai.rs     # OpenAI provider
│   │   │   └── mod.rs        # Providers module
│   │   ├── cache.rs      # Analysis caching
│   │   ├── config.rs     # Configuration management
│   │   ├── mod.rs        # AI module
│   │   ├── prompts.rs    # Shared prompts
│   │   └── provider.rs   # Provider trait
│   │
│   ├── analyzer/         # Log analysis
│   │   ├── grouper.rs    # Error grouping logic
│   │   └── mod.rs        # Analyzer module
│   │
│   ├── cli/              # Command-line interface
│   │   └── mod.rs        # CLI definitions
│   │
│   ├── output/           # Output formatters
│   │   ├── terminal.rs   # Terminal output
│   │   └── mod.rs        # Output module
│   │
│   ├── parser/           # Log parsers
│   │   ├── detector.rs   # Format detection
│   │   ├── json.rs       # JSON parser
│   │   ├── plain.rs      # Plain text parser
│   │   └── mod.rs        # Parser module
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

- **ai/** - AI provider integrations with shared prompts and config
- **analyzer/** - Log analysis and error grouping
- **cli/** - Command-line interface definitions
- **output/** - Output formatting (terminal, JSON, HTML)
- **parser/** - Log format detection and parsing

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

## Development Workflow

1. Make changes in `/src`
2. Run `make check` (fmt, lint, test)
3. Pre-commit hook runs automatically
4. Create PR with tests
5. CI runs on all platforms
6. Merge triggers release workflow

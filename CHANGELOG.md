<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to LogAI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2024-11-19

### Fixed

- **Amazon Linux 2 compatibility** - Added musl static builds for AL2 and older
  Linux distributions
- Build on Ubuntu 20.04 for better glibc compatibility (2.31 vs 2.39)
- Added platform-specific installation instructions for AL2

### Changed

- Release workflow now builds both standard and musl variants for Linux
- Updated documentation with AL2 installation guide and troubleshooting

## [0.1.0] - 2024-11-19

### Added

- Multiple log format support (JSON, plain text, Apache, Nginx, Syslog)
- Automatic format detection
- AI-powered error analysis (OpenAI, Claude, Gemini, Ollama, AWS Bedrock)
- **AWS Bedrock integration** with support for:
  - Claude 3.5 Sonnet (anthropic.claude-3-5-sonnet-20241022-v2:0)
  - Claude 3 Haiku (anthropic.claude-3-haiku-20240307-v1:0)
  - Llama 3.2 90B (meta.llama3-2-90b-instruct-v1:0)
  - Titan Text Premier (amazon.titan-text-premier-v1:0)
- **AWS credentials support** via environment variables, ~/.aws/credentials, and
  IAM roles
- **Region configuration** via CLI flag (--region), config file, or environment
  variable
- **Bedrock-specific configuration** (region, model, max_tokens, temperature)
- **Default AI provider configuration** via `ai.provider` config key
- **Verbose/debug logging** via `--verbose` or `-v` flag
- **Interactive HTML report** with search, expandable details, and AI analysis
- Parallel AI analysis with configurable concurrency
- Automatic retry with exponential backoff
- Response caching
- Configuration file support (~/.logai/config.toml)
- Configuration CLI commands (set/show) for easy setup
- MCP (Model Context Protocol) integration
- Rich terminal output with colors and formatting
- Comprehensive documentation (Quick Start, API, Deployment, FAQ, etc.)
- 13 Bedrock tests (8 unit + 5 integration)
- Debug logging for troubleshooting

### Changed

- Improved error grouping algorithm
- Enhanced JSON extraction from AI responses
- Better progress tracking with ETA
- Removed duplicate count from "Show All Occurrences" button (count already
  displayed in error badge)
- Improved terminal output with clickable hyperlink to open HTML report in
  browser

## [0.1.0-beta.1] - 2025-11-18

### Added

- Initial beta release
- Basic log parsing (JSON and plain text)
- Error grouping and deduplication
- AI integration (OpenAI, Claude, Gemini, Ollama)
- CLI interface with `investigate` command
- Terminal output

---

For detailed changes, see the
[commit history](https://github.com/ranjan-mohanty/logai/commits/main).

# Changelog

All notable changes to LogAI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

> These changes will be released as v0.1.0 (first stable release)

### Added

- Multiple log format support (JSON, plain text, Apache, Nginx, Syslog)
- Automatic format detection
- AI-powered error analysis (OpenAI, Claude, Gemini, Ollama)
- Parallel AI analysis with configurable concurrency
- Automatic retry with exponential backoff
- Response caching
- Configuration file support (~/.logai/config.toml)
- MCP (Model Context Protocol) integration
- Rich terminal output with colors and formatting
- Comprehensive documentation (Quick Start, API, Deployment, FAQ, etc.)
- 175+ tests (unit + integration)

### Changed

- Improved error grouping algorithm
- Enhanced JSON extraction from AI responses
- Better progress tracking with ETA

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

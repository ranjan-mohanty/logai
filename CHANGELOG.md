# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- JSON and plain text log parsing with auto-detection
- Intelligent error grouping with dynamic value normalization
- Beautiful terminal output with colors and formatting
- JSON output format for integration
- AI-powered error analysis with multiple providers (OpenAI, Claude, Gemini,
  Ollama)
- SQLite-based response caching to reduce API costs
- Configuration management with `~/.logai/config.toml`
- `logai config` commands for managing settings
- Multiple installation methods (cargo, curl script, homebrew, pre-built
  binaries)
- **MCP (Model Context Protocol) integration** - Connect external tools and data
  sources
- MCP client with stdio transport support
- Tool discovery and invocation with timeout management
- CLI flags: `--no-mcp`, `--mcp-config` for MCP control
- Enhanced AI analysis with MCP tool results
- Graceful degradation when MCP tools are unavailable

[Unreleased]: https://github.com/ranjan-mohanty/logai/commits/main

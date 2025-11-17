# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-11-17

### Added
- Initial release of Sherlog
- JSON and plain text log parsing with auto-detection
- Intelligent error grouping with dynamic value normalization
- Error deduplication and frequency tracking
- Beautiful terminal output with colors and formatting
- JSON output format for integration
- AI-powered error analysis with multiple providers:
  - OpenAI (GPT-4, GPT-4o-mini)
  - Claude (Claude 3.5 Sonnet, Haiku)
  - Gemini (Gemini 1.5 Flash, Pro)
  - Ollama (local models: Llama 3.2, Mistral, etc.)
- SQLite-based response caching to reduce API costs
- Cache hit/miss statistics
- CLI with investigate command
- Support for environment variables for API keys
- Comprehensive test suite
- MIT license

### Features
- Parse logs from files or stdin
- Group similar errors intelligently
- Track error frequency and timing
- AI explanations and fix suggestions
- Response caching
- Multiple output formats (terminal, JSON)

[Unreleased]: https://github.com/ranjan-mohanty/sherlog/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ranjan-mohanty/sherlog/releases/tag/v0.1.0

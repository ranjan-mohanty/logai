# 🤖 LogAI

[![CI](https://github.com/ranjan-mohanty/logai/workflows/CI/badge.svg)](https://github.com/ranjan-mohanty/logai/actions)
[![Crates.io](https://img.shields.io/crates/v/logai.svg)](https://crates.io/crates/logai)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**AI-powered log analysis** - Parse, group, and understand your logs with AI.

LogAI analyzes your application logs, groups similar errors, and uses AI to
explain what went wrong and how to fix it.

## What is LogAI?

LogAI is a CLI tool that analyzes application logs, groups similar errors, and
provides intelligent suggestions for fixing issues. Stop manually searching
through massive log files and let LogAI do the detective work.

## Features

✅ Parse JSON and plain text logs  
✅ Auto-detect log format  
✅ Group similar errors intelligently  
✅ Deduplicate repeated errors  
✅ Beautiful terminal output  
✅ Track error frequency and timing  
✅ AI-powered error explanations (OpenAI, Claude, Gemini, Ollama)  
✅ Solution suggestions with code examples  
✅ Response caching to reduce API costs  
✅ **MCP (Model Context Protocol) integration** - Connect external tools and
data sources

## Coming Soon

🚧 Built-in MCP tools (search_docs, check_metrics, search_code)  
🚧 Watch mode for real-time analysis  
🚧 HTML reports  
🚧 Advanced log format support (Apache, Nginx, Syslog)

## Quick Start

## Installation

### Quick Install (macOS/Linux)

```bash
curl -sSL https://raw.githubusercontent.com/ranjan-mohanty/logai/main/scripts/install.sh | bash
```

### Homebrew (macOS/Linux)

```bash
brew install https://raw.githubusercontent.com/ranjan-mohanty/logai/main/scripts/homebrew/logai.rb
```

### Cargo (All platforms)

```bash
cargo install logai
```

### Pre-built Binaries

Download from
[GitHub Releases](https://github.com/ranjan-mohanty/logai/releases/latest):

- macOS (Intel & Apple Silicon)
- Linux (x86_64 & ARM64)
- Windows (x86_64)

### From Source

```bash
git clone https://github.com/ranjan-mohanty/logai.git
cd logai
cargo install --path .
```

## Usage

Analyze a log file:

```bash
logai investigate app.log
```

Analyze multiple files:

```bash
logai investigate app.log error.log
```

Pipe logs from stdin:

```bash
tail -f app.log | logai investigate -
cat error.log | logai investigate -
```

Limit output:

```bash
logai investigate app.log --limit 10
```

JSON output:

```bash
logai investigate app.log --format json
```

## AI-Powered Analysis

Analyze with OpenAI:

```bash
export OPENAI_API_KEY=sk-...
logai investigate app.log --ai openai
logai investigate app.log --ai openai --model gpt-4
```

Analyze with Claude:

```bash
export ANTHROPIC_API_KEY=sk-ant-...
logai investigate app.log --ai claude
logai investigate app.log --ai claude --model claude-3-5-sonnet-20241022
```

Analyze with Gemini:

```bash
export GEMINI_API_KEY=...
logai investigate app.log --ai gemini
logai investigate app.log --ai gemini --model gemini-1.5-pro
```

Analyze with Ollama (local, free):

```bash
# Make sure Ollama is running: ollama serve
logai investigate app.log --ai ollama
logai investigate app.log --ai ollama --model llama3.2
```

Disable caching (force fresh analysis):

```bash
logai investigate app.log --ai openai --no-cache
```

## MCP Integration (Advanced)

LogAI supports [Model Context Protocol (MCP)](https://modelcontextprotocol.io/)
to connect external tools and data sources during analysis.

Create `~/.logai/mcp.toml`:

```toml
default_timeout = 30

[[servers]]
name = "filesystem"
enabled = true

[servers.connection]
type = "Stdio"
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
```

Use with MCP tools:

```bash
logai investigate app.log --ai ollama --mcp-config ~/.logai/mcp.toml
```

Disable MCP:

```bash
logai investigate app.log --ai ollama --no-mcp
```

See [MCP Integration Guide](docs/MCP_INTEGRATION.md) for more details.

## Example Output

```text
🤖 LogAI Analysis Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Summary
   Errors found: 3 unique patterns (9 occurrences)
   Time range: 2025-11-17 10:30:00 - 2025-11-17 10:35:00

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 Critical: Connection failed to database (3 occurrences)
   First seen: 5 minutes ago | Last seen: 4 minutes ago

   📋 Example:
   Connection failed to database
   📍 Location: db.rs:42

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 Critical: Timeout waiting for response from <DYNAMIC> (3 occurrences)
   First seen: 1 minute ago | Last seen: 30 seconds ago

   📋 Example:
   Timeout waiting for response from api.example.com
```

## Supported Log Formats

- **JSON logs** - Structured logs with fields like `level`, `message`,
  `timestamp`
- **Plain text logs** - Traditional text logs with timestamps and severity
  levels
- More formats coming soon (syslog, etc.)

## Development

Build:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Run with sample logs:

```bash
cargo run -- investigate tests/fixtures/sample.log
```

## Supported AI Providers

| Provider   | Models                   | Cost | Speed  | Setup            |
| ---------- | ------------------------ | ---- | ------ | ---------------- |
| **OpenAI** | GPT-4, GPT-4o-mini       | Paid | Fast   | API key required |
| **Claude** | Claude 3.5 Sonnet/Haiku  | Paid | Fast   | API key required |
| **Gemini** | Gemini 1.5 Flash/Pro     | Paid | Fast   | API key required |
| **Ollama** | Llama 3.2, Mistral, etc. | Free | Medium | Local install    |

## How It Works

1. **Parse** - Automatically detects log format (JSON, plain text)
2. **Group** - Clusters similar errors by normalizing dynamic values
3. **Deduplicate** - Shows unique patterns with occurrence counts
4. **Analyze** - Uses AI to explain errors and suggest fixes (optional)
5. **Cache** - Stores AI responses locally to reduce costs

## Roadmap

- [x] Core parsing and grouping
- [x] AI integration (OpenAI, Claude, Gemini, Ollama)
- [x] Response caching
- [x] MCP (Model Context Protocol) integration
- [ ] Built-in MCP tools (search_docs, check_metrics, search_code, query_logs)
- [ ] Watch mode for real-time analysis
- [ ] HTML reports
- [ ] Advanced log format support (Apache, Nginx, Syslog)
- [ ] Anomaly detection and trend analysis

## Documentation

- [Usage Guide](docs/USAGE.md) - Comprehensive usage examples
- [Compatibility](docs/COMPATIBILITY.md) - Supported log formats
- [Contributing](CONTRIBUTING.md) - How to contribute
- [Changelog](CHANGELOG.md) - Version history

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for
guidelines.

## Future Plans

See [GitHub Issues](https://github.com/ranjan-mohanty/logai/issues) for planned
features and known issues.

## License

MIT License - see [LICENSE](LICENSE) file

## Author

Built with ❤️ by [Ranjan Mohanty](https://github.com/ranjan-mohanty)

## Acknowledgments

- Inspired by the need for better log debugging tools
- Thanks to all AI providers for making this possible
- Built with Rust 🦀

## Star History

If you find LogAI useful, please consider giving it a star ⭐

## Support

- 🐛
  [Report a bug](https://github.com/ranjan-mohanty/logai/issues/new?labels=bug)
- 💡
  [Request a feature](https://github.com/ranjan-mohanty/logai/issues/new?labels=enhancement)
- 💬 [Start a discussion](https://github.com/ranjan-mohanty/logai/discussions)

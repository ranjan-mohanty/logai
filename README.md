# 🔍 Sherlog

**Elementary, my dear developer** - AI-powered log analyzer that helps you debug faster.

## What is Sherlog?

Sherlog is a CLI tool that analyzes application logs, groups similar errors, and provides intelligent suggestions for fixing issues. Stop manually searching through massive log files and let Sherlog do the detective work.

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

## Coming Soon  
🚧 Stack Overflow and GitHub search integration  
🚧 Watch mode for real-time analysis  
🚧 HTML reports  
🚧 Case history and caching  

## Installation

### From source (requires Rust)

```bash
git clone https://github.com/yourusername/sherlog.git
cd sherlog
cargo install --path .
```

### Pre-built binaries

Coming soon!

## Quick Start

Analyze a log file:
```bash
sherlog investigate app.log
```

Analyze multiple files:
```bash
sherlog investigate app.log error.log
```

Pipe logs from stdin:
```bash
tail -f app.log | sherlog investigate -
cat error.log | sherlog investigate -
```

Limit output:
```bash
sherlog investigate app.log --limit 10
```

JSON output:
```bash
sherlog investigate app.log --format json
```

## AI-Powered Analysis

Analyze with OpenAI:
```bash
export OPENAI_API_KEY=sk-...
sherlog investigate app.log --ai openai
sherlog investigate app.log --ai openai --model gpt-4
```

Analyze with Claude:
```bash
export ANTHROPIC_API_KEY=sk-ant-...
sherlog investigate app.log --ai claude
sherlog investigate app.log --ai claude --model claude-3-5-sonnet-20241022
```

Analyze with Gemini:
```bash
export GEMINI_API_KEY=...
sherlog investigate app.log --ai gemini
sherlog investigate app.log --ai gemini --model gemini-1.5-pro
```

Analyze with Ollama (local, free):
```bash
# Make sure Ollama is running: ollama serve
sherlog investigate app.log --ai ollama
sherlog investigate app.log --ai ollama --model llama3.2
```

Disable caching (force fresh analysis):
```bash
sherlog investigate app.log --ai openai --no-cache
```

## Example Output

```
🔍 Sherlog Investigation Report
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

- **JSON logs** - Structured logs with fields like `level`, `message`, `timestamp`
- **Plain text logs** - Traditional text logs with timestamps and severity levels
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

| Provider | Models | Cost | Speed | Setup |
|----------|--------|------|-------|-------|
| **OpenAI** | GPT-4, GPT-4o-mini | Paid | Fast | API key required |
| **Claude** | Claude 3.5 Sonnet/Haiku | Paid | Fast | API key required |
| **Gemini** | Gemini 1.5 Flash/Pro | Paid | Fast | API key required |
| **Ollama** | Llama 3.2, Mistral, etc. | Free | Medium | Local install |

## How It Works

1. **Parse** - Automatically detects log format (JSON, plain text)
2. **Group** - Clusters similar errors by normalizing dynamic values
3. **Deduplicate** - Shows unique patterns with occurrence counts
4. **Analyze** - Uses AI to explain errors and suggest fixes (optional)
4. **Cache** - Stores AI responses locally to reduce costs

## Roadmap

- [x] Core parsing and grouping
- [x] AI integration (OpenAI, Claude, Gemini, Ollama)
- [x] Response caching
- [ ] Watch mode for real-time analysis
- [ ] HTML reports
- [ ] Stack Overflow integration
- [ ] Configuration file support

## Contributing

Contributions welcome! This is an early-stage project.

## License

MIT License - see LICENSE file

## Author

Built with ❤️ for the developer community

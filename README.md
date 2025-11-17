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
✅ AI-powered error explanations (OpenAI, Ollama)  
✅ Solution suggestions with code examples  

## Coming Soon

🚧 More AI providers (Claude, Gemini)  
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

Analyze with Ollama (local, free):
```bash
# Make sure Ollama is running: ollama serve
sherlog investigate app.log --ai ollama
sherlog investigate app.log --ai ollama --model llama3.2
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

## Roadmap

- [x] Phase 1: Core parsing and grouping (Week 1-2)
- [ ] Phase 2: AI integration (Week 2-3)
- [ ] Phase 3: Advanced features (Week 3-4)
- [ ] Phase 4: Distribution (Week 4)

See [sherlog-spec.md](../sherlog-spec.md) for detailed roadmap.

## Contributing

Contributions welcome! This is an early-stage project.

## License

MIT License - see LICENSE file

## Author

Built with ❤️ for the developer community

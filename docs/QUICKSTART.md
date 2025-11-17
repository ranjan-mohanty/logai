# Sherlog Quick Start

Get started with Sherlog in 5 minutes!

## Installation

### Option 1: From Source (Requires Rust)

```bash
cargo install sherlog
```

### Option 2: Download Binary

Download from
[GitHub Releases](https://github.com/ranjan-mohanty/sherlog/releases/latest):

**macOS:**

```bash
# Intel
curl -L https://github.com/ranjan-mohanty/sherlog/releases/latest/download/sherlog-macos-x86_64 -o sherlog
chmod +x sherlog
sudo mv sherlog /usr/local/bin/

# Apple Silicon
curl -L https://github.com/ranjan-mohanty/sherlog/releases/latest/download/sherlog-macos-aarch64 -o sherlog
chmod +x sherlog
sudo mv sherlog /usr/local/bin/
```

**Linux:**

```bash
curl -L https://github.com/ranjan-mohanty/sherlog/releases/latest/download/sherlog-linux-x86_64 -o sherlog
chmod +x sherlog
sudo mv sherlog /usr/local/bin/
```

**Windows:** Download `sherlog-windows-x86_64.exe` and add to PATH.

## First Analysis

### 1. Basic Analysis (No AI)

```bash
sherlog investigate your-app.log
```

This will:

- Parse your logs
- Group similar errors
- Show frequency and timing
- Display in beautiful terminal output

### 2. With AI Analysis (Recommended)

**Using OpenAI (fastest setup):**

```bash
export OPENAI_API_KEY=sk-...
sherlog investigate your-app.log --ai openai
```

**Using Ollama (free, local):**

```bash
# Install Ollama first: https://ollama.ai
ollama pull llama3.2
ollama serve

# Then analyze
sherlog investigate your-app.log --ai ollama
```

## Example Output

```
🔍 Sherlog Investigation Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Summary
   Errors found: 3 unique patterns (47 occurrences)
   Time range: 2025-11-17 10:30:00 - 2025-11-17 14:45:32

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔴 Critical: Connection failed to database (47 occurrences)
   First seen: 2 hours ago | Last seen: 30 seconds ago

   📍 Location: db.rs:42

   🎯 Explanation:
   The database connection is failing, likely due to network issues or
   the database server being unavailable.

   🔍 Root Cause:
   Connection timeout or incorrect database credentials

   💡 Suggested Fixes:

   1. Check database server status and network connectivity

   2. Verify connection string and credentials:
      DATABASE_URL=postgresql://user:pass@localhost:5432/db

   3. Increase connection timeout in your config
```

## Common Use Cases

### Debugging Production Issues

```bash
# Quick overview
sherlog investigate production.log --limit 5

# Deep analysis with AI
sherlog investigate production.log --ai openai --limit 3
```

### Analyzing Docker Logs

```bash
docker logs my-container 2>&1 | sherlog investigate -
```

### Kubernetes Logs

```bash
kubectl logs deployment/my-app | sherlog investigate -
```

### CI/CD Integration

```bash
# Generate JSON report
sherlog investigate test.log -f json -o report.json
```

## Tips for Best Results

1. **Start without AI** to see what errors exist
2. **Use AI for top errors** with `--limit 3`
3. **Cache saves money** - run multiple times without extra cost
4. **Use Ollama for privacy** - keeps logs local
5. **Pipe from anywhere** - works with any log source

## Next Steps

- Read the [Full Usage Guide](USAGE.md)
- Check out [Examples](USAGE.md#examples)
- Learn about [AI Providers](USAGE.md#ai-providers)
- See [Advanced Features](USAGE.md#advanced-features)

## Getting Help

- 📖 [Documentation](https://github.com/ranjan-mohanty/sherlog)
- 🐛 [Report Issues](https://github.com/ranjan-mohanty/sherlog/issues)
- 💬 [Discussions](https://github.com/ranjan-mohanty/sherlog/discussions)

## What's Next?

Try analyzing your own logs and see how Sherlog helps you debug faster!

```bash
sherlog investigate --help
```

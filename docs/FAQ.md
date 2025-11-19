# LogAI Frequently Asked Questions

Common questions and answers about LogAI.

## Table of Contents

- [General Questions](#general-questions)
- [Installation & Setup](#installation--setup)
- [Usage Questions](#usage-questions)
- [AI Provider Questions](#ai-provider-questions)
- [Performance Questions](#performance-questions)
- [Configuration Questions](#configuration-questions)
- [Troubleshooting](#troubleshooting)
- [Advanced Topics](#advanced-topics)

## General Questions

### What is LogAI?

LogAI is a command-line tool that analyzes application logs using AI. It
automatically:

- Parses logs in various formats (JSON, plain text, etc.)
- Groups similar errors together
- Uses AI to explain what went wrong
- Suggests solutions to fix the issues

### Why use LogAI instead of grep/awk/traditional tools?

Traditional tools require you to know what you're looking for. LogAI:

- **Automatically discovers patterns** you might miss
- **Groups similar errors** to reduce noise
- **Explains root causes** using AI
- **Suggests fixes** with code examples
- **Saves time** by processing thousands of lines instantly

### Is LogAI free?

Yes! LogAI is open-source (MIT license) and free to use. However:

- **AI analysis** may incur costs depending on your provider:
  - Ollama: Free (runs locally)
  - OpenAI/Claude/Gemini: Paid API usage
- **Basic parsing and grouping** is always free (use `--ai none`)

### What log formats does LogAI support?

Currently supported:

- JSON structured logs
- Plain text logs with timestamps
- Apache access/error logs (coming soon)
- Nginx access/error logs (coming soon)
- Syslog format (coming soon)

See [Compatibility Guide](COMPATIBILITY.md) for details.

## Installation & Setup

### How do I install LogAI?

Multiple options:

```bash
# Cargo (recommended)
cargo install logai

# Homebrew (macOS/Linux)
brew install logai

# From source
git clone https://github.com/your-org/logai.git
cd logai
cargo install --path .

# Pre-built binaries
# Download from GitHub Releases
```

### What are the system requirements?

- **OS**: macOS, Linux, or Windows
- **Rust**: 1.70+ (if building from source)
- **Memory**: 256MB minimum, 1GB+ recommended for large files
- **Disk**: 50MB for binary, additional space for cache

### Do I need an AI provider to use LogAI?

No! You can use LogAI without AI:

```bash
# Parse and group errors without AI
logai investigate app.log --ai none
```

This is useful for:

- Quick error grouping
- Testing log parsing
- Environments without internet access
- Cost-sensitive scenarios

### Which AI provider should I choose?

| Use Case              | Recommended Provider | Why                                   |
| --------------------- | -------------------- | ------------------------------------- |
| **Getting started**   | Ollama               | Free, runs locally, no API key needed |
| **Best quality**      | OpenAI GPT-4         | Most accurate analysis                |
| **Best value**        | Ollama or Gemini     | Free (Ollama) or low cost (Gemini)    |
| **Privacy-sensitive** | Ollama               | All data stays local                  |
| **High volume**       | Ollama               | No API costs                          |

## Usage Questions

### How do I analyze a log file?

Basic usage:

```bash
# Auto-detect format, no AI
logai investigate app.log

# With AI analysis
logai investigate app.log --ai ollama

# Multiple files
logai investigate app.log error.log --ai ollama

# From stdin
tail -f app.log | logai investigate - --ai ollama
```

### How do I limit the output?

```bash
# Show only top 5 error groups
logai investigate app.log --ai ollama --limit 5

# Filter by severity
logai investigate app.log --ai ollama --severity error
```

### Can I analyze logs in real-time?

Currently, LogAI processes files in batch mode. Real-time watch mode is planned:

```bash
# Workaround: Use tail with pipe
tail -f app.log | logai investigate - --ai ollama
```

### How do I get JSON output?

```bash
# JSON format for programmatic use
logai investigate app.log --ai ollama --format json > analysis.json

# Pretty-printed JSON
logai investigate app.log --ai ollama --format json | jq .
```

### Can I analyze logs from multiple servers?

Yes! Combine logs first:

```bash
# Collect logs from multiple servers
ssh server1 'cat /var/log/app.log' > combined.log
ssh server2 'cat /var/log/app.log' >> combined.log
ssh server3 'cat /var/log/app.log' >> combined.log

# Analyze combined logs
logai investigate combined.log --ai ollama
```

Or use a log aggregation tool (ELK, Splunk, etc.) and export to a file.

## AI Provider Questions

### How do I set up Ollama?

```bash
# 1. Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# 2. Pull a model
ollama pull llama3.2

# 3. Start Ollama (if not running)
ollama serve

# 4. Use with LogAI
logai investigate app.log --ai ollama
```

### How do I set up OpenAI?

```bash
# 1. Get API key from https://platform.openai.com/api-keys

# 2. Set environment variable
export OPENAI_API_KEY="sk-your-key-here"

# 3. Use with LogAI
logai investigate app.log --ai openai

# Optional: Use specific model
logai investigate app.log --ai openai --model gpt-4
```

### How do I set up Claude?

```bash
# 1. Get API key from https://console.anthropic.com/

# 2. Set environment variable
export ANTHROPIC_API_KEY="sk-ant-your-key-here"

# 3. Use with LogAI
logai investigate app.log --ai claude
```

### How much does AI analysis cost?

Approximate costs per 1000 error groups:

| Provider | Model       | Cost   | Notes              |
| -------- | ----------- | ------ | ------------------ |
| Ollama   | Any         | $0     | Free, runs locally |
| OpenAI   | GPT-4o-mini | ~$0.50 | Fast and cheap     |
| OpenAI   | GPT-4       | ~$5.00 | Best quality       |
| Claude   | Haiku       | ~$0.30 | Fast and cheap     |
| Claude   | Sonnet      | ~$3.00 | High quality       |
| Gemini   | Flash       | ~$0.10 | Very cheap         |
| Gemini   | Pro         | ~$1.00 | Good quality       |

**Cost-saving tips:**

- Use caching (enabled by default)
- Use `--limit` to analyze fewer groups
- Use Ollama for free analysis
- Use cheaper models for initial analysis

### Does LogAI send my logs to AI providers?

Only when using AI analysis (`--ai <provider>`):

- **With AI**: Error patterns and examples are sent to the AI provider
- **Without AI** (`--ai none`): Nothing is sent, all processing is local

**Privacy tips:**

- Use Ollama for complete privacy (runs locally)
- Sanitize logs before analysis
- Use `--ai none` for sensitive logs
- Review cached responses in `~/.logai/cache/`

### Can I use my own AI model?

Yes! Use Ollama with any compatible model:

```bash
# Pull any Ollama model
ollama pull mistral
ollama pull codellama
ollama pull llama3.2:70b

# Use with LogAI
logai investigate app.log --ai ollama --model mistral
```

## Performance Questions

### How fast is LogAI?

**Parsing performance:**

- ~100,000 lines/second (single-threaded)
- Scales with CPU cores for large files

**AI analysis performance:**

- Sequential: ~1 group/second
- Parallel (default): ~5 groups/second
- Parallel (high): ~15 groups/second

**Example:** 100 error groups

- Sequential: ~25 minutes
- Default (concurrency=5): ~5 minutes
- High (concurrency=15): ~2 minutes

### How do I make analysis faster?

```bash
# 1. Increase concurrency (for local Ollama)
logai investigate app.log --ai ollama --concurrency 15

# 2. Use a faster model
ollama pull llama3.2:8b  # Smaller, faster
logai investigate app.log --ai ollama --model llama3.2:8b

# 3. Limit analysis scope
logai investigate app.log --ai ollama --limit 10

# 4. Enable caching (default)
# Subsequent runs will be much faster

# 5. Use parallel processing for large files
# (automatically enabled)
```

### Why is analysis slow?

Common causes:

1. **Low concurrency**: Increase with `--concurrency`
2. **Slow AI model**: Use a faster model
3. **Network latency**: Use local Ollama instead of API
4. **Large error groups**: Use `--limit` to reduce scope
5. **No caching**: Ensure caching is enabled (default)

### How much memory does LogAI use?

Typical memory usage:

- **Small files** (<1MB): ~50MB
- **Medium files** (1-100MB): ~100-500MB
- **Large files** (>100MB): ~500MB-1GB

Memory is released after processing. For very large files, use streaming:

```bash
# Process in chunks
tail -f large.log | logai investigate - --ai ollama
```

## Configuration Questions

### Where is the configuration file?

Default location: `~/.logai/config.toml`

Create it manually or use:

```bash
mkdir -p ~/.logai
cat > ~/.logai/config.toml << 'EOF'
default_provider = "ollama"

[analysis]
max_concurrency = 5
enable_cache = true

[providers.ollama]
enabled = true
model = "llama3.2"
EOF
```

### What can I configure?

See [Configuration Guide](QUICK_START.md#configuration) for full details.

Key settings:

- Default AI provider
- Concurrency level
- Retry behavior
- Cache settings
- Provider-specific options

### How do I disable caching?

```bash
# Disable for one run
logai investigate app.log --ai ollama --no-cache

# Disable in config
[analysis]
enable_cache = false

# Clear cache
rm -rf ~/.logai/cache/
```

### Can I use different configs for different projects?

Yes! Use the `--config` flag:

```bash
# Project-specific config
logai investigate app.log --ai ollama --config ./project-config.toml
```

## Troubleshooting

### "Connection refused" error with Ollama

**Problem:** Can't connect to Ollama

**Solution:**

```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# Start Ollama
ollama serve

# Verify model is available
ollama list
ollama pull llama3.2
```

### "Invalid API key" error

**Problem:** AI provider authentication failed

**Solution:**

```bash
# Check environment variable
echo $OPENAI_API_KEY
echo $ANTHROPIC_API_KEY

# Set correct key
export OPENAI_API_KEY="sk-your-actual-key"

# Or set in config file
[providers.openai]
api_key = "sk-your-actual-key"
```

### "Failed to parse" warnings

**Problem:** Many lines not parsed correctly

**Solution:**

```bash
# 1. Check log format
head -5 app.log

# 2. Force specific format
logai investigate app.log --format json --ai ollama

# 3. Check file encoding
file app.log
```

### Out of memory errors

**Problem:** System runs out of memory

**Solution:**

```bash
# 1. Process smaller files
head -10000 large.log > sample.log
logai investigate sample.log --ai ollama

# 2. Reduce concurrency
logai investigate app.log --ai ollama --concurrency 1

# 3. Use streaming
tail -f app.log | logai investigate - --ai ollama
```

### Slow analysis

See [Performance Questions](#performance-questions) above.

## Advanced Topics

### Can I use LogAI as a library?

Yes! See [API Documentation](API.md) for details.

```rust
use logai::parser::create_parser;
use logai::analyzer::Analyzer;

let parser = create_parser("json")?;
let entries = parser.parse_file("app.log")?;

let analyzer = Analyzer::new();
let groups = analyzer.analyze(&entries)?;
```

### Can I add custom log formats?

Yes! Implement the `LogParser` trait:

```rust
use logai::parser::{LogParser, LogEntry};

pub struct MyParser;

impl LogParser for MyParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        // Your parsing logic
    }

    fn name(&self) -> &str {
        "myformat"
    }
}
```

See [API Documentation](API.md#adding-a-new-log-format) for details.

### Can I add custom AI providers?

Yes! Implement the `AIProvider` trait:

```rust
use logai::ai::{AIProvider, AIAnalysis};

pub struct MyProvider;

#[async_trait]
impl AIProvider for MyProvider {
    async fn analyze(&self, group: &ErrorGroup) -> Result<AIAnalysis> {
        // Your AI logic
    }

    fn name(&self) -> &str {
        "myprovider"
    }
}
```

See [API Documentation](API.md#adding-a-new-ai-provider) for details.

### How do I integrate LogAI with CI/CD?

See [Deployment Guide](DEPLOYMENT.md#cicd-integration) for examples:

- GitHub Actions
- GitLab CI
- Jenkins
- CircleCI

Example GitHub Action:

```yaml
- name: Analyze logs
  run: |
    logai investigate test-logs/ --ai openai --format json > analysis.json

- name: Check for critical errors
  run: |
    if jq -e '.groups[] | select(.severity == "Critical")' analysis.json; then
      exit 1
    fi
```

### How do I deploy LogAI in production?

See [Deployment Guide](DEPLOYMENT.md) for:

- Docker deployment
- Kubernetes deployment
- Systemd service
- Monitoring integration
- Scaling strategies

### What is MCP integration?

MCP (Model Context Protocol) allows LogAI to connect external tools and data
sources during analysis.

Example use cases:

- Query metrics from Prometheus
- Search documentation
- Query databases for context
- Search code repositories

See [MCP Integration Guide](MCP_INTEGRATION.md) for details.

### How do I contribute to LogAI?

See [Contributing Guide](../CONTRIBUTING.md) for:

- Development setup
- Code style guidelines
- Testing requirements
- Pull request process

Quick start:

```bash
# Fork and clone
git clone https://github.com/your-username/logai.git
cd logai

# Build and test
cargo build
cargo test

# Make changes and submit PR
```

## Still Have Questions?

- **Bug reports**: [GitHub Issues](https://github.com/your-org/logai/issues)
- **Feature requests**:
  [GitHub Issues](https://github.com/your-org/logai/issues)
- **General questions**:
  [GitHub Discussions](https://github.com/your-org/logai/discussions)
- **Documentation**: Check the [docs/](.) directory

## Quick Reference

### Common Commands

```bash
# Basic analysis
logai investigate app.log

# With AI
logai investigate app.log --ai ollama

# Multiple files
logai investigate *.log --ai ollama

# High performance
logai investigate app.log --ai ollama --concurrency 15

# Limited output
logai investigate app.log --ai ollama --limit 10

# JSON output
logai investigate app.log --ai ollama --format json

# No caching
logai investigate app.log --ai ollama --no-cache

# Show statistics
logai investigate app.log --ai ollama --stats

# Configuration
logai config show
logai config set analysis.max_concurrency 10
```

### Environment Variables

```bash
# AI provider API keys
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GEMINI_API_KEY="..."

# Logging
export RUST_LOG=debug

# Config directory
export LOGAI_CONFIG_DIR="~/.logai"
```

### File Locations

```bash
# Configuration
~/.logai/config.toml

# Cache
~/.logai/cache/

# MCP config
~/.logai/mcp.toml
```

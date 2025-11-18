# LogAI Usage Guide

Complete guide to using LogAI for log analysis.

## Table of Contents

- [Basic Usage](#basic-usage)
- [AI Providers](#ai-providers)
- [Output Formats](#output-formats)
- [Advanced Features](#advanced-features)
- [Examples](#examples)
- [Tips & Tricks](#tips--tricks)

## Basic Usage

### Analyze a Single File

```bash
logai investigate app.log
```

### Analyze Multiple Files

```bash
logai investigate app.log error.log debug.log
```

### Analyze from stdin

```bash
tail -f app.log | logai investigate -
cat error.log | logai investigate -
kubectl logs pod-name | logai investigate -
docker logs container-id | logai investigate -
```

### Limit Output

```bash
logai investigate app.log --limit 5
```

## AI Providers

### OpenAI

**Setup:**

```bash
export OPENAI_API_KEY=sk-...
```

**Usage:**

```bash
# Default model (gpt-4o-mini)
logai investigate app.log --ai openai

# Specific model
logai investigate app.log --ai openai --model gpt-4

# One-time API key
logai investigate app.log --ai openai --api-key sk-...
```

**Models:**

- `gpt-4o-mini` (default, fast, cheap)
- `gpt-4` (more accurate, slower, expensive)
- `gpt-3.5-turbo` (fast, cheap)

### Claude

**Setup:**

```bash
export ANTHROPIC_API_KEY=sk-ant-...
```

**Usage:**

```bash
# Default model (claude-3-5-haiku)
logai investigate app.log --ai claude

# Specific model
logai investigate app.log --ai claude --model claude-3-5-sonnet-20241022
```

**Models:**

- `claude-3-5-haiku-20241022` (default, fast, cheap)
- `claude-3-5-sonnet-20241022` (balanced)
- `claude-3-opus-20240229` (most capable)

### Gemini

**Setup:**

```bash
export GEMINI_API_KEY=...
```

**Usage:**

```bash
# Default model (gemini-1.5-flash)
logai investigate app.log --ai gemini

# Specific model
logai investigate app.log --ai gemini --model gemini-1.5-pro
```

**Models:**

- `gemini-1.5-flash` (default, fast, cheap)
- `gemini-1.5-pro` (more capable)

### Ollama (Local)

**Setup:**

```bash
# Install Ollama: https://ollama.ai
ollama pull llama3.2
ollama serve
```

**Usage:**

```bash
# Default model (llama3.2)
logai investigate app.log --ai ollama

# Specific model
logai investigate app.log --ai ollama --model mistral

# Custom host
logai investigate app.log --ai ollama --ollama-host http://localhost:11434
```

**Popular Models:**

- `llama3.2` (default, good balance)
- `mistral` (fast, efficient)
- `codellama` (code-focused)
- `phi` (small, fast)

## Output Formats

### Terminal (Default)

Beautiful colored output with emojis and formatting:

```bash
logai investigate app.log
```

### JSON

Machine-readable output for integration:

```bash
logai investigate app.log --format json
logai investigate app.log -f json > output.json
```

### Save to File

```bash
logai investigate app.log --output report.txt
logai investigate app.log -o report.json -f json
```

## Advanced Features

### Caching

LogAI automatically caches AI responses to reduce costs.

**View cache statistics:**

```bash
logai investigate app.log --ai openai
# Output: 💾 Cache: 3 hits, 2 misses
```

**Disable caching:**

```bash
logai investigate app.log --ai openai --no-cache
```

**Cache location:**

- Linux/macOS: `~/.logai/cache/cache.db`
- Windows: `%USERPROFILE%\.logai\cache\cache.db`

### Filtering

**By severity:**

```bash
logai investigate app.log --severity error
logai investigate app.log --severity warn
```

## Examples

### Production Debugging

```bash
# Quick analysis without AI
logai investigate production.log --limit 10

# Deep analysis with AI
logai investigate production.log --ai openai --limit 5
```

### CI/CD Integration

```bash
# Generate JSON report
logai investigate test.log -f json -o report.json

# Check if errors exist (exit code)
logai investigate test.log || echo "Errors found!"
```

### Docker Logs

```bash
docker logs my-container 2>&1 | logai investigate -
```

### Kubernetes Logs

```bash
kubectl logs deployment/my-app | logai investigate -
kubectl logs -f pod/my-pod | logai investigate -
```

### Multiple Services

```bash
logai investigate \
  service1.log \
  service2.log \
  service3.log \
  --ai openai \
  --limit 20
```

### Cost-Effective Analysis

```bash
# Use cache for repeated analysis
logai investigate app.log --ai openai

# Use local Ollama (free)
logai investigate app.log --ai ollama

# Use cheaper models
logai investigate app.log --ai openai --model gpt-4o-mini
```

## Tips & Tricks

### 1. Start Without AI

First run without AI to see what errors exist:

```bash
logai investigate app.log
```

Then use AI for specific errors:

```bash
logai investigate app.log --ai openai --limit 3
```

### 2. Use Local Models for Privacy

For sensitive logs, use Ollama:

```bash
logai investigate sensitive.log --ai ollama
```

### 3. Combine with Other Tools

```bash
# Filter logs first
grep ERROR app.log | logai investigate -

# Analyze recent logs
tail -1000 app.log | logai investigate -

# Analyze specific time range
sed -n '/2025-11-17 10:00/,/2025-11-17 11:00/p' app.log | logai investigate -
```

### 4. Save Money with Caching

Cache persists across runs. Analyze the same logs multiple times without extra
cost:

```bash
logai investigate app.log --ai openai  # Calls API
logai investigate app.log --ai openai  # Uses cache
```

### 5. JSON for Automation

```bash
# Parse with jq
logai investigate app.log -f json | jq '.[] | select(.count > 10)'

# Count error types
logai investigate app.log -f json | jq 'length'

# Extract patterns
logai investigate app.log -f json | jq '.[].pattern'
```

## Environment Variables

- `OPENAI_API_KEY` - OpenAI API key
- `ANTHROPIC_API_KEY` - Claude API key
- `GEMINI_API_KEY` - Gemini API key
- `HOME` / `USERPROFILE` - Cache directory location

## Exit Codes

- `0` - Success
- `1` - Error (invalid arguments, file not found, etc.)

## Getting Help

```bash
logai --help
logai investigate --help
```

## Troubleshooting

### "API key not provided"

Set the appropriate environment variable:

```bash
export OPENAI_API_KEY=sk-...
```

### "Connection refused" (Ollama)

Make sure Ollama is running:

```bash
ollama serve
```

### "No log entries found"

Check file path and format. LogAI supports JSON and plain text logs.

### Slow performance

- Use `--limit` to reduce output
- Use faster models (gpt-4o-mini, claude-3-5-haiku)
- Use local Ollama
- Disable AI with `--ai none`

## More Information

- [GitHub Repository](https://github.com/ranjan-mohanty/logai)
- [Contributing Guide](../CONTRIBUTING.md)
- [Changelog](../CHANGELOG.md)

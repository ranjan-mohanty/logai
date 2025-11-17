# Sherlog Usage Guide

Complete guide to using Sherlog for log analysis.

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
sherlog investigate app.log
```

### Analyze Multiple Files

```bash
sherlog investigate app.log error.log debug.log
```

### Analyze from stdin

```bash
tail -f app.log | sherlog investigate -
cat error.log | sherlog investigate -
kubectl logs pod-name | sherlog investigate -
docker logs container-id | sherlog investigate -
```

### Limit Output

```bash
sherlog investigate app.log --limit 5
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
sherlog investigate app.log --ai openai

# Specific model
sherlog investigate app.log --ai openai --model gpt-4

# One-time API key
sherlog investigate app.log --ai openai --api-key sk-...
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
sherlog investigate app.log --ai claude

# Specific model
sherlog investigate app.log --ai claude --model claude-3-5-sonnet-20241022
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
sherlog investigate app.log --ai gemini

# Specific model
sherlog investigate app.log --ai gemini --model gemini-1.5-pro
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
sherlog investigate app.log --ai ollama

# Specific model
sherlog investigate app.log --ai ollama --model mistral

# Custom host
sherlog investigate app.log --ai ollama --ollama-host http://localhost:11434
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
sherlog investigate app.log
```

### JSON

Machine-readable output for integration:

```bash
sherlog investigate app.log --format json
sherlog investigate app.log -f json > output.json
```

### Save to File

```bash
sherlog investigate app.log --output report.txt
sherlog investigate app.log -o report.json -f json
```

## Advanced Features

### Caching

Sherlog automatically caches AI responses to reduce costs.

**View cache statistics:**
```bash
sherlog investigate app.log --ai openai
# Output: 💾 Cache: 3 hits, 2 misses
```

**Disable caching:**
```bash
sherlog investigate app.log --ai openai --no-cache
```

**Cache location:**
- Linux/macOS: `~/.sherlog/cache/cache.db`
- Windows: `%USERPROFILE%\.sherlog\cache\cache.db`

### Filtering

**By severity:**
```bash
sherlog investigate app.log --severity error
sherlog investigate app.log --severity warn
```

## Examples

### Production Debugging

```bash
# Quick analysis without AI
sherlog investigate production.log --limit 10

# Deep analysis with AI
sherlog investigate production.log --ai openai --limit 5
```

### CI/CD Integration

```bash
# Generate JSON report
sherlog investigate test.log -f json -o report.json

# Check if errors exist (exit code)
sherlog investigate test.log || echo "Errors found!"
```

### Docker Logs

```bash
docker logs my-container 2>&1 | sherlog investigate -
```

### Kubernetes Logs

```bash
kubectl logs deployment/my-app | sherlog investigate -
kubectl logs -f pod/my-pod | sherlog investigate -
```

### Multiple Services

```bash
sherlog investigate \
  service1.log \
  service2.log \
  service3.log \
  --ai openai \
  --limit 20
```

### Cost-Effective Analysis

```bash
# Use cache for repeated analysis
sherlog investigate app.log --ai openai

# Use local Ollama (free)
sherlog investigate app.log --ai ollama

# Use cheaper models
sherlog investigate app.log --ai openai --model gpt-4o-mini
```

## Tips & Tricks

### 1. Start Without AI

First run without AI to see what errors exist:
```bash
sherlog investigate app.log
```

Then use AI for specific errors:
```bash
sherlog investigate app.log --ai openai --limit 3
```

### 2. Use Local Models for Privacy

For sensitive logs, use Ollama:
```bash
sherlog investigate sensitive.log --ai ollama
```

### 3. Combine with Other Tools

```bash
# Filter logs first
grep ERROR app.log | sherlog investigate -

# Analyze recent logs
tail -1000 app.log | sherlog investigate -

# Analyze specific time range
sed -n '/2025-11-17 10:00/,/2025-11-17 11:00/p' app.log | sherlog investigate -
```

### 4. Save Money with Caching

Cache persists across runs. Analyze the same logs multiple times without extra cost:
```bash
sherlog investigate app.log --ai openai  # Calls API
sherlog investigate app.log --ai openai  # Uses cache
```

### 5. JSON for Automation

```bash
# Parse with jq
sherlog investigate app.log -f json | jq '.[] | select(.count > 10)'

# Count error types
sherlog investigate app.log -f json | jq 'length'

# Extract patterns
sherlog investigate app.log -f json | jq '.[].pattern'
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
sherlog --help
sherlog investigate --help
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

Check file path and format. Sherlog supports JSON and plain text logs.

### Slow performance

- Use `--limit` to reduce output
- Use faster models (gpt-4o-mini, claude-3-5-haiku)
- Use local Ollama
- Disable AI with `--ai none`

## More Information

- [GitHub Repository](https://github.com/ranjan-mohanty/sherlog)
- [Contributing Guide](../CONTRIBUTING.md)
- [Changelog](../CHANGELOG.md)

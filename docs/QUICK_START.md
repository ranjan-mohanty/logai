# LogAI Quick Start Guide

Get up and running with LogAI in 5 minutes.

## Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build LogAI
git clone https://github.com/your-org/logai.git
cd logai
cargo build --release

# Install to system
cargo install --path .
```

## Setup AI Provider

### Option 1: Ollama (Recommended for Getting Started)

```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Pull a model
ollama pull llama3.2

# Start Ollama (if not running)
ollama serve
```

### Option 2: OpenAI

```bash
# Set your API key
export OPENAI_API_KEY="sk-your-key-here"
```

### Option 3: Claude

```bash
# Set your API key
export ANTHROPIC_API_KEY="sk-ant-your-key-here"
```

## Your First Analysis

### 1. Create a Sample Log File

```bash
cat > sample.log << 'EOF'
2024-01-15 10:30:15 ERROR [UserService] Failed to authenticate user: invalid credentials for user_id=12345
2024-01-15 10:30:16 WARN [DatabasePool] Connection pool exhausted, waiting for available connection
2024-01-15 10:30:17 ERROR [PaymentService] Payment processing failed: insufficient funds for transaction_id=tx_789012
2024-01-15 10:30:18 ERROR [UserService] Failed to authenticate user: invalid credentials for user_id=67890
2024-01-15 10:30:19 ERROR [EmailService] SMTP connection timeout: failed to send email to user@example.com
2024-01-15 10:30:20 ERROR [PaymentService] Payment processing failed: card declined for transaction_id=tx_789013
EOF
```

### 2. Run Basic Analysis

```bash
# Analyze with Ollama
logai investigate sample.log --ai ollama

# Or with OpenAI
logai investigate sample.log --ai openai

# Or just parse without AI
logai investigate sample.log --ai none
```

### 3. Understanding the Output

You'll see:

1. **Parsing Statistics**: How many lines were processed
2. **Error Groups**: Similar errors grouped together
3. **AI Analysis**: Root causes and solutions for each group
4. **Recommendations**: Actionable fixes

Example output:

```
📊 Parsing Statistics:
   Total lines: 6
   Parsed entries: 6 (100.0%)
   Error entries: 5
   Warning entries: 1

🔍 Error Groups Found: 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Group 1: UserService Authentication Failures (2 occurrences)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Pattern: Failed to authenticate user: invalid credentials for user_id=<ID>

🤖 AI Analysis:
Root Cause: Invalid user credentials being submitted
Impact: Users unable to log in, potential security concern
Solution:
  1. Verify user credential validation logic
  2. Check for brute force attempts
  3. Implement rate limiting on authentication endpoints
```

## Common Use Cases

### Analyze Application Logs

```bash
# Single file
logai investigate /var/log/myapp/app.log --ai ollama

# Multiple files
logai investigate /var/log/myapp/*.log --ai ollama

# Specific time range (future feature)
logai investigate app.log --ai ollama --since "2024-01-15 10:00:00"
```

### Analyze Web Server Logs

```bash
# Apache logs
logai investigate /var/log/apache2/error.log --format apache --ai ollama

# Nginx logs
logai investigate /var/log/nginx/error.log --format nginx --ai ollama
```

### High-Performance Analysis

```bash
# Increase concurrency for faster analysis
logai investigate large.log --ai ollama --concurrency 10

# Show performance statistics
logai investigate large.log --ai ollama --stats
```

### Focus on Critical Errors

```bash
# Only analyze errors (skip warnings)
logai investigate app.log --ai ollama --severity error

# Limit to top 5 error groups
logai investigate app.log --ai ollama --limit 5
```

## Configuration

### Create Configuration File

```bash
# Create config directory
mkdir -p ~/.logai

# Create configuration
cat > ~/.logai/config.toml << 'EOF'
# Default AI provider
default_provider = "ollama"

# Analysis settings
[analysis]
max_concurrency = 5
enable_retry = true
max_retries = 3
enable_cache = true

# Ollama configuration
[providers.ollama]
enabled = true
model = "llama3.2"
host = "http://localhost:11434"
EOF
```

### View Current Configuration

```bash
logai config show
```

### Update Configuration

```bash
# Set default provider
logai config set default_provider openai

# Set concurrency
logai config set analysis.max_concurrency 10

# Set Ollama model
logai config set providers.ollama.model llama3.2:8b
```

## Tips for Better Results

### 1. Use Appropriate Concurrency

```bash
# Low concurrency for API providers (rate limits)
logai investigate logs/ --ai openai --concurrency 2

# High concurrency for local Ollama
logai investigate logs/ --ai ollama --concurrency 15
```

### 2. Enable Caching

Caching saves time and API costs on repeated analysis:

```bash
# Caching is enabled by default
logai investigate app.log --ai ollama

# Disable for fresh analysis
logai investigate app.log --ai ollama --no-cache

# Clear cache
rm -rf ~/.logai/cache/
```

### 3. Use the Right Format

```bash
# Auto-detect (default)
logai investigate app.log --ai ollama

# Force specific format for better accuracy
logai investigate app.log --format json --ai ollama
```

### 4. Start Small

```bash
# Test with a small sample first
head -100 large.log > sample.log
logai investigate sample.log --ai ollama

# Then process the full file
logai investigate large.log --ai ollama
```

## Next Steps

### Learn More

- **[Usage Guide](USAGE.md)** - Comprehensive usage examples
- **[Architecture](ARCHITECTURE.md)** - How LogAI works
- **[Troubleshooting](TROUBLESHOOTING.md)** - Common issues and solutions

### Explore Examples

```bash
# Try the example files
cd examples/
logai investigate sample-errors.log --ai ollama
logai investigate spring-boot.log --ai ollama
logai investigate json-structured.log --ai ollama
```

### Customize Your Setup

1. **Configure your preferred AI provider**
2. **Set up aliases for common commands**
3. **Integrate with your monitoring tools**
4. **Create automated analysis scripts**

### Get Help

- **GitHub Issues**: Report bugs or request features
- **GitHub Discussions**: Ask questions and share tips
- **Documentation**: Check the docs/ directory

## Common Commands Reference

```bash
# Basic analysis
logai investigate <file> --ai ollama

# Multiple files
logai investigate <pattern> --ai ollama

# Specific format
logai investigate <file> --format <format> --ai ollama

# High performance
logai investigate <file> --ai ollama --concurrency 15

# Show statistics
logai investigate <file> --ai ollama --stats

# Limit results
logai investigate <file> --ai ollama --limit 10

# Configuration
logai config show
logai config set <key> <value>

# Help
logai --help
logai investigate --help
```

## Troubleshooting Quick Fixes

### Ollama Not Connecting

```bash
# Check if running
curl http://localhost:11434/api/tags

# Start Ollama
ollama serve
```

### Model Not Found

```bash
# Pull the model
ollama pull llama3.2

# List available models
ollama list
```

### Slow Analysis

```bash
# Increase concurrency
logai investigate logs.txt --ai ollama --concurrency 10

# Use faster model
ollama pull llama3.2:8b
logai investigate logs.txt --ai ollama --model llama3.2:8b
```

### Wrong Format Detected

```bash
# Force correct format
logai investigate logs.txt --format json --ai ollama
```

## You're Ready!

You now have LogAI set up and know the basics. Start analyzing your logs and let
AI help you find and fix issues faster! 🚀

For more advanced usage, check out the [full documentation](../README.md).

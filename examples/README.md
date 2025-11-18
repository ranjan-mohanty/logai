# LogAI Examples

This directory contains example log files and usage scenarios.

## Quick Examples

### Basic Analysis (No AI)

```bash
logai investigate examples/logs/nginx-sample.log --ai none
```

### AI-Powered Analysis with Ollama

```bash
logai investigate examples/logs/spring-boot-sample.log --ai ollama --limit 2
```

### AI-Powered Analysis with OpenAI

```bash
export OPENAI_API_KEY="sk-..."
logai investigate examples/logs/cloudwatch-sample.log --ai openai --limit 3
```

### JSON Output

```bash
logai investigate examples/logs/nginx-sample.log --ai none --format json
```

### Multiple Files

```bash
logai investigate examples/logs/*.log --ai ollama
```

## Sample Log Files

- `nginx-sample.log` - Nginx error logs with connection issues
- `spring-boot-sample.log` - Spring Boot application logs with database errors
- `cloudwatch-sample.log` - AWS CloudWatch logs in JSON format

## Configuration Examples

### Configure Ollama

```bash
logai config set ollama.model llama3.1:8b
logai config set ollama.host http://localhost:11434
```

### Configure OpenAI

```bash
logai config set openai.api_key sk-...
logai config set openai.model gpt-4o-mini
```

### View Configuration

```bash
logai config show
```

## Advanced Usage

See [docs/USAGE.md](../docs/USAGE.md) for more detailed examples and options.

# LogAI Compatibility Guide

This document outlines which log formats LogAI supports and any known limitations.

## ✅ Fully Supported Formats

### 1. JSON Logs (95% confidence)

**Works with:**
- Standard JSON with `level`, `message`, `timestamp` fields
- Application logs from Node.js, Python, Go, Rust
- Docker container logs in JSON format
- Kubernetes pod logs (JSON format)
- Most modern logging frameworks

**Example:**
```json
{"level":"error","message":"Connection failed","timestamp":"2025-11-17T10:30:00Z"}
```

**Tested with:**
- ✅ Node.js (Winston, Pino, Bunyan)
- ✅ Python (structlog, python-json-logger)
- ✅ Go (zap, logrus)
- ✅ Rust (tracing, slog)

### 2. Plain Text Logs (85% confidence)

**Works with:**
- Standard format: `[timestamp] LEVEL message`
- Java/Spring Boot logs
- Python logging module
- Ruby on Rails logs
- Basic syslog format

**Example:**
```
2025-11-17 10:30:00.123  ERROR 12345 --- [thread] Class : Message
```

**Tested with:**
- ✅ Spring Boot / Java logs
- ✅ Python logging module
- ✅ Nginx error logs
- ✅ Basic syslog

### 3. CloudWatch Logs (80% confidence)

**Works with:**
- Lambda function logs
- ECS task logs
- EC2 instance logs
- CloudWatch Logs Insights output

**Example:**
```
2025-11-17T10:30:00.123Z	request-id	ERROR	Message
```

**Notes:**
- Tab-separated format works
- Request IDs are normalized automatically
- Timestamps are parsed correctly

### 4. Nginx Logs (85% confidence)

**Works with:**
- Nginx error logs
- Access logs with errors

**Example:**
```
2025/11/17 10:30:00 [error] 12345#12345: *67890 connect() failed
```

**Notes:**
- Process IDs and connection IDs are normalized
- IP addresses are normalized
- Groups similar connection errors

## ⚠️ Partially Supported

### 1. Multi-line Stack Traces (70% confidence)

**Works if:**
- Stack trace is in the message field (JSON)
- Stack trace lines are indented (plain text)

**Limitations:**
- May not perfectly parse all stack trace formats
- Some frameworks have custom formats

**Example that works:**
```
2025-11-17 10:30:00 ERROR Failed to process
	at com.example.Service.method(Service.java:42)
	at com.example.Controller.handle(Controller.java:89)
```

### 2. Custom Application Logs (60% confidence)

**Works if:**
- Has recognizable timestamp
- Has severity level (ERROR, WARN, etc.)
- Has clear error message

**May need adjustment for:**
- Proprietary formats
- Unusual timestamp formats
- Non-standard severity levels

### 3. Syslog (75% confidence)

**Works with:**
- Basic syslog format
- RFC 3164 format

**Limitations:**
- RFC 5424 format may need testing
- Facility/priority codes are treated as text

## ❌ Not Supported

### 1. Binary Formats

- Protobuf logs
- MessagePack logs
- Binary encoded logs

**Workaround:** Decode to JSON or text first

### 2. Compressed Logs

- gzip files
- bzip2 files
- zip archives

**Workaround:** Decompress first:
```bash
gunzip -c app.log.gz | logai investigate -
zcat app.log.gz | logai investigate -
```

### 3. Encrypted Logs

- Encrypted log files
- Logs requiring decryption

**Workaround:** Decrypt first, then analyze

### 4. Database Logs (Direct)

- PostgreSQL logs (need export)
- MySQL logs (need export)
- MongoDB logs (need export)

**Workaround:** Export to file first

## Testing Results

### Real-world Log Sources Tested

| Source | Format | Status | Notes |
|--------|--------|--------|-------|
| Docker containers | JSON | ✅ Works | Perfect |
| Kubernetes pods | JSON | ✅ Works | Perfect |
| Spring Boot | Plain text | ✅ Works | Groups correctly |
| Node.js (Winston) | JSON | ✅ Works | Perfect |
| Python (logging) | Plain text | ✅ Works | Good |
| Nginx | Plain text | ✅ Works | Good |
| CloudWatch Lambda | Tab-separated | ✅ Works | Good |
| Rails | Plain text | 🟡 Partial | Needs testing |
| Apache | Plain text | 🟡 Partial | Needs testing |

## Known Limitations

### 1. Timestamp Parsing

- Supports ISO 8601, RFC 3339, and common formats
- May not parse unusual custom formats
- Falls back to current time if unparseable

### 2. Severity Detection

- Recognizes: ERROR, ERR, FATAL, CRITICAL, WARN, WARNING, INFO, DEBUG, TRACE
- Case-insensitive
- Unknown levels treated as "Unknown"

### 3. Grouping Accuracy

**Normalized automatically:**
- ✅ UUIDs
- ✅ Large numbers (5+ digits)
- ✅ IP addresses
- ✅ URLs
- ✅ File paths with line numbers
- ✅ Thread IDs
- ✅ Timestamps

**May not normalize:**
- Custom ID formats
- Application-specific patterns
- Unusual dynamic values

### 4. Performance

- Handles files up to 1GB efficiently
- Memory usage scales with unique error patterns
- Very large files (10GB+) may need streaming (future feature)

## Improving Compatibility

### For Custom Formats

If your logs aren't grouping correctly:

1. **Check the pattern:**
```bash
logai investigate app.log -f json | jq '.[].pattern'
```

2. **Look for dynamic values that should be normalized**

3. **Open an issue with:**
   - Sample log lines (sanitized)
   - Expected grouping behavior
   - Log format description

### For Better Results

1. **Use structured logging (JSON)** when possible
2. **Include standard fields:** level, message, timestamp
3. **Use consistent formats** across services
4. **Avoid embedding dynamic data** in error messages when possible

## Future Improvements

Planned enhancements:

- [ ] Plugin system for custom parsers
- [ ] More log format auto-detection
- [ ] Better multi-line handling
- [ ] Streaming for huge files
- [ ] Custom normalization rules
- [ ] Format-specific optimizations

## Getting Help

If LogAI doesn't work with your logs:

1. Check this compatibility guide
2. Try with a small sample first
3. Open an issue with:
   - Log format description
   - Sample lines (sanitized)
   - Expected vs actual behavior

## Contributing

Help us improve compatibility:

- Test with your log formats
- Report issues with samples
- Contribute parsers for new formats
- Share success stories

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

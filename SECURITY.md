# Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue in
LogAI, please report it responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:

**security@logai.dev**

Or create a private security advisory on GitHub:
https://github.com/ranjan-mohanty/logai/security/advisories/new

### What to Include

Please include the following information:

- **Description**: Clear description of the vulnerability
- **Impact**: What an attacker could achieve
- **Reproduction**: Step-by-step instructions to reproduce
- **Environment**: LogAI version, OS, Rust version
- **Proof of Concept**: Minimal example if possible

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 5 business days
- **Status Updates**: Every 7 days until resolved
- **Fix Timeline**: We aim to release fixes within 90 days

### Disclosure Policy

We follow responsible disclosure:

- We'll work with you to understand and resolve the issue
- We'll credit you in the security advisory (unless you prefer anonymity)
- Please don't publicly disclose until we've released a fix
- We'll coordinate disclosure timing with you

## Security Considerations

### Data Handling

LogAI processes log files that may contain sensitive information:

- **Local Processing**: Logs are processed locally by default
- **AI Analysis**: When enabled, log excerpts are sent to AI providers
- **Caching**: Analysis results are cached locally in `~/.logai/cache/`
- **API Keys**: Stored in config files or environment variables

### Best Practices for Users

#### Protect Sensitive Data

```bash
# Use environment variables for API keys (not config files)
export OPENAI_API_KEY="your-key"
export ANTHROPIC_API_KEY="your-key"

# Secure config file permissions
chmod 600 ~/.logai/config.toml

# Use Ollama for complete privacy (runs locally)
logai investigate app.log --ai ollama

# Sanitize logs before AI analysis
logai investigate app.log --ai none  # No data sent externally
```

#### Review What's Sent to AI

When using AI analysis:

- Error patterns and examples are sent to the AI provider
- Original log files are NOT uploaded
- Only grouped error samples are analyzed
- Review cached responses: `~/.logai/cache/`

#### Secure Your Environment

```bash
# Run with minimal permissions
sudo -u logai logai investigate /var/log/app.log

# Use read-only access to logs
chmod 444 /var/log/app.log

# Clear cache periodically
rm -rf ~/.logai/cache/
```

## Known Security Considerations

### AI Provider Communication

**Risk**: Log excerpts sent to external AI services

**Mitigation**:

- Use Ollama for local processing (no external calls)
- Sanitize logs before analysis
- Review what's being sent
- Use `--ai none` for sensitive logs

### API Key Storage

**Risk**: API keys in config files or environment

**Mitigation**:

- Use environment variables (not config files)
- Secure file permissions: `chmod 600 ~/.logai/config.toml`
- Never commit config files with keys
- Rotate keys regularly

### File System Access

**Risk**: LogAI reads files from the filesystem

**Mitigation**:

- Run with minimal required permissions
- Validate file paths (directory traversal protection built-in)
- Use appropriate file permissions
- Audit file access patterns

### Dependency Vulnerabilities

**Risk**: Third-party dependencies may have vulnerabilities

**Mitigation**:

- Regular dependency updates via Dependabot
- Security audits with `cargo audit`
- Automated vulnerability scanning in CI
- Minimal dependency footprint

## Security Features

### Input Validation

- File path validation prevents directory traversal
- Log content sanitization before AI analysis
- Configuration validation prevents injection

### Network Security

- All AI provider communications use HTTPS
- Certificate validation enforced
- Timeout and retry limits prevent DoS

### Local Data Protection

- Config files support restricted permissions
- Cache stored in user-specific directories
- Temporary files cleaned up properly

## Security Audits

We perform regular security reviews:

- **Dependency Audits**: Weekly via `cargo audit`
- **Code Reviews**: All PRs reviewed for security
- **Automated Scanning**: CI pipeline security checks
- **Third-party Audits**: Periodic external assessments

## Vulnerability Disclosure

Recent security advisories will be listed here:

_No security advisories published yet._

## Security Resources

- [Rust Security Guidelines](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [OWASP Secure Coding](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [GitHub Security Advisories](https://github.com/advisories)

## Contact

- **Security Issues**: security@logai.dev
- **General Questions**:
  [GitHub Discussions](https://github.com/ranjan-mohanty/logai/discussions)
- **Bug Reports**:
  [GitHub Issues](https://github.com/ranjan-mohanty/logai/issues) (non-security
  only)

## Acknowledgments

We appreciate security researchers who responsibly disclose vulnerabilities.
Contributors will be acknowledged in:

- Security advisories
- CONTRIBUTORS.md
- Release notes

Thank you for helping keep LogAI secure! 🔒

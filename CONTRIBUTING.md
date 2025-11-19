# Contributing to LogAI

Thanks for your interest in contributing to LogAI! This document provides
guidelines for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/logai.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Run clippy: `cargo clippy`
7. Format code: `cargo fmt`
8. Commit with conventional commits: `git commit -m "feat: add new feature"`
9. Push and create a pull request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo
- (Optional) Python 3.7+ for pre-commit framework

### Quick Start

```bash
# Clone the repository
git clone https://github.com/ranjan-mohanty/logai.git
cd logai

# Install Git hooks (recommended)
make install-hooks
# OR using pre-commit framework
pip install pre-commit
pre-commit install

# Build the project
make build

# Run tests
make test
```

### Building

```bash
cargo build
# OR
make build
```

### Running Tests

```bash
cargo test
# OR
make test
```

### Running Locally

```bash
cargo run -- investigate tests/fixtures/sample.log
# OR
make run
```

### Code Quality

Before committing, ensure your code passes all checks:

```bash
make check  # Runs fmt, lint, and test
```

Or run individually:

```bash
make fmt    # Format code
make lint   # Run clippy
make test   # Run tests
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Pass all clippy lints (`cargo clippy`)
- Write tests for new features
- Update documentation for user-facing changes

## Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions or changes
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

Examples:

```text
feat: add support for syslog format
fix: handle empty log files gracefully
docs: update README with new examples
```

## Pull Request Process

1. Update README.md with details of changes if needed
2. Add tests for new functionality
3. Ensure all tests pass
4. Update CHANGELOG.md (if exists)
5. Request review from maintainers

## Adding New Features

### Adding a New Log Parser

1. Create a new file in `src/parser/formats/your_format.rs`
2. Implement the `LogParser` trait:

   ```rust
   pub struct YourFormatParser;

   impl LogParser for YourFormatParser {
       fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
           // Your parsing logic
       }

       fn supports_multiline(&self) -> bool {
           false  // or true if format supports multi-line
       }
   }
   ```

3. Add comprehensive tests with real log examples
4. Update `detector.rs` to recognize the format
5. Add to `formats/mod.rs` exports
6. Update documentation and README

**Example PR**: See existing parsers in `src/parser/formats/`

### Adding a New AI Provider

1. Create a new file in `src/ai/providers/your_provider.rs`
2. Implement the `AIProvider` trait:

   ```rust
   pub struct YourProvider {
       api_key: String,
       model: Option<String>,
   }

   #[async_trait]
   impl AIProvider for YourProvider {
       async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis> {
           // Your API call logic
       }

       fn name(&self) -> &str {
           "your_provider"
       }
   }
   ```

3. Add to `create_provider()` in `src/ai/mod.rs`
4. Add configuration support in `src/ai/config.rs`
5. Update CLI help text in `src/cli/mod.rs`
6. Add tests with mock responses
7. Update README with usage examples and API key setup

**Example PR**: See existing providers in `src/ai/providers/`

### Adding a New Output Format

1. Create a new file in `src/output/your_format.rs`
2. Implement the `OutputFormatter` trait:

   ```rust
   pub struct YourFormatter;

   impl OutputFormatter for YourFormatter {
       fn format(&self, groups: &[ErrorGroup]) -> Result<String> {
           // Your formatting logic
       }
   }
   ```

3. Add to output format matching in `src/commands/investigate.rs`
4. Add tests with expected output
5. Update CLI help text
6. Update documentation

**Example PR**: See `src/output/terminal.rs`

### Adding a New Command

1. Create a new file in `src/commands/your_command.rs`
2. Implement command struct and execute method:

   ```rust
   pub struct YourCommand;

   impl YourCommand {
       pub async fn execute(opts: YourOptions) -> Result<()> {
           // Command logic
       }
   }
   ```

3. Add command to `src/cli/mod.rs`
4. Add to main.rs command matching
5. Add tests
6. Update documentation

**Example PR**: See `src/commands/investigate.rs`

## Good First Issues

Looking for a place to start? Check out issues labeled `good first issue`:

**Easy Contributions**:

- Add new log format parsers (CSV, XML, etc.)
- Improve error messages
- Add more test cases
- Fix typos in documentation
- Add examples to docs

**Medium Contributions**:

- Add new output formats (HTML, CSV)
- Improve AI prompts
- Add new metadata extractors
- Performance optimizations

**Advanced Contributions**:

- Add new AI providers
- Implement watch mode
- Add anomaly detection
- Build web UI

## Architecture Overview

Before contributing, familiarize yourself with the architecture:

- **Parser Layer**: Converts raw logs to structured data
- **Analyzer Layer**: Groups similar errors
- **AI Layer**: Analyzes errors with AI providers
- **Commands Layer**: Business logic for CLI commands
- **Output Layer**: Formats results for display

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed architecture
documentation.

## Testing

### Unit Tests

Test individual functions and methods:

```bash
cargo test --lib
```

### Integration Tests

Test complete workflows:

```bash
cargo test --test '*'
```

### Doc Tests

Test documentation examples:

```bash
cargo test --doc
```

### Test Coverage

We aim for >80% test coverage. Add tests for:

- New features
- Bug fixes
- Edge cases
- Error handling

### Test Fixtures

Add sample log files to `tests/fixtures/` for integration tests.

## Documentation

### Code Documentation

- Add rustdoc comments to public APIs
- Include examples in doc comments
- Explain complex logic with inline comments

### User Documentation

- Update README.md for user-facing changes
- Update docs/ for detailed guides
- Add examples to examples/ directory

### Architecture Documentation

- Update ARCHITECTURE.md for structural changes
- Document design decisions
- Explain trade-offs

## Code of Conduct

This project follows the
[Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating,
you agree to uphold this code. Please report unacceptable behavior to
conduct@logai.dev.

## Questions?

- **General Questions**:
  [GitHub Discussions](https://github.com/ranjan-mohanty/logai/discussions)
- **Bug Reports**:
  [GitHub Issues](https://github.com/ranjan-mohanty/logai/issues)
- **Security Issues**: See [SECURITY.md](SECURITY.md)
- **Support**: See [.github/SUPPORT.md](.github/SUPPORT.md)

## License

By contributing, you agree that your contributions will be licensed under the
MIT License.

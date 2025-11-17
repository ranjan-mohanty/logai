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

1. Create a new file in `src/parser/`
2. Implement the `LogParser` trait
3. Add tests
4. Update `detector.rs` to recognize the format
5. Update documentation

### Adding a New AI Provider

1. Create a new file in `src/ai/`
2. Implement the `AIProvider` trait
3. Add to `create_provider()` in `src/ai/mod.rs`
4. Update CLI help text
5. Update README with usage examples

## Testing

- Unit tests: Test individual functions
- Integration tests: Test complete workflows
- Add test fixtures in `tests/fixtures/`

## Documentation

- Update README.md for user-facing changes
- Add inline documentation for public APIs
- Include examples in documentation

## Questions?

Open an issue or discussion on GitHub!

## License

By contributing, you agree that your contributions will be licensed under the
MIT License.

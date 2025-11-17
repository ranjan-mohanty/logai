# Development Guide

Complete guide for developing LogAI.

## Quick Start

```bash
# Clone and setup
git clone https://github.com/ranjan-mohanty/logai.git
cd logai

# Install Git hooks (maintains code quality)
make install-hooks

# Build and test
make build
make test

# Run logai
make run
```

## Development Workflow

### 1. Make Changes

Edit code in `src/` directory following Rust best practices.

### 2. Run Checks Locally

```bash
make check  # Runs fmt, lint, and test
```

Or individually:
```bash
make fmt    # Format code with rustfmt
make lint   # Run clippy linter
make test   # Run all tests
```

### 3. Commit Changes

Commits must follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git commit -m "feat: add new feature"
git commit -m "fix(parser): handle edge case"
git commit -m "docs: update README"
```

The commit-msg hook will validate your message format.

### 4. Push Changes

```bash
git push origin your-branch
```

CI will run automatically on push.

## Available Make Commands

```bash
make help           # Show all commands
make install        # Install dependencies
make install-hooks  # Install Git hooks
make build          # Build release binary
make test           # Run tests
make fmt            # Format code
make lint           # Run clippy
make check          # Run all checks
make clean          # Clean build artifacts
make run            # Run with sample logs
make run-ai         # Run with AI (needs API key)
make docs           # Generate documentation
```

## Git Hooks

### Pre-commit Hook

Runs automatically before each commit:
- Checks code formatting
- Runs clippy linter
- Runs all tests

### Commit Message Hook

Validates commit messages follow Conventional Commits format.

### Skip Hooks (Not Recommended)

```bash
git commit --no-verify
```

## Testing

### Run All Tests

```bash
cargo test
# OR
make test
```

### Run Specific Test

```bash
cargo test test_name
```

### Run Integration Tests Only

```bash
cargo test --test integration_test
```

### Run with Output

```bash
cargo test -- --nocapture
```

## Code Quality

### Format Code

```bash
cargo fmt --all
# OR
make fmt
```

### Run Clippy

```bash
cargo clippy --all-targets --all-features -- -D warnings
# OR
make lint
```

### Check Without Building

```bash
cargo check
```

## Adding New Features

### 1. Add a New Log Parser

```rust
// src/parser/myformat.rs
use super::LogParser;
use crate::types::LogEntry;
use crate::Result;

pub struct MyFormatParser;

impl LogParser for MyFormatParser {
    fn parse_line(&self, line: &str) -> Result<Option<LogEntry>> {
        // Implementation
    }
    
    fn can_parse(&self, sample: &str) -> bool {
        // Detection logic
    }
}
```

Add to `src/parser/mod.rs` and update detector.

### 2. Add a New AI Provider

```rust
// src/ai/myprovider.rs
use super::provider::AIProvider;
use crate::types::{ErrorAnalysis, ErrorGroup};
use crate::Result;
use async_trait::async_trait;

pub struct MyProvider {
    // Fields
}

#[async_trait]
impl AIProvider for MyProvider {
    async fn analyze(&self, group: &ErrorGroup) -> Result<ErrorAnalysis> {
        // Implementation
    }
    
    fn name(&self) -> &str {
        "myprovider"
    }
}
```

Add to `src/ai/mod.rs` and update `create_provider()`.

### 3. Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_feature() {
        // Test implementation
    }
}
```

## Debugging

### Enable Rust Backtrace

```bash
RUST_BACKTRACE=1 cargo run -- investigate app.log
```

### Enable Logging

```bash
RUST_LOG=debug cargo run -- investigate app.log
```

### Run with Debugger

```bash
rust-lldb target/debug/logai
# OR
rust-gdb target/debug/logai
```

## Performance Profiling

### Build with Profiling

```bash
cargo build --release --profile profiling
```

### Benchmark

```bash
cargo bench
# OR
make bench
```

## Documentation

### Generate Docs

```bash
cargo doc --no-deps --open
# OR
make docs
```

### Write Doc Comments

```rust
/// Brief description
///
/// # Examples
///
/// ```
/// use logai::parser::JsonParser;
/// let parser = JsonParser::new();
/// ```
pub fn my_function() {}
```

## Continuous Integration

CI runs on every push and PR:
- ✅ Format check
- ✅ Clippy linting
- ✅ Tests on Linux, macOS, Windows
- ✅ Build for all platforms

View CI status: [GitHub Actions](https://github.com/ranjan-mohanty/logai/actions)

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit: `git commit -m "chore: bump version to X.Y.Z"`
4. Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
5. Push: `git push && git push --tags`

GitHub Actions will automatically:
- Build binaries for all platforms
- Create GitHub release
- Publish to crates.io

## Troubleshooting

### Build Fails

```bash
cargo clean
cargo build
```

### Tests Fail

```bash
cargo test -- --nocapture
```

### Hooks Not Working

```bash
make install-hooks
```

### Clippy Errors

```bash
cargo clippy --fix --allow-dirty
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [Conventional Commits](https://www.conventionalcommits.org/)

## Getting Help

- Open an [issue](https://github.com/ranjan-mohanty/logai/issues)
- Start a [discussion](https://github.com/ranjan-mohanty/logai/discussions)
- Check [CONTRIBUTING.md](../CONTRIBUTING.md)

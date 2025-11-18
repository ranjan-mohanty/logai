.PHONY: help setup build test fmt lint check clean run docs

# Default target
help:
	@echo "LogAI Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make setup          Setup development environment (hooks + deps)"
	@echo ""
	@echo "Development:"
	@echo "  make build          Build release binary"
	@echo "  make test           Run tests"
	@echo "  make fmt            Format code (Rust + Markdown)"
	@echo "  make lint           Run clippy"
	@echo "  make check          Run all checks (fmt, lint, test)"
	@echo "  make run            Run with example logs"
	@echo ""
	@echo "Documentation:"
	@echo "  make docs           Generate and open Rust docs"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean          Clean build artifacts"

# Setup development environment
setup:
	@echo "🚀 Setting up development environment..."
	@git config core.hooksPath hooks
	@chmod +x hooks/pre-commit
	@cargo fetch
	@echo "✅ Setup complete! Git hooks configured and dependencies fetched."

# Build release binary
build:
	@echo "🔨 Building release binary..."
	@cargo build --release

# Run tests
test:
	@echo "🧪 Running tests..."
	@cargo test --all-features

# Format code (Rust + Markdown)
fmt:
	@echo "📝 Formatting Rust code..."
	@cargo fmt --all
	@if command -v prettier >/dev/null 2>&1; then \
		echo "📄 Formatting markdown..."; \
		prettier --write "*.md" "docs/*.md" "examples/*.md" "scripts/*.md" 2>/dev/null || true; \
	fi

# Run clippy
lint:
	@echo "🔧 Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Run all checks (used by CI)
check: fmt lint test
	@echo "✅ All checks passed!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	@cargo clean

# Run with example logs
run:
	@echo "🔍 Running logai with example logs..."
	@cargo run -- investigate examples/logs/nginx-sample.log --ai none

# Generate and open documentation
docs:
	@echo "📚 Generating documentation..."
	@cargo doc --no-deps --open

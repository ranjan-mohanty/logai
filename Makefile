.PHONY: help install build test fmt lint check clean run install-hooks

# Default target
help:
	@echo "LogAI Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make install        Install dependencies"
	@echo "  make install-hooks  Install Git hooks"
	@echo ""
	@echo "Development:"
	@echo "  make build          Build the project"
	@echo "  make test           Run tests"
	@echo "  make fmt            Format code"
	@echo "  make lint           Run clippy"
	@echo "  make check          Run all checks (fmt, lint, test)"
	@echo "  make run            Run logai with sample logs"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean          Clean build artifacts"

# Install dependencies
install:
	@echo "📦 Installing dependencies..."
	cargo fetch

# Install Git hooks
install-hooks:
	@echo "🪝 Installing Git hooks..."
	@bash hooks/install.sh

# Build the project
build:
	@echo "🔨 Building..."
	cargo build --release

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test --all-features

# Format code
fmt:
	@echo "📝 Formatting code..."
	cargo fmt --all

# Run clippy
lint:
	@echo "🔧 Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

# Run all checks
check: fmt lint test
	@echo "✅ All checks passed!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	cargo clean

# Run logai with sample logs
run:
	@echo "🔍 Running logai..."
	cargo run -- investigate tests/fixtures/sample.log

# Run with AI (requires OPENAI_API_KEY)
run-ai:
	@echo "🤖 Running sherlog with AI..."
	cargo run -- investigate tests/fixtures/sample.log --ai openai --limit 2

# Watch for changes and run tests
watch:
	@echo "👀 Watching for changes..."
	cargo watch -x test

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --open

# Benchmark
bench:
	@echo "⚡ Running benchmarks..."
	cargo bench

# Install pre-commit (Python tool)
install-pre-commit:
	@echo "🐍 Installing pre-commit..."
	pip install pre-commit
	pre-commit install
	pre-commit install --hook-type commit-msg

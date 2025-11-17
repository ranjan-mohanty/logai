# Git Hooks

This directory contains Git hooks to maintain code quality and commit message standards.

## Installation

### Option 1: Simple Installation (Recommended)

```bash
make install-hooks
```

Or manually:
```bash
bash hooks/install.sh
```

### Option 2: Using pre-commit Framework

If you prefer the `pre-commit` framework:

```bash
pip install pre-commit
pre-commit install
pre-commit install --hook-type commit-msg
```

## Available Hooks

### pre-commit

Runs before each commit to ensure code quality:

- ✅ Code formatting check (`cargo fmt`)
- ✅ Linting with clippy (`cargo clippy`)
- ✅ All tests pass (`cargo test`)

### commit-msg

Validates commit messages follow Conventional Commits format:

```
type(optional-scope): subject

body (optional)
```

**Valid types:** feat, fix, docs, style, refactor, perf, test, chore, ci, build, revert

**Examples:**
```
feat: add new feature
fix(parser): handle edge case
docs: update README
```

## Skipping Hooks

If you need to skip hooks temporarily (not recommended):

```bash
git commit --no-verify
```

## Troubleshooting

### Hooks not running

Make sure hooks are executable:
```bash
chmod +x .git/hooks/pre-commit
chmod +x .git/hooks/commit-msg
```

### Hooks failing

Run checks manually to see detailed errors:
```bash
make check
```

Or individually:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Customization

You can modify the hooks in this directory and reinstall them:

```bash
# Edit hooks/pre-commit or hooks/commit-msg
bash hooks/install.sh
```

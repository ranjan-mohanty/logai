# Installation Scripts

This directory contains installation scripts and package definitions for LogAI.

## Quick Install Script

`install.sh` - One-line installer for macOS and Linux:

```bash
curl -sSL https://raw.githubusercontent.com/ranjan-mohanty/logai/main/scripts/install.sh | bash
```

## Homebrew Formula

`homebrew/logai.rb` - Homebrew formula for tap installation:

```bash
brew tap ranjan-mohanty/logai
brew install logai
```

## Manual Installation

### From Cargo

```bash
cargo install logai
```

### From Source

```bash
git clone https://github.com/ranjan-mohanty/logai.git
cd logai
cargo install --path .
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/ranjan-mohanty/logai/releases/latest)

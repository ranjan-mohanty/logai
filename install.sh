#!/bin/bash
# LogAI installer script
set -e

REPO="ranjan-mohanty/logai"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        OS_TYPE="linux"
        ;;
    Darwin*)
        OS_TYPE="macos"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        ARCH_TYPE="x86_64"
        ;;
    arm64|aarch64)
        ARCH_TYPE="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Get latest release
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "Failed to get latest release"
    exit 1
fi

BINARY_NAME="logai-${OS_TYPE}-${ARCH_TYPE}"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/${BINARY_NAME}.tar.gz"

echo "Installing LogAI $LATEST_RELEASE for $OS_TYPE-$ARCH_TYPE..."

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download and extract
echo "Downloading from $DOWNLOAD_URL..."
curl -sL "$DOWNLOAD_URL" | tar xz

# Install
mkdir -p "$INSTALL_DIR"
mv logai "$INSTALL_DIR/logai"
chmod +x "$INSTALL_DIR/logai"

# Cleanup
cd -
rm -rf "$TMP_DIR"

echo "✅ LogAI installed to $INSTALL_DIR/logai"
echo ""
echo "Add to PATH if needed:"
echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
echo ""
echo "Try it:"
echo "  logai --version"

#!/bin/bash

# Installation script for flash
set -e

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Map OS names and determine binary name
case $OS in
    linux)
        TARGET="x86_64-unknown-linux-gnu"
        BINARY_NAME="flash"
        ;;
    darwin)
        if [[ $ARCH == "aarch64" ]]; then
            TARGET="aarch64-apple-darwin"
        else
            TARGET="x86_64-apple-darwin"
        fi
        BINARY_NAME="flash"
        ;;
    mingw*|cygwin*|msys*)
        TARGET="x86_64-pc-windows-gnu"
        BINARY_NAME="flash.exe"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "🔍 Detected platform: $OS-$ARCH (target: $TARGET)"

# Installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Check if releases directory exists
if [[ ! -d "releases" ]]; then
    echo "❌ Releases directory not found. Please run ./build.sh first."
    exit 1
fi

# Check if binary exists
BINARY_PATH="releases/flash-$TARGET"
if [[ $OS == *"windows"* ]]; then
    BINARY_PATH="$BINARY_PATH.exe"
fi

if [[ ! -f "$BINARY_PATH" ]]; then
    echo "❌ Binary not found: $BINARY_PATH"
    echo "Available binaries:"
    ls -la releases/
    exit 1
fi

# Copy binary to install directory
echo "📦 Installing flash to $INSTALL_DIR..."
cp "$BINARY_PATH" "$INSTALL_DIR/flash"
chmod +x "$INSTALL_DIR/flash"

echo "✅ Installation complete!"
echo ""
echo "📋 To use flash, make sure $INSTALL_DIR is in your PATH:"
echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
echo ""
echo "📋 Add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.) to make it permanent"
echo ""
echo "🧪 Test the installation:"
echo "  flash --help"

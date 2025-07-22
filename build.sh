#!/bin/bash

# Build script for flash - Cross-platform compression and upload tool
set -e

echo "🚀 Building flash for multiple platforms..."

# Create release directory
mkdir -p releases

# Build for current platform (development)
echo "📦 Building for current platform..."
cargo build --release

# Define target platforms
TARGETS=(
    "x86_64-unknown-linux-gnu"     # Linux x64
    "x86_64-apple-darwin"          # macOS x64
    "aarch64-apple-darwin"         # macOS ARM64 (M1/M2)
    "x86_64-pc-windows-gnu"        # Windows x64
)

# Install required targets
echo "🔧 Installing required targets..."
for target in "${TARGETS[@]}"; do
    echo "Installing target: $target"
    rustup target add "$target" || echo "Target $target already installed or not available"
done

# Build for each target
echo "🏗️  Building for all platforms..."
for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    
    if cargo build --release --target "$target" 2>/dev/null; then
        # Determine the executable extension
        if [[ "$target" == *"windows"* ]]; then
            ext=".exe"
        else
            ext=""
        fi
        
        # Copy binary to releases directory
        cp "target/$target/release/flash$ext" "releases/flash-$target$ext"
        echo "✅ Successfully built for $target"
    else
        echo "❌ Failed to build for $target (may need cross-compilation setup)"
    fi
done

echo ""
echo "🎉 Build complete! Binaries available in releases/ directory:"
ls -la releases/

echo ""
echo "📋 To test the binary:"
echo "  ./target/release/flash --help"
echo ""
echo "📋 To install locally:"
echo "  cargo install --path ."

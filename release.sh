#!/bin/bash

# Release preparation script
set -e

VERSION=$(grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)
echo "📦 Preparing release for flash v$VERSION"

# Run tests
echo "🧪 Running tests..."
cargo test

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build release
echo "🏗️  Building release..."
./build.sh

# Create release archive
echo "📦 Creating release archives..."
mkdir -p dist

for binary in releases/flash-*; do
    if [[ -f "$binary" ]]; then
        # Extract target from filename
        target=$(basename "$binary" | sed 's/flash-//' | sed 's/\.exe$//')
        
        # Clean up target name for better readability
        clean_target="$target"
        clean_target="${clean_target/x86_64-unknown-linux-gnu/linux-x64}"
        clean_target="${clean_target/x86_64-apple-darwin/macos-x64}"
        clean_target="${clean_target/aarch64-apple-darwin/macos-arm64}"
        clean_target="${clean_target/x86_64-pc-windows-gnu/windows-x64}"
        
        # Create directory for this target
        mkdir -p "dist/flash-v$VERSION-$clean_target"
        
        # Copy binary
        cp "$binary" "dist/flash-v$VERSION-$clean_target/"
        
        # Copy documentation
        cp README.md "dist/flash-v$VERSION-$clean_target/"
        cp LICENSE "dist/flash-v$VERSION-$clean_target/" 2>/dev/null || echo "LICENSE file not found, skipping..."
        
        # Create archive
        cd dist
        if [[ "$target" == *"windows"* ]]; then
            zip -r "flash-v$VERSION-$clean_target.zip" "flash-v$VERSION-$clean_target"
        else
            tar -czf "flash-v$VERSION-$clean_target.tar.gz" "flash-v$VERSION-$clean_target"
        fi
        cd ..
        
        echo "✅ Created archive for $clean_target (from $target)"
    fi
done

echo ""
echo "🎉 Release preparation complete!"
echo "📁 Archives available in dist/ directory:"
ls -la dist/*.{tar.gz,zip} 2>/dev/null || true

echo ""
echo "📋 Next steps for release:"
echo "1. Test the binaries on target platforms"
echo "2. Create a GitHub release"
echo "3. Upload the archives as release assets"
echo "4. Update package managers (brew, chocolatey, etc.)"

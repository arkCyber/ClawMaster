#!/bin/bash
# Build and run Tauri app with full WebUI

set -e

echo "🚀 Building ClawMaster Tauri App with Full WebUI..."
echo ""

# Check if we're in the right directory
if [ ! -f "src-tauri/Cargo.toml" ]; then
    echo "❌ Error: Must run from apps/tauri directory"
    exit 1
fi

# Verify dist directory exists and has content
if [ ! -d "dist" ]; then
    echo "❌ Error: dist directory not found"
    exit 1
fi

echo "✅ Checking dist directory..."
echo "   - HTML: $(ls -lh dist/index.html 2>/dev/null | awk '{print $5}' || echo 'NOT FOUND')"
echo "   - CSS files: $(find dist/css -name '*.css' 2>/dev/null | wc -l | tr -d ' ')"
echo "   - JS files: $(find dist/js -name '*.js' -o -name '*.mjs' 2>/dev/null | wc -l | tr -d ' ')"
echo "   - Icons: $(find dist/icons -type f 2>/dev/null | wc -l | tr -d ' ')"
echo ""

# Build the Tauri app
echo "🔨 Building Tauri application..."
cd src-tauri
cargo build --release
cd ..

echo ""
echo "✅ Build complete!"
echo ""
echo "🎯 Running Tauri app..."
echo ""

# Run the app
cd src-tauri
cargo run --release

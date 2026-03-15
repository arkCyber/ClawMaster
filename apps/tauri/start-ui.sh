#!/bin/bash
# Quick start Tauri UI

set -e

echo "🚀 Starting ClawMaster Tauri UI..."
echo ""

# Check if we're in the right directory
if [ ! -f "src-tauri/Cargo.toml" ]; then
    echo "❌ Error: Must run from apps/tauri directory"
    exit 1
fi

# Check if binary exists
if [ -f "src-tauri/target/release/clawmaster-tauri" ]; then
    echo "✅ Found release binary"
    cd src-tauri
    cargo run --release
elif [ -f "src-tauri/target/debug/clawmaster-tauri" ]; then
    echo "✅ Found debug binary"
    cd src-tauri
    cargo run
else
    echo "🔨 Binary not found, building..."
    cd src-tauri
    cargo build --release
    echo "✅ Build complete, starting..."
    cargo run --release
fi

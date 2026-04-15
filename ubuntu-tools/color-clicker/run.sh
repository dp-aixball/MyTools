#!/bin/bash

# Color Clicker 运行脚本 (Linux/Ubuntu)

echo "🎯 Building Color Clicker..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful"
    echo "🚀 Starting Color Clicker..."
    ./target/release/color-clicker
else
    echo "✗ Build failed"
    exit 1
fi

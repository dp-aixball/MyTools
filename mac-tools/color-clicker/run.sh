#!/bin/bash
# Color Clicker 运行脚本

echo "🎯 启动 Color Clicker..."
echo ""

# 检查是否已编译
if [ ! -f "./target/debug/color-clicker" ]; then
    echo "📦 首次运行,正在编译..."
    cargo build
    echo ""
fi

# 运行程序
echo "运行中... 关闭窗口即可退出"
echo ""
./target/debug/color-clicker

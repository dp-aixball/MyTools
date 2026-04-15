#!/bin/bash

# Color Clicker - Linux 权限检查和诊断工具

echo "========================================="
echo "  Color Clicker - 权限诊断工具"
echo "========================================="
echo ""

# 检查 1: 会话类型
echo "📋 检查 1: 显示服务器类型"
SESSION_TYPE=$(echo $XDG_SESSION_TYPE)
echo "   当前会话: $SESSION_TYPE"
if [ "$SESSION_TYPE" = "x11" ]; then
    echo "   ✅ X11 会话 - 支持自动点击"
else
    echo "   ❌ 非 X11 会话 - 自动点击可能无法工作"
    echo "   💡 建议: 注销后选择 'Ubuntu on Xorg' 登录"
fi
echo ""

# 检查 2: X11 库
echo "📋 检查 2: X11 库"
if dpkg -l | grep -q "libx11-dev"; then
    echo "   ✅ libx11-dev 已安装"
else
    echo "   ⚠️  libx11-dev 未安装"
    echo "   💡 运行: sudo apt install libx11-dev"
fi

if dpkg -l | grep -q "libxtst-dev"; then
    echo "   ✅ libxtst-dev 已安装"
else
    echo "   ⚠️  libxtst-dev 未安装"
    echo "   💡 运行: sudo apt install libxtst-dev"
fi
echo ""

# 检查 3: X11 访问权限
echo "📋 检查 3: X11 访问权限"
if xhost 2>/dev/null | grep -q "LOCAL:"; then
    echo "   ✅ X11 访问正常"
else
    echo "   ⚠️  X11 访问可能受限"
    echo "   💡 运行: xhost +SI:localuser:\$USER"
fi
echo ""

# 检查 4: 窗口管理器
echo "📋 检查 4: 桌面环境"
if [ "$XDG_CURRENT_DESKTOP" = "ubuntu:GNOME" ] || [ "$XDG_CURRENT_DESKTOP" = "GNOME" ]; then
    echo "   桌面环境: GNOME"
    echo "   💡 窗口置顶: 右键窗口标题栏 → 'Always on Top'"
    echo "   💡 或安装 GNOME 扩展: 'Always on Top'"
elif [[ "$XDG_CURRENT_DESKTOP" == *"KDE"* ]]; then
    echo "   桌面环境: KDE"
    echo "   💡 窗口置顶: 窗口规则中设置 '保持置顶'"
else
    echo "   桌面环境: $XDG_CURRENT_DESKTOP"
    echo "   💡 窗口置顶: 查阅您的窗口管理器文档"
fi
echo ""

# 检查 5: 文件权限
echo "📋 检查 5: 程序文件权限"
if [ -f "./target/release/color-clicker" ]; then
    if [ -x "./target/release/color-clicker" ]; then
        echo "   ✅ 可执行文件存在且有执行权限"
    else
        echo "   ⚠️  可执行文件存在但无执行权限"
        echo "   💡 运行: chmod +x ./target/release/color-clicker"
    fi
else
    echo "   ❌ 未找到可执行文件"
    echo "   💡 运行: cargo build --release"
fi
echo ""

# 检查 6: 缩放比例
echo "📋 检查 6: 显示缩放"
if command -v gsettings &> /dev/null; then
    SCALE=$(gsettings get org.gnome.desktop.interface scaling-factor 2>/dev/null)
    if [ ! -z "$SCALE" ]; then
        echo "   GNOME 缩放因子: $SCALE"
        if [ "$SCALE" -gt 1 ] 2>/dev/null; then
            echo "   ⚠️  检测到 HiDPI 缩放"
            echo "   💡 程序已自动处理 DPI 缩放，应该正常工作"
        else
            echo "   ✅ 标准缩放 (100%)"
        fi
    fi
fi
echo ""

echo "========================================="
echo "  诊断完成"
echo "========================================="
echo ""
echo "快速修复命令:"
echo "  sudo apt install libx11-dev libxtst-dev"
echo "  cargo build --release"
echo "  ./target/release/color-clicker"
echo ""

# Color Clicker - 构建指南 (Linux/Ubuntu)

## 系统要求

- Ubuntu 20.04 或更高版本
- Rust 1.70 或更高版本
- X11 显示服务器（Wayland 需要额外配置）

## 安装系统依赖

```bash
# 更新包管理器
sudo apt update

# 安装构建工具和依赖
sudo apt install -y \
    build-essential \
    pkg-config \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libssl-dev \
    libdbus-1-dev \
    libx11-dev
```

## 安装 Rust

如果尚未安装 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## 构建项目

### 开发版本

```bash
cd ubuntu-tools/color-clicker
cargo build
```

### Release 版本（推荐）

```bash
cd ubuntu-tools/color-clicker
cargo build --release
```

编译后的二进制文件位于：`target/release/color-clicker`

## 运行程序

### 方法 1：使用运行脚本

```bash
./run.sh
```

### 方法 2：使用 Cargo

```bash
# 开发模式
cargo run

# Release 模式
cargo run --release
```

### 方法 3：直接运行二进制文件

```bash
./target/release/color-clicker
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_screenshot_colors
```

## 常见问题

### 1. 权限问题

如果程序无法捕获屏幕，可能需要授予权限：

```bash
# 确保在 X11 会话中运行
echo $XDG_SESSION_TYPE
```

如果是 Wayland，需要切换到 X11 或配置 Wayland 权限。

### 2. 依赖缺失

如果遇到编译错误，确保所有系统依赖已安装：

```bash
sudo apt install --reinstall \
    build-essential \
    pkg-config \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev
```

### 3. 屏幕捕获失败

- 确保程序在 X11 环境下运行
- 检查显示器配置是否正常
- 尝试以普通用户权限运行（不需要 sudo）

## 清理构建

```bash
# 清理构建产物
cargo clean

# 清理并重新构建
cargo clean && cargo build --release
```

## 更新依赖

```bash
cargo update
cargo build --release
```

## 性能优化

Release 版本已经启用了优化。如果需要进一步优化：

```bash
# 在 Cargo.toml 的 [profile.release] 中添加：
# [profile.release]
# opt-level = 3
# lto = true
# codegen-units = 1
```

## 打包分发

```bash
# 构建 release 版本
cargo build --release

# 创建压缩包
mkdir -p color-clicker-linux
cp target/release/color-clicker color-clicker-linux/
cp config.json color-clicker-linux/
cp README.md color-clicker-linux/
cp run.sh color-clicker-linux/
chmod +x color-clicker-linux/run.sh

tar -czf color-clicker-linux.tar.gz color-clicker-linux/
```

## 技术栈

- **GUI 框架**: eframe/egui 0.28
- **屏幕捕获**: xcap 0.0.14 (基于 X11)
- **鼠标控制**: enigo 0.2
- **序列化**: serde + serde_json
- **异步运行时**: tokio

## 与 Mac 版本的区别

| 特性 | Mac 版本 | Linux 版本 |
|------|----------|-----------|
| 屏幕捕获库 | screenshots | xcap |
| 显示系统 | Quartz | X11 |
| 系统依赖 | 较少 | 需要 X11 开发库 |
| 权限要求 | 需要屏幕录制权限 | X11 环境下无需额外权限 |

## 许可证

MIT License

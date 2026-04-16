# Color Clicker (Cross-Platform)

悬浮框颜色检测自动点击工具。支持 Windows, macOS 和 Linux (Ubuntu)。

## 功能特点
- **跨平台支持**: 一个工程支持所有主流操作系统。
- **自动点击**: 检测到特定颜色比例时自动触发鼠标点击。
- **透明悬浮窗**: 简洁的 GUI 界面，支持实时预览检测到的颜色占比。
- **配置保存**: 自动保存窗口位置和检测参数。

## 快速开始

### 依赖准备
- **Linux**: 需安装 X11 库
  ```bash
  sudo apt install libx11-dev libxtst-dev
  ```
- **Rust**: 请确保已安装 Rust 环境。

### 运行
```bash
cargo run --release
```

## 项目结构
- `src/main.rs`: 入口逻辑与跨平台初始化。
- `src/app.rs`: 统一的 UI 处理。
- `src/screen.rs`: 屏幕区域提取。
- `src/click.rs`: 鼠标模拟控制。
- `src/color.rs`: 核心颜色算法。
- `src/config.rs`: 配置管理。
- `src/window.rs`: 置顶逻辑处理。

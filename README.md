# MyTools - 个人实用工具集合

## 项目结构

```
MyTools/
├── color-clicker/      # 跨平台颜色检测自动点击器 (Rust)
├── shared/             # 跨平台共享库
├── resources/          # 资源文件 (图标、配置等)
└── README.md           # 项目说明
```

## 工具列表

### 1. Color Clicker (Cross-Platform)
悬浮框颜色检测自动点击工具。支持 Windows, macOS 和 Linux (Ubuntu)。
将悬浮框拖到指定位置，检测到特定颜色占比后自动触发鼠标点击。

## 开发环境

- **语言**: Rust 1.70+
- **构建工具**: Cargo
- **跨平台支持**: Windows (MSVC), macOS (App bundle/DMG), Linux (X11/DEB)

## 使用方法

各工具的具体使用方法请参考其子目录中的 README。

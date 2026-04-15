# Color Clicker 自动构建指南

## 🚀 快速开始

### 发布新版本

当你准备好发布新版本时，只需要推送一个 tag：

```bash
# 1. 确保代码已提交
git add .
git commit -m "准备发布 v0.1.0"

# 2. 创建标签（必须以 v 开头）
git tag v0.1.0

# 3. 推送标签到 GitHub
git push origin v0.1.0
```

推送标签后，GitHub Actions 会自动：
- ✅ 构建 macOS Universal Binary（支持 Intel 和 Apple Silicon）
- ✅ 构建 Linux x86_64 版本
- ✅ 打包为 DMG（macOS）和 tar.gz（Linux）
- ✅ 自动创建 GitHub Release 并上传安装包

## 📦 构建产物

### macOS
- **文件名**: `color-clicker-v0.1.0-macos.dmg`
- **架构**: Universal Binary (Intel + Apple Silicon)
- **包含内容**:
  - `color-clicker` - 可执行文件
  - `config.json` - 配置文件
  - `README.md` - 使用说明

### Linux
- **文件名**: `color-clicker-v0.1.0-linux.tar.gz`
- **架构**: x86_64
- **包含内容**: 同上

## 💿 安装使用

### macOS
1. 从 GitHub Releases 页面下载 `.dmg` 文件
2. 双击打开 DMG
3. 将 `color-clicker` 拖到 Applications 文件夹（可选）
4. 首次运行可能需要授权：
   - **屏幕录制权限**：用于捕获屏幕颜色
   - **辅助功能权限**：用于自动点击
   
   前往 **系统设置 > 隐私与安全性** 授予权限

### Linux
```bash
# 1. 解压
tar -xzf color-clicker-v0.1.0-linux.tar.gz
cd color-clicker

# 2. 添加执行权限
chmod +x color-clicker

# 3. 运行
./color-clicker
```

## 🔧 构建配置

GitHub Actions 工作流文件位于：`.github/workflows/build-release.yml`

### 触发条件
- 推送 `v*` 格式的 tag（如 `v0.1.0`, `v1.0.0`）
- 手动在 GitHub Actions 页面触发

### 自定义构建
如需修改构建流程，编辑 `build-release.yml` 文件。

## ⚠️ 注意事项

1. **标签命名**：必须以 `v` 开头，否则不会触发构建
2. **构建时间**：首次构建约 5-10 分钟（需要下载依赖）
3. **macOS 权限**：必须在系统设置中手动授权屏幕录制和辅助功能
4. **Linux 依赖**：Ubuntu 需要安装 X11 相关库（工作流已自动安装）

## 📝 版本管理建议

遵循语义化版本规范（SemVer）：
- `v0.1.0` - 初始版本
- `v0.2.0` - 新增功能
- `v0.1.1` - Bug 修复
- `v1.0.0` - 正式稳定版本

查看当前版本：
```bash
cd mac-tools/color-clicker
grep version Cargo.toml
```

更新版本后，推送新 tag 即可触发新的构建。

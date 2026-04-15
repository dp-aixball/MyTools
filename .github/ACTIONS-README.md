# GitHub Actions 自动构建说明

## 触发方式

### 方式 1: 推送标签 (推荐)

```bash
# 打标签
git tag v0.1.0

# 推送标签到 GitHub
git push origin v0.1.0
```

这会自动触发构建并创建 Release。

### 方式 2: 手动触发

1. 进入 GitHub 仓库页面
2. 点击 **Actions** 标签
3. 选择 **Build and Release** 工作流
4. 点击 **Run workflow** 按钮

## 构建产物

### macOS
- **文件**: `color-clicker-v0.1.0-macos.dmg`
- **架构**: Universal Binary (Intel + Apple Silicon)
- **内容**: 可执行文件 + 配置文件 + 说明文档

### Linux (Ubuntu)
- **文件**: `color-clicker-v0.1.0-linux.tar.gz`
- **架构**: x86_64
- **内容**: 可执行文件 + 配置文件 + 说明文档

## 使用发布的安装包

### macOS
1. 下载 `.dmg` 文件
2. 双击打开 DMG
3. 将 `color-clicker` 拖到 Applications 文件夹 (可选)
4. 运行程序:
   ```bash
   ./color-clicker
   ```

### Linux
1. 下载 `.tar.gz` 文件
2. 解压:
   ```bash
   tar -xzf color-clicker-v0.1.0-linux.tar.gz
   cd color-clicker
   ```
3. 运行:
   ```bash
   ./color-clicker
   ```

## 自定义构建

如果需要修改构建配置,编辑 `.github/workflows/build-release.yml`

### 支持的平台
- macOS 11+ (Intel & Apple Silicon)
- Ubuntu 20.04+

### 添加新平台

在 `build-release.yml` 中添加新的 job:

```yaml
build-windows:
  runs-on: windows-latest
  steps:
    - name: Checkout code
      uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Build
      run: |
        cd mac-tools/color-clicker
        cargo build --release
```

## 注意事项

1. **macOS 权限**: 首次运行可能需要授权屏幕录制和辅助功能权限
2. **Linux 依赖**: Ubuntu 需要安装 X11 相关库
3. **标签命名**: 必须以 `v` 开头 (如 `v0.1.0`, `v1.0.0`)
4. **构建时间**: 首次构建可能需要 5-10 分钟 (下载依赖)

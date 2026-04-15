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
- **格式**: 标准 DMG 安装包
- **特性**:
  - 包含 Applications 快捷方式（拖拽安装）
  - 包含可执行文件、配置文件和文档

### Linux (Ubuntu)
- **文件**: `color-clicker-v0.1.0-linux.deb`
- **架构**: amd64 (x86_64)
- **格式**: DEB 安装包
- **特性**:
  - 标准 Debian/Ubuntu 包格式
  - 自动安装到 `/usr/local/bin`
  - 创建桌面启动器 (Applications 菜单)
  - 自动处理依赖关系
  - 支持 `apt` 和 `dpkg` 管理

## 使用发布的安装包

### macOS
1. 下载 `.dmg` 文件
2. 双击打开 DMG
3. 将 `color-clicker` 拖到 Applications 文件夹
4. 从 Applications 启动，或运行:
   ```bash
   ./color-clicker
   ```

**首次运行权限**:
- 系统可能提示需要**屏幕录制权限**
- 系统可能提示需要**辅助功能权限**
- 在 系统偏好设置 > 安全性与隐私 中授权

### Linux (Ubuntu/Debian)
1. 下载 `.deb` 文件
2. 安装:
   ```bash
   sudo dpkg -i color-clicker-v0.1.0-linux.deb
   ```
   或使用 apt:
   ```bash
   sudo apt install ./color-clicker-v0.1.0-linux.deb
   ```
3. 从 Applications 菜单启动，或运行:
   ```bash
   color-clicker
   ```

**卸载**:
```bash
sudo dpkg -r color-clicker
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

1. **macOS 权限**: 首次运行需要授权屏幕录制和辅助功能权限
2. **Linux 依赖**: DEB 包会自动声明依赖，安装时 apt 会自动处理
3. **标签命名**: 必须以 `v` 开头 (如 `v0.1.0`, `v1.0.0`)
4. **构建时间**: 首次构建可能需要 5-10 分钟 (下载依赖)
5. **DEB 包管理**: Linux 版本支持标准包管理工具 (apt/dpkg)

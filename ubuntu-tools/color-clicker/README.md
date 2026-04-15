# Color Clicker - Ubuntu/Linux 版本

悬浮框颜色检测自动点击工具 (Linux/Ubuntu)

## 功能

- 🎯 实时颜色检测（蓝色占比）
- 🖱️ 自动点击功能
- 📦 半透明悬浮窗口
- ⚙️ 可配置参数
- 💾 自动保存窗口位置

## 依赖

### 系统依赖

```bash
sudo apt update
sudo apt install build-essential pkg-config libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libssl-dev
```

## 构建

```bash
cd ubuntu-tools/color-clicker
cargo build --release
```

## 运行

```bash
# 使用运行脚本
./run.sh

# 或直接运行
cargo run --release
```

## 配置

编辑 `config.json` 文件:

```json
{
  "color_ratio": 0.5,          // 蓝色占比阈值 (0.0-1.0)
  "detection_interval_ms": 500, // 检测间隔 (毫秒)
  "box_size": {
    "width": 100,               // 检测区域宽度
    "height": 100               // 检测区域高度
  },
  "click_delay_ms": 5000,      // 点击冷却时间 (毫秒)
  "window_pos": {
    "x": 100.0,                 // 窗口 X 位置
    "y": 100.0                  // 窗口 Y 位置
  }
}
```

## 使用方法

1. 运行程序后会出现一个半透明悬浮窗口
2. 将悬浮窗口拖拽到需要检测的位置
3. 点击 "▶ Start" 按钮启用自动点击
4. 当检测区域的蓝色占比超过阈值时，会自动点击
5. 点击 "⏹ Stop" 停止自动点击
6. 关闭窗口退出程序

## Linux 权限说明

### 窗口置顶问题

Linux 窗口管理器（特别是 GNOME/KDE）可能限制应用强制置顶：

**解决方案：**
1. GNOME: 安装扩展 "Always on Top" 或在窗口标题栏右键选择 "Always on Top"
2. KDE: 窗口规则中设置 "保持置顶"
3. 或者使用 `wmctrl` 工具: `wmctrl -r :ACTIVE -b add,above`

### 自动点击权限

enigo 库需要 X11 权限才能模拟鼠标事件：

**必需条件：**
- ✅ 必须在 **X11 会话**中运行（不支持 Wayland）
- ✅ 安装 X11 开发库: `sudo apt install libx11-dev libxtst-dev`
- ✅ 普通用户权限即可（不需要 sudo）

**检查当前会话类型：**
```bash
echo $XDG_SESSION_TYPE
# 应该输出: x11
# 如果输出: wayland，需要切换到 X11
```

**切换到 X11 会话：**
1. 注销当前用户
2. 在登录界面选择 "Ubuntu on Xorg" 或 "GNOME on Xorg"
3. 重新登录

**如果点击仍然失败：**
```bash
# 方案 1: 设置文件能力
sudo setcap cap_sys_admin+ep ./color-clicker

# 方案 2: 检查 X11 权限
xhost +SI:localuser:$USER
```

## 与 Mac 版本的区别

- 使用 `xcap` 库替代 `screenshots` 库进行屏幕捕获
- 针对 Linux/X11 环境优化
- 其他功能完全一致

## 测试

```bash
cargo test
```

## 许可

MIT License

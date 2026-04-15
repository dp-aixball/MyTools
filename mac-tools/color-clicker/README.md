# Color Clicker - 颜色检测点击器

## 功能说明

在 macOS 上创建一个可拖动的悬浮框,当框内大部分区域是蓝色时,自动在该位置触发鼠标单击。

## 快速开始

### 方法 1: 使用运行脚本 (推荐)

```bash
chmod +x run.sh
./run.sh
```

### 方法 2: 手动编译运行

```bash
# 编译
cargo build

# 运行
./target/debug/color-clicker
```

### 方法 3: 直接运行

```bash
cargo run
```

## 使用步骤

1. **运行程序** - 会出现一个半透明的蓝色悬浮框
2. **拖动悬浮框** - 将框移动到你想检测的位置
3. **自动检测** - 程序会实时显示框内蓝色像素占比
4. **触发点击** - 当蓝色占比超过 50% (可配置)时自动点击
5. **暂停/继续** - 点击按钮可以暂停或继续检测

## 配置说明

编辑 `config.json` 文件自定义参数:

```json
{
  "blue_threshold": {
    "min_blue": 150,    // 蓝色通道最小值 (0-255)
    "max_red": 100,     // 红色通道最大值
    "max_green": 100    // 绿色通道最大值
  },
  "color_ratio": 0.5,           // 蓝色占比阈值 (0.0-1.0)
  "detection_interval_ms": 500, // 检测间隔 (毫秒)
  "box_size": {
    "width": 100,    // 检测框宽度
    "height": 100    // 检测框高度
  },
  "click_delay_ms": 1000        // 点击冷却时间 (毫秒)
}
```

### 配置项说明

- **blue_threshold**: 定义什么颜色算"蓝色"
  - 默认: B > 150 且 R < 100 且 G < 100
  
- **color_ratio**: 多少比例的蓝色像素触发点击
  - 0.5 = 50% 的像素是蓝色时触发
  
- **detection_interval_ms**: 多久检测一次
  - 越小越灵敏,但消耗更多 CPU
  
- **box_size**: 检测区域大小
  - 根据实际需求调整
  
- **click_delay_ms**: 两次点击之间的最小间隔
  - 防止重复点击

## 技术栈

- **Rust** - 高性能系统级语言
- **eframe/egui** - 轻量级跨平台 GUI 框架
- **screenshots** - 屏幕捕获
- **enigo** - 鼠标控制

## 系统要求

- macOS 10.15+ 或 Ubuntu 18.04+
- Rust 1.70+

## 编译发布版本

```bash
# 编译优化版本
cargo build --release

# 可执行文件在
./target/release/color-clicker
```

## 故障排除

### macOS 权限问题

首次运行可能需要授权:
- **屏幕录制权限** - 系统会提示,点击"允许"
- **辅助功能权限** - 用于鼠标控制

前往: 系统偏好设置 > 安全性与隐私 > 隐私 > 屏幕录制/辅助功能

### 编译错误

```bash
# 清理并重新编译
cargo clean
cargo build
```

## 许可证

MIT

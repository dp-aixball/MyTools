use enigo::{Enigo, Mouse};
use std::thread;
use std::time::Duration;

/// 自动点击模块
pub struct AutoClicker;

impl AutoClicker {
    /// 在指定位置执行鼠标左键单击
    pub fn click(x: i32, y: i32) -> Result<(), String> {
        let mut enigo = match Enigo::new(&Default::default()) {
            Ok(e) => e,
            Err(e) => {
                #[cfg(target_os = "linux")]
                return Err(format!(
                    "鼠标控制初始化失败: {}. Linux 解决方案: \
                    1) 确保已安装 X11 库: sudo apt install libx11-dev libxtst-dev, \
                    2) 如果在 Wayland 下，切换到 X11 会话, \
                    3) 检查是否有权限访问 X11 显示服务器",
                    e
                ));
                #[cfg(not(target_os = "linux"))]
                return Err(format!("初始化鼠标控制失败: {}", e));
            }
        };

        // 移动鼠标到目标位置
        if let Err(e) = enigo.move_mouse(x, y, enigo::Coordinate::Abs) {
            return Err(format!(
                "移动鼠标失败: {}\n提示: 检查坐标是否有效 ({}, {})",
                e, x, y
            ));
        }

        // 短暂延迟确保鼠标移动到位
        thread::sleep(Duration::from_millis(50));

        // 执行左键点击
        if let Err(e) = enigo.button(enigo::Button::Left, enigo::Direction::Click) {
            #[cfg(target_os = "linux")]
            return Err(format!(
                "点击失败: {}. Linux 解决方案: \
                1) 检查是否有输入设备权限, \
                2) 尝试: sudo setcap cap_sys_admin+ep ./color-clicker, \
                3) 确保在 X11 会话中运行",
                e
            ));
            #[cfg(not(target_os = "linux"))]
            return Err(format!("点击失败: {}", e));
        }

        Ok(())
    }

    /// 延迟指定时间
    pub fn delay(ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
}

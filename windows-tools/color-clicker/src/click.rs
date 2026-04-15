use enigo::{Enigo, Mouse};
use std::thread;
use std::time::Duration;

/// 自动点击模块
pub struct AutoClicker;

impl AutoClicker {
    /// 在指定位置执行鼠标左键单击
    pub fn click(x: i32, y: i32) -> Result<(), String> {
        let mut enigo = Enigo::new(&Default::default()).map_err(|e| format!("初始化鼠标控制失败: {}", e))?;
        
        // 移动鼠标到目标位置
        enigo.move_mouse(x, y, enigo::Coordinate::Abs).map_err(|e| format!("移动鼠标失败: {}", e))?;
        
        // 短暂延迟确保鼠标移动到位
        thread::sleep(Duration::from_millis(50));
        
        // 执行左键点击
        enigo.button(enigo::Button::Left, enigo::Direction::Click).map_err(|e| format!("点击失败: {}", e))?;
        
        Ok(())
    }
    
    /// 延迟指定时间
    pub fn delay(ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
}

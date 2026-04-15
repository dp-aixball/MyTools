use screenshots::Screen;

/// 屏幕捕获模块
pub struct ScreenCapture;

impl ScreenCapture {
    /// 捕获指定区域的屏幕截图
    /// 返回 (x, y, width, height) 区域的像素数据
    pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> Result<Vec<u8>, String> {
        // 获取主屏幕
        let screens = Screen::all().map_err(|e| format!("Failed to get screen list: {}", e))?;
        let primary = screens.first().ok_or("No screen found")?;
        
        // 捕获指定区域
        let image = primary.capture_area(x, y, width, height).map_err(|e| format!("Screenshot failed: {}", e))?;
        
        // 在新版本中，screenshots 直接支持输出准确的 RGBA 字节流
        let raw = image.into_raw();
        Ok(raw)
    }
}

use xcap::Monitor;

/// 屏幕捕获模块
pub struct ScreenCapture;

impl ScreenCapture {
    /// 捕获指定区域的屏幕截图
    /// 注意：x, y, width, height 已经是物理像素坐标
    pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> Result<Vec<u8>, String> {
        // 获取主显示器
        let monitors = Monitor::all().map_err(|e| format!("Failed to get monitor list: {}", e))?;
        let primary = monitors.first().ok_or("No monitor found")?;
        
        // 捕获整个屏幕
        let image = primary.capture_image().map_err(|e| format!("Screenshot failed: {}", e))?;
        
        // 获取图像尺寸
        let screen_width = image.width() as i32;
        let screen_height = image.height() as i32;
        
        // 边界检查
        let start_x = x.max(0) as u32;
        let start_y = y.max(0) as u32;
        let end_x = (x + width as i32).min(screen_width) as u32;
        let end_y = (y + height as i32).min(screen_height) as u32;
        
        if start_x >= end_x || start_y >= end_y {
            return Err("Capture region is out of screen bounds".to_string());
        }
        
        let actual_width = end_x - start_x;
        let actual_height = end_y - start_y;
        
        // 提取区域像素
        let mut region_pixels = Vec::with_capacity((actual_width * actual_height * 4) as usize);
        
        for row in start_y..end_y {
            for col in start_x..end_x {
                let pixel = image.get_pixel(col, row);
                region_pixels.push(pixel[0]); // R
                region_pixels.push(pixel[1]); // G
                region_pixels.push(pixel[2]); // B
                region_pixels.push(pixel[3]); // A
            }
        }
        
        Ok(region_pixels)
    }
}

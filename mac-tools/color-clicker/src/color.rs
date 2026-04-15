use crate::config::Config;

/// 颜色检测模块
pub struct ColorDetector;

/// RGB 颜色分析结果
pub struct ColorAnalysis {
    pub avg_red: f64,
    pub avg_green: f64,
    pub avg_blue: f64,
    pub blue_ratio: f64,
}

impl ColorDetector {
    /// 分析像素数据,返回 RGB 平均值和总体蓝色占比
    pub fn analyze_colors(pixels: &[u8], _config: &Config) -> ColorAnalysis {
        let mut total_red: u64 = 0;
        let mut total_green: u64 = 0;
        let mut total_blue: u64 = 0;
        let mut total_count = 0;
        
        // 每个像素 4 个字节 (RGBA)
        for chunk in pixels.chunks(4) {
            if chunk.len() == 4 {
                let r = chunk[0] as u64;
                let g = chunk[1] as u64;
                let b = chunk[2] as u64;
                
                total_red += r;
                total_green += g;
                total_blue += b;
                total_count += 1;
            }
        }
        
        let avg_red = if total_count > 0 {
            total_red as f64 / total_count as f64
        } else {
            0.0
        };
        
        let avg_green = if total_count > 0 {
            total_green as f64 / total_count as f64
        } else {
            0.0
        };
        
        let avg_blue = if total_count > 0 {
            total_blue as f64 / total_count as f64
        } else {
            0.0
        };
        
        let total_rgb = avg_red + avg_green + avg_blue;
        let blue_ratio = if total_rgb > 0.0 {
            avg_blue / total_rgb
        } else {
            0.0
        };
        
        ColorAnalysis {
            avg_red,
            avg_green,
            avg_blue,
            blue_ratio,
        }
    }
    
    /// 检查是否应该触发点击
    pub fn should_click(blue_ratio: f64, config: &Config) -> bool {
        blue_ratio >= config.color_ratio
    }
}

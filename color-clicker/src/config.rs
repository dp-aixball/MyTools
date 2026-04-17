use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub color_ratio: f64,
    pub detection_interval_ms: u64,
    pub box_size: BoxSize,
    pub bg_opacity: u8,
    pub click_delay_ms: u64,
    pub window_pos: WindowPos,
}

/// 窗口位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPos {
    pub x: f32,
    pub y: f32,
}

impl Default for WindowPos {
    fn default() -> Self {
        WindowPos { x: 100.0, y: 100.0 }
    }
}
/// 窗口大小
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxSize {
    pub width: u32,
    pub height: u32,
}

impl Config {
    /// 从文件加载配置
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if Path::new(path).exists() {
            let config_str = fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&config_str)?;
            Ok(config)
        } else {
            // 使用默认配置
            let config = Config::default();
            config.save(path)?;
            Ok(config)
        }
    }

    /// 保存配置到文件
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(path, config_str)?;
        Ok(())
    }
}

impl Default for Config {
    /// 默认配置
    fn default() -> Self {
        Config {
            color_ratio: 0.5,
            detection_interval_ms: 500,
            box_size: BoxSize {
                width: 70,
                height: 35,
            },
            bg_opacity: 50,
            click_delay_ms: 5000,
            window_pos: WindowPos::default(),
        }
    }
}

mod config;
mod screen;
mod color;
mod click;
mod app;

use config::Config;
use app::ColorClickerApp;
use eframe::egui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();
    
    println!("🎯 Color Clicker starting...");
    println!("Drag the floating box to target position, it will auto-click when blue detected");
    println!("Close window to exit\n");
    
    // 加载配置
    let config_path = "config.json";
    let config = match Config::load(config_path) {
        Ok(c) => {
            println!("✓ Config loaded successfully");
            c
        }
        Err(e) => {
            eprintln!("✗ Config load failed: {}", e);
            eprintln!("Using default config");
            Config::default()
        }
    };
    
    let initial_x = config.window_pos.x;
    let initial_y = config.window_pos.y;
    
    // 创建应用
    let app = ColorClickerApp::new(config);
    
    // 窗口选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([95.0, 140.0])  // 宽高极简
            .with_position([initial_x, initial_y])  // 记住上次位置
            .with_resizable(false)  // 禁止调整大小
            .with_maximize_button(false)  // 禁止最大化
            .with_always_on_top()
            .with_transparent(true)
            .with_decorations(true), // 保留标题栏以便拖动
        ..Default::default()
    };
    
    // 运行应用
    eframe::run_native(
        "Color Clicker",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )?;
    
    Ok(())
}

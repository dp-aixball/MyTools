mod app;
mod click;
mod color;
mod config;
mod screen;

use app::ColorClickerApp;
use config::Config;
use eframe::egui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🎯 Color Clicker starting...");
    println!("Drag the floating box to target position, it will auto-click when blue detected");
    println!("Close window to exit\n");

    // 记录启动信息

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
            .with_inner_size([95.0, 160.0]) // 宽高极简，略微增加高度确保显示全
            .with_position([initial_x, initial_y]) // 记住上次位置
            .with_resizable(false) // 禁止调整大小
            .with_maximize_button(false) // 禁止最大化
            .with_always_on_top()
            .with_transparent(true)
            .with_decorations(true) // 保留标题栏以便拖动
            .with_icon(load_icon()),
        ..Default::default()
    };

    // 运行应用
    eframe::run_native("Color Clicker", options, Box::new(|_cc| Ok(Box::new(app))))?;

    Ok(())
}

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../../resources/icon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to open icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    egui::IconData {
        rgba,
        width,
        height,
    }
}

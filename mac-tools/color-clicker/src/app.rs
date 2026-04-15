use eframe::egui;
use crate::config::Config;
use crate::screen::ScreenCapture;
use crate::color::ColorDetector;
use crate::click::AutoClicker;
use std::time::{Instant, Duration};
// No need for unused imports

/// 悬浮框应用状态
pub struct ColorClickerApp {
    config: Config,
    window_x: i32,
    window_y: i32,
    avg_red: f64,
    avg_green: f64,
    avg_blue: f64,
    blue_ratio: f64,
    is_detecting: bool,
    auto_click_enabled: bool,
    last_click_time: Instant,
    detection_status: String,
    error_message: String,
    last_avg_red: f64,
    last_avg_green: f64,
    last_avg_blue: f64,
}

impl ColorClickerApp {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            window_x: 200,
            window_y: 200,
            avg_red: 0.0,
            avg_green: 0.0,
            avg_blue: 0.0,
            blue_ratio: 0.0,
            is_detecting: true,
            auto_click_enabled: false,
            last_click_time: Instant::now(),
            detection_status: String::from("Running"),
            error_message: String::new(),
            last_avg_red: -1.0,
            last_avg_green: -1.0,
            last_avg_blue: -1.0,
        }
    }
    
    /// 执行颜色检测和点击
    fn detect_and_click(&mut self, ctx: &egui::Context) {
        if !self.is_detecting {
            return;
        }
        
        let width = self.config.box_size.width as i32;
        let height = self.config.box_size.height as i32;
        
        // 获取窗口在屏幕上的真实位置
        if let Some(window_pos) = ctx.input(|i| i.viewport().outer_rect) {
            self.window_x = window_pos.min.x as i32;
            self.window_y = window_pos.min.y as i32;
            
            let capture_x = window_pos.center().x as i32 - (width / 2);
            let capture_y = window_pos.max.y as i32;
            
            match ScreenCapture::capture_region(capture_x, capture_y, width as u32, height as u32) {
            Ok(pixels) => {
                if pixels.is_empty() {
                    self.detection_status = String::from("No pixels captured");
                    return;
                }
                
                // 分析颜色
                let analysis = ColorDetector::analyze_colors(&pixels, &self.config);
                self.avg_red = analysis.avg_red;
                self.avg_green = analysis.avg_green;
                self.avg_blue = analysis.avg_blue;
                self.blue_ratio = analysis.blue_ratio;
                
                // 只在新颜色值时才输出日志
                if (self.avg_red - self.last_avg_red).abs() > 1.0 ||
                   (self.avg_green - self.last_avg_green).abs() > 1.0 ||
                   (self.avg_blue - self.last_avg_blue).abs() > 1.0 {
                    println!("[COLOR] R={:.0} G={:.0} B={:.0}", 
                             self.avg_red, self.avg_green, self.avg_blue);
                    self.last_avg_red = self.avg_red;
                    self.last_avg_green = self.avg_green;
                    self.last_avg_blue = self.avg_blue;
                }
                
                // 检查是否需要点击
                if ColorDetector::should_click(self.blue_ratio, &self.config) {
                    if self.auto_click_enabled {
                        // 检查点击冷却时间
                        if self.last_click_time.elapsed() >= Duration::from_millis(self.config.click_delay_ms) {
                            let click_x = capture_x + width / 2;
                            let click_y = capture_y + height / 2;
                            
                            match AutoClicker::click(click_x, click_y) {
                                Ok(_) => {
                                    self.detection_status = format!("Clicked! Blue: {:.1}%", self.blue_ratio * 100.0);
                                    self.last_click_time = Instant::now();
                                }
                                Err(e) => {
                                    self.error_message = e;
                                    self.detection_status = String::from("Click failed");
                                }
                            }
                        } else {
                            self.detection_status = format!("Cooldown... Blue: {:.1}%", self.blue_ratio * 100.0);
                        }
                    } else {
                        self.detection_status = format!("Target Matched (Auto-click OFF)");
                    }
                } else {
                    self.detection_status = format!("Detecting... Blue: {:.1}%", self.blue_ratio * 100.0);
                }
            }
            Err(e) => {
                self.error_message = e;
                self.detection_status = String::from("Screenshot failed");
            }
        }
        }
    }
}

impl eframe::App for ColorClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 执行检测和点击
        self.detect_and_click(ctx);
        
        // 创建透明窗口
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_unmultiplied(100, 149, 237, 60), // 半透明蓝色
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(2.0);
                    
                    // 1. Controls
                    ui.horizontal(|ui| {
                        let is_on = self.auto_click_enabled;
                        let btn_text = if is_on { "⏹ Stop " } else { "▶ Start" };
                        let btn_color = if is_on { egui::Color32::RED } else { egui::Color32::GREEN };
                        
                        if ui.add(egui::Button::new(egui::RichText::new(btn_text).color(btn_color).size(14.0))).clicked() {
                            self.auto_click_enabled = !self.auto_click_enabled;
                            self.detection_status = if self.auto_click_enabled {
                                String::from("Auto-click ON")
                            } else {
                                String::from("Auto-click OFF")
                            };
                        }
                        
                        if ui.button(egui::RichText::new("↺").size(14.0)).clicked() {
                            self.avg_red = 0.0;
                            self.avg_green = 0.0;
                            self.avg_blue = 0.0;
                            self.blue_ratio = 0.0;
                            self.error_message.clear();
                        }
                    });
                    
                    ui.add_space(6.0);
                    
                    // 2. Mock Box (Fixed Position near top!)
                    let area_size = egui::Vec2::new(60.0, 40.0);
                    let (rect, _) = ui.allocate_exact_size(area_size, egui::Sense::hover());
                    ui.painter().rect_stroke(
                        rect,
                        2.0,
                        egui::Stroke::new(2.0, egui::Color32::YELLOW),
                    );
                    
                    ui.add_space(6.0);
                    
                    // 3. Blue Ratio Only (Big!)
                    ui.label(egui::RichText::new(format!("Blue: {:.1}%", self.blue_ratio * 100.0)).size(16.0).color(egui::Color32::LIGHT_BLUE));
                    
                    ui.add_space(2.0);
                    
                    // 4. Status (At the bottom, it can wrap without pushing the box)
                    let status_color = if self.detection_status.contains("Clicked") { egui::Color32::GREEN }
                                       else if self.detection_status.contains("Cooldown") { egui::Color32::YELLOW }
                                       else if self.detection_status.contains("Matched") { egui::Color32::LIGHT_BLUE }
                                       else { egui::Color32::WHITE };
                    ui.colored_label(status_color, egui::RichText::new(&self.detection_status).size(11.0));
                    
                    // Error message
                    if !self.error_message.is_empty() {
                        ui.colored_label(egui::Color32::RED, &self.error_message);
                    }
                });
            });
        
        // 请求重绘以实现持续检测
        ctx.request_repaint_after(Duration::from_millis(self.config.detection_interval_ms));
    }
}

impl Drop for ColorClickerApp {
    fn drop(&mut self) {
        // 保存最后的窗口位置
        if self.window_x > 0 && self.window_y > 0 {
            self.config.window_pos.x = self.window_x as f32;
            self.config.window_pos.y = self.window_y as f32;
            let _ = self.config.save("config.json");
        }
    }
}

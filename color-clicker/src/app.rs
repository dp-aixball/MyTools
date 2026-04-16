use crate::click::AutoClicker;
use crate::color::ColorDetector;
use crate::config::Config;
use crate::screen::ScreenCapture;
use eframe::egui;
use std::time::{Duration, Instant};

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

            #[cfg(target_os = "linux")]
            let (capture_x, capture_y, capture_width, capture_height) = {
                // 使用 egui 的缩放因子转换坐标 (Linux 下通常需要)
                let scale = ctx.pixels_per_point();
                let x = (window_pos.center().x * scale - width as f32 * scale / 2.0) as i32;
                let y = (window_pos.max.y * scale) as i32;
                let w = (width as f32 * scale) as u32;
                let h = (height as f32 * scale) as u32;
                (x, y, w, h)
            };

            #[cfg(not(target_os = "linux"))]
            let (capture_x, capture_y, capture_width, capture_height) = {
                let x = window_pos.center().x as i32 - (width / 2);
                let y = window_pos.max.y as i32;
                (x, y, width as u32, height as u32)
            };

            match ScreenCapture::capture_region(capture_x, capture_y, capture_width, capture_height)
            {
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
                    if (self.avg_red - self.last_avg_red).abs() > 1.0
                        || (self.avg_green - self.last_avg_green).abs() > 1.0
                        || (self.avg_blue - self.last_avg_blue).abs() > 1.0
                    {
                        println!(
                            "[COLOR] R={:.0} G={:.0} B={:.0}",
                            self.avg_red, self.avg_green, self.avg_blue
                        );
                        self.last_avg_red = self.avg_red;
                        self.last_avg_green = self.avg_green;
                        self.last_avg_blue = self.avg_blue;
                    }

                    // 检查是否需要点击
                    if ColorDetector::should_click(self.blue_ratio, &self.config) {
                        if self.auto_click_enabled {
                            // 检查点击冷却时间
                            if self.last_click_time.elapsed()
                                >= Duration::from_millis(self.config.click_delay_ms)
                            {
                                let click_x = capture_x + capture_width as i32 / 2;
                                let click_y = capture_y + capture_height as i32 / 2;

                                match AutoClicker::click(click_x, click_y) {
                                    Ok(_) => {
                                        self.detection_status = format!(
                                            "Clicked! Blue: {:.1}%",
                                            self.blue_ratio * 100.0
                                        );
                                        self.last_click_time = Instant::now();
                                    }
                                    Err(e) => {
                                        self.error_message = e;
                                        self.detection_status = String::from("Click failed");
                                    }
                                }
                            } else {
                                self.detection_status =
                                    format!("Cooldown... Blue: {:.1}%", self.blue_ratio * 100.0);
                            }
                        } else {
                            self.detection_status = format!("Target Matched (Auto-click OFF)");
                        }
                    } else {
                        self.detection_status =
                            format!("Detecting... Blue: {:.1}%", self.blue_ratio * 100.0);
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
        // 处理键盘输入调节阈值
        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowUp) {
                self.config.color_ratio = (self.config.color_ratio + 0.1).min(1.0);
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.config.color_ratio = (self.config.color_ratio - 0.1).max(0.0);
            }
        });

        // 定期强制置顶 (使用 egui 原生 ViewportCommand，更可靠)
        {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            if COUNTER.fetch_add(1, Ordering::Relaxed) % 60 == 0 {
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                    egui::WindowLevel::AlwaysOnTop,
                ));
            }
        }

        // 执行检测和点击
        self.detect_and_click(ctx);

        // 创建主体面板
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_unmultiplied(20, 20, 25, 230)) // 深色半透明底色
                    .inner_margin(egui::Margin::symmetric(6.0, 4.0))
                    .rounding(4.0),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // 1. 顶部控制栏
                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() / 2.0 - 22.5); // 手动居中 RUN/STOP 按钮 (大约 45.0 的一半)
                        let is_on = self.auto_click_enabled;
                        let btn_text = if is_on { "STOP" } else { "RUN" };
                        let btn_color = if is_on {
                            egui::Color32::from_rgb(220, 50, 50)
                        } else {
                            egui::Color32::from_rgb(50, 200, 80)
                        };

                        let btn = egui::Button::new(
                            egui::RichText::new(btn_text)
                                .color(egui::Color32::WHITE)
                                .strong()
                                .size(11.0),
                        )
                        .fill(btn_color)
                        .rounding(2.0)
                        .min_size(egui::Vec2::new(45.0, 18.0));

                        if ui.add(btn).clicked() {
                            self.auto_click_enabled = !self.auto_click_enabled;
                            self.detection_status = if self.auto_click_enabled {
                                String::from("System Active")
                            } else {
                                String::from("System Paused")
                            };
                        }
                    });

                    ui.add_space(2.0);
                    ui.add(egui::Separator::default().spacing(4.0));

                    // 2. 指标区域
                    ui.vertical_centered(|ui| {
                        // 阈值控制显示
                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() / 2.0 - 25.0); // 辅助居中
                            ui.spacing_mut().item_spacing.x = 2.0;
                            ui.label(
                                egui::RichText::new("THR:")
                                    .size(10.0)
                                    .color(egui::Color32::LIGHT_GRAY),
                            );
                            ui.label(
                                egui::RichText::new(format!("{:.1}", self.config.color_ratio))
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(255, 215, 0))
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new("↑↓")
                                    .size(9.0)
                                    .color(egui::Color32::DARK_GRAY),
                            );
                        });

                        ui.add_space(2.0);

                        // 核心数据：当前占比
                        let ratio_color = if self.blue_ratio >= self.config.color_ratio {
                            egui::Color32::from_rgb(100, 200, 255)
                        } else {
                            egui::Color32::from_rgb(200, 200, 200)
                        };

                        ui.label(
                            egui::RichText::new(format!("{:.1}%", self.blue_ratio * 100.0))
                                .size(20.0)
                                .color(ratio_color)
                                .strong(),
                        );
                    });

                    ui.add_space(2.0);

                    // 3. 视觉预览框 (强制居中)
                    let area_size = egui::Vec2::new(50.0, 30.0);
                    let (rect, _) = ui.allocate_exact_size(area_size, egui::Sense::hover());

                    // 修正 rect 位置确保居中
                    let center_rect = egui::Rect::from_center_size(
                        egui::pos2(ui.min_rect().center().x, rect.center().y),
                        area_size,
                    );

                    let stroke_color = if self.blue_ratio >= self.config.color_ratio {
                        egui::Color32::from_rgb(255, 255, 0)
                    } else {
                        egui::Color32::from_gray(100)
                    };

                    ui.painter().rect_stroke(
                        center_rect,
                        1.0,
                        egui::Stroke::new(1.5, stroke_color),
                    );

                    ui.add_space(4.0);
                    ui.add(egui::Separator::default().spacing(4.0));

                    // 4. 状态栏
                    let (status_text, status_color) = if !self.error_message.is_empty() {
                        (
                            format!("! {}", self.error_message),
                            egui::Color32::from_rgb(255, 100, 100),
                        )
                    } else if self.detection_status.contains("Clicked") {
                        (
                            String::from("● SUCCESS"),
                            egui::Color32::from_rgb(100, 255, 150),
                        )
                    } else if self.detection_status.contains("Active")
                        || self.detection_status.contains("Running")
                    {
                        (
                            String::from("● SCANNING"),
                            egui::Color32::from_rgb(150, 150, 150),
                        )
                    } else {
                        (
                            format!("○ {}", self.detection_status.to_uppercase()),
                            egui::Color32::from_gray(120),
                        )
                    };

                    ui.label(
                        egui::RichText::new(status_text)
                            .size(9.0)
                            .color(status_color),
                    );
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

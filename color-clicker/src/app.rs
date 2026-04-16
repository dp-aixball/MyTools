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
    click_count: u64,
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
            click_count: 0,
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
                // 使用 egui 的缩放因子转换坐标
                let scale = ctx.pixels_per_point();
                let x = (window_pos.center().x * scale - width as f32 * scale / 2.0) as i32;
                let y = (window_pos.center().y * scale - height as f32 * scale / 2.0) as i32;
                let w = (width as f32 * scale) as u32;
                let h = (height as f32 * scale) as u32;
                (x, y, w, h)
            };

            #[cfg(not(target_os = "macos"))]
            let window_pos = ctx.input(|i| i.viewport().outer_rect).unwrap_or_default().min;
            
            #[cfg(not(target_os = "linux"))]
            let (capture_x, capture_y, capture_width, capture_height, click_x, click_y) = {
                let scale = ctx.pixels_per_point();
                // 1. 获取当前窗口在屏幕上的绝对位置（包括内容区相对于物理屏幕的偏移）
                let viewport = ctx.input(|i| i.viewport().clone());
                let inner_rect = viewport.inner_rect.unwrap_or_else(|| {
                    let pos = viewport.outer_rect.unwrap_or(egui::Rect::ZERO).min;
                    egui::Rect::from_min_size(pos, egui::Vec2::ZERO)
                });
                
                // 2. 获取 UI 内容的中心（逻辑点）
                let screen_center = ctx.screen_rect().center();
                let logical_target = egui::pos2(screen_center.x, screen_center.y + 15.0);
                
                // 3. 计算屏幕上的绝对物理位置（逻辑点 -> 物理位置）
                let screen_pos = inner_rect.min + egui::vec2(logical_target.x, logical_target.y);
                
                // 4. 物理像素坐标（用于 ScreenCapture）
                let cap_x = (screen_pos.x * scale - (width as f32 * scale / 2.0)) as i32;
                let cap_y = (screen_pos.y * scale - (height as f32 * scale / 2.0)) as i32;
                let cap_w = (width as f32 * scale) as u32;
                let cap_h = (height as f32 * scale) as u32;
                
                // 5. 逻辑点坐标（用于 Enigo 点击）
                let clk_x = screen_pos.x as i32;
                let clk_y = screen_pos.y as i32;
                
                (cap_x, cap_y, cap_w, cap_h, clk_x, clk_y)
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
                                // 异步穿透点击核心逻辑
                                // 1. 先开启鼠标穿透
                                ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
                                
                                let ctx_clone = ctx.clone();
                                std::thread::spawn(move || {
                                    // 2. 给予操作系统足够的反应时间（约 40ms），确保穿透指令已生效
                                    std::thread::sleep(std::time::Duration::from_millis(40));
                                    
                                    // 3. 执行真正的鼠标点击（此时会穿透窗口点击到后台程序）
                                    let _ = AutoClicker::click(click_x, click_y);
                                    
                                    // 4. 点击完成后，恢复窗口交互
                                    ctx_clone.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(false));
                                });

                                self.click_count += 1;
                                self.last_click_time = Instant::now();
                                self.detection_status = format!(
                                    "Clicked! Blue: {:.1}%",
                                    self.blue_ratio * 100.0
                                );
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
                self.config.color_ratio = (self.config.color_ratio + 0.01).min(1.0);
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.config.color_ratio = (self.config.color_ratio - 0.01).max(0.0);
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
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 0)))
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
                                egui::RichText::new(format!("{:.2}", self.config.color_ratio))
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(255, 215, 0))
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new("+/-")
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
                    
                    // 3. 视觉预览框 (准星模式：大小由 config.box_size 决定)
                    let area_size = egui::Vec2::new(
                        self.config.box_size.width as f32,
                        self.config.box_size.height as f32,
                    );
                    let (_rect, _) = ui.allocate_exact_size(area_size, egui::Sense::hover());

                    // 计算准星的逻辑中心位置（Window-local 坐标系）
                    let screen_center = ui.ctx().screen_rect().center();
                    let logical_center = egui::pos2(screen_center.x, screen_center.y + 15.0);
                    
                    let center_rect = egui::Rect::from_center_size(
                        logical_center,
                        area_size,
                    );

                    let stroke_color = if self.blue_ratio >= self.config.color_ratio {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::from_gray(180) // 调亮未击中时的预览框
                    };

                    ui.painter().rect_stroke(
                        center_rect,
                        1.0,
                        egui::Stroke::new(1.5, stroke_color),
                    );

                    // 绘制准星 (+)：荧光绿，线宽 2.0，极度显眼
                    let center = center_rect.center();
                    let cross_size = 6.0; // 稍微拉长一点
                    let cross_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 0));
                    ui.painter().line_segment(
                        [center - egui::vec2(cross_size, 0.0), center + egui::vec2(cross_size, 0.0)],
                        cross_stroke,
                    );
                    ui.painter().line_segment(
                        [center - egui::vec2(0.0, cross_size), center + egui::vec2(0.0, cross_size)],
                        cross_stroke,
                    );

                    ui.add_space(3.0);
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

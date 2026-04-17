use crate::click::AutoClicker;
use crate::color::ColorDetector;
use crate::config::Config;
use crate::screen::ScreenCapture;
use eframe::egui;
use std::time::{Duration, Instant};

/// 颜色检测结果
#[derive(Default, Clone)]
struct DetectionResult {
    avg_red: f64,
    avg_green: f64,
    avg_blue: f64,
    blue_ratio: f64,
    status: String,
    clicked: bool,
    error: Option<String>,
    duration: Duration,
}

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

    // 异步检测相关
    result_receiver: std::sync::mpsc::Receiver<DetectionResult>,
    result_sender: std::sync::mpsc::Sender<DetectionResult>,
    is_working: bool,
    last_thread_duration: Duration,
}

impl ColorClickerApp {
    pub fn new(config: Config) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
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
            result_receiver: rx,
            result_sender: tx,
            is_working: false,
            last_thread_duration: Duration::ZERO,
        }
    }

    /// 启动后台检测线程
    fn start_async_detection(&mut self, ctx: &egui::Context) {
        if self.is_working || !self.is_detecting {
            return;
        }

        let scale = ctx.pixels_per_point();
        // 获取视口信息
        let viewport = ctx.input(|i| i.viewport().clone());
        let inner_rect = viewport.inner_rect.unwrap_or_else(|| {
            let pos = viewport.outer_rect.unwrap_or(egui::Rect::ZERO).min;
            egui::Rect::from_min_size(pos, egui::Vec2::ZERO)
        });

        // 更新状态，启动线程
        self.is_working = true;
        let tx = self.result_sender.clone();
        let ctx_clone = ctx.clone();
        let config = self.config.clone();
        let auto_click = self.auto_click_enabled;
        let last_click = self.last_click_time;

        std::thread::spawn(move || {
            let start = Instant::now();
            let mut result = DetectionResult::default();

            let width = config.box_size.width as i32;
            let height = config.box_size.height as i32;
            let screen_rect = ctx_clone.screen_rect();
            let screen_center = screen_rect.center();
            let logical_target = egui::pos2(screen_center.x, screen_center.y + 15.0);
            let screen_pos = inner_rect.min + egui::vec2(logical_target.x, logical_target.y);

            #[cfg(target_os = "macos")]
            let (capture_x, capture_y) = (
                (screen_pos.x - (width as f32 / 2.0)) as i32,
                (screen_pos.y - (height as f32 / 2.0)) as i32,
            );
            #[cfg(not(target_os = "macos"))]
            let (capture_x, capture_y) = (
                (screen_pos.x * scale - (width as f32 * scale / 2.0)) as i32,
                (screen_pos.y * scale - (height as f32 * scale / 2.0)) as i32,
            );

            let capture_width = if cfg!(target_os = "macos") {
                width as u32
            } else {
                (width as f32 * scale) as u32
            };
            let capture_height = if cfg!(target_os = "macos") {
                height as u32
            } else {
                (height as f32 * scale) as u32
            };

            #[cfg(target_os = "macos")]
            let (click_x, click_y) = (screen_pos.x as i32, screen_pos.y as i32);
            #[cfg(not(target_os = "macos"))]
            let (click_x, click_y) = ((screen_pos.x * scale) as i32, (screen_pos.y * scale) as i32);

            match ScreenCapture::capture_region(capture_x, capture_y, capture_width, capture_height)
            {
                Ok(pixels) => {
                    if pixels.is_empty() {
                        result.status = String::from("No pixels captured");
                    } else {
                        let analysis = ColorDetector::analyze_colors(&pixels, &config);
                        result.avg_red = analysis.avg_red;
                        result.avg_green = analysis.avg_green;
                        result.avg_blue = analysis.avg_blue;
                        result.blue_ratio = analysis.blue_ratio;

                        if ColorDetector::should_click(result.blue_ratio, &config) {
                            if auto_click
                                && last_click.elapsed()
                                    >= Duration::from_millis(config.click_delay_ms as u64)
                            {
                                // 执行点击
                                println!(
                                    "🚀 [DETECTED] Starting click sequence at ({}, {})",
                                    click_x, click_y
                                );
                                ctx_clone.send_viewport_cmd(
                                    egui::ViewportCommand::MousePassthrough(true),
                                );
                                std::thread::sleep(Duration::from_millis(100)); // 稍微拉长等待时间确保穿透生效

                                match AutoClicker::click(click_x, click_y) {
                                    Ok(_) => {
                                        println!("✅ [SUCCESS] Click event sent successfully");
                                        result.status =
                                            format!("CLICKED! Blue: {:.2}", result.blue_ratio);
                                    }
                                    Err(e) => {
                                        println!("❌ [ERROR] AutoClicker failed: {}", e);
                                        result.status = format!("Click Error: {}", e);
                                    }
                                }

                                ctx_clone.send_viewport_cmd(
                                    egui::ViewportCommand::MousePassthrough(false),
                                );
                                result.clicked = true;
                            } else if auto_click {
                                result.status = format!("Wait... Blue: {:.2}", result.blue_ratio);
                            } else {
                                result.status = format!("Target Matched (Auto-click OFF)");
                            }
                        } else {
                            result.status = format!("Detecting... Blue: {:.2}", result.blue_ratio);
                        }
                    }
                }
                Err(e) => {
                    result.error = Some(e);
                    result.status = String::from("Screenshot failed");
                }
            }

            result.duration = start.elapsed();
            let _ = tx.send(result);
        });
    }

    /// 处理检测结果
    fn handle_detection_results(&mut self) {
        while let Ok(result) = self.result_receiver.try_recv() {
            self.is_working = false;
            self.avg_red = result.avg_red;
            self.avg_green = result.avg_green;
            self.avg_blue = result.avg_blue;
            self.blue_ratio = result.blue_ratio;
            self.detection_status = result.status;
            self.last_thread_duration = result.duration;

            if result.clicked {
                self.click_count += 1;
                self.last_click_time = Instant::now();
            }

            if let Some(err) = result.error {
                self.error_message = err;
            }

            // 输出日志
            if (self.avg_red - self.last_avg_red).abs() > 1.0
                || (self.avg_green - self.last_avg_green).abs() > 1.0
                || (self.avg_blue - self.last_avg_blue).abs() > 1.0
            {
                println!(
                    "[COLOR] R={:.0} G={:.0} B={:.0} (in {:?})",
                    self.avg_red, self.avg_green, self.avg_blue, self.last_thread_duration
                );
                self.last_avg_red = self.avg_red;
                self.last_avg_green = self.avg_green;
                self.last_avg_blue = self.avg_blue;
            }
        }
    }
}

impl eframe::App for ColorClickerApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // 返回动态背景清除色，与 bg_opacity 同步
        let alpha = self.config.bg_opacity as f32 / 100.0;
        [0.0, 0.0, 0.0, alpha]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 设置全局视觉样式，支持可调节透明度
        let alpha = (self.config.bg_opacity as f32 * 2.55) as u8;
        let fill_color = if self.config.bg_opacity == 0 {
            egui::Color32::TRANSPARENT
        } else {
            egui::Color32::from_black_alpha(alpha)
        };

        // 直接修改全局样式，确保彻底覆盖
        ctx.style_mut(|style| {
            let visuals = &mut style.visuals;
            visuals.panel_fill = fill_color;
            visuals.window_fill = fill_color;
            visuals.extreme_bg_color = fill_color;
            visuals.faint_bg_color = fill_color;
            visuals.window_shadow = egui::Shadow::NONE;
            visuals.popup_shadow = egui::Shadow::NONE;
            // 确保非交互状态的背景也透明
            style.visuals.widgets.noninteractive.bg_fill = fill_color;
            style.visuals.widgets.noninteractive.weak_bg_fill = fill_color;
        });

        // 处理键盘输入调节阈值和透明度
        ctx.input(|i| {
            if i.modifiers.ctrl {
                if i.key_pressed(egui::Key::ArrowUp) {
                    self.config.bg_opacity = self.config.bg_opacity.saturating_add(1).min(100);
                    println!("[UI] Transparency adjusted: {}%", self.config.bg_opacity);
                }
                if i.key_pressed(egui::Key::ArrowDown) {
                    self.config.bg_opacity = self.config.bg_opacity.saturating_sub(1);
                    println!("[UI] Transparency adjusted: {}%", self.config.bg_opacity);
                }
            } else {
                if i.key_pressed(egui::Key::ArrowUp) {
                    self.config.color_ratio = (self.config.color_ratio + 0.01).min(1.0);
                }
                if i.key_pressed(egui::Key::ArrowDown) {
                    self.config.color_ratio = (self.config.color_ratio - 0.01).max(0.0);
                }
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

        // 异步检测流
        self.handle_detection_results();
        self.start_async_detection(ctx);

        // 创建主体面板
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(fill_color)
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
                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() / 2.0 - 35.0); // 居中辅助
                            ui.spacing_mut().item_spacing.x = 1.0;
                            ui.label(
                                egui::RichText::new("THR:")
                                    .size(10.0)
                                    .color(egui::Color32::LIGHT_GRAY),
                            );

                            // 阈值 / 实际值
                            let ratio_color = if self.blue_ratio >= self.config.color_ratio {
                                egui::Color32::from_rgb(100, 200, 255) // 达标色
                            } else {
                                egui::Color32::from_rgb(255, 215, 0) // 默认金黄色
                            };

                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.2}/{:.2}",
                                    self.blue_ratio, self.config.color_ratio
                                ))
                                .size(11.0)
                                .color(ratio_color)
                                .strong(),
                            );
                        });
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

                    let center_rect = egui::Rect::from_center_size(logical_center, area_size);

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
                        [
                            center - egui::vec2(cross_size, 0.0),
                            center + egui::vec2(cross_size, 0.0),
                        ],
                        cross_stroke,
                    );
                    ui.painter().line_segment(
                        [
                            center - egui::vec2(0.0, cross_size),
                            center + egui::vec2(0.0, cross_size),
                        ],
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

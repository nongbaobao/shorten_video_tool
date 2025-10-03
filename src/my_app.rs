use eframe::{egui, emath::Numeric};

use crate::video_util;

#[derive(Default)]
pub struct MyApp {
    picked_path: Option<String>,
    video_from_time: f64,
    video_to_time: f64,
    video_output_name: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple Shorten Video Tool");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            // 先选中文件,再处理缩减逻辑
            if let Some(picked_path) = &self.picked_path {
                ui.label("Picked file:");
                ui.monospace(picked_path);

                if picked_path.contains(".mp4") {
                    let video_duration = video_util::get_video_duration(picked_path); // 秒
                    let from_time = self.video_from_time.clone();
                    let to_time = self.video_to_time.clone();
                    ui.label("\nFrom Time: ");
                    ui.add(
                        egui::Slider::new(&mut self.video_from_time, 0.0..=video_duration.to_f64())
                            .show_value(false)
                            .text(video_util::seconds_to_time_string(from_time as u64)),
                    );

                    ui.label("\nTo Time: ");
                    ui.add(
                        egui::Slider::new(&mut self.video_to_time, 0.0..=video_duration.to_f64())
                            .show_value(false)
                            .text(video_util::seconds_to_time_string(to_time as u64)),
                    );

                    if self.video_from_time > self.video_to_time {
                        ui.label("Wrong Input! Do not click the button.");
                    }

                    ui.horizontal(|ui| {
                        ui.label("Input the output name: ");
                        ui.text_edit_singleline(&mut self.video_output_name);
                    });

                    // 显示输出路径信息
                    if !self.video_output_name.is_empty() {
                        let output_path = std::path::Path::new(picked_path)
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .join(&self.video_output_name);
                        ui.label(format!(
                            "Output will be saved to: {}",
                            output_path.display()
                        ));
                    }

                    if ui.button("Shorten!").clicked() {
                        video_util::shorten_video(
                            picked_path,
                            &self.video_output_name,
                            &self.video_from_time,
                            &self.video_to_time,
                        );
                    }
                } else {
                    ui.label("Warning: Picked file might not a video file");
                }
            }
        });
    }
}

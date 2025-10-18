use eframe::{egui, emath::Numeric};

use crate::video;

#[derive(Default)]
pub struct MyApp {
    selected_path: Option<String>,
    start_time: f64,
    end_time: f64,
    output_filename: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple Shorten Video Tool");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.selected_path = Some(path.display().to_string());
                }
            }

            // Process video shortening logic after file selection
            if let Some(selected_path) = &self.selected_path {
                ui.label("Selected file:");
                ui.monospace(selected_path);

                if selected_path.contains(".mp4") {
                    let video_duration = match video::get_video_duration(selected_path) {
                        Ok(duration) => duration,
                        Err(e) => {
                            ui.label(format!("Error getting video duration: {}", e));
                            return;
                        }
                    };
                    let from_time = self.start_time;
                    let to_time = self.end_time;

                    ui.label("\nStart Time: ");
                    ui.add(
                        egui::Slider::new(&mut self.start_time, 0.0..=video_duration.to_f64())
                            .show_value(false)
                            .text(video::seconds_to_time_string(from_time as u64)),
                    );

                    ui.label("\nEnd Time: ");
                    ui.add(
                        egui::Slider::new(&mut self.end_time, 0.0..=video_duration.to_f64())
                            .show_value(false)
                            .text(video::seconds_to_time_string(to_time as u64)),
                    );

                    if self.start_time > self.end_time {
                        ui.label("Invalid time range! Do not click the button.");
                    }

                    ui.horizontal(|ui| {
                        ui.label("Output filename: ");
                        ui.text_edit_singleline(&mut self.output_filename);
                    });

                    // Display output path information
                    if !self.output_filename.is_empty() {
                        let output_path = std::path::Path::new(selected_path)
                            .parent()
                            .unwrap_or_else(|| std::path::Path::new("."))
                            .join(&self.output_filename);
                        ui.label(format!(
                            "Output will be saved to: {}",
                            output_path.display()
                        ));
                    }

                    if ui.button("Shorten!").clicked() {
                        match video::shorten_video(
                            selected_path,
                            &self.output_filename,
                            self.start_time,
                            self.end_time,
                        ) {
                            Ok(()) => {
                                ui.label("Video shortened successfully!");
                            }
                            Err(e) => {
                                ui.label(format!("Error shortening video: {}", e));
                            }
                        }
                    }
                } else {
                    ui.label("Warning: Selected file might not be a video file");
                }
            }
        });
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod app;
mod video;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Shorten Video Tool",
        options,
        Box::new(|_| Ok(Box::<app::MyApp>::default())),
    )
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod app;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        decorated: false, 
        transparent: true,
        min_window_size: Some(egui::Vec2 { x: 400.0, y: 100.0 }),
        initial_window_size: Some(egui::Vec2 { x: 400.0, y: 240.0 }),
        ..Default::default()
    };

    eframe::run_native(
        "Bingo",
        options,
        Box::new(|_cc| Box::<app::MyApp>::default()),
    )
}
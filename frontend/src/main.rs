use eframe::egui::Vec2;
use eframe::{run_native, NativeOptions};
use ui::Main;
mod widgets;

mod ui;
fn main() {
    let mut win_options = NativeOptions::default();

    win_options.initial_window_size = Some(Vec2::new(600., 800.));

    run_native(
        "App Tracker",
        win_options,
        Box::new(|cc| Box::new(Main::new(cc))),
    )
    .expect("Application Error");
}

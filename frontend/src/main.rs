use eframe::egui::Vec2;
use eframe::{run_native, NativeOptions};
use ui::Main;

mod ui;
fn main() {
    let mut win_options = NativeOptions::default();
    win_options.always_on_top = false;
    win_options.initial_window_size = Some(Vec2 { x: 400., y: 600. });
    win_options.min_window_size = Some(Vec2 { x: 400., y: 600. });
    win_options.max_window_size = None;
    win_options.follow_system_theme = false;
    win_options.default_theme = eframe::Theme::Dark;
    win_options.decorated = false;
    win_options.centered = true;
    win_options.app_id = Some("app-tracker".to_string());

    run_native(
        "App Tracker",
        win_options,
        Box::new(|cc| Box::new(Main::new(cc))),
    )
    .expect("Application Error");
}

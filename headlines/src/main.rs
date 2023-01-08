use eframe::{NativeOptions,egui::Vec2, run_native};
use headlines::Headlines;


fn main() {
    tracing_subscriber::fmt::init();
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(800., 650.));
    run_native(Box::new(app), win_options);
}


use std::cell::RefCell;

use eframe::{NativeOptions,egui::Vec2, run_native};
use headlines::Headlines;
use tokio::runtime::Runtime;


fn main() {
    let mut rt  = Runtime::new().expect("Unable to create Runtime");
    tracing_subscriber::fmt::init();
    let app = Headlines::new(RefCell::new(rt));
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(800., 650.));
    run_native(Box::new(app), win_options);
}

pub mod headlines;

use eframe::egui;
use eframe::epi::App;
pub use headlines::{Headlines,render_message_bottom};


impl App for Headlines{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
           render_message_bottom(ctx);
       });
    }

    fn name(&self) -> &str {
        "ChatGPT Desktop App"
    }
}
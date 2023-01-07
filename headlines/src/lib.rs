pub mod headlines;

use eframe::epi::App;
pub use headlines::Headlines;

impl App for Headlines{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }
}
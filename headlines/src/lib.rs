pub mod headlines;

use eframe::egui::{self, ScrollArea, Ui, RichText, Label};
use eframe::epi::App;
pub use headlines::{Headlines,render_message_bottom};


impl App for Headlines{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
           let mut scroll_area_ui : &mut Ui;
           ScrollArea::vertical().show(ui, | ui |{
                ui.add(Label::new(RichText::new("Hello")));
                render_message_bottom(ctx,&mut self.search, ui);
           });
           
       });
    }

    fn name(&self) -> &str {
        "ChatGPT Desktop App"
    }
}
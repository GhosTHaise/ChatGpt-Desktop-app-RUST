pub mod headlines;

use eframe::egui::{self, ScrollArea, Ui, RichText, Label};
use eframe::epi::App;
pub use headlines::{Headlines};


impl App for Headlines{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
           let mut scroll_area_ui : &mut Ui;
           ScrollArea::vertical().show(ui, | ui |{
                ui.add(Label::new(RichText::new("Hello")));
                self.render_message_bottom(ctx,&mut self.search.borrow_mut(), ui);
           });
           
       });
    }

    fn name(&self) -> &str {
        "ChatGPT Desktop App"
    }
}
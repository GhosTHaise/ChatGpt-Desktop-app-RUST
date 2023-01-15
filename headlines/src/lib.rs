pub mod headlines;


use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use eframe::egui::{self, ScrollArea};
use eframe::epi::App;
pub use headlines::{Headlines};


impl App for Headlines{

    fn setup(&mut self, _ctx: &egui::Context, 
        _frame: &eframe::epi::Frame, 
        _storage: Option<&dyn eframe::epi::Storage>) {
        
        let (api_tx,api_rx) = mpsc::channel::<api::Payload>();
        
        //self.api_rx = Some(api_rx);
        self.api_tx = Some(api_tx);
        
        let add_strapped = |is_bot,content| self.add_new_dialog(is_bot, content);
        let safe_add = Arc::new(Mutex::new(add_strapped));
        let api_rcv_safe = Arc::new(Mutex::new(&api_rx));
        thread::spawn( move || {
            loop{
                match api_rcv_safe.lock().unwrap().try_recv(){
                    Ok(v) => {
                        //let data = v.bot;
                        //safe_add.lock().unwrap()(true,String::from(""));
                    },
                    Err(_) => todo!(),
                }
            }
        });

    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        ctx.request_repaint();
        
        egui::CentralPanel::default().show(ctx, |ui| {
           ui.vertical_centered_justified(|ui|{
                ui.heading("GhosT - Ai")
           });
           ScrollArea::vertical().show(ui, | ui |{
                //ui.add(Label::new(RichText::new("Hello")));
                self.render_new_message(ui);
           });
           self.render_message_bottom(ctx,&mut self.search.borrow_mut(), ui);
       });
    }

    fn name(&self) -> &str {
        "ChatGPT Desktop App"
    }
}
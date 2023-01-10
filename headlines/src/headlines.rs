use std::{cell::RefCell, ops::Deref};

use confy;
use eframe::{egui::{Context, TopBottomPanel,TextEdit, output, self, TextStyle, Label, RichText, Ui}, epaint::FontId};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct HeadlinesConfig{
    dark_mode : bool,
    api_key : String
}

#[derive(Serialize,Deserialize)]
pub struct NewBotResponse{
    bot : String
}

pub struct Userbot{
    pub is_bot : bool,
    pub expose : String
}

impl Default for HeadlinesConfig{
    fn default() -> Self {
        Self { dark_mode: Default::default(), api_key: Default::default() }
    }
}

//Method
fn clear_intput(mut value : &mut String){
    value.clear();
}

fn loader(curent_text : &str,label : egui::Label){
    let mut i = 0;
    loop{
        if(i != 4){
            //label.set_text(format!("{}.",curentText));
        }else{
            i = 0;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        i = i + 1;
    }
}

//end Method
pub struct Headlines{
    pub config : HeadlinesConfig,
    pub api_key_initialized : bool,
    pub search :  RefCell<String>,
    pub dialog : RefCell<Vec<RefCell<Userbot>>>
}

impl Headlines {
    pub fn new() -> Headlines {
        let config : HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        Headlines { 
            api_key_initialized: !config.api_key.is_empty(),
            config,
            search: RefCell::new("Ask GhosT ...".to_string()),
            dialog: RefCell::new(vec![]),
        }
    }

    pub fn render_new_message(&self,ui : &mut eframe::egui::Ui){
        let data = self.dialog.borrow();
        for m in data.deref() {
            let textual_content = m.borrow();
            let mut label = Label::new(RichText::new(format!("{}",textual_content.expose)).text_style(egui::TextStyle::Body));
            ui.add(label);
        }
        
    }
    
    pub fn render_message_bottom(&self,ctx : &Context, content : &mut String,parrent_ui : &mut Ui)-> () {
        TopBottomPanel::bottom("message").show(ctx , move |ui|{
            let mut style = (*ctx.style()).clone();
            //Adjust global font size
            style.text_styles = [
            (TextStyle::Heading,FontId::new(32.0, eframe::epaint::FontFamily::Proportional)), 
            (TextStyle::Body,FontId::new(18.0, eframe::epaint::FontFamily::Proportional)),
            (TextStyle::Monospace,FontId::new(14.0, eframe::epaint::FontFamily::Proportional)),
            (TextStyle::Button,FontId::new(14.0, eframe::epaint::FontFamily::Proportional)),
            (TextStyle::Small,FontId::new(10.0, eframe::epaint::FontFamily::Proportional))   
            ].into();
            ctx.set_style(style);
            ui.horizontal(move |ui|{
                let mess = ui.add_sized(
                        ui.available_size(),
                          TextEdit::singleline(content ).hint_text("Ask GhosT ...").font(egui::TextStyle::Heading)
                        );
                if mess.changed(){
                    //println!("{:?}",mess);
                }
                if mess.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    println!("{}",content);
                    
                    
                    self.dialog.borrow_mut().push(
                        RefCell::new(Userbot{
                            is_bot: false,
                            expose: content.to_string(),
                        })
                    );
                    //clear text Edit -> search
                    clear_intput(content);
                }
            });
            ui.add_space(1.);
        });
    }   
    

}



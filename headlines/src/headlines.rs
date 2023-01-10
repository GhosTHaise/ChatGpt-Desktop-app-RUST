use std::cell::RefCell;

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

pub struct user2bot{
    is_bot : bool,
    expose : String
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
    pub dialog : Vec<RefCell<String>>
}

impl Headlines {
    pub fn new() -> Headlines {
        let config : HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        Headlines { 
            api_key_initialized: !config.api_key.is_empty(),
            config,
            search: RefCell::new("Ask GhosT ...".to_string()),
            dialog: vec![],
        }
    }

    pub fn render_new_message(&self,is_bot : bool,content : String,ui : &mut eframe::egui::Ui){
        let mut label = Label::new(RichText::new(content).text_style(egui::TextStyle::Body));
        ui.add(label);
    }
    
    pub fn render_message_bottom(&self,ctx : &Context, content : &mut String,parrent_ui : &mut Ui)-> () {
        TopBottomPanel::bottom("message").show(ctx , |ui|{
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
            ui.horizontal(|ui|{
                ui.set_height(50.);
                let mess = ui.add_sized(
                        ui.available_size(),
                          TextEdit::singleline(content ).hint_text("Ask GhosT ...").font(egui::TextStyle::Body)
                        );
                if mess.changed(){
                    //println!("{:?}",mess);
                }
                if mess.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    println!("{}",content);
    
                    //new Message
                    self.render_new_message(false, content.to_string(), parrent_ui);
                    //clear text Edit -> search
                    clear_intput(content);
                }
            });
            ui.add_space(1.);
        });
    }   
    

}



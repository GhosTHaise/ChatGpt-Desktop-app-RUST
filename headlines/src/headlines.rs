use confy;
use eframe::egui::{Context, TopBottomPanel,TextEdit, output, self};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct HeadlinesConfig{
    dark_mode : bool,
    api_key : String
}


impl Default for HeadlinesConfig{
    fn default() -> Self {
        Self { dark_mode: Default::default(), api_key: Default::default() }
    }
}

pub struct Headlines{
    pub config : HeadlinesConfig,
    pub api_key_initialized : bool,
    pub search : String
}

impl Headlines {
    pub fn new() -> Headlines {
        let config : HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        Headlines { 
            api_key_initialized: !config.api_key.is_empty(),
            config,
            search: String::from("Ask GhosT ..."),
        }
    }

}


pub fn render_message_bottom(ctx : &Context, content : &mut String)-> () {
    TopBottomPanel::bottom("message").show(ctx , |ui|{
        
        ui.horizontal(|ui|{
            let mess = ui.add_sized(ui.available_size(),TextEdit::singleline(content ));
            if mess.changed(){
                //println!("{:?}",mess);
            }
            if(mess.lost_focus() && ui.input().key_pressed(egui::Key::Enter)){

            }
        });
    });
}   
pub struct NewBotResponse{
    bot : String
}
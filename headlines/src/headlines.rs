use std::{cell::RefCell, ops::{Deref}, future::Future, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}};


use confy;
use eframe::{egui::{Context, TopBottomPanel,TextEdit, output, self, TextStyle, Label, RichText, Ui, }, epaint::{FontId, Color32, Vec2}};
use serde::{Serialize,Deserialize};
use api::*;
use tokio::runtime::{ Runtime};
const WHITE : Color32 = Color32::from_rgb(255, 255, 255);
const DARK_LIGHT : Color32 = Color32::from_rgb(52, 53, 65);
const PADDING : f32 = 5.0;

#[derive(Serialize,Deserialize)]
pub struct HeadlinesConfig{
    dark_mode : bool,
    api_key : String
}

#[derive(Serialize,Deserialize)]
pub struct NewBotResponse{
    bot : String
}



impl Default for HeadlinesConfig{
    fn default() -> Self {
        Self { dark_mode: Default::default(), api_key: Default::default() }
    }
}

//Method
fn clear_intput(value : &mut String){
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
    pub dialog : Arc<Mutex<RefCell<Vec<RefCell<Userbot>>>>>,
    pub api_rx : Option<Receiver<Payload>>,
    pub api_tx : Option<Sender<Payload>>,
    pub rt: RefCell<Runtime>
}

impl Headlines {
    pub fn new(rt : RefCell<tokio::runtime::Runtime>) -> Headlines {
        let config : HeadlinesConfig = confy::load("headlines").unwrap_or_default();
        Headlines { 
            api_key_initialized: !config.api_key.is_empty(),
            config,
            search: RefCell::new("".to_string()),
            dialog: Arc::new(Mutex::new(RefCell::new(vec![]))),
            api_rx : None,
            api_tx : None,
            rt
        }
    }

    pub fn add_new_dialog(&self,is_bot : bool,content : String){
        self.dialog.lock().unwrap().borrow_mut().push(
            RefCell::new(Userbot{
                is_bot,
                expose: content.to_string(),
            })
        );
    }

    pub fn render_new_message(&self,parrent_ui : &mut eframe::egui::Ui){
        let data = self.dialog.lock().unwrap();

        for m in data.borrow().deref() {
            let textual_content = m.borrow();
            let label = Label::new(
                RichText::new(format!("{}",textual_content.expose))
                .text_style(egui::TextStyle::Body)) ;
            parrent_ui.add_space(10.);
                //ajout
                parrent_ui.allocate_ui_with_layout(Vec2{
                    x : parrent_ui.available_width(),
                    y : 50.
                }, if textual_content.is_bot {
                        egui::Layout::right_to_left()
                    }else{
                        egui::Layout::left_to_right()
                    }, |ui|{
                    ui.add(label);
                    //ui.set_height(50.);
                });
                //end -> ajout
            parrent_ui.add_space(PADDING);
            parrent_ui.add(egui::Separator::default());
            }
        parrent_ui.add_space(50.)
    }

    pub fn fecth_api() -> impl Future<Output = ()>{
        async{

        }
    }
    pub  fn render_message_bottom(&self,ctx : &Context, content : &mut String,parrent_ui : &mut Ui)-> () {
        TopBottomPanel::bottom("message").show(ctx ,  |ui| {
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
                ui.vertical_centered( |ui| {
                    let mess = ui.add_sized(
                        ui.available_size(),
                          TextEdit::singleline(content ).hint_text("Ask GhosT ...").font(egui::TextStyle::Body)
                        );
                        
                        if mess.changed(){
                            //println!("{:?}",mess);
                        }
                        if mess.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            println!("{}",content);
                            
                            
                            self.add_new_dialog(false,content.to_string());
                            //Loader
                            //fetch_api
                            /* -> implement to thread */
                            
                             if let Some(tx) = self.api_tx.clone(){
                                    let api_cursor = Api::new("",&self.rt,tx);
                                    let clone = &self.dialog.clone();
                                    api_cursor.asynchrounous_fetch(content.to_string(),clone);
                             }else{
                                println!("No channel open");
                             }

                            //start reqwest async  thread
                            
                            /* let bot_response = self.fetch_cursor.fetch(content.to_string());
                            //preload response
                            match bot_response {
                                Ok(r) => {
                                    self.add_new_dialog(true,r.bot)
                                }
                                Err(e) => {
                                    println!("{:?}",e);
                                    self.add_new_dialog(true, String::from("Sorry , I am Unable to give you a correct reponse"))
                                }
                            } */


                            //clear text Edit -> search
                            clear_intput(content);
                        }
                        

                });
                
            });
            ui.add_space(1.);
            
        });
        
    }   
    
    /* fn fetch_sync(&self,content : String,ctx : egui::Context){
    
        //let mut  rt = Runtime::new().unwrap();
           self.rt.borrow_mut().block_on(async move {
                tokio::spawn( async move {
                    let reqwest = Api::direct_fetch(content.to_string());
                   print!("Async request sent");
                   let response_body_parsed = reqwest.await.unwrap();
                   //if let Some(tx_sender) = tx{
                       println!("{:?}",response_body_parsed);
                   //        tx_sender.send(response_body_parsed);
                   //    }
                       ctx.request_repaint();
                });
           });
   } */
}




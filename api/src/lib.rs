use std::{string, collections::HashMap, cell::RefCell, borrow::BorrowMut, sync::{mpsc::Sender, Arc, Mutex}};

use reqwest::{Url};
use serde_json::json;
use ureq::{self, request};
use serde::{Serialize,Deserialize};
use exitfailure::{self, ExitFailure};
use futures::future::{self, ok};
pub struct Api<'a>{
    url : String,
    rt : &'a RefCell<tokio::runtime::Runtime>,
    api_tx : Sender<Payload>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Payload {
    pub bot : String
}


#[derive(Serialize,Deserialize,Debug)]
pub struct Userbot{
    pub is_bot : bool,
    pub expose : String
}

#[derive(Serialize,Deserialize,Debug,Clone)]

pub struct Choice{
        pub text: String,
        index: i32,
        logprobs: f32,
        finish_reason: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Usage{
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct DirectPayload{
    id: String,
    object: String,
    created: i32,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage
}


impl Api<'_> {
    pub fn new<'a>(url : &'a str,rt : &'a RefCell<tokio::runtime::Runtime>,api_tx : Sender<Payload>) -> Api<'a> {
        Api{
            url : String::from(url),
            rt,
            api_tx 
        }
    }

    pub fn asynchrounous_fetch(self,prompt : String,mut container : &Arc<Mutex<RefCell<Vec<RefCell<Userbot>>>>>) -> (){
        //let url  = String::from(self.url);
        //let url = Url::parse(&url)?;
        //body
        let body_json = json!({
            "prompt" : prompt
        });
        //let arc_share = Arc::new(Mutex::new(container)).clone();
        //fetch api
        let cnt = container.to_owned();
        self.rt.borrow_mut().block_on(async move{
            tokio::spawn(async move {
                let client = reqwest::Client::new();
                let response  = client
                    .post("https://ghost-chatgpt.onrender.com")
                    .json(&body_json)
                    .send()
                    .await.expect("unable to send request body")
                    .json::<Payload>()
                    .await.expect("No json matched")
                    ;
                    
                println!("Payload fetched : {:?}",response); 
                cnt.lock().unwrap().borrow_mut().take().push(RefCell::new(Userbot{
                    is_bot: true,
                    expose: response.bot,
                }))
                /* if  let Err(e)= self.api_tx.send(response){
                    println!("Unable to send value , {:?}",e.to_string());
                }  */
            });
        });
    }

    pub fn fetch(&self,prompt : String) -> Result<Payload,ExitFailure>{
        println!("Fetch Init");
        let reqwest = ureq::post(&self.url)
        .set("Content-Type","application/json")
        .set("Connection","keep-alive")
        .send_json(
            ureq::json!({
                "propmt" : prompt
            })
        ).expect("No request found");
        //parameter reqwest
        //end -> parameter
        println!("Get response :");
        let response : Payload = reqwest.into_json().expect("Unable Deserialize data");
        println!("{:?}",response);

        Ok(response)
    }
    
    pub  fn direct_fetch(&self,prompt : String) -> (){
        //need to add api key to header as authorization
        let body_json = json!({
            "model": "text-davinci-003",
            "prompt": "write code to say hello world in rust",
            "temperature": 0i32,
            "max_tokens": 3000i32,
            "top_p": 1i32,
            "stream": false,
            "frequency_penalty": 0.5f32,
            "presence_penalty": 0i32
        });
        self.rt.borrow_mut().block_on(async move{
            tokio::spawn(async move {
                
                let client = reqwest::Client::new();
                println!("Direct fetch init");
                let request = client.post("https://api.openai.com/v1/completions")
                .json(&body_json)
                .send();
                println!("Direct fetch have response");
                let fetched_data = request.await.expect("Error to fetched data");
                println!("Found data : {:?}",fetched_data);
                
            });
        });
        
    }
    #[cfg(test)]
    fn get_status(){
        
    }
        
}


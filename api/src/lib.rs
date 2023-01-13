use std::{string, collections::HashMap, cell::RefCell, borrow::BorrowMut};

use reqwest::{Url};
use serde_json::json;
use ureq::{self, request};
use serde::{Serialize,Deserialize};
use exitfailure::{self, ExitFailure};
pub struct Api<'a>{
    url : String,
    rt : &'a RefCell<tokio::runtime::Runtime>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Payload {
    pub bot : String
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
    pub fn new<'a>(url : &'a str,rt : &'a RefCell<tokio::runtime::Runtime>) -> Api<'a> {
        Api{
            url : String::from(url),
            rt 
        }
    }

    pub async fn asynchrounous_fetch(self,prompt : String) ->  Result<Payload,ExitFailure> {
        let url  = String::from(self.url);
        let url = Url::parse(&url)?;
        //body
        let mut body_map_json = HashMap::new();
        body_map_json.insert("prompt", prompt);
        //fetch api
        let client = reqwest::Client::new();
        let response  = client
                    .post("https://ghost-chatgpt.onrender.com")
                    .json(&body_map_json)
                    .send()
                    .await?
                    .json::<Payload>()
                    .await?
                    ;
        Ok(response)    
    }

    pub fn fetch(&self,prompt : String) -> Result<Payload,ExitFailure>{
        let reqwest = ureq::post(&self.url)
        .set("Content-Type","application/json")
        .set("Connection","keep-alive")
        .send_json(
            ureq::json!({
                "propmt" : prompt
            })
        )?;
        //parameter reqwest
        //end -> parameter
        let response : Payload = reqwest.into_json()?;
        println!("{:?}",response);
        Ok(response)
    }
    
    pub  fn direct_fetch(&self,prompt : String) -> (){
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


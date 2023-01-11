use std::{string, collections::HashMap};

use reqwest::{Url};
use ureq;
use serde::{Serialize,Deserialize};
use exitfailure::{self, ExitFailure};
pub struct Api{
    url : String
}

#[derive(Serialize,Deserialize)]
pub struct Payload {
    bot : String
}

impl Api {
    pub fn new(url : &str) -> Api {
        Api{
            url : String::from(url)
        }
    }

    pub async fn asynchrounous_fetch(&self,prompt : String) ->  Result<Payload,ExitFailure> {
        let url  = String::from(&self.url);
        let url = Url::parse(&url)?;
        //body
        let mut body_map_json = HashMap::new();
        body_map_json.insert("prompt", prompt);
        //fetch api
        let client = reqwest::Client::new();
        let response  = client
                    .post(url)
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
        .send_json(
            ureq::json!({
                "propmt" : prompt
            })
        )?;
        let response : Payload = reqwest.into_json()?;
        
        Ok(response)
    }
        
}
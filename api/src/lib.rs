use std::{string, collections::HashMap};

use reqwest::{Url, Response};
use serde::{Serialize,Deserialize};
use serde_derive::{Serialize, Deserialize};
use exitfailure::{self, ExitFailure};
struct Api<'a>{
    pub url : &'a String
}

#[derive(Serialize,Deserialize)]
struct Payload {
    bot : String
}

impl Api<'_> {
    pub fn new(url : &String) -> Api {
        Api{
            url
        }
    }
    #[cfg(test)]
    pub async fn fetch(&self,prompt : String) ->  Result<Payload,ExitFailure> {
        let url  = String::from(self.url);
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
        
}
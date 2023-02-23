use reqwest;
use std::collections::HashMap;
use serde::Deserialize;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://postman-echo.com/time/object")
        .send()
        .await
        .unwrap();

    let time_object: HashMap<String, i32> = response
        .json()
        .await
        .unwrap();
    println!("{:#?}", time_object);

}





use reqwest::{get, blocking};
use rustp2p::{Connection, ConnectionStatus};
use std::env;
use std::net::{self, IpAddr, Ipv4Addr, Ipv6Addr};
use std::io;
use futures::executor::block_on;
use tokio::runtime::Runtime;
use dotenv::dotenv;
use std::collections::HashMap;

fn main(){
    dotenv().ok();
    // /*
    let host = env::var("HOST").expect("Host not provided.");
    let port = env::var("PORT").expect("Port not provided");
    let tc = Connection::new(host, port);
    println!("{:?}", tc);
    // */
    Runtime::new()
    .expect("Failed to create Tokio runtime")
    .block_on(porra());

}

async fn porra(){
    // let json =  "{\"body\": { \"name\" : \" \", \"ip\": \"\" \" \" : \"\"}}";
    let file_name = "teste.zip";
    let base_url = env::var("SERVER_URL").expect("Server url not found.");
    let password = env::var("SENHA").expect("Senha not found.");
    let client = reqwest::Client::new();
    
    let res = client.get(format!("{}/leech?name={}",base_url, file_name).as_str()).header("auth", password.clone()).send().await.unwrap();

    let mut body = HashMap::new();
body.insert("name", file_name);
body.insert("ip", "xxxxxx");
    let res = client.post(format!("{}/seed",base_url).as_str()).header("auth", password).json(&body).send().await.unwrap();
    
    //println!("body = {:?}", body);

    println!("res: {}", res.text().await.unwrap());

}

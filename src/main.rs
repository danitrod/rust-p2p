use dotenv::dotenv;
use futures::executor::block_on;
use reqwest::{blocking, get};
use rustp2p::*;
use std::collections::HashMap;
use std::env;
use std::io;
use std::net::{self, IpAddr, Ipv4Addr, Ipv6Addr};
use tokio::runtime::Runtime;

fn main() {
    dotenv().ok();
    let host = env::var("HOST").expect("Host not provided.");
    let port = env::var("PORT").expect("Port not provided");
    let tc = Connection::new(host, port);
    println!("{:?}", tc);
    loop {
        eprint!("Select an option\n1 - Seed a file\n2 - Leech a file from an IP\n3 - Find a seeder for file\n");
        match option_input(1, 3) {
            1 => println!("seed"),
            2 => {
                let ip = ip_input();
                println!("Leeching from {}", ip)
            }
            3 => println!("find"),
            _ => panic!("Unexpected option received."),
        }
    }
    Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(porra());
}

async fn porra() {
    // let json =  "{\"body\": { \"name\" : \" \", \"ip\": \"\" \" \" : \"\"}}";
    let file_name = "teste.zip";
    let base_url = env::var("SERVER_URL").expect("Server url not found.");
    let password = env::var("SENHA").expect("Senha not found.");
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/leech?name={}", base_url, file_name).as_str())
        .header("auth", password.clone())
        .send()
        .await
        .unwrap();

    let mut body = HashMap::new();
    body.insert("name", file_name);
    body.insert("ip", "xxxxxx");

    let res = client
        .post(format!("{}/seed", base_url).as_str())
        .header("auth", password)
        .json(&body)
        .send()
        .await
        .unwrap();
    //println!("body = {:?}", body);

    println!("res: {}", res.text().await.unwrap());
}

use dotenv::dotenv;
use reqwest::Client;
use rustp2p::*;
use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use tokio::runtime::Runtime;

fn main() {
    // Get environment variables
    dotenv().ok();
    let host: Ipv4Addr = env::var("HOST")
        .expect("Host not provided.")
        .parse()
        .expect("Invalid host provided.");
    let port: u16 = env::var("PORT")
        .expect("Port not provided")
        .parse()
        .expect("Invalid port provided.");

    // Start seeding server
    let tc = Connection::new(host, port);
    println!("{:?}", tc);

    // Main event loop
    loop {
        eprint!("Select an option\n1 - Seed a file\n2 - Leech a file from an IP\n3 - Find a seeder for file\n");
        match option_input(1, 3) {
            // Seed file
            1 => Runtime::new()
                .expect("Failed to create Tokio runtime")
                .block_on(seed(host, port)),
            // Leech file
            2 => leech(),
            // Get IP list for file
            3 => Runtime::new()
                .expect("Failed to create Tokio runtime")
                .block_on(find()),
            _ => panic!("Unexpected option received."),
        }
    }
}

async fn seed(ip: Ipv4Addr, port: u16) {
    let file_name = String::from("teste.zip");
    let base_url = env::var("SERVER_URL").expect("Server url not found.");
    let password = env::var("SENHA").expect("Senha not found.");

    let mut body = HashMap::new();
    body.insert("name", file_name);
    body.insert("ip", format!("{}:{}", ip, port));

    let client = Client::new();

    let res = client
        .post(format!("{}/seed", base_url).as_str())
        .header("auth", password)
        .json(&body)
        .send()
        .await
        .unwrap();

    println!("res: {}", res.text().await.unwrap());
}

fn leech() {
    println!("Enter an IP");
    let ip = ip_input();
    println!("Enter a port");
    let port = port_input();
    println!("Leeching from {}:{}", ip, port);
}

async fn find() {
    // let json =  "{\"body\": { \"name\" : \" \", \"ip\": \"\" \" \" : \"\"}}";
    let file_name = "teste.zip";
    let base_url = env::var("SERVER_URL").expect("Server url not found.");
    let password = env::var("SENHA").expect("Senha not found.");
    let client = Client::new();

    let res = client
        .get(format!("{}/leech?name={}", base_url, file_name).as_str())
        .header("auth", password.clone())
        .send()
        .await
        .unwrap();

    println!("IP list for {}: {}", file_name, res.text().await.unwrap());
}

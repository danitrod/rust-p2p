use dotenv::dotenv;
use reqwest::Client;
use rustp2p::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};
use std::net::{IpAddr::V4, Ipv4Addr, SocketAddr, TcpStream};
use tokio::runtime::Runtime;

// checks if another client has requested a file
fn poll_requests() {}

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
    std::thread::spawn(move || -> io::Result<()> { start_server(host, port) });
    // Create Tokio runtime
    let mut rt = Runtime::new().expect("Failed to create Tokio runtime");

    // Main event loop
    loop {
        eprint!("Select an option\n1 - Seed a file\n2 - Leech a file from an IP\n3 - Find a seeder for file\n");
        match option_input(1, 3) {
            // Seed file
            1 => rt.block_on(seed(host, port)),
            // Leech file
            2 => leech().unwrap(),
            // Get IP list for file
            3 => rt.block_on(find()),
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

fn leech() -> io::Result<()> {
    println!("Enter an IP");
    let ip = ip_input();
    println!("Enter a port");
    let port = port_input();
    println!("Enter the file name");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");

    leech_from_peer(ip, port, file_name)?;
    Ok(())
}

async fn find() {
    let file_name = "banana.txt";
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

// Asks for file from peer
fn leech_from_peer(ip: Ipv4Addr, port: u16, file_name: String) -> io::Result<()> {
    let mut connection: TcpStream = TcpStream::connect(SocketAddr::new(V4(ip), port))?;
    connection.write(file_name.as_bytes())?;
    println!("Sent message to stream");

    let path = format!("share/{}", file_name);
    let mut file = File::create(path)?;

    let mut buf = [0; 4096];
    loop {
        let n = connection.read(&mut buf)?;
        println!("n is {}", n);
        if n == 0 {
            // reached end of file
            break;
        }
        println!("Message Received: {}", String::from_utf8_lossy(&buf[..]));
        file.write(&buf[..n]).expect("Error writing file");
    }

    

    Ok(())
}

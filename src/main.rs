use dotenv::dotenv;
use reqwest::Client;
use rustp2p::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};
use std::net::{IpAddr::V4, Ipv4Addr, SocketAddr, TcpStream};
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
    let url: String = env::var("SERVER_URL").expect("Server url not found");
    let password: String = env::var("SENHA").expect("Senha not found");

    // Start seeding server
    std::thread::spawn(move || -> io::Result<()> { start_server(host, port) });

    // Create Tokio runtime
    let mut rt = Runtime::new().expect("Failed to create Tokio runtime");

    // Main event loop
    loop {
        print!("\nSelect an option\n1 - Seed a file\n2 - Leech a file from an IP\n3 - Find a seeder for file\n");
        print!(">>");
        match option_input(1, 3) {
            // Seed file
            1 => rt.block_on(seed(url.clone(), password.clone())),
            // Leech file
            2 => leech().unwrap(),
            // Get IP list for file
            3 => rt.block_on(find(url.clone(), password.clone())),
            _ => panic!("Unexpected option received."),
        }
    }
}

async fn seed(url: String, password: String) {
    print!("Enter the file name\n>>");
    let file_name = text_input();
    print!("Enter your public IP\n>>");
    let ip = ip_input();
    print!("Enter your port\n>>");
    let port = port_input();

    let mut body = HashMap::new();
    body.insert("name", file_name);
    body.insert("ip", format!("{}:{}", ip, port));

    let client = Client::new();

    let res = client
        .post(format!("{}/seed", url).as_str())
        .header("auth", password)
        .json(&body)
        .send()
        .await
        .unwrap();

    println!("Server response: {}", res.text().await.unwrap());
}

fn leech() -> io::Result<()> {
    print!("Enter an IP\n>>");
    let ip = ip_input();
    print!("Enter a port\n>>");
    let port = port_input();
    print!("Enter the file name\n>>");
    let file_name = text_input();

    leech_from_peer(ip, port, file_name)?;
    Ok(())
}

async fn find(url: String, password: String) {
    print!("Enter the file name\n>>");
    let file_name = text_input();
    let client = Client::new();

    let res = client
        .get(format!("{}/leech?name={}", url, file_name).as_str())
        .header("auth", password)
        .send()
        .await
        .unwrap();

    println!("IP list for {}: {}", file_name, res.text().await.unwrap());
}

// Asks for file from peer
fn leech_from_peer(ip: Ipv4Addr, port: u16, file_name: String) -> io::Result<()> {
    let mut connection: TcpStream = TcpStream::connect(SocketAddr::new(V4(ip), port))?;
    connection.write(file_name.as_bytes())?;
    println!("Sent request to peer");

    let path = format!("share/{}", file_name);
    let mut file = File::create(path)?;

    let mut buf = [0; MAX_FILE_SIZE];
    loop {
        let n = connection.read(&mut buf)?;
        if n == 0 {
            // reached end of file
            break;
        }
        println!("File Received: {}", file_name);
        println!("File size {}bytes", n);
        file.write(&buf[..n]).expect("Error writing file");
    }

    Ok(())
}

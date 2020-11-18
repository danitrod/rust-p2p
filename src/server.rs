use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{IpAddr::V4, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::path::Path;

// start peer server
pub fn start_server(host: Ipv4Addr, port: u16) -> io::Result<()> {
  println!("Starting server on port {}", port);

  let tcp_listener = TcpListener::bind(SocketAddr::new(V4(host), port))?;
  let mut buf = [0; 4096];
  // Handle each connection
  for stream in tcp_listener.incoming() {
    // Read file name from stream
    let mut tcp_stream: TcpStream = stream.unwrap();
    let read_bytes = tcp_stream.read(&mut buf).unwrap();
    let file_name = String::from_utf8_lossy(&buf[..read_bytes]);
    println!(
      "Received connection from {} requesting {}",
      tcp_stream.peer_addr().unwrap(),
      file_name
    );

    // Search for the file in the share folder
    let path = format!("share/{}", file_name);
    let mut file = match File::open(Path::new(path.as_str())) {
      Ok(f) => (f),
      _ => {
        println!("Connection searched for invalid file.");
        tcp_stream.shutdown(Shutdown::Both).unwrap();
        continue;
      }
    };

    file.read(&mut buf)?;

    // Return file
    tcp_stream.write(&buf)?;
  }

  Ok(())
}

/*
  let mut buf = [0; 4096];
  loop {
    n = file.read(&mut buf)?;
    if n == 0 {
      // reached end of file
      break;
    }
  }
*/
//println!("Message Received: {}", String::from_utf8_lossy(&buf[..]));

/*

use std::fs::File;
use std::io::{self, Read, Write};
use std::net::IpAddr::V4;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpStream;

pub struct Client {}

impl Client {
  pub fn new(host: Ipv4Addr, port: u16) -> io::Result<()> {
    let mut buf = [0; 4096];
    let mut file = File::create("recebido.txt")?;
    file.set_len(0)?;
    let mut tcp = TcpStream::connect(SocketAddr::new(V4(host), port))?;
    loop {
      let n = tcp.read(&mut buf)?;
      println!("n is {}", n);
      if n == 0 {
        // reached end of file
        break;
      }
      println!("Message Received: {}", String::from_utf8_lossy(&buf[..]));
      file.write_all(&mut buf[..n])?;
    }
    Ok(())
  }
}
*/

/*

  let mut file = File::open("resposta.txt")?;
  let mut n;
  let mut buf = [0; 4096];
  loop {
    n = file.read(&mut buf)?;
    if n == 0 {
      // reached end of file
      break;
    }
  }

*/

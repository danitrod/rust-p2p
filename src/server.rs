use crate::MAX_FILE_SIZE;
use std::env::current_dir;
use std::fs::read;
use std::io::{self, Read, Write};
use std::net::{IpAddr::V4, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};

// start peer server
pub fn start_server(host: Ipv4Addr, port: u16) -> io::Result<()> {
  println!("Starting server on port {}", port);

  let tcp_listener = TcpListener::bind(SocketAddr::new(V4(host), port))?;
  let mut buf = [0; MAX_FILE_SIZE];
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
    let path = format!("{}/share/{}", current_dir().unwrap().display(), file_name);
    let file = match read(path) {
      Ok(f) => (f),
      Err(err) => {
        println!("Error searching for file: {}", err);
        tcp_stream.shutdown(Shutdown::Both).unwrap();
        continue;
      }
    };

    // Return file
    tcp_stream.write(&file)?;
  }

  Ok(())
}

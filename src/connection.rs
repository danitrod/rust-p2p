use std::net::Ipv4Addr;

#[derive(Debug)]
pub enum ConnectionStatus {
    DEAD,
    ALIVE,
}
#[derive(Debug)]
pub struct Connection {
    host: Ipv4Addr,
    port: u16,
    status: ConnectionStatus,
}

impl Connection {
    pub fn new(host: String, port: String) -> Connection {
        Connection {
            host: host.parse().expect("Invalid host provided."),
            port: port.parse().expect("Invalid port provided."),
            status: ConnectionStatus::DEAD,
        }
    }
}

// P2P Connection manager
#[derive(Debug)]
pub struct ConnManager {
    // Number of connections being mangaged
    n_alive: u16,
    connections: Vec<Connection>,
}

impl ConnManager {
    pub fn new() -> ConnManager {
        ConnManager {
            n_alive: 0,
            connections: Vec::with_capacity(4),
        }
    }
}

use std::io;
use std::net::Ipv4Addr;

mod connection;

pub use connection::Connection;
pub use connection::ConnectionStatus;

pub fn ip_input() -> Ipv4Addr {
    eprint!(">>");
    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let inp: Ipv4Addr = match inp.trim().parse() {
            Ok(ip) => ip,
            Err(_) => {
                eprint!("Please enter a valid IPv4\n>>");
                continue;
            }
        };
        return inp;
    }
}

pub fn option_input(min: u8, max: u8) -> u8 {
    eprint!(">>");
    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let inp: u8 = match inp.trim().parse() {
            Ok(num) => {
                if num < min || num > max {
                    eprint!("Please enter a valid option\n>>");
                    continue;
                }
                num
            }
            Err(_) => {
                eprint!("Please enter a valid option\n>>");
                continue;
            }
        };
        return inp;
    }
}

use std::io;
use std::net::Ipv4Addr;

mod server;

pub use server::start_server;

pub fn ip_input() -> Ipv4Addr {
    print!(">>");
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
    print!(">>");
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

pub fn port_input() -> u16 {
    print!(">>");
    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let inp: u16 = match inp.trim().parse() {
            Ok(num) => {
                if num < 10 {
                    eprint!("Please enter a valid port\n>>");
                    continue;
                }
                num
            }
            Err(_) => {
                eprint!("Please enter a valid port\n>>");
                continue;
            }
        };
        return inp;
    }
}

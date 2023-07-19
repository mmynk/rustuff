use socket2::{Domain, Socket, Type};

use crate::common;

pub fn send() {
    let sock = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();
    let msg = "🤖".as_bytes();

    if let Ok(addr) = common::ip() {
        if let Ok(_) = sock.send_to(msg, &addr) {
            println!("Success! :)");
        } else {
            println!("Failed to send msg! :(");
        }
    } else {
        println!("Failed to parse ip! :(");
    }
}

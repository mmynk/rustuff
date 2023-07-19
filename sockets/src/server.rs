use std::str;
use std::io::Read;

use socket2::{Domain, Socket, Type};

use crate::common;

pub(crate) fn recv() {
    let mut sock = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();

    if let Ok(addr) = common::ip() {
        if sock.bind(&addr).is_err() {
            println!("Failed to bind! :(")
        }

        let mut buf = vec![0; 1024];
        loop {
            let amt = sock.read(&mut buf).unwrap();
            if !buf.is_empty() {
                if let Ok(data) = str::from_utf8(&buf[..amt]) {
                    println!("Received {amt} bytes of data: {data}");
                }
            }
        }
    } else {
        println!("Failed to parse ip! :(");
    }
}

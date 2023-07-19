use std::net::{AddrParseError, SocketAddr};

use socket2::SockAddr;

pub const IP: &str = "127.0.0.1";
pub const PORT: u16 = 12345;

pub fn ip() -> Result<SockAddr, AddrParseError> {
    let ip_addr_result = IP.parse();
    let ip = match ip_addr_result {
        Ok(ip_addr) => ip_addr,
        Err(e) => return Err(e),
    };

    let sock_addr = SocketAddr::new(ip, PORT);
    Ok(SockAddr::from(sock_addr))
}

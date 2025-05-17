use anyhow::{Result, anyhow};
use std::env;
use std::net::UdpSocket;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

fn parse_tx_or_rx(tx_or_rx: &str) -> Result<bool> {
    if tx_or_rx == "rx" {
        return Ok(true);
    } else if tx_or_rx == "tx" {
        return Ok(false);
    }
    Err(anyhow!("Neither tx or rx option passed as param"))
}

fn parse_ip(ip: &str) -> Result<SocketAddr> {
    match SocketAddrV4::from_str(ip) {
        Ok(parsed_ip) => Ok(SocketAddr::V4(parsed_ip)),
        Err(_) => match SocketAddrV6::from_str(ip) {
            Ok(parsed_ip) => Ok(SocketAddr::V6(parsed_ip)),
            Err(_) => Err(anyhow!("Neither ipv4 or ipv6 passed as param")),
        },
    }
}

fn tx(args: &Vec<String>) -> Result<()> {
    let tx_socket_addr = parse_ip(&args[2])?;
    let socket1 = UdpSocket::bind("127.0.0.1:34254")?;
    socket1
        .send_to(&[2; 10], tx_socket_addr)
        .expect("failed to send");
    Ok(())
}

fn rx() -> Result<()> {
    let mut buf: [u8; 10] = [3; 10];
    let rx_socket = UdpSocket::bind("127.0.0.1:12345")?;
    rx_socket.recv_from(&mut buf).expect("failed to rec");
    for elem in buf {
        println!("{}", elem);
    }
    Ok(())
}

fn main() -> Result<()> {
    {
        let args: Vec<String> = env::args().collect();
        let rx_option = parse_tx_or_rx(&args[1])?;

        if !rx_option {
            tx(&args);
        };
        rx();
    } // the socket is closed here
    Ok(())
}

use anyhow::{Result, anyhow};
use std::env;
use std::net::UdpSocket;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::{FromStr,from_utf8};
use std::{thread,io};

fn parse_ip(ip: &str) -> Result<SocketAddr> {
    match SocketAddrV4::from_str(ip) {
        Ok(parsed_ip) => Ok(SocketAddr::V4(parsed_ip)),
        Err(_) => match SocketAddrV6::from_str(ip) {
            Ok(parsed_ip) => Ok(SocketAddr::V6(parsed_ip)),
            Err(_) => Err(anyhow!("Neither ipv4 or ipv6 passed as param")),
        },
    }
}

fn tx(tx_socket_addr: &SocketAddr, message:&String) -> Result<()> {
    let socket1 = UdpSocket::bind("127.0.0.1:12336")?;
    socket1
        .send_to(message.as_bytes(), tx_socket_addr)
        .expect("failed to send");
    Ok(())
}

fn rx() -> Result<()> {
    let mut buf: [u8; 1000] = [0; 1000];
    let rx_socket = UdpSocket::bind("127.0.0.1:12346")?;
    while true {
    let (bytes_read,_)= rx_socket.recv_from(&mut buf)?;
    println!("Received:{}",from_utf8(&buf[..bytes_read])?);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let tx_socket_addr = parse_ip(&args[1])?;
    //thread::spawn(move || {tx(&tx_socket_addr);});
    thread::spawn(move || {rx()});
    while true{
    let mut buffer = String::new();
    let line_length = io::stdin().read_line(&mut buffer)?;
    print!("\x1B[F"); 
    println!("Sending:{}",&buffer);
    tx(&tx_socket_addr,&buffer);
    //println!("{}",buffer);

    }

    Ok(())
}

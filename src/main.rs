use anyhow::{Result, anyhow};
use std::env;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};
use std::str::{FromStr, from_utf8};
use std::{io, thread};

fn parse_ip(ip: &str) -> Result<SocketAddr> {
    match SocketAddrV4::from_str(ip) {
        Ok(parsed_ip) => Ok(SocketAddr::V4(parsed_ip)),
        Err(_) => match SocketAddrV6::from_str(ip) {
            Ok(parsed_ip) => Ok(SocketAddr::V6(parsed_ip)),
            Err(_) => Err(anyhow!("Neither ipv4 or ipv6 passed as param")),
        },
    }
}

fn tx(udp_socket: &UdpSocket, tx_socket_addr: &SocketAddr, message: &String) -> Result<()> {
    udp_socket
        .send_to(message.as_bytes(), tx_socket_addr)
        .expect("failed to send");
    Ok(())
}

fn rx(udp_socket: &UdpSocket) -> Result<()> {
    let mut buf: [u8; 1000] = [0; 1000];

    while true {
        let (bytes_read, socket_addr) = udp_socket.recv_from(&mut buf)?;

        // Will still receive messages from a non target address
        // Maybe add logic to only show messages from target
        println!(
            "Received from {}:{}",
            socket_addr,
            from_utf8(&buf[..bytes_read])?
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let local_socket_addr = parse_ip(&args[1])?;
    let target_socket_addr = parse_ip(&args[2])?;
    let udp_socket = UdpSocket::bind(local_socket_addr)?;

    // We do this as we can't borrow the udp_socket
    // for both the tx and rx funcs
    let udp_socket_clone = udp_socket.try_clone()?;

    // We start the rx thread
    thread::spawn(move || rx(&udp_socket_clone));

    while true {
        let mut buffer = String::new();
        let line_length = io::stdin().read_line(&mut buffer)?;

        // Clear the output from the user input
        print!("\x1B[F");
        print!("Sending:{}", &buffer);

        tx(&udp_socket, &target_socket_addr, &buffer);
    }

    Ok(())
}

use chacha20poly1305::{
    ChaCha20Poly1305, Key,
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};
use std::str::{FromStr, from_utf8};
use std::{env, io, thread};

use anyhow::{Result, anyhow};
mod crypto;

fn parse_ip(ip: &str) -> Result<SocketAddr> {
    match SocketAddrV4::from_str(ip) {
        Ok(parsed_ip) => Ok(SocketAddr::V4(parsed_ip)),
        Err(_) => match SocketAddrV6::from_str(ip) {
            Ok(parsed_ip) => Ok(SocketAddr::V6(parsed_ip)),
            Err(_) => Err(anyhow!("Neither ipv4 or ipv6 passed as param")),
        },
    }
}

fn tx(udp_socket: &UdpSocket, tx_socket_addr: &SocketAddr, message: &Vec<u8>) -> Result<()> {
    udp_socket.send_to(message, tx_socket_addr)?;
    Ok(())
}

fn rx(udp_socket: &UdpSocket, key: &Key) -> Result<()> {
    //let mut buf: Vec<u8> = Vec::new();
    let mut buf: [u8; 1000] = [0; 1000];
    let mut rx_counter: u32 = 0;

    while true {
        let (bytes_read, socket_addr) = udp_socket.recv_from(&mut buf)?;
        //let mut buf_vec : Vec<u8> = buf[..bytes_read].to_vec();
        let mut buf_vec: Vec<u8> = buf[..bytes_read].to_vec(); // Note: buffer needs 16-bytes overhead for auth tag
        match crypto::decrypt_ciphertext(&mut buf_vec, key, &mut rx_counter) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        }

        // Will still receive messages from a non target address
        // Maybe add logic to only show messages from target
        println!("Received from {}:{}", socket_addr, from_utf8(&buf_vec)?);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let local_socket_addr = parse_ip(&args[1])?;
    let target_socket_addr = parse_ip(&args[2])?;
    let udp_socket = UdpSocket::bind(local_socket_addr)?;
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let mut tx_counter :u32 = 0;

    // We do this as we can't borrow the udp_socket
    // for both the tx and rx funcs
    let udp_socket_clone = udp_socket.try_clone()?;
    let key_clone = key.clone();

    // We start the rx thread
    thread::spawn(move || rx(&udp_socket_clone, &key_clone));
    println!("Started...\n");

    while true {
        let mut buffer = String::new();
        let line_length = io::stdin().read_line(&mut buffer)?;

        // Clear the output from the user input
        print!("\x1B[F");
        print!("Sending:{}", &buffer);

        let mut buffer_vec: Vec<u8> = buffer.into_bytes();
        crypto::encrypt_plaintext(&mut buffer_vec, &key,&mut tx_counter)?;

        let _ = tx(&udp_socket, &target_socket_addr, &buffer_vec);
    }

    Ok(())
}

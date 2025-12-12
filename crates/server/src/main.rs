use std::{io::Read, net::UdpSocket};

fn main() {
    let address = "127.0.0.1";
    let port = "7777";
    let socket = UdpSocket::bind(format!("{address}:{port}")).expect("Couldn't bind to address");

    let mut buf = [0u8; 1024];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let data = &buf[..number_of_bytes];

        handle_packet(data);
    }
}

fn handle_packet(data: &[u8]) {
    let text = String::from_utf8_lossy(data);

    println!("Hello, Client!");
    println!("Received message: {text}");
}

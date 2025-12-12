use std::{net::UdpSocket, thread, time::Duration};

fn main() {
    let address = "127.0.0.1";
    let port = "7777";
    let socket = UdpSocket::bind(format!("{address}:0")).expect("Couldn't bind to address");
    socket
        .connect(format!("{address}:{port}"))
        .expect("Couldn't connect to server");

    let mut buf = [0u8; 1024];

    loop {
        socket
            .send("Hello, server!".as_bytes())
            .expect("Couldn't send packet");

        thread::sleep(Duration::from_secs(1));
    }
}

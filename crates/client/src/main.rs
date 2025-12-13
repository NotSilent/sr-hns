use std::{net::UdpSocket, thread, time::Duration};

use common::{Packet, PacketBuilder};

fn main() {
    let address = "127.0.0.1";
    let port = "7777";
    let socket = UdpSocket::bind(format!("{address}:0")).expect("Couldn't bind to address");
    socket
        .connect(format!("{address}:{port}"))
        .expect("Couldn't connect to server");

    let mut packet_builder = PacketBuilder::default();

    let packet = Packet::Input(0.9, -0.7);
    let encoded_packet = common::encode(&mut packet_builder, &packet).unwrap();
    loop {
        socket.send(encoded_packet).expect("Couldn't send packet");

        thread::sleep(Duration::from_secs(1));
    }
}

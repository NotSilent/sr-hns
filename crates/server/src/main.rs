use std::{
    net::{SocketAddr, UdpSocket},
    thread,
    time::{Duration, Instant},
};

// use common::TestPosition;

const TIMEOUT: f32 = 5.0;

struct Connection {
    timeout: f32,
    src_addr: SocketAddr,
}

impl Connection {
    pub fn new(src_addr: SocketAddr) -> Self {
        Self {
            timeout: TIMEOUT,
            src_addr,
        }
    }
}

#[derive(Default)]
struct Connections(Vec<Connection>);

impl Connections {
    fn drop_timeouts(&mut self, delta_time: f32) {
        let mut index = 0;
        while index < self.0.len() {
            let connection = &mut self.0[index];
            connection.timeout -= delta_time;

            if connection.timeout < 0.0 {
                println!("Client timeout: {}", connection.src_addr);
                self.0.swap_remove(index);
            }

            index += 1;
        }
    }

    fn update(&mut self, src_addr: &SocketAddr) {
        let index = self.0.iter().position(|c| c.src_addr == *src_addr);
        let index = if let Some(index) = index {
            index
        } else {
            let new_connection = Connection::new(*src_addr);
            self.0.push(new_connection);

            self.0.len() - 1
        };

        let conection = &mut self.0[index];
        conection.timeout = TIMEOUT;
    }
}

fn main() {
    let mut connections = Connections::default();

    let address = "127.0.0.1";
    let port = "7777";
    let socket = UdpSocket::bind(format!("{address}:{port}")).expect("Couldn't bind to address");
    socket.set_nonblocking(true).unwrap();

    println!("Server started\n");

    let mut buf = [0u8; 1024];

    let timestep = Duration::from_secs_f64(1.0 / 20.0);

    println!("===== Config =====");
    println!("Timestep: {}\n", timestep.as_secs_f64());

    // let mut test_position = TestPosition::default();

    loop {
        let start = Instant::now();

        while let Ok((number_of_bytes, src_addr)) = socket.recv_from(&mut buf) {
            let data = &buf[..number_of_bytes];

            handle_packet(data);
            connections.update(&src_addr);
        } // TODO: Error handling

        connections.drop_timeouts(timestep.as_secs_f32());

        let duration = start.elapsed();

        thread::sleep(timestep - duration);
    }
}

fn handle_packet(data: &[u8]) {
    let packet = common::decode(data).unwrap();

    println!("Received packet: {:?}", packet);
}

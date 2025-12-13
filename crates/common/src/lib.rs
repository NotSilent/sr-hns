use std::io::Read;

struct EntityId(usize);

#[derive(Default)]
pub struct TestPosition {
    x: f32,
    y: f32,
}

const PROTOCOL_ID: u32 = 0xDB97;

// Packet Layout
// [u32]        [u32]
// [PROTOCOL_ID][PACKET_TYPE_ID]
// [[u8]]
// [DATA]

#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum Packet {
    Ping,
    Input(f32, f32) = 1,
}

pub fn encode<'a>(
    packet_builder: &'a mut PacketBuilder,
    packet_data: &Packet,
) -> Result<&'a [u8], ()> {
    match packet_data {
        Packet::Ping => Err(()),
        Packet::Input(x, y) => {
            let packet_type_id = 1_u32.to_ne_bytes();
            let x = x.to_ne_bytes();
            let y = y.to_ne_bytes();

            packet_builder.reset();
            packet_builder.push_data(&packet_type_id);
            packet_builder.push_data(&x);
            packet_builder.push_data(&y);

            Ok(packet_builder.get())
        }
    }
}

pub fn decode(packet: &[u8]) -> Result<Packet, ()> {
    // Better Error handling

    // TODO: Validate protocol id and maybe incorrect address before passing the rest here

    if packet.len() > 1024 {
        return Err(());
    }

    let _protocol_id = u32::from_ne_bytes(packet[0..4].try_into().unwrap());
    let packet_type_id = u32::from_ne_bytes(packet[4..8].try_into().unwrap());

    match &packet_type_id {
        1 => {
            let x = f32::from_ne_bytes(packet[8..12].try_into().unwrap());
            let y = f32::from_ne_bytes(packet[12..16].try_into().unwrap());

            Ok(Packet::Input(x, y))
        }
        _ => Err(()),
    }
}

pub struct PacketBuilder {
    data: [u8; 1024],
    byte_index: usize,
}

impl Default for PacketBuilder {
    fn default() -> Self {
        Self {
            data: [0; 1024],
            byte_index: Default::default(),
        }
    }
}

impl PacketBuilder {
    fn reset(&mut self) {
        self.byte_index = 0;

        self.push_data(&PROTOCOL_ID.to_ne_bytes());
    }

    fn push_data(&mut self, data: &[u8]) {
        // Error handling

        self.data[self.byte_index..self.byte_index + data.len()].copy_from_slice(&data);

        self.byte_index += data.len();
    }

    fn get(&self) -> &[u8] {
        &self.data[..self.byte_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_input() {
        let mut packet_builder = PacketBuilder::default();

        let packet = Packet::Input(0.33, 0.66);
        let encoded = encode(&mut packet_builder, &packet).unwrap();
        let decoded = decode(encoded).unwrap();

        assert_eq!(packet, decoded);
    }
}

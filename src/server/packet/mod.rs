pub mod packet_type;
use packet_type::PacketType;

pub mod packet_data;
use packet_data::PacketData;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Packet {
    pub len: usize,
    pub packet_type: PacketType,
    pub data: Vec<PacketData>
}

impl Clone for Packet {
    fn clone(&self) -> Self{
        Packet{
            len: self.len,
            packet_type: self.packet_type,
            data: self.data.clone()
        }
    }
}

impl Packet {
    pub fn new(len: usize, raw_data: &[u8]) -> Packet {
        let packet_type: PacketType = PacketType::from_byte(raw_data[0]);
        let data = vec![PacketData::Bool(true)];
        
        Packet{
            len: len,
            packet_type: packet_type,
            data: data
        }
    }
}
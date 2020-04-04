#[derive(Debug, PartialEq, Eq, Hash, Copy)]
pub enum PacketType {
    Unknown
}

impl Clone for PacketType {
    fn clone(&self) -> Self{
        match self {
            PacketType::Unknown => PacketType::Unknown,
            // _ => {}
        }
    }
}

impl PacketType {
    pub fn from_byte(byte: u8) -> PacketType {
        match byte {
            _ => PacketType::Unknown
        }
    }

    pub fn into_byte(&self) -> u8 {
        match self {
            PacketType::Unknown => 0u8
        }
    }
}
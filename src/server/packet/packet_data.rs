#[derive(Debug, PartialEq, Eq, Hash, Copy)]
pub enum PacketData {
    Bool(bool),
    Byte(i8),
    UByte(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    Long(i64),
    Float(u8),
    Double(u8),
    Str(u8),
    Chat(u8),
    Identifier(u8),
    VarInt(u8),
    VarLong(u8),
    EntityMetadata(u8),
    Slot(u8),
    NBTTag(u8),
    Position(u8),
    Angle(usize),
    UUID([u64; 2]),
    ByteArray(u8)
}

impl Clone for PacketData {
    fn clone(&self) -> Self {
        match self {
            PacketData::Bool(inner) => PacketData::Bool(*inner),
            PacketData::Byte(inner) => PacketData::Byte(*inner),
            PacketData::UByte(inner) => PacketData::UByte(*inner),
            PacketData::Short(inner) => PacketData::Short(*inner),
            PacketData::UShort(inner) => PacketData::UShort(*inner),
            PacketData::Int(inner) => PacketData::Int(*inner),
            PacketData::Long(inner) => PacketData::Long(*inner),
            PacketData::Float(inner) => PacketData::Float(*inner),
            PacketData::Double(inner) => PacketData::Double(*inner),
            PacketData::Str(inner) => PacketData::Str(*inner),
            PacketData::Chat(inner) => PacketData::Chat(*inner),
            PacketData::Identifier(inner) => PacketData::Identifier(*inner),
            PacketData::VarInt(inner) => PacketData::VarInt(*inner),
            PacketData::VarLong(inner) => PacketData::VarLong(*inner),
            PacketData::EntityMetadata(inner) => PacketData::EntityMetadata(*inner),
            PacketData::Slot(inner) => PacketData::Slot(*inner),
            PacketData::NBTTag(inner) => PacketData::NBTTag(*inner),
            PacketData::Position(inner) => PacketData::Position(*inner),
            PacketData::Angle(inner) => PacketData::Angle(*inner),
            PacketData::UUID(inner) => PacketData::UUID(*inner),
            PacketData::ByteArray(inner) => PacketData::ByteArray(*inner)
        }
    }
}

// Functions for reading data types
pub fn read_bool(byte: u8) -> bool {
    byte != 0
}
pub fn read_byte(byte: u8) -> i8 {
    if byte < 128 {
        byte as i8
    } else if byte == 128 {
        -128
    } else {
        -1 * (!(byte ^ 0) + 1) as i8
    }
}
pub fn read_ushort(bytes: [u8; 2]) -> u16 {
    // bytes[0] as u16 * 256 + bytes[1] as u16
    let mut byte_size = 16;
    let mut total = 0u16;
    for byte in &bytes {
        byte_size -= 8;
        total += *byte as u16 * 2u16.pow(byte_size);
    }
    total
}
pub fn read_short(bytes: [u8; 2]) -> i16 {
    // Change array of u8 into one u16
    let ushort = read_ushort(bytes);

    // Change unsigned to signed
    if ushort < 32768 {
        ushort as i16
    } else if ushort == 32768 {
        -32768
    } else {
        -1 * (!(ushort ^ 0) + 1) as i16
    }
}
pub fn read_int(bytes: [u8; 4]) -> i32 {
    // Change array of u8 into one u32
    // let unsigned = bytes[0] as u32 * 16777216 + bytes[1] as u32 * 65536 + bytes[2] as u32 * 256 + bytes[3] as u32;
    let mut byte_size = 32;
    let mut unsigned = 0u32;
    for byte in &bytes {
        byte_size -= 8;
        unsigned += *byte as u32 * 2u32.pow(byte_size);
    }

    // Change unsigned to signed
    if unsigned < 2147483648 {
        unsigned as i32
    } else if unsigned == 2147483648 {
        -2147483648
    } else {
        -1 * (!(unsigned ^ 0) + 1) as i32
    }
}
pub fn read_long(bytes: [u8; 8]) -> i64 {
    // Change array of u8 into one u64
    let mut byte_size = 64;
    let mut unsigned = 0u64;
    for byte in &bytes {
        byte_size -= 8;
        unsigned += *byte as u64 * 2u64.pow(byte_size);
    }

    // Change unsigned to signed
    if unsigned < 9223372036854775808 {
        unsigned as i64
    } else if unsigned == 9223372036854775808 {
        -9223372036854775808
    } else {
        -1 * (!(unsigned ^ 0) + 1) as i64
    }
}
pub fn read_float(bytes: [u8; 4]) {}
pub fn read_double(bytes: [u8; 8]) {}
pub fn read_string(byte: u8) {}
pub fn read_chat(byte: u8) {}
pub fn read_identifier(byte: u8) {}
pub fn read_var_int(byte: u8) {}
pub fn read_var_long(byte: u8) {}
pub fn read_entity_metadata(byte: u8) {}
pub fn read_slot(byte: u8) {}
pub fn read_nbt_tag(byte: u8) {}
pub fn read_position(bytes: [u8; 8]) {}
pub fn read_angle(byte: u8) -> usize {
    (byte as usize + 1) * 360 / 256
}
pub fn read_uuid(bytes: [u8; 16]) -> [u64; 2] {
    let mut byte_size = 128;
    let mut uuid = [0u64; 2];

    for byte in &bytes {
        byte_size -= 8;
        if byte_size < 64 {
            uuid[1] += *byte as u64 * 2u64.pow(byte_size);
        }
        uuid[0] += *byte as u64 * 2u64.pow(byte_size);
    }

    uuid
}
pub fn read_byte_array(byte: u8) {}


// Functions for encoding datatypes
pub fn encode_bool(data: bool) -> &'static[u8] {
    match data {
        true => &[1u8],
        false => &[0u8]
    }
}
pub fn encode_byte(data: i8) -> &'static[u8] {
    if data >= 0 {
        &[data as u8]
    } else {
        &[!(((-1 * data) as u8 - 1) ^ 0)]
    }
}
pub fn encode_ushort(data: u16) -> &'static[u8] {}
pub fn encode_short(data: i16) -> &'static[u8] {}
pub fn encode_int(data: i32) -> &'static[u8] {}
pub fn encode_long(data: i64) -> &'static[u8] {}
pub fn encode_float(data: u8) -> &'static[u8] {}
pub fn encode_double(data: u8) -> &'static[u8] {}
pub fn encode_string(data: u8) -> &'static[u8] {}
pub fn encode_chat(data: u8) -> &'static[u8] {}
pub fn encode_identifier(data: u8) -> &'static[u8] {}
pub fn encode_var_int(data: u8) -> &'static[u8] {}
pub fn encode_var_long(data: u8) -> &'static[u8] {}
pub fn encode_entity_metadata(data: u8) -> &'static[u8] {}
pub fn encode_slot(data: u8) -> &'static[u8] {}
pub fn encode_nbt_tag(data: u8) -> &'static[u8] {}
pub fn encode_position(data: u8) -> &'static[u8] {}
pub fn encode_angle(data: usize) -> &'static[u8] {}
pub fn encode_uuid(data: [u64; 2]) -> &'static[u8] {}
pub fn encode_byte_array(data: u8) -> &'static[u8] {}
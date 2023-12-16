use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum Opcode {
    Null = 0x00,
    LdaImmediate = 0xA9,
    Tax = 0xAA,
    Inx = 0xE8,
}
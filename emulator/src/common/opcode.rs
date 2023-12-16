pub enum Opcode {
    Null,
    LdaImmediate,
    Tax,
}

impl Opcode {
    pub fn from_u8(value: u8) -> Opcode {
        match value {
            0x00 => Opcode::Null,
            0xA9 => Opcode::LdaImmediate,
            0xAA => Opcode::Tax,
            _ => Opcode::Null,
        }
    }
}
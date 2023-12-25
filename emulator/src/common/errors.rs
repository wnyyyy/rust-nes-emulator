use std::fmt;

#[derive(Debug)]
pub enum EmulatorError {
    InvalidOpcode(u8),
    UnimplementedOpcode(u8),
    UnimplementedAddressingMode(String),
    AccessViolation(u16),
}

impl fmt::Display for EmulatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmulatorError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: {:x}", opcode),
            EmulatorError::UnimplementedOpcode(opcode) => write!(f, "Unimplemented opcode: {:x}", opcode),
            EmulatorError::UnimplementedAddressingMode(name) => write!(f, "Address mode does not have a parameter address: {}", name),
            EmulatorError::AccessViolation(address) => write!(f, "Access violation at address: {:x}", address),
        }
    }
}
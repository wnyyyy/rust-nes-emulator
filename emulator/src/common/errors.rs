use std::fmt;

#[derive(Debug)]
pub enum EmulatorError {
    InvalidOpcode(u8),
}

impl fmt::Display for EmulatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmulatorError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: {:x}", opcode),
        }
    }
}

impl std::error::Error for EmulatorError {}
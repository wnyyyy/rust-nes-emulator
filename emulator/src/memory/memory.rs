use crate::common::errors::EmulatorError;

pub trait Memory {
    fn read(&self, address: u16) -> Result<u8, EmulatorError>;
    fn read_u16(&self, address: u16) -> Result<u16, EmulatorError>;
    fn write(&mut self, address: u16, value: u8) -> Result<(), EmulatorError>;
    fn write_u16(&mut self, address: u16, value: u16) -> Result<(), EmulatorError>;
}
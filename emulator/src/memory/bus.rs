use crate::memory::memory::Memory;
use crate::common::errors::EmulatorError;
use crate::common::constants::{PPU_END, PPU_START, RAM_END, RAM_SIZE, RAM_START, MEMORY_SIZE, PRG_ROM_START, PRG_ROM_END};

pub struct Bus {
   memory: [u8; MEMORY_SIZE]
}

impl Bus {
   pub fn new() -> Self{
       Bus {
           memory: [0; MEMORY_SIZE]
       }
   }
}

impl Memory for Bus {
    fn read(&self, address: u16) -> Result<u8, EmulatorError> {
        match address {
            RAM_START ..= RAM_END => {
                let mirror_address = (address % RAM_SIZE as u16) as usize;
                Ok(self.memory[mirror_address])
            }
            PPU_START ..= PPU_END => {
                Err(EmulatorError::AccessViolation(address))
            }
            PRG_ROM_START ..= PRG_ROM_END => {
                Ok(self.memory[address as usize])
            }
            _ => {
                Err(EmulatorError::AccessViolation(address))
            }
        }
    }

    fn read_u16(&self, address: u16) -> Result<u16, EmulatorError> {
        let low_byte = self.read(address)?;
        let high_byte = self.read(address + 1)?;
        Ok(u16::from_le_bytes([low_byte, high_byte]))
    }

    fn write(&mut self, address: u16, data: u8) -> Result<(), EmulatorError> {
        match address {
            RAM_START ..= RAM_END => {
                let mirror_address = (address % RAM_SIZE as u16) as usize;
                self.memory[mirror_address] = data;
                Ok(())
            }
            PPU_START ..= PPU_END => {
                Err(EmulatorError::AccessViolation(address))
            }
            PRG_ROM_START ..= PRG_ROM_END => {
                self.memory[address as usize] = data;
                Ok(())
            }
            _ => {
                Err(EmulatorError::AccessViolation(address))
            }
        }
    }

    fn write_u16(&mut self, address: u16, value: u16) -> Result<(), EmulatorError> {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = (value >> 8) as u8;

        self.write(address, low_byte)?;
        self.write(address + 1, high_byte)?;
        Ok(())
    }
}
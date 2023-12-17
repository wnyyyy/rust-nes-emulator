use crate::common::constants::{RAM_SIZE, MEMORY_SIZE};
use crate::common::errors::EmulatorError;

pub struct CpuMemoryMap {
    memory: [u8; MEMORY_SIZE as usize],
}

impl CpuMemoryMap {
    pub fn new() -> CpuMemoryMap {
        CpuMemoryMap {
            memory: [0; MEMORY_SIZE as usize],
        }
    }

    pub fn read(&self, address: u16) -> Result<u8, EmulatorError> {
        match address {
            // CPU Ram and mirrors
            0..=0x1FFF => {
                let mirror_address = (address % RAM_SIZE) as usize;
                Ok(self.memory[mirror_address])
            }
            0x2000.. => {
                Ok(self.memory[address as usize])
            }
        }
    }

    pub fn read_little_endian(&self, address: u16) -> Result<u16, EmulatorError> {
        let low_byte = self.read(address)?;
        let high_byte = self.read(address + 1)?;
        Ok(u16::from_le_bytes([low_byte, high_byte]))
    }

    pub fn write_little_endian(&mut self, address: u16, value: u16) -> Result<(), EmulatorError> {
        let low_byte = (value & 0xFF) as u8;
        let high_byte = (value >> 8) as u8;

        self.write(address, low_byte)?;
        self.write(address + 1, high_byte)?;
        Ok(())
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), EmulatorError>{
        match address {
            // CPU Ram and mirrors
            0..=0x1FFF => {
                let mirror_address = (address % RAM_SIZE) as usize;
                self.memory[mirror_address] = value;
            }

            // IO Registers
            0x2000..=0x401F => {}

            // Expansion ROM
            0x4020..=0x5FFF => {}

            // Save RAM
            0x6000..=0x7FFF => {}

            // PRG ROM
            0x8000.. => {
                self.memory[address as usize] = value;
            }
        }
        Ok(())
    }

    pub fn write_program(&mut self, program: Vec<u8>) {
        let offset = 0x8000;
        for (i, byte) in program.iter().enumerate() {
            self.write((i + offset) as u16, *byte).unwrap();
        }
    }
}
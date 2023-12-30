use std::fs::File;
use std::io::Write;
use crate::memory::memory::Memory;
use crate::cartridge::rom::Rom;
use crate::common::errors::EmulatorError;
use crate::common::constants::{PPU_END, PPU_START, RAM_END, RAM_SIZE, RAM_START, PRG_ROM_START, PRG_ROM_END, PRG_ROM_PAGE_SIZE, DEBUG};

pub struct Bus {
   cpu_ram: [u8; RAM_SIZE],
   rom: Option<Rom>
}

impl Bus {
   pub fn new() -> Self{
       Bus {
           cpu_ram: [0; RAM_SIZE],
           rom: None
       }
   }

    pub fn load_rom(&mut self, rom: Rom) {
         self.rom = Some(rom);
    }

    pub fn dump_memory(&self) {
        let mut dump = String::new();
        let prg_rom= &self.rom.as_ref().unwrap().prg_rom;
        for i in 0..RAM_SIZE {
            dump.push_str(&format!("\n{:0>4x}: {:0>2X} ", i, self.cpu_ram[i]));
        }
        for i in 0..prg_rom.len() {
            dump.push_str(&format!("\n{:0>4x}: {:0>2X} ", i + PRG_ROM_START as usize, prg_rom[i]));
        }
        let mut file = File::create("../dump.txt").expect("TODO: panic message");
        let _ = file.write_all(dump.as_bytes());
    }
}

impl Memory for Bus {
    fn read(&self, address: u16) -> Result<u8, EmulatorError> {
        match address {
            RAM_START ..= RAM_END => {
                let mirror_address = (address % RAM_SIZE as u16) as usize;
                Ok(self.cpu_ram[mirror_address])
            }
            PPU_START ..= PPU_END => {
                Err(EmulatorError::AccessViolation(address))
            }
            PRG_ROM_START ..= PRG_ROM_END => {
                let v_address = address - PRG_ROM_START;
                match &self.rom {
                    Some(rom) => Ok({
                        if rom.prg_rom.len() == PRG_ROM_PAGE_SIZE && v_address >= PRG_ROM_PAGE_SIZE as u16 {
                            let v_address = v_address % PRG_ROM_PAGE_SIZE as u16;
                            rom.prg_rom[v_address as usize]
                        }
                        else {
                            rom.prg_rom[v_address as usize]
                        }
                    }),
                    None => Err(EmulatorError::RomNotLoaded)
                }
            }
            _ => {
                Err(EmulatorError::AccessViolation(address))
            }
        }
    }

    fn read_u16(&self, address: u16) -> Result<u16, EmulatorError> {
        let low_byte = self.read(address)?;
        let high_byte = self.read(address.wrapping_add(1))?;
        Ok(u16::from_le_bytes([low_byte, high_byte]))
    }

    fn read_u16_zero_page(&self, address: u8) -> Result<u16, EmulatorError> {
        let low_byte = self.read(address as u16)?;
        let high_byte = self.read(address.wrapping_add(1) as u16)?;
        Ok(u16::from_le_bytes([low_byte, high_byte]))
    }

    fn write(&mut self, address: u16, data: u8) -> Result<(), EmulatorError> {
        match address {
            RAM_START ..= RAM_END => {
                let mirror_address = (address % RAM_SIZE as u16) as usize;
                self.cpu_ram[mirror_address] = data;
                Ok(())
            }
            PPU_START ..= PPU_END => {
                Err(EmulatorError::AccessViolation(address))
            }
            PRG_ROM_START ..= PRG_ROM_END => {
                let v_address = address - PRG_ROM_START;
                match &mut self.rom {
                    Some(rom) => Ok({
                        if rom.prg_rom.len() == PRG_ROM_PAGE_SIZE && v_address >= PRG_ROM_PAGE_SIZE as u16 {
                            rom.prg_rom[(v_address % PRG_ROM_PAGE_SIZE as u16) as usize] = data;
                        }
                        else {
                            rom.prg_rom[v_address as usize] = data;
                        }
                    }),
                    None => Err(EmulatorError::RomNotLoaded)
                }
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
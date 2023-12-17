use crate::common::errors::EmulatorError;
use crate::cpu::opcode::{get_opcode};
use crate::cpu::types::{AddressingMode, ProcessorStatus};
use crate::memory::cpu_memory_map::CpuMemoryMap;

mod types;
mod instructions;
mod tests;
pub mod opcode;

pub struct CPU {
    program_counter: u16,
    stack_pointer: u8,
    register_a: u8,
    register_x: u8,
    register_y: u8,
    status: ProcessorStatus,
    memory: CpuMemoryMap,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            program_counter: 0,
            stack_pointer: 0,
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: ProcessorStatus::new(),
            memory: CpuMemoryMap::new(),
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) -> Result<(), EmulatorError> {
        self.memory.write_program(program);
        self.run()
    }

    pub fn run(&mut self) -> Result<(), EmulatorError>{
        self.program_counter = 0;

        loop {
            let opcode_u8 = self.memory.read(self.program_counter)?;
            let opcode= get_opcode(opcode_u8).ok_or(EmulatorError::InvalidOpcode(opcode_u8))?;
            self.program_counter += 1;

            match opcode.name {
                // Load and Store
                "LDA" =>  {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    instructions::lda(self, param);
                }
                // Register Transfer
                "TAX"  => {
                    instructions::tax(self);
                }
                // Increment and Decrement
                "INX" => {
                    instructions::inx(self);
                }
                // Subroutine and Interrupt
                "BRK" => {
                    instructions::brk(self);
                    break;
                }
                _ => return Err(EmulatorError::UnimplementedOpcode(opcode_u8)),
            }
        }
        Ok(())
    }

    fn get_param_address(&self, mode: AddressingMode) -> Result<(u16), EmulatorError> {
        match mode {
            AddressingMode::Immediate => Ok(self.program_counter),
            AddressingMode::ZeroPage => {
                let address = self.memory.read(self.program_counter)?;
                Ok(address as u16)
            }
            AddressingMode::ZeroPageX => {
                let address = self.memory.read(self.program_counter)?;
                Ok((address.wrapping_add(self.register_x)) as u16)
            }
            AddressingMode::ZeroPageY => {
                let address = self.memory.read(self.program_counter)?;
                Ok((address.wrapping_add(self.register_y)) as u16)
            }
            AddressingMode::Absolute => {
                let address = self.memory.read_little_endian(self.program_counter)?;
                Ok(address)
            }
            AddressingMode::AbsoluteX => {
                let address = self.memory.read_little_endian(self.program_counter)?;
                Ok(address.wrapping_add(self.register_x as u16))
            }
            AddressingMode::AbsoluteY => {
                let address = self.memory.read_little_endian(self.program_counter)?;
                Ok(address.wrapping_add(self.register_y as u16))
            }
            AddressingMode::Indirect => {
                let address = self.memory.read_little_endian(self.program_counter)?;
                Ok(self.memory.read_little_endian(address)?)
            }
            AddressingMode::IndexedIndirect => {
                let address = self.memory.read(self.program_counter)?.wrapping_add(self.register_x);
                Ok(self.memory.read_little_endian(address as u16)?)
            }
            AddressingMode::IndirectIndexed => {
                let address = self.memory.read(self.program_counter)?;
                Ok(self.memory.read_little_endian(address as u16)?.wrapping_add(self.register_y as u16))
            }
            _ => Err(EmulatorError::UnimplementedAddressingMode(format!("{:?}", mode))),
        }
    }
}

use crate::common::errors::EmulatorError;
use crate::cpu::opcode::{get_opcode};
use crate::cpu::types::ProcessorStatus;

mod types;
mod instructions;
mod tests;
pub mod opcode;

pub struct CPU {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: ProcessorStatus,
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
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) -> Result<(), EmulatorError>{
        self.program_counter = 0;

        loop {
            let opcode_u8 = program[self.program_counter as usize];
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
}

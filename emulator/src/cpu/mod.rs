use crate::common::errors::EmulatorError;
use crate::common::opcode::Opcode;
use crate::cpu::types::ProcessorStatus;

mod types;
mod instructions;
mod tests;

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
            let opcode= Opcode::try_from(opcode_u8).map_err(|_| EmulatorError::InvalidOpcode(opcode_u8))?;
            self.program_counter += 1;

            match opcode {
                Opcode::Brk => {
                    instructions::brk(self);
                    break;
                }
                Opcode::LdaImmediate =>  {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    instructions::lda(self, param);
                }
                Opcode::Tax  => {
                    instructions::tax(self);
                }
                Opcode::Inx => {
                    instructions::inx(self);
                }
            }
        }
        Ok(())
    }
}
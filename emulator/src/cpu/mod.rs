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

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = param;

                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    if self.register_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }
                }
                0xAA =>  {
                    self.register_x = self.register_a;

                    if self.register_x == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    if self.register_x & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }

                }
                0x00 => {
                    return;
                }
                _ => todo!()
            }
        }
    }
}
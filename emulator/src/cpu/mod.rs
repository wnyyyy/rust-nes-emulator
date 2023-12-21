use crate::common::constants::STACK_POINTER_INIT;
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
            stack_pointer: STACK_POINTER_INIT,
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
        loop {
            let opcode_u8 = self.memory.read(self.program_counter)?;
            let opcode = get_opcode(opcode_u8).ok_or(EmulatorError::InvalidOpcode(opcode_u8))?;
            self.program_counter += 1;
            let mut increase_pc = true;

            match opcode.name {
                // Load and Store
                "LDA" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::lda(self, param);
                }
                "LDX" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::ldx(self, param);
                }
                "LDY" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::ldy(self, param);
                }
                "STA" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    instructions::sta(self, param_address);
                }
                "STX" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    instructions::stx(self, param_address);
                }
                "STY" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    instructions::sty(self, param_address);
                }
                // Arithmetic
                "ADC" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::adc(self, param);
                }
                "SBC" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::sbc(self, param);
                }
                // Increment and Decrement
                "INC" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    instructions::inc(self, param_address)?;
                }
                "INX" => {
                    instructions::inx(self);
                }
                "INY" => {
                    instructions::iny(self);
                }
                "DEC" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    instructions::dec(self, param_address)?;
                }
                "DEX" => {
                    instructions::dex(self);
                }
                "DEY" => {
                    instructions::dey(self);
                }
                // Register Transfer
                "TAX" => {
                    instructions::tax(self);
                }
                "TAY" => {
                    instructions::tay(self);
                }
                "TXA" => {
                    instructions::txa(self);
                }
                "TYA" => {
                    instructions::tya(self);
                }
                // Logical
                "AND" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::and(self, param);
                }
                "EOR" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::eor(self, param);
                }
                "ORA" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::ora(self, param);
                }
                // Compare and Bit Test
                "CMP" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::cmp(self, param);
                }
                "CPX" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::cpx(self, param);
                }
                "CPY" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::cpy(self, param);
                }
                "BIT" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    let param = self.memory.read(param_address)?;
                    instructions::bit(self, param);
                }
                // Shift and Rotate
                "ASL" => {
                    if opcode.address_mode == AddressingMode::Accumulator {
                        instructions::asl_accumulator(self);
                    } else {
                        let param_address = self.get_param_address(&opcode.address_mode)?;
                        instructions::asl(self, param_address)?;
                    }
                }
                "LSR" => {
                    if opcode.address_mode == AddressingMode::Accumulator {
                        instructions::lsr_accumulator(self);
                    } else {
                        let param_address = self.get_param_address(&opcode.address_mode)?;
                        instructions::lsr(self, param_address)?;
                    }
                }
                "ROL" => {
                    if opcode.address_mode == AddressingMode::Accumulator {
                        instructions::rol_accumulator(self);
                    } else {
                        let param_address = self.get_param_address(&opcode.address_mode)?;
                        instructions::rol(self, param_address)?;
                    }
                }
                "ROR" => {
                    if opcode.address_mode == AddressingMode::Accumulator {
                        instructions::ror_accumulator(self);
                    } else {
                        let param_address = self.get_param_address(&opcode.address_mode)?;
                        instructions::ror(self, param_address)?;
                    }
                }
                // Jump and Branch
                "JMP" => {
                    let param_address = self.get_param_address(&opcode.address_mode)?;
                    instructions::jmp(self, param_address);
                    increase_pc = false;
                }
                "BCC" => {
                    let offset = self.get_param_address(&opcode.address_mode)?;
                    instructions::bcc(self, offset)?;
                }
                "BCS" => {
                    let offset = self.get_param_address(&opcode.address_mode)?;
                    instructions::bcs(self, offset)?;
                }
                "BEQ" => {
                    let offset = self.get_param_address(&opcode.address_mode)?;
                    instructions::beq(self, offset)?;
                }
                "BMI" => {
                    let offset = self.get_param_address(&opcode.address_mode)?;
                    instructions::bmi(self, offset)?;
                }
                "BNE" => {
                    let offset = self.get_param_address(&opcode.address_mode)?;
                    instructions::bne(self, offset)?;
                }
                "BPL" => {
                    let offset = self.get_param_address(&opcode.address_mode)?;
                    instructions::bpl(self, offset)?;
                }
                "BVC" => {
                    let offset = self.get_param_address(&opcode.address_mode)?;
                    instructions::bvc(self, offset)?;
                }
                "BVS" => {
                    let offset = self.get_param_address(&opcode.address_mode)?;
                    instructions::bvs(self, offset)?;
                }
                // Stack
                "TSX" => {
                    instructions::tsx(self);
                }
                "TXS" => {
                    instructions::txs(self);
                }
                "PHA" => {
                    instructions::pha(self)?;
                }
                "PHP" => {
                    instructions::php(self)?;
                }
                "PLA" => {
                    instructions::pla(self)?;
                }
                "PLP" => {
                    instructions::plp(self)?;
                }
                // Subroutine and Interrupt
                "BRK" => {
                    instructions::brk(self);
                    break;
                }
                _ => return Err(EmulatorError::UnimplementedOpcode(opcode_u8)),
            }
            if increase_pc {
                self.program_counter += opcode.bytes as u16 - 1;
            }
        }
        Ok(())
    }

    fn get_param_address(&self, mode: &AddressingMode) -> Result<u16, EmulatorError> {
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
            AddressingMode::Relative => {
                let address = self.memory.read(self.program_counter)?;
                Ok(address as u16)
            }
            _ => Err(EmulatorError::UnimplementedAddressingMode(format!("{:?}", mode))),
        }
    }
}

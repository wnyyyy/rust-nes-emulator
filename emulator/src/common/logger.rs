use crate::common::errors::EmulatorError;
use crate::cpu::CPU;
use crate::cpu::types::AddressingMode;
use crate::cpu::opcode::get_opcode;
use crate::memory::memory::Memory;

const PC_WIDTH: usize = 6;
const CODE_WIDTH: usize = 10;
const INSTRUCTION_WIDTH: usize = 32;
const AXY_WIDTH: usize = 5;
const STATUS_WIDTH: usize = 5;
const STACK_POINTER_WIDTH: usize = 6;
const PPU_HALF_WITDH: usize = 4;

pub fn trace(cpu: &CPU) -> Result<String, EmulatorError> {
    let mut line = String::new();
    line.push_str(&get_pc_str(cpu));
    line.push_str(&get_code_str(cpu)?);
    Ok(line)
}

fn get_pc_str(cpu: &CPU) -> String {
    format!("{:0>width$X}", cpu.program_counter, width = PC_WIDTH)
}

fn get_code_str(cpu: &CPU) -> Result<String, EmulatorError> {
    let mut code = String::new();
    let opcode_code = cpu.read(cpu.program_counter)?;
    if let Some(opcode) = get_opcode(opcode_code) {
        for i in 0..opcode.bytes {
            let byte = cpu.read(cpu.program_counter + i as u16)?;
            code.push_str(&format!("{:0X} ", byte));
        }
    }
    code = format!("{:>width$}", code, width = CODE_WIDTH);
    Ok(code)
}

fn get_instruction_str(cpu: &CPU) -> Result<String, EmulatorError> {
    let mut instruction = String::new();

    let opcode_code = cpu.read(cpu.program_counter)?;
    if let Some(opcode) = get_opcode(opcode_code) {
        instruction.push_str(&format!("{} ", opcode.name));

    }

    instruction = format!("{:>width$}", instruction, width = INSTRUCTION_WIDTH);
    Ok(instruction)
}

fn get_address_string(mode: AddressingMode, high_byte: Option<u8>, low_byte: Option<u8>) -> String {
    let mut address = String::new();
    match mode {
        AddressingMode::Accumulator => {
            address.push('A');
        }
        AddressingMode::Immediate => {
            address.push_str(&format!("#{:0>2X}", high_byte.unwrap()));
        }
        AddressingMode::ZeroPage => {
            address.push_str(&format!("${:0>2X}", high_byte.unwrap()));
        }
        AddressingMode::ZeroPageX => {
            address.push_str(&format!("${:0>2X},X", high_byte.unwrap()));
        }
        AddressingMode::ZeroPageY => {
            address.push_str(&format!("${:0>2X},Y", high_byte.unwrap()));
        }
        AddressingMode::Relative | AddressingMode::Absolute => {
            address.push_str(&format!("${:0>2X}{:0>2X}", high_byte.unwrap(), low_byte.unwrap()));
        }
        AddressingMode::AbsoluteX => {
            address.push_str(&format!("${:0>2X}{:0>2X},X", high_byte.unwrap(), low_byte.unwrap()));
        }
        AddressingMode::AbsoluteY => {
            address.push_str(&format!("${:0>2X}{:0>2X},Y", high_byte.unwrap(), low_byte.unwrap()));
        }
        AddressingMode::Indirect => {
            address.push_str(&format!("(${:0>2X}{:0>2X})", high_byte.unwrap(), low_byte.unwrap()));
        }
        AddressingMode::IndexedIndirect => {
            address.push_str(&format!("(${:0>2X},X)", high_byte.unwrap()));
        }
        AddressingMode::IndirectIndexed => {
            address.push_str(&format!("(${:0>2X}),Y", high_byte.unwrap()));
        }
        _ => {}
    }
    address = format!("{:>width$}", address, width = INSTRUCTION_WIDTH);
    address
}
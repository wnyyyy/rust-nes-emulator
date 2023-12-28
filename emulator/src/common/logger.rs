use crate::common::errors::EmulatorError;
use crate::cpu::CPU;
use crate::cpu::types::AddressingMode;
use crate::cpu::opcode::get_opcode;
use crate::memory::memory::Memory;

const PC_WIDTH: usize = 6;
const CODE_WIDTH: usize = 10;
const INSTRUCTION_WIDTH: usize = 32;
const PPU_HALF_WITDH: usize = 4;

pub fn trace(cpu: &CPU) -> Result<String, EmulatorError> {
    let mut line = String::new();
    line.push_str(&get_pc_str(cpu));
    line.push_str(&get_code_str(cpu)?);
    line.push_str(&get_instruction_str(cpu)?);
    line.push_str(&get_register_string(cpu));
    Ok(line)
}

fn get_pc_str(cpu: &CPU) -> String {
    let mut pc = String::new();
    pc.push_str(&format!("{:0>4X}", cpu.program_counter));
    pc = format!("{:<width$}", pc, width = PC_WIDTH);
    pc
}

fn get_code_str(cpu: &CPU) -> Result<String, EmulatorError> {
    let mut code = String::new();
    let opcode_code = cpu.read(cpu.program_counter)?;
    if let Some(opcode) = get_opcode(opcode_code) {
        for i in 0..opcode.bytes {
            let byte = cpu.read(cpu.program_counter + i as u16)?;
            code.push_str(&format!("{:0>2X} ", byte));
        }
    }
    code = format!("{:<width$}", code, width = CODE_WIDTH);
    Ok(code)
}

fn get_instruction_str(cpu: &CPU) -> Result<String, EmulatorError> {
    let mut instruction = String::new();

    let opcode_code = cpu.read(cpu.program_counter)?;
    if let Some(opcode) = get_opcode(opcode_code) {
        instruction.push_str(&format!("{} ", opcode.name));
        let high_byte = if opcode.bytes > 1 { Some(cpu.read(cpu.program_counter + 1)?) } else { None };
        let low_byte = if opcode.bytes == 3 { Some(cpu.read(cpu.program_counter + 2)?) } else { None };
        instruction.push_str(&get_address_string(opcode.address_mode, cpu, high_byte, low_byte)?);
    }

    instruction = format!("{:<width$}", instruction, width = INSTRUCTION_WIDTH);
    Ok(instruction)
}

fn get_address_string(mode: AddressingMode, cpu: &CPU, high_byte: Option<u8>, low_byte: Option<u8>) -> Result<String, EmulatorError> {
    let mut address = String::new();
    match mode {
        AddressingMode::Accumulator => {
            address.push('A');
        }
        AddressingMode::Immediate => {
            address.push_str(&format!("#${:0>2X}", high_byte.unwrap()));
        }
        AddressingMode::ZeroPage => {
            address.push_str(&format!("${:0>2X}", high_byte.unwrap()));
            let value = cpu.read(high_byte.unwrap() as u16)?;
            address.push_str(&format!(" = {:0>2X}", value));
        }
        AddressingMode::ZeroPageX => {
            address.push_str(&format!("${:0>2X},X", high_byte.unwrap()));
            let real_address = high_byte.unwrap().wrapping_add(cpu.register_x);
            let value = cpu.read(real_address as u16)?;
            address.push_str(&format!(" @ {:0>2X} = {:0>2X}", real_address, value));
        }
        AddressingMode::ZeroPageY => {
            address.push_str(&format!("${:0>2X},Y", high_byte.unwrap()));
            let real_address = high_byte.unwrap().wrapping_add(cpu.register_y);
            let value = cpu.read(real_address as u16)?;
            address.push_str(&format!(" @ {:0>2X} = {:0>2X}", real_address, value));
        }
        AddressingMode::Relative | AddressingMode::Absolute => {
            address.push_str(&format!("${:0>2X}", high_byte.unwrap()));
        }
        AddressingMode::AbsoluteX => {
            address.push_str(&format!("${:0>2X}{:0>2X},X", high_byte.unwrap(), low_byte.unwrap()));
            let addr = u16::from_le_bytes([low_byte.unwrap(), high_byte.unwrap()]);
            let real_address = addr.wrapping_add(cpu.register_x as u16);
            let value = cpu.read(real_address)?;
            address.push_str(&format!(" @ {:0>4X} = {:0>2X}", real_address, value));
        }
        AddressingMode::AbsoluteY => {
            address.push_str(&format!("${:0>2X}{:0>2X},Y", high_byte.unwrap(), low_byte.unwrap()));
            let addr = u16::from_le_bytes([low_byte.unwrap(), high_byte.unwrap()]);
            let real_address = addr.wrapping_add(cpu.register_y as u16);
            let value = cpu.read(real_address)?;
            address.push_str(&format!(" @ {:0>4X} = {:0>2X}", real_address, value));
        }
        AddressingMode::Indirect => {
            address.push_str(&format!("(${:0>2X}{:0>2X})", high_byte.unwrap(), low_byte.unwrap()));
        }
        AddressingMode::IndexedIndirect => {
            address.push_str(&format!("(${:0>2X},X)", high_byte.unwrap()));
            let reference = high_byte.unwrap().wrapping_add(cpu.register_x);
            let real_address = cpu.read_u16(reference as u16)?;
            let value = cpu.read(real_address)?;
            address.push_str(&format!(" @ {:0>2X} = {:0>4X} = {:0>2X}", reference, real_address, value));
        }
        AddressingMode::IndirectIndexed => {
            address.push_str(&format!("(${:0>2X}),Y", high_byte.unwrap()));
            let reference = cpu.read_u16(high_byte.unwrap() as u16)?;
            let real_address = reference.wrapping_add(cpu.register_y as u16);
            let value = cpu.read(real_address)?;
            address.push_str(&format!(" = {:0>4X} @ {:0>4X} = {:0>2X}", reference, real_address, value));
        }
        _ => {}
    }
    Ok(address)
}

fn get_register_string(cpu: &CPU) -> String {
    let mut registers = String::new();
    registers.push_str(&format!("A:{:0>2X} ", cpu.register_a));
    registers.push_str(&format!("X:{:0>2X} ", cpu.register_x));
    registers.push_str(&format!("Y:{:0>2X} ", cpu.register_y));
    registers.push_str(&format!("P:{:0>2X} ", cpu.status.to_u8()));
    registers.push_str(&format!("SP:{:0>2X}", cpu.stack_pointer));
    registers
}
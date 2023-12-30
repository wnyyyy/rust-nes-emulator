use crate::cpu::types::AddressingMode;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Opcode {
    pub name: &'static str,
    pub code: u8,
    pub bytes: usize,
    pub cycles: usize,
    pub address_mode: AddressingMode,
}

impl Opcode {
    fn new(name: &'static str, code: u8, bytes: usize, cycles: usize, address_mode: AddressingMode) -> Opcode {
        Opcode {
            name,
            code,
            bytes,
            cycles,
            address_mode,
        }
    }
}

pub fn get_opcode(code: u8) -> Option<&'static Opcode> {
    OPCODES.get(&code)
}

pub fn get_opcode_by_name_and_address_mode(name: &str, address_mode: AddressingMode) -> Option<&'static Opcode> {
    OPCODES.values().find(|opcode| opcode.name == name && opcode.address_mode == address_mode)
}

lazy_static! {
    static ref OPCODES: HashMap<u8, Opcode> = {
        let mut map = HashMap::new();
        let m = &mut map;

        // Load and Store
        add_opcode(m, Opcode::new("LDA", 0xA9, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("LDA", 0xA5, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("LDA", 0xB5, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("LDA", 0xAD, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("LDA", 0xBD, 3, 4, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("LDA", 0xB9, 3, 4, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("LDA", 0xA1, 2, 4, AddressingMode::IndexedIndirect));
        add_opcode(m, Opcode::new("LDA", 0xB1, 2, 4, AddressingMode::IndirectIndexed));
        add_opcode(m, Opcode::new("LDX", 0xA2, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("LDX", 0xA6, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("LDX", 0xB6, 2, 4, AddressingMode::ZeroPageY));
        add_opcode(m, Opcode::new("LDX", 0xAE, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("LDX", 0xBE, 3, 4, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("LDY", 0xA0, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("LDY", 0xA4, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("LDY", 0xB4, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("LDY", 0xAC, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("LDY", 0xBC, 3, 4, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("STA", 0x85, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("STA", 0x95, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("STA", 0x8D, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("STA", 0x9D, 3, 5, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("STA", 0x99, 3, 5, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("STA", 0x81, 2, 6, AddressingMode::IndexedIndirect));
        add_opcode(m, Opcode::new("STA", 0x91, 2, 6, AddressingMode::IndirectIndexed));
        add_opcode(m, Opcode::new("STX", 0x86, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("STX", 0x96, 2, 4, AddressingMode::ZeroPageY));
        add_opcode(m, Opcode::new("STX", 0x8E, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("STY", 0x84, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("STY", 0x94, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("STY", 0x8C, 3, 4, AddressingMode::Absolute));

        // Arithmetic
        add_opcode(m, Opcode::new("ADC", 0x69, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("ADC", 0x65, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("ADC", 0x75, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("ADC", 0x6D, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("ADC", 0x7D, 3, 4, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("ADC", 0x79, 3, 4, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("ADC", 0x61, 2, 6, AddressingMode::IndexedIndirect));
        add_opcode(m, Opcode::new("ADC", 0x71, 2, 5, AddressingMode::IndirectIndexed));
        add_opcode(m, Opcode::new("SBC", 0xE9, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("SBC", 0xE5, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("SBC", 0xF5, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("SBC", 0xED, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("SBC", 0xFD, 3, 4, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("SBC", 0xF9, 3, 4, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("SBC", 0xE1, 2, 6, AddressingMode::IndexedIndirect));
        add_opcode(m, Opcode::new("SBC", 0xF1, 2, 5, AddressingMode::IndirectIndexed));

        // Increment and Decrement
        add_opcode(m, Opcode::new("INC", 0xE6, 2, 5, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("INC", 0xF6, 2, 6, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("INC", 0xEE, 3, 6, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("INC", 0xFE, 3, 7, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("INX", 0xE8, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("INY", 0xC8, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("DEC", 0xC6, 2, 5, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("DEC", 0xD6, 2, 6, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("DEC", 0xCE, 3, 6, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("DEC", 0xDE, 3, 7, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("DEX", 0xCA, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("DEY", 0x88, 1, 2, AddressingMode::Implied));

        // Register Transfer
        add_opcode(m, Opcode::new("TAX", 0xAA, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("TAY", 0xA8, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("TXA", 0x8A, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("TYA", 0x98, 1, 2, AddressingMode::Implied));

        // Logical
        add_opcode(m, Opcode::new("AND", 0x29, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("AND", 0x25, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("AND", 0x35, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("AND", 0x2D, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("AND", 0x3D, 3, 4, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("AND", 0x39, 3, 4, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("AND", 0x21, 2, 6, AddressingMode::IndexedIndirect));
        add_opcode(m, Opcode::new("AND", 0x31, 2, 5, AddressingMode::IndirectIndexed));
        add_opcode(m, Opcode::new("EOR", 0x49, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("EOR", 0x45, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("EOR", 0x55, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("EOR", 0x4D, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("EOR", 0x5D, 3, 4, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("EOR", 0x59, 3, 4, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("EOR", 0x41, 2, 6, AddressingMode::IndexedIndirect));
        add_opcode(m, Opcode::new("EOR", 0x51, 2, 5, AddressingMode::IndirectIndexed));
        add_opcode(m, Opcode::new("ORA", 0x09, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("ORA", 0x05, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("ORA", 0x15, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("ORA", 0x0D, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("ORA", 0x1D, 3, 4, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("ORA", 0x19, 3, 4, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("ORA", 0x01, 2, 6, AddressingMode::IndexedIndirect));
        add_opcode(m, Opcode::new("ORA", 0x11, 2, 5, AddressingMode::IndirectIndexed));

        // Compare and Bit Test
        add_opcode(m, Opcode::new("CMP", 0xC9, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("CMP", 0xC5, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("CMP", 0xD5, 2, 4, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("CMP", 0xCD, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("CMP", 0xDD, 3, 4, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("CMP", 0xD9, 3, 4, AddressingMode::AbsoluteY));
        add_opcode(m, Opcode::new("CMP", 0xC1, 2, 6, AddressingMode::IndexedIndirect));
        add_opcode(m, Opcode::new("CMP", 0xD1, 2, 5, AddressingMode::IndirectIndexed));
        add_opcode(m, Opcode::new("CPX", 0xE0, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("CPX", 0xE4, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("CPX", 0xEC, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("CPY", 0xC0, 2, 2, AddressingMode::Immediate));
        add_opcode(m, Opcode::new("CPY", 0xC4, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("CPY", 0xCC, 3, 4, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("BIT", 0x24, 2, 3, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("BIT", 0x2C, 3, 4, AddressingMode::Absolute));

        // Shift and Rotate
        add_opcode(m, Opcode::new("ASL", 0x0A, 1, 2, AddressingMode::Accumulator));
        add_opcode(m, Opcode::new("ASL", 0x06, 2, 5, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("ASL", 0x16, 2, 6, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("ASL", 0x0E, 3, 6, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("ASL", 0x1E, 3, 7, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("LSR", 0x4A, 1, 2, AddressingMode::Accumulator));
        add_opcode(m, Opcode::new("LSR", 0x46, 2, 5, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("LSR", 0x56, 2, 6, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("LSR", 0x4E, 3, 6, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("LSR", 0x5E, 3, 7, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("ROL", 0x2A, 1, 2, AddressingMode::Accumulator));
        add_opcode(m, Opcode::new("ROL", 0x26, 2, 5, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("ROL", 0x36, 2, 6, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("ROL", 0x2E, 3, 6, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("ROL", 0x3E, 3, 7, AddressingMode::AbsoluteX));
        add_opcode(m, Opcode::new("ROR", 0x6A, 1, 2, AddressingMode::Accumulator));
        add_opcode(m, Opcode::new("ROR", 0x66, 2, 5, AddressingMode::ZeroPage));
        add_opcode(m, Opcode::new("ROR", 0x76, 2, 6, AddressingMode::ZeroPageX));
        add_opcode(m, Opcode::new("ROR", 0x6E, 3, 6, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("ROR", 0x7E, 3, 7, AddressingMode::AbsoluteX));

        // Jump and Branch
        add_opcode(m, Opcode::new("JMP", 0x4C, 3, 3, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("JMP", 0x6C, 3, 5, AddressingMode::Indirect));
        add_opcode(m, Opcode::new("BCC", 0x90, 2, 2, AddressingMode::Relative));
        add_opcode(m, Opcode::new("BCS", 0xB0, 2, 2, AddressingMode::Relative));
        add_opcode(m, Opcode::new("BEQ", 0xF0, 2, 2, AddressingMode::Relative));
        add_opcode(m, Opcode::new("BMI", 0x30, 2, 2, AddressingMode::Relative));
        add_opcode(m, Opcode::new("BNE", 0xD0, 2, 2, AddressingMode::Relative));
        add_opcode(m, Opcode::new("BPL", 0x10, 2, 2, AddressingMode::Relative));
        add_opcode(m, Opcode::new("BVC", 0x50, 2, 2, AddressingMode::Relative));
        add_opcode(m, Opcode::new("BVS", 0x70, 2, 2, AddressingMode::Relative));

        // Stack
        add_opcode(m, Opcode::new("TSX", 0xBA, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("TXS", 0x9A, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("PHA", 0x48, 1, 3, AddressingMode::Implied));
        add_opcode(m, Opcode::new("PHP", 0x08, 1, 3, AddressingMode::Implied));
        add_opcode(m, Opcode::new("PLA", 0x68, 1, 4, AddressingMode::Implied));
        add_opcode(m, Opcode::new("PLP", 0x28, 1, 4, AddressingMode::Implied));

        // Status Flag Change
        add_opcode(m, Opcode::new("CLC", 0x18, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("CLD", 0xD8, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("CLI", 0x58, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("CLV", 0xB8, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("SEC", 0x38, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("SED", 0xF8, 1, 2, AddressingMode::Implied));
        add_opcode(m, Opcode::new("SEI", 0x78, 1, 2, AddressingMode::Implied));

        // Subroutine and Interrupt
        add_opcode(m, Opcode::new("JSR", 0x20, 3, 6, AddressingMode::Absolute));
        add_opcode(m, Opcode::new("RTS", 0x60, 1, 6, AddressingMode::Implied));
        add_opcode(m, Opcode::new("BRK", 0x00, 1, 7, AddressingMode::Implied));
        add_opcode(m, Opcode::new("RTI", 0x40, 1, 6, AddressingMode::Implied));
        add_opcode(m, Opcode::new("NOP", 0xEA, 1, 2, AddressingMode::Implied));

        map
    };
}

fn add_opcode(m: &mut HashMap<u8, Opcode>, opcode: Opcode) {
    m.insert(opcode.code, opcode);
}
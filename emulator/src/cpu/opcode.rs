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
    pub unofficial: bool,
}

impl Opcode {
    fn new(name: &'static str, code: u8, bytes: usize, cycles: usize, address_mode: AddressingMode, unofficial: bool) -> Opcode {
        Opcode {
            name,
            code,
            bytes,
            cycles,
            address_mode,
            unofficial,
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
        add_opcode(m, Opcode::new("LDA", 0xA9, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("LDA", 0xA5, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("LDA", 0xB5, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("LDA", 0xAD, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("LDA", 0xBD, 3, 4, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("LDA", 0xB9, 3, 4, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("LDA", 0xA1, 2, 4, AddressingMode::IndexedIndirect, false));
        add_opcode(m, Opcode::new("LDA", 0xB1, 2, 4, AddressingMode::IndirectIndexed, false));
        add_opcode(m, Opcode::new("LDX", 0xA2, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("LDX", 0xA6, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("LDX", 0xB6, 2, 4, AddressingMode::ZeroPageY, false));
        add_opcode(m, Opcode::new("LDX", 0xAE, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("LDX", 0xBE, 3, 4, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("LDY", 0xA0, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("LDY", 0xA4, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("LDY", 0xB4, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("LDY", 0xAC, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("LDY", 0xBC, 3, 4, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("STA", 0x85, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("STA", 0x95, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("STA", 0x8D, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("STA", 0x9D, 3, 5, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("STA", 0x99, 3, 5, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("STA", 0x81, 2, 6, AddressingMode::IndexedIndirect, false));
        add_opcode(m, Opcode::new("STA", 0x91, 2, 6, AddressingMode::IndirectIndexed, false));
        add_opcode(m, Opcode::new("STX", 0x86, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("STX", 0x96, 2, 4, AddressingMode::ZeroPageY, false));
        add_opcode(m, Opcode::new("STX", 0x8E, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("STY", 0x84, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("STY", 0x94, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("STY", 0x8C, 3, 4, AddressingMode::Absolute, false));

        // Arithmetic
        add_opcode(m, Opcode::new("ADC", 0x69, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("ADC", 0x65, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("ADC", 0x75, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("ADC", 0x6D, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("ADC", 0x7D, 3, 4, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("ADC", 0x79, 3, 4, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("ADC", 0x61, 2, 6, AddressingMode::IndexedIndirect, false));
        add_opcode(m, Opcode::new("ADC", 0x71, 2, 5, AddressingMode::IndirectIndexed, false));
        add_opcode(m, Opcode::new("SBC", 0xE9, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("SBC", 0xE5, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("SBC", 0xF5, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("SBC", 0xED, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("SBC", 0xFD, 3, 4, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("SBC", 0xF9, 3, 4, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("SBC", 0xE1, 2, 6, AddressingMode::IndexedIndirect, false));
        add_opcode(m, Opcode::new("SBC", 0xF1, 2, 5, AddressingMode::IndirectIndexed, false));

        // Increment and Decrement
        add_opcode(m, Opcode::new("INC", 0xE6, 2, 5, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("INC", 0xF6, 2, 6, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("INC", 0xEE, 3, 6, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("INC", 0xFE, 3, 7, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("INX", 0xE8, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("INY", 0xC8, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("DEC", 0xC6, 2, 5, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("DEC", 0xD6, 2, 6, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("DEC", 0xCE, 3, 6, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("DEC", 0xDE, 3, 7, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("DEX", 0xCA, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("DEY", 0x88, 1, 2, AddressingMode::Implied, false));

        // Register Transfer
        add_opcode(m, Opcode::new("TAX", 0xAA, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("TAY", 0xA8, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("TXA", 0x8A, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("TYA", 0x98, 1, 2, AddressingMode::Implied, false));

        // Logical
        add_opcode(m, Opcode::new("AND", 0x29, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("AND", 0x25, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("AND", 0x35, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("AND", 0x2D, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("AND", 0x3D, 3, 4, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("AND", 0x39, 3, 4, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("AND", 0x21, 2, 6, AddressingMode::IndexedIndirect, false));
        add_opcode(m, Opcode::new("AND", 0x31, 2, 5, AddressingMode::IndirectIndexed, false));
        add_opcode(m, Opcode::new("EOR", 0x49, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("EOR", 0x45, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("EOR", 0x55, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("EOR", 0x4D, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("EOR", 0x5D, 3, 4, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("EOR", 0x59, 3, 4, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("EOR", 0x41, 2, 6, AddressingMode::IndexedIndirect, false));
        add_opcode(m, Opcode::new("EOR", 0x51, 2, 5, AddressingMode::IndirectIndexed, false));
        add_opcode(m, Opcode::new("ORA", 0x09, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("ORA", 0x05, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("ORA", 0x15, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("ORA", 0x0D, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("ORA", 0x1D, 3, 4, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("ORA", 0x19, 3, 4, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("ORA", 0x01, 2, 6, AddressingMode::IndexedIndirect, false));
        add_opcode(m, Opcode::new("ORA", 0x11, 2, 5, AddressingMode::IndirectIndexed, false));

        // Compare and Bit Test
        add_opcode(m, Opcode::new("CMP", 0xC9, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("CMP", 0xC5, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("CMP", 0xD5, 2, 4, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("CMP", 0xCD, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("CMP", 0xDD, 3, 4, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("CMP", 0xD9, 3, 4, AddressingMode::AbsoluteY, false));
        add_opcode(m, Opcode::new("CMP", 0xC1, 2, 6, AddressingMode::IndexedIndirect, false));
        add_opcode(m, Opcode::new("CMP", 0xD1, 2, 5, AddressingMode::IndirectIndexed, false));
        add_opcode(m, Opcode::new("CPX", 0xE0, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("CPX", 0xE4, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("CPX", 0xEC, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("CPY", 0xC0, 2, 2, AddressingMode::Immediate, false));
        add_opcode(m, Opcode::new("CPY", 0xC4, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("CPY", 0xCC, 3, 4, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("BIT", 0x24, 2, 3, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("BIT", 0x2C, 3, 4, AddressingMode::Absolute, false));

        // Shift and Rotate
        add_opcode(m, Opcode::new("ASL", 0x0A, 1, 2, AddressingMode::Accumulator, false));
        add_opcode(m, Opcode::new("ASL", 0x06, 2, 5, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("ASL", 0x16, 2, 6, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("ASL", 0x0E, 3, 6, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("ASL", 0x1E, 3, 7, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("LSR", 0x4A, 1, 2, AddressingMode::Accumulator, false));
        add_opcode(m, Opcode::new("LSR", 0x46, 2, 5, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("LSR", 0x56, 2, 6, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("LSR", 0x4E, 3, 6, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("LSR", 0x5E, 3, 7, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("ROL", 0x2A, 1, 2, AddressingMode::Accumulator, false));
        add_opcode(m, Opcode::new("ROL", 0x26, 2, 5, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("ROL", 0x36, 2, 6, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("ROL", 0x2E, 3, 6, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("ROL", 0x3E, 3, 7, AddressingMode::AbsoluteX, false));
        add_opcode(m, Opcode::new("ROR", 0x6A, 1, 2, AddressingMode::Accumulator, false));
        add_opcode(m, Opcode::new("ROR", 0x66, 2, 5, AddressingMode::ZeroPage, false));
        add_opcode(m, Opcode::new("ROR", 0x76, 2, 6, AddressingMode::ZeroPageX, false));
        add_opcode(m, Opcode::new("ROR", 0x6E, 3, 6, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("ROR", 0x7E, 3, 7, AddressingMode::AbsoluteX, false));

        // Jump and Branch
        add_opcode(m, Opcode::new("JMP", 0x4C, 3, 3, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("JMP", 0x6C, 3, 5, AddressingMode::Indirect, false));
        add_opcode(m, Opcode::new("BCC", 0x90, 2, 2, AddressingMode::Relative, false));
        add_opcode(m, Opcode::new("BCS", 0xB0, 2, 2, AddressingMode::Relative, false));
        add_opcode(m, Opcode::new("BEQ", 0xF0, 2, 2, AddressingMode::Relative, false));
        add_opcode(m, Opcode::new("BMI", 0x30, 2, 2, AddressingMode::Relative, false));
        add_opcode(m, Opcode::new("BNE", 0xD0, 2, 2, AddressingMode::Relative, false));
        add_opcode(m, Opcode::new("BPL", 0x10, 2, 2, AddressingMode::Relative, false));
        add_opcode(m, Opcode::new("BVC", 0x50, 2, 2, AddressingMode::Relative, false));
        add_opcode(m, Opcode::new("BVS", 0x70, 2, 2, AddressingMode::Relative, false));

        // Stack
        add_opcode(m, Opcode::new("TSX", 0xBA, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("TXS", 0x9A, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("PHA", 0x48, 1, 3, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("PHP", 0x08, 1, 3, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("PLA", 0x68, 1, 4, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("PLP", 0x28, 1, 4, AddressingMode::Implied, false));

        // Status Flag Change
        add_opcode(m, Opcode::new("CLC", 0x18, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("CLD", 0xD8, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("CLI", 0x58, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("CLV", 0xB8, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("SEC", 0x38, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("SED", 0xF8, 1, 2, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("SEI", 0x78, 1, 2, AddressingMode::Implied, false));

        // Subroutine and Interrupt
        add_opcode(m, Opcode::new("JSR", 0x20, 3, 6, AddressingMode::Absolute, false));
        add_opcode(m, Opcode::new("RTS", 0x60, 1, 6, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("BRK", 0x00, 1, 7, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("RTI", 0x40, 1, 6, AddressingMode::Implied, false));
        add_opcode(m, Opcode::new("NOP", 0xEA, 1, 2, AddressingMode::Implied, false));

        // Unofficial
        add_opcode(m, Opcode::new("AAC", 0x0B, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("AAC", 0x2B, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("AAX", 0x87, 2, 3, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("AAX", 0x97, 2, 4, AddressingMode::ZeroPageY, true));
        add_opcode(m, Opcode::new("AAX", 0x83, 2, 6, AddressingMode::IndexedIndirect, true));
        add_opcode(m, Opcode::new("AAX", 0x8F, 3, 4, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("ARR", 0x6B, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("ASR", 0x4B, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("ATX", 0xAB, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("AXA", 0x9F, 3, 5, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("AXA", 0x93, 2, 6, AddressingMode::IndirectIndexed, true));
        add_opcode(m, Opcode::new("AXS", 0xCB, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("DCP", 0xC7, 2, 5, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("DCP", 0xD7, 2, 6, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("DCP", 0xCF, 3, 6, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("DCP", 0xDF, 3, 7, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("DCP", 0xDB, 3, 7, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("DCP", 0xC3, 2, 8, AddressingMode::IndexedIndirect, true));
        add_opcode(m, Opcode::new("DCP", 0xD3, 2, 8, AddressingMode::IndirectIndexed, true));
        add_opcode(m, Opcode::new("NOP", 0x04, 2, 3, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("NOP", 0x14, 2, 4, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("NOP", 0x34, 2, 4, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("NOP", 0x44, 2, 3, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("NOP", 0x54, 2, 4, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("NOP", 0x64, 2, 3, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("NOP", 0x74, 2, 4, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("NOP", 0x80, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("NOP", 0x82, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("NOP", 0x89, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("NOP", 0xC2, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("NOP", 0xD4, 2, 4, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("NOP", 0xE2, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("NOP", 0xF4, 2, 4, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("ISC", 0xE7, 2, 5, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("ISC", 0xF7, 2, 6, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("ISC", 0xEF, 3, 6, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("ISC", 0xFF, 3, 7, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("ISC", 0xFB, 3, 7, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("ISC", 0xE3, 2, 8, AddressingMode::IndexedIndirect, true));
        add_opcode(m, Opcode::new("ISC", 0xF3, 2, 8, AddressingMode::IndirectIndexed, true));
        add_opcode(m, Opcode::new("KIL", 0x02, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0x12, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0x22, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0x32, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0x42, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0x52, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0x62, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0x72, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0x92, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0xB2, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0xD2, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("KIL", 0xF2, 1, 0, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("LAR", 0xBB, 3, 4, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("LAX", 0xA7, 2, 3, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("LAX", 0xB7, 2, 4, AddressingMode::ZeroPageY, true));
        add_opcode(m, Opcode::new("LAX", 0xAF, 3, 4, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("LAX", 0xBF, 3, 4, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("LAX", 0xA3, 2, 6, AddressingMode::IndexedIndirect, true));
        add_opcode(m, Opcode::new("LAX", 0xB3, 2, 5, AddressingMode::IndirectIndexed, true));
        add_opcode(m, Opcode::new("NOP", 0x1A, 1, 2, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("NOP", 0x3A, 1, 2, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("NOP", 0x5A, 1, 2, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("NOP", 0x7A, 1, 2, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("NOP", 0xDA, 1, 2, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("NOP", 0xFA, 1, 2, AddressingMode::Implied, true));
        add_opcode(m, Opcode::new("RLA", 0x27, 2, 5, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("RLA", 0x37, 2, 6, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("RLA", 0x2F, 3, 6, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("RLA", 0x3F, 3, 7, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("RLA", 0x3B, 3, 7, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("RLA", 0x23, 2, 8, AddressingMode::IndexedIndirect, true));
        add_opcode(m, Opcode::new("RLA", 0x33, 2, 8, AddressingMode::IndirectIndexed, true));
        add_opcode(m, Opcode::new("RRA", 0x67, 2, 5, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("RRA", 0x77, 2, 6, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("RRA", 0x6F, 3, 6, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("RRA", 0x7F, 3, 7, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("RRA", 0x7B, 3, 7, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("RRA", 0x63, 2, 8, AddressingMode::IndexedIndirect, true));
        add_opcode(m, Opcode::new("RRA", 0x73, 2, 8, AddressingMode::IndirectIndexed, true));
        add_opcode(m, Opcode::new("SBC", 0xEB, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("SLO", 0x07, 2, 5, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("SLO", 0x17, 2, 6, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("SLO", 0x0F, 3, 6, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("SLO", 0x1F, 3, 7, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("SLO", 0x1B, 3, 7, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("SLO", 0x03, 2, 8, AddressingMode::IndexedIndirect, true));
        add_opcode(m, Opcode::new("SLO", 0x13, 2, 8, AddressingMode::IndirectIndexed, true));
        add_opcode(m, Opcode::new("SRE", 0x47, 2, 5, AddressingMode::ZeroPage, true));
        add_opcode(m, Opcode::new("SRE", 0x57, 2, 6, AddressingMode::ZeroPageX, true));
        add_opcode(m, Opcode::new("SRE", 0x4F, 3, 6, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("SRE", 0x5F, 3, 7, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("SRE", 0x5B, 3, 7, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("SRE", 0x43, 2, 8, AddressingMode::IndexedIndirect, true));
        add_opcode(m, Opcode::new("SRE", 0x53, 2, 8, AddressingMode::IndirectIndexed, true));
        add_opcode(m, Opcode::new("SXA", 0x9E, 3, 5, AddressingMode::AbsoluteY, true));
        add_opcode(m, Opcode::new("SYA", 0x9C, 3, 5, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("NOP", 0x0C, 3, 4, AddressingMode::Absolute, true));
        add_opcode(m, Opcode::new("NOP", 0x1C, 3, 4, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("NOP", 0x3C, 3, 4, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("NOP", 0x5C, 3, 4, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("NOP", 0x7C, 3, 4, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("NOP", 0xDC, 3, 4, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("NOP", 0xFC, 3, 4, AddressingMode::AbsoluteX, true));
        add_opcode(m, Opcode::new("XAA", 0x8B, 2, 2, AddressingMode::Immediate, true));
        add_opcode(m, Opcode::new("XAS", 0x9B, 3, 5, AddressingMode::AbsoluteY, true));

        map
    };
}

fn add_opcode(m: &mut HashMap<u8, Opcode>, opcode: Opcode) {
    m.insert(opcode.code, opcode);
}
use crate::cpu::types::AddressingMode;
use lazy_static::lazy_static;
use std::collections::HashMap;

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

pub fn get_opcode_by_name(name: &str) -> Option<&'static Opcode> {
    OPCODES.values().find(|opcode| opcode.name == name)
}

lazy_static! {
    static ref OPCODES: HashMap<u8, Opcode> = {
        let mut map = HashMap::new();
        let m = &mut map;

        // Load and Store
        add_opcode(m, Opcode::new("LDA", 0xA9, 2, 2, AddressingMode::Immediate));

        // Register Transfer
        add_opcode(m, Opcode::new("TAX", 0xAA, 1, 2, AddressingMode::Implied));

        // Increment and Decrement
        add_opcode(m, Opcode::new("INX", 0xE8, 1, 2, AddressingMode::Implied));

        // Subroutine and Interrupt
        add_opcode(m, Opcode::new("BRK", 0x00, 1, 7, AddressingMode::Implied));
        
        map
    };
}

fn add_opcode(m: &mut HashMap<u8, Opcode>, opcode: Opcode) {
    m.insert(opcode.code, opcode);
}
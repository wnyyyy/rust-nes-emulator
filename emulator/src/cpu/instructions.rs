use crate::cpu::CPU;
use crate::util::is_negative;

pub enum Opcode {
    Tax = 0xAA,
    LdaImmediate = 0xA9,
    Null = 0x00,
}

pub fn tax(cpu: &mut CPU) {
    cpu.register_x = cpu.register_a;

    cpu.status.zero = cpu.register_x == 0;
    cpu.status.negative = is_negative(cpu.register_x);
}

pub fn lda(cpu: &mut CPU, param: u8) {
    cpu.register_a = param;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}


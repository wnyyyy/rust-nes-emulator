use crate::common::util::is_negative;
use crate::cpu::CPU;

pub fn brk(cpu: &mut CPU) {
    cpu.status.break_command = true;
}

pub fn lda(cpu: &mut CPU, param: u8) {
    cpu.register_a = param;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn ldx(cpu: &mut CPU, param: u8) {
    cpu.register_x = param;

    cpu.status.zero = cpu.register_x == 0;
    cpu.status.negative = is_negative(cpu.register_x);
}

pub fn ldy(cpu: &mut CPU, param: u8) {
    cpu.register_y = param;

    cpu.status.zero = cpu.register_y == 0;
    cpu.status.negative = is_negative(cpu.register_y);
}

pub fn sta(cpu: &mut CPU, param: u16) {
    cpu.memory.write(param, cpu.register_a).unwrap();
}

pub fn stx(cpu: &mut CPU, param: u16) {
    cpu.memory.write(param, cpu.register_x).unwrap();
}

pub fn sty(cpu: &mut CPU, param: u16) {
    cpu.memory.write(param, cpu.register_y).unwrap();
}

pub fn tax(cpu: &mut CPU) {
    cpu.register_x = cpu.register_a;

    cpu.status.zero = cpu.register_x == 0;
    cpu.status.negative = is_negative(cpu.register_x);
}

pub fn inx(cpu: &mut CPU) {
    cpu.register_x = cpu.register_x.wrapping_add(1);

    cpu.status.zero = cpu.register_x == 0;
    cpu.status.negative = is_negative(cpu.register_x);
}
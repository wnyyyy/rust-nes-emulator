use crate::common::util::{is_negative, overflows_negative, overflows_positive};
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

pub fn adc(cpu: &mut CPU, param: u8) {
    let carry = if cpu.status.carry { 1 } else { 0 };
    let result = cpu.register_a as u16 + param as u16 + carry as u16;
    let old_a = cpu.register_a;
    cpu.register_a = result as u8;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(result as u8);
    cpu.status.carry = result > cpu.register_a as u16;
    cpu.status.overflow = overflows_positive(result, old_a, param);
}

pub fn sbc(cpu: &mut CPU, param: u8) {
    let carry = if cpu.status.carry { 0u8 } else { 1u8 };
    let (result, cout) = cpu.register_a.overflowing_sub(param.wrapping_add(carry));
    let old_a = cpu.register_a;
    cpu.register_a = result;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(result);
    cpu.status.carry = !cout;
    cpu.status.overflow = overflows_negative(result as u16, old_a, param);
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
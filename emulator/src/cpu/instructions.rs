use crate::common::errors::EmulatorError;
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
    let carry = if cpu.status.carry { 0 } else { 1 };
    let (result, cout) = cpu.register_a.overflowing_sub(param.wrapping_add(carry));
    let old_a = cpu.register_a;
    cpu.register_a = result;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(result);
    cpu.status.carry = !cout;
    cpu.status.overflow = overflows_negative(result as u16, old_a, param);
}

pub fn inc(cpu: &mut CPU, param: u16) -> Result<(), EmulatorError>{
    let value = cpu.memory.read(param)?;
    let result = value.wrapping_add(1);
    cpu.memory.write(param, result).unwrap();

    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
    Ok(())
}

pub fn inx(cpu: &mut CPU) {
    cpu.register_x = cpu.register_x.wrapping_add(1);

    cpu.status.zero = cpu.register_x == 0;
    cpu.status.negative = is_negative(cpu.register_x);
}

pub fn iny(cpu: &mut CPU) {
    cpu.register_y = cpu.register_y.wrapping_add(1);

    cpu.status.zero = cpu.register_y == 0;
    cpu.status.negative = is_negative(cpu.register_y);
}

pub fn dec(cpu: &mut CPU, param: u16) -> Result<(), EmulatorError>{
    let value = cpu.memory.read(param)?;
    let result = value.wrapping_sub(1);
    cpu.memory.write(param, result).unwrap();

    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
    Ok(())
}

pub fn dex(cpu: &mut CPU) {
    cpu.register_x = cpu.register_x.wrapping_sub(1);

    cpu.status.zero = cpu.register_x == 0;
    cpu.status.negative = is_negative(cpu.register_x);
}

pub fn dey(cpu: &mut CPU) {
    cpu.register_y = cpu.register_y.wrapping_sub(1);

    cpu.status.zero = cpu.register_y == 0;
    cpu.status.negative = is_negative(cpu.register_y);
}

pub fn tax(cpu: &mut CPU) {
    cpu.register_x = cpu.register_a;

    cpu.status.zero = cpu.register_x == 0;
    cpu.status.negative = is_negative(cpu.register_x);
}

pub fn tay(cpu: &mut CPU) {
    cpu.register_y = cpu.register_a;

    cpu.status.zero = cpu.register_y == 0;
    cpu.status.negative = is_negative(cpu.register_y);
}

pub fn txa(cpu: &mut CPU) {
    cpu.register_a = cpu.register_x;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn tya(cpu: &mut CPU) {
    cpu.register_a = cpu.register_y;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn and(cpu: &mut CPU, param: u8) {
    cpu.register_a &= param;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn eor(cpu: &mut CPU, param: u8) {
    cpu.register_a ^= param;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn ora(cpu: &mut CPU, param: u8) {
    cpu.register_a |= param;

    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}
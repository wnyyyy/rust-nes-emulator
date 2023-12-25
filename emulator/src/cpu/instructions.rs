use crate::common::constants::{DEBUG, IRQ_VECTOR, STACK_START};
use crate::common::errors::EmulatorError;
use crate::common::util::{is_negative, overflows_negative, overflows_positive};
use crate::cpu::types::ProcessorStatus;
use crate::cpu::cpu::CPU;
use crate::memory::memory::Memory;

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
    cpu.write(param, cpu.register_a).unwrap();
}

pub fn stx(cpu: &mut CPU, param: u16) {
    cpu.write(param, cpu.register_x).unwrap();
}

pub fn sty(cpu: &mut CPU, param: u16) {
    cpu.write(param, cpu.register_y).unwrap();
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

pub fn inc(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError>{
    let value = cpu.read(address)?;
    let result = value.wrapping_add(1);
    cpu.write(address, result).unwrap();

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
    let value = cpu.read(param)?;
    let result = value.wrapping_sub(1);
    cpu.write(param, result).unwrap();

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

pub fn cmp(cpu: &mut CPU, param: u8) {
    let result = cpu.register_a.wrapping_sub(param);
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
    cpu.status.carry = cpu.register_a >= param;
    if DEBUG {
        print!("\n  Comparing A: {:02X} to {:02X} | Result: {:02X} | Zero: {} | Negative: {} | Carry: {}",
            cpu.register_a, param, result, cpu.status.zero, cpu.status.negative, cpu.status.carry);
    }
}

pub fn cpx(cpu: &mut CPU, param: u8) {
    let result = cpu.register_x.wrapping_sub(param);
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
    cpu.status.carry = cpu.register_x >= param;
}

pub fn cpy(cpu: &mut CPU, param: u8) {
    let result = cpu.register_y.wrapping_sub(param);
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
    cpu.status.carry = cpu.register_y >= param;
}

pub fn bit(cpu: &mut CPU, param: u8) {
    cpu.status.zero = cpu.register_a & param == 0;
    cpu.status.negative = is_negative(param);
    cpu.status.overflow = param & 0b0100_0000 != 0;
}

pub fn asl_accumulator(cpu: &mut CPU) {
    cpu.status.carry = cpu.register_a & 0b1000_0000 != 0;
    cpu.register_a = cpu.register_a.wrapping_shl(1);
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn asl(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError>{
    let value = cpu.read(address)?;
    cpu.status.carry = value & 0b1000_0000 != 0;
    let result = value.wrapping_shl(1);
    cpu.write(address, result)?;
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
    Ok(())
}

pub fn lsr_accumulator(cpu: &mut CPU) {
    cpu.status.carry = cpu.register_a & 0b0000_0001 != 0;
    cpu.register_a = cpu.register_a.wrapping_shr(1);
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = false;
}

pub fn lsr(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError>{
    let value = cpu.read(address)?;
    cpu.status.carry = value & 0b0000_0001 != 0;
    let result = value.wrapping_shr(1);
    cpu.write(address, result)?;
    cpu.status.zero = result == 0;
    cpu.status.negative = false;
    Ok(())
}

pub fn rol_accumulator(cpu: &mut CPU) {
    let carry = if cpu.status.carry { 1 } else { 0 };
    cpu.status.carry = cpu.register_a & 0b1000_0000 != 0;
    cpu.register_a = cpu.register_a.wrapping_shl(1) | carry;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn rol(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError>{
    let value = cpu.read(address)?;
    let carry = if cpu.status.carry { 1 } else { 0 };
    cpu.status.carry = value & 0b1000_0000 != 0;
    let result = value.wrapping_shl(1) | carry;
    cpu.write(address, result)?;
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
    Ok(())
}

pub fn ror_accumulator(cpu: &mut CPU) {
    let carry = if cpu.status.carry { 0b1000_0000 } else { 0 };
    cpu.status.carry = cpu.register_a & 0b0000_0001 != 0;
    cpu.register_a = cpu.register_a.wrapping_shr(1) | carry;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn ror(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError>{
    let value = cpu.read(address)?;
    let carry = if cpu.status.carry { 0b1000_0000 } else { 0 };
    cpu.status.carry = value & 0b0000_0001 != 0;
    let result = value.wrapping_shr(1) | carry;
    cpu.write(address, result)?;
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
    Ok(())
}

pub fn jmp(cpu: &mut CPU, address: u16) {
    cpu.program_counter = address;
    if DEBUG {
        print!("\n  Jumped to address: {:04X}", address);
    }
}

pub fn bcc(cpu: &mut CPU, offset: i8) -> Result<(), EmulatorError> {
    if !cpu.status.carry {
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
    Ok(())
}

pub fn bcs(cpu: &mut CPU, offset: i8) -> Result<(), EmulatorError> {
    if cpu.status.carry {
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
    Ok(())
}

pub fn beq(cpu: &mut CPU, offset: i8) -> Result<(), EmulatorError> {
    if cpu.status.zero {
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
    Ok(())
}

pub fn bmi(cpu: &mut CPU, offset: i8) -> Result<(), EmulatorError> {
    if cpu.status.negative {
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
    Ok(())
}

pub fn bne(cpu: &mut CPU, offset: i8) -> Result<(), EmulatorError> {
    if !cpu.status.zero {
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
    Ok(())
}

pub fn bpl(cpu: &mut CPU, offset: i8) -> Result<(), EmulatorError> {
    if !cpu.status.negative {
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
    Ok(())
}

pub fn bvc(cpu: &mut CPU, offset: i8) -> Result<(), EmulatorError> {
    if !cpu.status.overflow {
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
    Ok(())
}

pub fn bvs(cpu: &mut CPU, offset: i8) -> Result<(), EmulatorError> {
    if cpu.status.overflow {
        cpu.program_counter = cpu.program_counter.wrapping_add(offset as u16);
    }
    Ok(())
}

pub fn tsx(cpu: &mut CPU) {
    cpu.register_x = cpu.stack_pointer;
    cpu.status.zero = cpu.register_x == 0;
    cpu.status.negative = is_negative(cpu.register_x);
}

pub fn txs(cpu: &mut CPU) {
    cpu.stack_pointer = cpu.register_x;
}

pub fn pha(cpu: &mut CPU) -> Result<(), EmulatorError> {
    stack_push(cpu, cpu.register_a)?;
    Ok(())
}

pub fn php(cpu: &mut CPU) -> Result<(), EmulatorError> {
    let status = cpu.status.to_u8() | 0b0001_0000;
    stack_push(cpu, status)?;
    Ok(())
}

pub fn pla(cpu: &mut CPU) -> Result<(), EmulatorError> {
    cpu.register_a = stack_pop(cpu)?;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
    Ok(())
}

pub fn plp(cpu: &mut CPU) -> Result<(), EmulatorError> {
    let status_bits = stack_pop(cpu)?;
    cpu.status = ProcessorStatus::from_u8(status_bits);
    cpu.status.break_command = false;
    Ok(())
}

pub fn clc(cpu: &mut CPU) {
    cpu.status.carry = false;
}

pub fn cli(cpu: &mut CPU) {
    cpu.status.interrupt_disable = false;
}

pub fn clv(cpu: &mut CPU) {
    cpu.status.overflow = false;
}

pub fn sec(cpu: &mut CPU) {
    cpu.status.carry = true;
}

pub fn sei(cpu: &mut CPU) {
    cpu.status.interrupt_disable = true;
}

pub fn jsr(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let return_address = cpu.program_counter + 2;
    let return_address_low = (return_address & 0x00FF) as u8;
    let return_address_high = ((return_address & 0xFF00) >> 8) as u8;

    stack_push(cpu, return_address_high)?;
    stack_push(cpu, return_address_low)?;

    if DEBUG {
        print!("\n  Jumped to address: {:04X}", address);
        print!("\n  Stored address: {:02X}{:02X}", return_address_high, return_address_low);
    }

    cpu.program_counter = address;
    Ok(())
}

pub fn rts(cpu: &mut CPU) -> Result<(), EmulatorError> {
    let return_address_low = stack_pop(cpu)?;
    let return_address_high = stack_pop(cpu)?;
    if DEBUG {
        print!("\n  Returned to address: {:02X}{:02X}", return_address_high, return_address_low);
    }
    cpu.program_counter = u16::from_le_bytes([return_address_low, return_address_high]);
    Ok(())
}

pub fn rti(cpu: &mut CPU) -> Result<(), EmulatorError> {
    let status_bits = stack_pop(cpu)?;
    let return_address_low = stack_pop(cpu)?;
    let return_address_high = stack_pop(cpu)?;
    if DEBUG {
        print!("\n  Returned to address: {:02X}{:02X}", return_address_high, return_address_low);
    }
    cpu.program_counter = u16::from_le_bytes([return_address_low, return_address_high]);
    cpu.status = ProcessorStatus::from_u8(status_bits);
    cpu.status.break_command = false;
    Ok(())
}

pub fn brk(cpu: &mut CPU) -> Result<(), EmulatorError> {
    stack_push(cpu, (cpu.program_counter >> 8) as u8)?;
    stack_push(cpu, cpu.program_counter as u8)?;
    let status = cpu.status.to_u8() | 0b0001_0000;
    stack_push(cpu, status)?;
    cpu.program_counter = cpu.read_u16(IRQ_VECTOR)?;
    Ok(())
}

fn stack_push(cpu: &mut CPU, value: u8) -> Result<(), EmulatorError> {
    let sp_address = cpu.stack_pointer as u16 + STACK_START;
    cpu.write(sp_address, value)?;
    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
    if DEBUG {
        print!("\n  Pushed {:02X} to stack at {:#04X}", value, sp_address);
    }
    Ok(())
}

fn stack_pop(cpu: &mut CPU) -> Result<u8, EmulatorError> {
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
    let sp_address = cpu.stack_pointer as u16 + STACK_START;
    let value = cpu.read(sp_address)?;
    if DEBUG {
        print!("\n  Popped {:02X} from stack at {:#04X}", value, sp_address);
    }
    Ok(value)
}
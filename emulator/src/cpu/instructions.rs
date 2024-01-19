use crate::common::constants::{DEBUG, IRQ_VECTOR, STACK_START};
use crate::common::errors::EmulatorError;
use crate::common::util::{is_negative, overflows_negative, overflows_positive};
use crate::cpu::types::ProcessorStatus;
use crate::cpu::CPU;
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
    let param = param.wrapping_add(carry);
    let (result, cout) = cpu.register_a.overflowing_sub(param);
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

pub fn cld(cpu: &mut CPU) {
    cpu.status.decimal_mode = false;
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

pub fn sed(cpu: &mut CPU) {
    cpu.status.decimal_mode = true;
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

pub fn aac(cpu: &mut CPU, param: u8) {
    cpu.register_a &= param;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
    cpu.status.carry = cpu.status.negative;
}

pub fn sax(cpu: &mut CPU, address: u16) {
    let result = cpu.register_a & cpu.register_x;
    cpu.write(address, result).unwrap();
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
}

pub fn arr(cpu: &mut CPU, param: u8) {
    let mut result = cpu.register_a & param;
    let bit_0 = result & 0b0000_0001;
    result >>= 1;
    result |= bit_0 << 7;
    if cpu.status.carry {
        result |= 0b1000_0000;
    }
    let bit_5 = result & 0b0010_0000 != 0;
    let bit_6 = result & 0b0100_0000 != 0;
    match (bit_5, bit_6) {
        (true, true) => {
            cpu.status.carry = true;
            cpu.status.overflow = false;
        }
        (false, false) => {
            cpu.status.carry = false;
            cpu.status.overflow = false;
        }
        (true, false) => {
            cpu.status.carry = false;
            cpu.status.overflow = true;
        }
        (false, true) => {
            cpu.status.carry = true;
            cpu.status.overflow = true;
        }
    }
    cpu.register_a = result;
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
}

pub fn asr(cpu: &mut CPU, param: u8) {
    cpu.register_a &= param;
    cpu.status.carry = (cpu.register_a & 1) != 0;
    cpu.register_a = cpu.register_a >> 1;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
}

pub fn atx(cpu: &mut CPU, param: u8) {
    cpu.register_a &= param;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
    cpu.register_x = cpu.register_a;
}

pub fn axa(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let mut result = cpu.register_a & cpu.register_x;
    result = result & 7;
    cpu.write(address, result)?;
    Ok(())
}

pub fn dcp(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let value = cpu.read(address)?;
    let decrement = value.wrapping_sub(1);
    cpu.write(address, decrement)?;
    cpu.status.zero = cpu.register_a == decrement;
    cpu.status.negative = is_negative(decrement);
    cpu.status.carry = cpu.register_a >= decrement;
    Ok(())
}

pub fn isb(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let value = cpu.read(address)?;
    let increment = value.wrapping_add(1);
    cpu.write(address, increment)?;
    let carry = if cpu.status.carry { 0 } else { 1 };
    let increment = increment.wrapping_add(carry);
    let (result, cout) = cpu.register_a.overflowing_sub(increment);
    let old_a = cpu.register_a;
    cpu.register_a = result;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(result);
    cpu.status.carry = !cout;
    cpu.status.overflow = overflows_negative(result as u16, old_a, increment);
    Ok(())
}

pub fn lar(cpu: &mut CPU, param: u8) {
    let result = cpu.stack_pointer & param;
    cpu.register_a = result;
    cpu.register_x = result;
    cpu.stack_pointer = result;
    cpu.status.zero = result == 0;
    cpu.status.negative = is_negative(result);
}

pub fn rla(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let value = cpu.read(address)?;
    let carry = if cpu.status.carry { 1 } else { 0 };
    cpu.status.carry = value & 0b1000_0000 != 0;
    let result = value.wrapping_shl(1) | carry;
    cpu.write(address, result)?;
    let result_and = result & cpu.register_a;
    cpu.register_a = result_and;
    cpu.status.zero = result_and == 0;
    cpu.status.negative = is_negative(result_and);
    Ok(())
}

pub fn rra(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let value = cpu.read(address)?;
    let carry_old = if cpu.status.carry { 0b1000_0000 } else { 0 };
    let carry_rotate = value & 1;
    let result = value.wrapping_shr(1) | carry_old;
    cpu.write(address, result)?;
    let old_a = cpu.register_a;
    let (result_sum, cout) = cpu.register_a.overflowing_add(result + carry_rotate);
    cpu.register_a = result_sum;
    cpu.status.carry = cout;
    cpu.status.zero = result_sum == 0;
    cpu.status.negative = is_negative(result_sum);
    cpu.status.overflow = overflows_positive(result_sum as u16, old_a, result + carry_rotate);
    Ok(())
}

pub fn axs(cpu: &mut CPU, address: u16) ->  Result<(), EmulatorError> {
    let value = cpu.read(address)?;
    let result = cpu.register_a & cpu.register_x;
    let (sub_result, cout) = result.overflowing_sub(value);
    cpu.status.zero = sub_result == 0;
    cpu.status.negative = is_negative(sub_result);
    cpu.status.carry = !cout;
    cpu.register_x = sub_result;
    Ok(())
}

pub fn lax(cpu: &mut CPU, address: u16) ->  Result<(), EmulatorError> {
    let value = cpu.read(address)?;
    cpu.status.zero = value ==0;
    cpu.status.negative = is_negative(value);
    cpu.register_a = value;
    cpu.register_x = value;
    Ok(())
}

pub fn slo(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let value = cpu.read(address)?;
    cpu.status.carry = value & 0b1000_0000 != 0;
    let result = value.wrapping_shl(1);
    cpu.write(address, result)?;
    cpu.register_a |= result;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
    Ok(())
}

pub fn sre(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let value = cpu.read(address)?;
    cpu.status.carry = value & 0b0000_0001 != 0;
    let result = value.wrapping_shr(1);
    cpu.write(address, result)?;
    cpu.register_a ^= result;
    cpu.status.zero = cpu.register_a == 0;
    cpu.status.negative = is_negative(cpu.register_a);
    Ok(())
}

pub fn sxa(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let address_high = (address >> 8) as u8;
    let result = (cpu.register_x & address_high).wrapping_add(1);
    cpu.write(address, result)?;
    Ok(())
}

pub fn sya(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let address_high = (address >> 8) as u8;
    let result = (cpu.register_y & address_high).wrapping_add(1);
    cpu.write(address, result)?;
    Ok(())
}

pub fn xas(cpu: &mut CPU, address: u16) -> Result<(), EmulatorError> {
    let new_sp = cpu.register_a & cpu.register_x;
    cpu.stack_pointer = new_sp;
    let address_high = (address >> 8) as u8;
    let result = (new_sp & address_high).wrapping_add(1);
    cpu.write(address, result)?;
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
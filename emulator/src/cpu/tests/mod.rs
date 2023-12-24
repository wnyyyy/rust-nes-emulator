#[cfg(test)]
mod test {
    use crate::common::constants::{IRQ_VECTOR, RAM_SIZE};
    use crate::cpu::opcode::get_opcode_by_name_and_address_mode;
    use super::super::*;

    fn initialize_cpu() -> CPU {
        let mut cpu = CPU::new();
        cpu.tests = true;
        cpu.program_counter = 0x8000;
        cpu
    }

    #[test]
    fn test_lda_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDA", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0x05, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_lda_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDA", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0x85, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_lda_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDA", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ldx_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDX", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0x05, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ldx_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDX", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0x85, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_ldx_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDX", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ldy_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDY", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0x05, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ldy_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDY", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0x85, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_ldy_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LDY", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_sta_stores_in_memory() {
        let mut cpu = initialize_cpu();
        let test_addr = 0x08;
        let test_value = 0x05;
        cpu.register_a = test_value;
        let code = get_opcode_by_name_and_address_mode("STA", AddressingMode::ZeroPage).unwrap().code;
        cpu.load_and_run(vec![code, test_addr, 0]).unwrap();
        let stored = cpu.memory.read(test_addr as u16).unwrap();
        assert_eq!(stored, test_value);
    }

    #[test]
    fn test_stx_stores_in_memory() {
        let mut cpu = initialize_cpu();
        let test_addr = 0x1A;
        let test_value = 0x05;
        cpu.register_x = test_value;
        let code = get_opcode_by_name_and_address_mode("STX", AddressingMode::ZeroPage).unwrap().code;
        cpu.load_and_run(vec![code, test_addr, 0]).unwrap();
        let stored = cpu.memory.read(test_addr as u16).unwrap();
        assert_eq!(stored, test_value);
    }

    #[test]
    fn test_sty_stores_in_memory() {
        let mut cpu = initialize_cpu();
        let test_addr = 0x1E;
        let test_value = 0x05;
        cpu.register_y = test_value;
        let code = get_opcode_by_name_and_address_mode("STY", AddressingMode::ZeroPage).unwrap().code;
        cpu.load_and_run(vec![code, test_addr, 0]).unwrap();
        let stored = cpu.memory.read(test_addr as u16).unwrap();
        assert_eq!(stored, test_value);
    }

    #[test]
    fn test_arithmetic_1() {
        let mut cpu = initialize_cpu();
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [10, 3, 20, 4]; // 5 + 10 - 13 - 8 + 50
        let expected = 22;
        cpu.register_a = 0;
        cpu.load_and_run(vec![adc, values[0], sub, values[1], adc, values[2], sub, values[3], 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_arithmetic_2() {
        let mut cpu = initialize_cpu();
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [1, 1, 1, 10, 9]; // - 1 + 1 + 1 - 10 - 9
        let expected: i8 = -1;
        cpu.register_a = 0;
        cpu.load_and_run(vec![sub, values[0], adc, values[1], adc, values[2], sub, values[3], adc, values[4], 0]).unwrap();
        assert_eq!(cpu.register_a, expected as u8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_arithmetic_3() {
        let mut cpu = initialize_cpu();
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [5, 7, 1, 10, 6]; // - 5 + 7 + 1 - 10 + 6
        let expected = -2;
        cpu.register_a = 0;
        cpu.load_and_run(vec![sub, values[0], adc, values[1], adc, values[2], sub, values[3], adc, values[4], 0]).unwrap();
        assert_eq!(cpu.register_a, expected as u8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_arithmetic_4() {
        let mut cpu = initialize_cpu();
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [128, 128, 1, 1]; // - 5 + 7 + 1 - 10 + 6
        let expected = 0;
        cpu.register_a = 0;
        cpu.load_and_run(vec![adc, values[0], adc, values[1], sub, values[2], adc, values[3], 0]).unwrap();
        assert_eq!(cpu.register_a, expected as u8);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_arithmetic_5() {
        let mut cpu = initialize_cpu();
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [1, 2, 5, 10]; // - 5 + 7 + 1 - 10 + 6
        let expected = 249;
        cpu.register_a = 0;
        cpu.load_and_run(vec![adc, values[0], sub, values[1], adc, values[2], sub, values[3], 0]).unwrap();
        assert_eq!(cpu.register_a, expected as u8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_adc_no_flags_plus_carry() {
        let mut cpu = initialize_cpu();
        let base_value = 0x05;
        let add_value = 0x01;
        cpu.register_a = base_value;
        cpu.status.carry = true;
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, add_value, 0]).unwrap();
        assert_eq!(cpu.register_a, base_value + add_value + 1);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_adc_overflow_negative() {
        let mut cpu = initialize_cpu();
        let base_value:i8 = -128;
        let add_value:i8 = -1;
        cpu.register_a = base_value as u8;
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, add_value as u8, 0]).unwrap();
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_adc_overflow_positive() {
        let mut cpu = initialize_cpu();
        let base_value = 127;
        let add_value = 1;
        cpu.register_a = base_value;
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, add_value, 0]).unwrap();
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_adc_negative() {
        let mut cpu = initialize_cpu();
        let base_value = 59;
        let add_value: i8 = -60;
        cpu.register_a = base_value;
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, add_value as u8, 0]).unwrap();
        assert_eq!(cpu.register_a as i8, -1);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_adc_zero_and_carry() {
        let mut cpu = initialize_cpu();
        let base_value = 255;
        let add_value = 1;
        cpu.register_a = base_value;
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, add_value, 0]).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_sbc_no_flags_plus_borrow() {
        let mut cpu = initialize_cpu();
        let base_value = 5;
        let sub_value = 1;
        cpu.status.carry = false;
        cpu.register_a = base_value;
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, sub_value, 0]).unwrap();
        assert_eq!(cpu.register_a, base_value - sub_value - 1);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_sbc_carry() {
        let mut cpu = initialize_cpu();
        let base_value = 5;
        let sub_value = 3;
        cpu.status.carry = true;
        cpu.register_a = base_value;
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, sub_value, 0]).unwrap();
        assert_eq!(cpu.register_a, base_value - sub_value);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_sbc_carry_borrow() {
        let mut cpu = initialize_cpu();
        let base_value = 0;
        let sub_value = 1;
        cpu.status.carry = false;
        cpu.register_a = base_value;
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, sub_value, 0]).unwrap();
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_sbc_overflow_negative() {
        let mut cpu = initialize_cpu();
        let base_value:i8  = -120;
        let sub_value = 9;
        cpu.status.carry = true;
        cpu.register_a = base_value as u8;
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, sub_value, 0]).unwrap();
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_sbc_overflow_positive() {
        let mut cpu = initialize_cpu();
        let base_value = 120;
        let sub_value :i8 = -8;
        cpu.status.carry = true;
        cpu.register_a = base_value;
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, sub_value as u8, 0]).unwrap();
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_sbc_negative() {
        let mut cpu = initialize_cpu();
        let base_value= 5;
        let sub_value = 6;
        cpu.status.carry = true;
        cpu.register_a = base_value as u8;
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, sub_value as u8, 0]).unwrap();
        assert_eq!(cpu.register_a, (base_value - sub_value) as u8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_sbc_zero() {
        let mut cpu = initialize_cpu();
        let base_value = 10;
        let sub_value = 10;
        cpu.status.carry = true;
        cpu.register_a = base_value;
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        cpu.load_and_run(vec![code, sub_value, 0]).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_inc_positive() {
        let mut cpu = initialize_cpu();
        let address = 0x10;
        let value = 0x05;
        let code = get_opcode_by_name_and_address_mode("INC", AddressingMode::ZeroPage).unwrap().code;
        cpu.memory.write(address, value as u8).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let stored = cpu.memory.read(address).unwrap();
        assert_eq!(stored, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_inc_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("INC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0x7F;
        cpu.memory.write(address, value as u8).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let stored = cpu.memory.read(address).unwrap();
        assert_eq!(stored, 0x80);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_inc_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("INC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0xFF;
        cpu.memory.write(address, value as u8).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let stored = cpu.memory.read(address).unwrap();
        assert_eq!(stored, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_inx_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_x = 0x05;
        let code = get_opcode_by_name_and_address_mode("INX", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_inx_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("INX", AddressingMode::Implied).unwrap().code;
        cpu.register_x = 0x7F;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x80);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_inx_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("INX", AddressingMode::Implied).unwrap().code;
        cpu.register_x = 0xFF;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_iny_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_y = 0x05;
        let code = get_opcode_by_name_and_address_mode("INY", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_iny_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("INY", AddressingMode::Implied).unwrap().code;
        cpu.register_y = 0x7F;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x80);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_iny_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("INY", AddressingMode::Implied).unwrap().code;
        cpu.register_y = 0xFF;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dec_positive() {
        let mut cpu = initialize_cpu();
        let address = 0x10;
        let value = 0x07;
        let code = get_opcode_by_name_and_address_mode("DEC", AddressingMode::ZeroPage).unwrap().code;
        cpu.memory.write(address, value as u8).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let stored = cpu.memory.read(address).unwrap();
        assert_eq!(stored, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dec_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("DEC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0x00;
        cpu.memory.write(address, value as u8).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let stored = cpu.memory.read(address).unwrap();
        assert_eq!(stored, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_dec_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("DEC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0x01;
        cpu.memory.write(address, value as u8).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let stored = cpu.memory.read(address).unwrap();
        assert_eq!(stored, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dex_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_x = 0x07;
        let code = get_opcode_by_name_and_address_mode("DEX", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dex_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("DEX", AddressingMode::Implied).unwrap().code;
        cpu.register_x = 0x00;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_dex_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("DEX", AddressingMode::Implied).unwrap().code;
        cpu.register_x = 0x01;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dey_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_y = 0x07;
        let code = get_opcode_by_name_and_address_mode("DEY", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dey_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("DEY", AddressingMode::Implied).unwrap().code;
        cpu.register_y = 0x00;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_dey_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("DEY", AddressingMode::Implied).unwrap().code;
        cpu.register_y = 0x01;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tax_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0x05;
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tax_negative() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0x85;
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tax_zero() {
        let mut cpu = initialize_cpu();
        cpu.register_x = 1;
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tay_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0x05;
        let code = get_opcode_by_name_and_address_mode("TAY", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tay_negative() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0x85;
        let code = get_opcode_by_name_and_address_mode("TAY", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tay_zero() {
        let mut cpu = initialize_cpu();
        cpu.register_y = 1;
        let code = get_opcode_by_name_and_address_mode("TAY", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_y, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_txa_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_x = 0x05;
        let code = get_opcode_by_name_and_address_mode("TXA", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_txa_negative() {
        let mut cpu = initialize_cpu();
        cpu.register_x = 0x85;
        let code = get_opcode_by_name_and_address_mode("TXA", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_txa_zero() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 1;
        let code = get_opcode_by_name_and_address_mode("TXA", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tya_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_y = 0x05;
        let code = get_opcode_by_name_and_address_mode("TYA", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tya_negative() {
        let mut cpu = initialize_cpu();
        cpu.register_y = 0x85;
        let code = get_opcode_by_name_and_address_mode("TYA", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tya_zero() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 1;
        let code = get_opcode_by_name_and_address_mode("TYA", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_and_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("AND", AddressingMode::ZeroPage).unwrap().code;
        let address= 0x10;
        let memory_value = 0b1010_1010;
        let expected = 0b0010_1000;
        cpu.memory.write(address, memory_value).unwrap();
        cpu.register_a = 0b0010_1100;
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_and_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("AND", AddressingMode::ZeroPage).unwrap().code;
        let address= 0x10;
        let memory_value = 0b1010_1010;
        let expected = 0b1010_1000;
        cpu.memory.write(address, memory_value).unwrap();
        cpu.register_a = 0b1110_1100;
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_and_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("AND", AddressingMode::ZeroPage).unwrap().code;
        let address= 0x10;
        let memory_value = 0b0000_0011;
        let expected = 0b0000_0000;
        cpu.memory.write(address, memory_value).unwrap();
        cpu.register_a = 0b1110_1100;
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_eor_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("EOR", AddressingMode::Immediate).unwrap().code;
        let value = 0b1100_1010;
        cpu.register_a = 0b1010_1010;
        let expected = 0b0110_0000;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_eor_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("EOR", AddressingMode::Immediate).unwrap().code;
        let value = 0b0100_1010;
        cpu.register_a = 0b1010_1010;
        let expected = 0b1110_0000;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_eor_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("EOR", AddressingMode::Immediate).unwrap().code;
        let value = 0b0100_1010;
        cpu.register_a = 0b0100_1010;
        let expected = 0b0000_0000;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ora_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ORA", AddressingMode::Immediate).unwrap().code;
        let value = 0b0100_1010;
        cpu.register_a = 0b0110_1010;
        let expected = 0b0110_1010;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ora_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ORA", AddressingMode::Immediate).unwrap().code;
        let value = 0b0100_1010;
        cpu.register_a = 0b1010_1001;
        let expected = 0b1110_1011;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_ora_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ORA", AddressingMode::Immediate).unwrap().code;
        let value = 0b0000_0000;
        cpu.register_a = 0b0000_0000;
        let expected = 0b0000_0000;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_cmp_carry() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CMP", AddressingMode::Immediate).unwrap().code;
        let value = 0x10;
        cpu.register_a = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cmp_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CMP", AddressingMode::Immediate).unwrap().code;
        let value = 0x20;
        cpu.register_a = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_cmp_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CMP", AddressingMode::Immediate).unwrap().code;
        let value = 0x21;
        cpu.register_a = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(!cpu.status.carry);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cpx_carry() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CPX", AddressingMode::Immediate).unwrap().code;
        let value = 0x10;
        cpu.register_x = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cpx_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CPX", AddressingMode::Immediate).unwrap().code;
        let value = 0x20;
        cpu.register_x = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_cpx_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CPX", AddressingMode::Immediate).unwrap().code;
        let value = 0x21;
        cpu.register_x = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(!cpu.status.carry);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cpy_carry() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CPY", AddressingMode::Immediate).unwrap().code;
        let value = 0x10;
        cpu.register_y = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cpy_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CPY", AddressingMode::Immediate).unwrap().code;
        let value = 0x20;
        cpu.register_y = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_cpy_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CPY", AddressingMode::Immediate).unwrap().code;
        let value = 0x21;
        cpu.register_y = 0x20;
        cpu.load_and_run(vec![code, value, 0]).unwrap();
        assert!(!cpu.status.carry);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_bit_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("BIT", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0011_1111;
        cpu.memory.write(address, value).unwrap();
        cpu.register_a = 0b1111_1111;
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_bit_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("BIT", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b1000_0000;
        cpu.memory.write(address, value).unwrap();
        cpu.register_a = 0b1111_1111;
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_bit_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("BIT", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0011_1100;
        cpu.memory.write(address, value).unwrap();
        cpu.register_a = 0b0000_0011;
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_bit_overflow() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("BIT", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0100_0000;
        cpu.memory.write(address, value).unwrap();
        cpu.register_a = 0;
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_asl_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0010_0001;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0100_0010);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_asl_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b1000_0000;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_asl_carry() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b1000_0001;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0010);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_asl_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0100_0001;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b1000_0010);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_asl_memory() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0100_0001;
        cpu.memory.write(address, value).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let store = cpu.memory.read(address).unwrap();
        assert_eq!(store, 0b1000_0010);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_lsr_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LSR", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0010_1000;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0001_0100);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_lsr_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LSR", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0000_0001;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_lsr_carry() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LSR", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b1000_0001;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0100_0000);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_lsr_memory() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("LSR", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b1000_0010;
        cpu.memory.write(address, value).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let store = cpu.memory.read(address).unwrap();
        assert_eq!(store, 0b0100_0001);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_rol_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0010_0001;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0100_0010);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_rol_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b1000_0000;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_rol_carry() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b1000_0001;
        cpu.status.carry = true;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0011);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_rol_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0100_0001;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b1000_0010);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_rol_memory() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0100_0001;
        cpu.status.carry = true;
        cpu.memory.write(address, value).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let store = cpu.memory.read(address).unwrap();
        assert_eq!(store, 0b1000_0011);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_ror_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0010_1000;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0001_0100);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_ror_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0000_0001;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_ror_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b0000_0100;
        cpu.status.carry = true;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b1000_0010);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_ror_carry() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::Accumulator).unwrap().code;
        cpu.register_a = 0b1000_0001;
        cpu.status.carry = true;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0b1100_0000);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_ror_memory() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b1000_0011;
        cpu.memory.write(address, value).unwrap();
        cpu.load_and_run(vec![code, address as u8, 0]).unwrap();
        let store = cpu.memory.read(address).unwrap();
        assert_eq!(store, 0b0100_0001);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("JMP", AddressingMode::Absolute).unwrap().code;
        let address = 0x1ABC;
        cpu.load_and_run(vec![code, address as u8, (address >> 8) as u8, 0]).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, address + brk_bytes);
    }

    #[test]
    fn test_jmp_indirect() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("JMP", AddressingMode::Indirect).unwrap().code;
        let jump_address = 0x4EFF;
        let address = 0x1ABC;
        cpu.memory.write(address + 1, (jump_address >> 8) as u8).unwrap();
        cpu.memory.write(address, jump_address as u8).unwrap();
        cpu.load_and_run(vec![code, address as u8, (address >> 8) as u8, 0]).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, jump_address + brk_bytes);
    }

    #[test]
    fn test_bcc_true() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BCC", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x05;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.carry = false;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bcc_false() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BCC", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x05;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.carry = true;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bcs_true() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BCS", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0xFB;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.carry = true;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bcs_false() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BCS", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0xFB;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.carry = false;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_beq_true() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BEQ", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x01;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.zero = true;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_beq_false() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BEQ", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x01;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.zero = false;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bmi_true() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BMI", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x81;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.negative = true;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bmi_false() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BMI", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x81;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.negative = false;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bne_true() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BNE", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0xAA;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.zero = false;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bne_false() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BNE", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0xAA;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.zero = true;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bpl_true() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BPL", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x79;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.negative = false;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bpl_false() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BPL", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x79;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.negative = true;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bvc_true() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BVC", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0xDD;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.overflow = false;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bvc_false() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BVC", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0xDD;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.overflow = true;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bvs_true() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BVS", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x04;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.overflow = true;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bvs_false() {
        let mut cpu = initialize_cpu();
        let opcode = get_opcode_by_name_and_address_mode("BVS", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        cpu.program_counter += 0x0F;
        let old_pc = cpu.program_counter;
        let branch = 0x04;
        let branch_address = 0x10;
        cpu.memory.write(branch_address, branch).unwrap();
        cpu.status.overflow = false;
        let mut program = vec![0; 0x0F];
        program.extend_from_slice(&[code, branch_address as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, old_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_tsx_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("TSX", AddressingMode::Implied).unwrap().code;
        cpu.stack_pointer = 0x10;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x10);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tsx_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("TSX", AddressingMode::Implied).unwrap().code;
        cpu.stack_pointer = 0xFF;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tsx_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("TSX", AddressingMode::Implied).unwrap().code;
        cpu.stack_pointer = 0x00;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_txs() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("TXS", AddressingMode::Implied).unwrap().code;
        cpu.register_x = 0x10;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.stack_pointer, 0x10);
    }

    #[test]
    fn test_pha() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0xAA;
        let address = STACK_POINTER_INIT as u16 + 0x0100;
        let code = get_opcode_by_name_and_address_mode("PHA", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.memory.read(address).unwrap(), 0xAA);
        assert_eq!(cpu.stack_pointer, STACK_POINTER_INIT - 1);
    }

    #[test]
    fn test_pha_wrapping() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0xAA;
        cpu.stack_pointer = 0x00;
        let address = 0x0100;
        let code = get_opcode_by_name_and_address_mode("PHA", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.memory.read(address).unwrap(), 0xAA);
        assert_eq!(cpu.stack_pointer, STACK_POINTER_INIT);
    }

    #[test]
    fn test_php() {
        let mut cpu = initialize_cpu();
        let status = ProcessorStatus::from_u8(0b1000_1010);
        cpu.status = status;
        let address = STACK_POINTER_INIT as u16 + 0x0100;
        let code = get_opcode_by_name_and_address_mode("PHP", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.memory.read(address).unwrap(), status.to_u8() | 0b0001_0000);
        assert_eq!(cpu.stack_pointer, STACK_POINTER_INIT - 1);
    }

    #[test]
    fn test_php_wrapping() {
        let mut cpu = initialize_cpu();
        let status = ProcessorStatus::from_u8(0b1000_1010);
        cpu.status = status;
        cpu.stack_pointer = 0x00;
        let address = 0x0100;
        let code = get_opcode_by_name_and_address_mode("PHP", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.memory.read(address).unwrap(), status.to_u8() | 0b0001_0000);
        assert_eq!(cpu.stack_pointer, STACK_POINTER_INIT);
    }

    #[test]
    fn test_pla_positive() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("PLA", AddressingMode::Implied).unwrap().code;
        let test_offset = 0x10;
        let test_value = 0x03;
        let address = STACK_POINTER_INIT as u16 + 0x0100 - test_offset;
        cpu.memory.write(address, test_value).unwrap();
        cpu.stack_pointer -= test_offset as u8 + 1;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, test_value);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_pla_negative() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("PLA", AddressingMode::Implied).unwrap().code;
        let test_offset = 0xFF;
        let test_value = 0xF0;
        let address = STACK_POINTER_INIT as u16 + 0x0100 - test_offset;
        cpu.memory.write(address, test_value).unwrap();
        cpu.stack_pointer = (cpu.stack_pointer - test_offset as u8).wrapping_sub(1);
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, test_value);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_pla_zero() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0x01;
        let code = get_opcode_by_name_and_address_mode("PLA", AddressingMode::Implied).unwrap().code;
        let test_offset = 0x00;
        let test_value = 0;
        let address = STACK_POINTER_INIT as u16 + 0x0100 + test_offset;
        cpu.memory.write(address, test_value).unwrap();
        cpu.stack_pointer = STACK_POINTER_INIT - 1;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, test_value);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_plp() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("PLP", AddressingMode::Implied).unwrap().code;
        let test_offset = 0x10;
        let mut status = ProcessorStatus::new();
        status.zero = true;
        status.negative = true;
        status.overflow = true;
        status.carry = true;
        status.interrupt_disable = true;
        let test_value = status.to_u8();
        let address = STACK_POINTER_INIT as u16 + 0x0100 - test_offset;
        cpu.memory.write(address, test_value).unwrap();
        cpu.stack_pointer -= test_offset as u8 + 1;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        let new = cpu.status.to_u8();
        assert_eq!(new, test_value);
    }

    #[test]
    fn test_clc() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CLC", AddressingMode::Implied).unwrap().code;
        cpu.status.carry = true;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_cli() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CLI", AddressingMode::Implied).unwrap().code;
        cpu.status.interrupt_disable = true;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert!(!cpu.status.interrupt_disable);
    }

    #[test]
    fn test_clv() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("CLV", AddressingMode::Implied).unwrap().code;
        cpu.status.overflow = true;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_sec() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("SEC", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_sei() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("SEI", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert!(cpu.status.interrupt_disable);
    }

    #[test]
    fn test_jsr() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("JSR", AddressingMode::Absolute).unwrap().code;
        cpu.program_counter += 0x0200;
        cpu.stack_pointer = 0xFF;
        let target_address = 0x1ABC;
        let return_address = cpu.program_counter + 2;
        let mut program = vec![0; 0x0200];
        program.extend_from_slice(&[code, target_address as u8, (target_address >> 8) as u8]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, target_address + brk_bytes);
        assert_eq!(cpu.stack_pointer, 0xFD);
        let high_byte = cpu.memory.read(0x01FF).unwrap();
        let low_byte = cpu.memory.read(0x01FE).unwrap();
        let pushed_address = (high_byte as u16) << 8 | low_byte as u16;
        assert_eq!(pushed_address, return_address);
    }

    #[test]
    fn test_rts() {
        let mut cpu = initialize_cpu();
        let rts = get_opcode_by_name_and_address_mode("RTS", AddressingMode::Implied).unwrap().code;
        let jsr = get_opcode_by_name_and_address_mode("JSR", AddressingMode::Absolute).unwrap();
        cpu.program_counter += 0x0200;
        let initial_pc = cpu.program_counter;
        cpu.stack_pointer = 0xFF;
        let target_address = 0x1ABC;
        cpu.memory.write(0x1ABC, rts).unwrap();
        let mut program = vec![0; 0x0200];
        program.extend_from_slice(&[jsr.code, target_address as u8, (target_address >> 8) as u8]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        cpu.load_and_run(program).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        let jsr_bytes = jsr.bytes as u16;
        assert_eq!(cpu.program_counter, initial_pc + jsr_bytes + brk_bytes);
    }

    #[test]
    fn test_rti() {
        let mut cpu = initialize_cpu();
        let rti = get_opcode_by_name_and_address_mode("RTI", AddressingMode::Implied).unwrap().code;
        cpu.stack_pointer = 0xFC;
        cpu.status = ProcessorStatus::from_u8(0b0000_0000);
        let return_status = ProcessorStatus::from_u8(0b1000_0011);
        let return_status_u8 = return_status.to_u8();
        let return_address = 0x1ABC;
        cpu.memory.write(0x01FD, return_status_u8 | 0b0001_0000).unwrap();
        cpu.memory.write(0x01FE, return_address as u8).unwrap();
        cpu.memory.write(0x01FF, (return_address >> 8) as u8).unwrap();
        cpu.load_and_run(vec![rti, 0, 0]).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.status, return_status);
        assert_eq!(cpu.program_counter, return_address + brk_bytes);
    }

    #[test]
    fn test_brk() {
        let mut cpu = initialize_cpu();
        cpu.tests = false;
        cpu.program_counter += 0x0200;
        cpu.stack_pointer = 0x05;
        cpu.status = ProcessorStatus::from_u8(0b1000_0001);
        let initial_pc = cpu.program_counter;
        let initial_status = cpu.status.to_u8();
        cpu.memory.write(IRQ_VECTOR, 0xBC).unwrap();
        cpu.memory.write(IRQ_VECTOR+1, 0x1A).unwrap();
        let code = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        let stored_status = cpu.memory.read(0x0103).unwrap();
        let stored_pc_low = cpu.memory.read(0x0104).unwrap();
        let stored_pc_high = cpu.memory.read(0x0105).unwrap();
        let stored_pc = (stored_pc_high as u16) << 8 | stored_pc_low as u16;
        assert_eq!(cpu.stack_pointer, 0x02);
        assert_eq!(cpu.program_counter, 0x1ABC);
        assert_eq!(stored_status, initial_status | 0b0001_0000);
        assert_eq!(stored_pc, initial_pc);
    }
}
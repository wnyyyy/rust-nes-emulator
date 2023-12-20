#[cfg(test)]
mod test {
    use crate::cpu::opcode::get_opcode_by_name_and_address_mode;
    use super::super::*;

    fn initialize_cpu() -> CPU {
        let mut cpu = CPU::new();
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
    fn test_brk_flag() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert!(cpu.status.break_command);
    }
}
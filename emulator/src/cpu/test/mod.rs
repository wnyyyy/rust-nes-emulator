#[cfg(test)]
mod test {
    use crate::common::constants::{IRQ_VECTOR, RAM_SIZE, PRG_ROM_START, STACK_START};
    use crate::cpu::opcode::get_opcode_by_name_and_address_mode;
    use super::super::*;

    fn initialize_cpu(program: Vec<u8>) -> CPU {
        let mut cpu = CPU::new();
        cpu.tests = true;
        cpu.program_counter = PRG_ROM_START;
        let mut rom = Rom::default();
        for (i, byte) in program.iter().enumerate() {
            rom.prg_rom[i] = *byte;
        }
        cpu.bus.load_rom(rom);
        cpu
    }

    #[test]
    fn test_lda_positive() {
        let code = get_opcode_by_name_and_address_mode("LDA", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0x05, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_lda_negative() {
        let code = get_opcode_by_name_and_address_mode("LDA", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0x85, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_lda_zero() {
        let code = get_opcode_by_name_and_address_mode("LDA", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ldx_positive() {
        let code = get_opcode_by_name_and_address_mode("LDX", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0x05, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ldx_negative() {
        let code = get_opcode_by_name_and_address_mode("LDX", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0x85, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_ldx_zero() {
        let code = get_opcode_by_name_and_address_mode("LDX", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ldy_positive() {
        let code = get_opcode_by_name_and_address_mode("LDY", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0x05, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ldy_negative() {
        let code = get_opcode_by_name_and_address_mode("LDY", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0x85, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_ldy_zero() {
        let code = get_opcode_by_name_and_address_mode("LDY", AddressingMode::Immediate).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_sta_stores_in_memory() {
        let code = get_opcode_by_name_and_address_mode("STA", AddressingMode::ZeroPage).unwrap().code;
        let test_addr = 0x08;
        let test_value = 0x05;
        let program = vec![code, test_addr, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = test_value;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(test_addr as u16).unwrap();
        assert_eq!(stored, test_value);
    }

    #[test]
    fn test_stx_stores_in_memory() {
        let code = get_opcode_by_name_and_address_mode("STX", AddressingMode::ZeroPage).unwrap().code;
        let test_addr = 0x1A;
        let test_value = 0x05;
        let program = vec![code, test_addr, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = test_value;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(test_addr as u16).unwrap();
        assert_eq!(stored, test_value);
    }

    #[test]
    fn test_sty_stores_in_memory() {
        let code = get_opcode_by_name_and_address_mode("STY", AddressingMode::ZeroPage).unwrap().code;
        let test_addr = 0x1E;
        let test_value = 0x05;
        let program = vec![code, test_addr, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = test_value;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(test_addr as u16).unwrap();
        assert_eq!(stored, test_value);
    }

    #[test]
    fn test_arithmetic_1() {
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [10, 3, 20, 4]; // 5 + 10 - 13 - 8 + 50
        let expected = 22;
        let program = vec![adc, values[0], sub, values[1], adc, values[2], sub, values[3], 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_arithmetic_2() {
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [1, 1, 1, 10, 9]; // - 1 + 1 + 1 - 10 - 9
        let expected: i8 = -1;
        let program = vec![sub, values[0], adc, values[1], adc, values[2], sub, values[3], adc, values[4], 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected as u8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_arithmetic_3() {
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [5, 7, 1, 10, 6]; // - 5 + 7 + 1 - 10 + 6
        let expected = -2;
        let program = vec![sub, values[0], adc, values[1], adc, values[2], sub, values[3], adc, values[4], 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected as u8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_arithmetic_4() {
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [128, 128, 1, 1]; // - 5 + 7 + 1 - 10 + 6
        let expected = 0;
        let program = vec![adc, values[0], adc, values[1], sub, values[2], adc, values[3], 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected as u8);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_arithmetic_5() {
        let adc = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let sub = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let values = [1, 2, 5, 10]; // - 5 + 7 + 1 - 10 + 6
        let expected = 249;
        let program = vec![adc, values[0], sub, values[1], adc, values[2], sub, values[3], 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected as u8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_adc_no_flags_plus_carry() {
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let base_value = 0x05;
        let add_value = 0x01;
        let program = vec![code, add_value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = base_value;
        cpu.status.carry = true;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, base_value + add_value + 1);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_adc_overflow_negative() {
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let base_value:i8 = -128;
        let add_value:i8 = -1;
        let program = vec![code, add_value as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = base_value as u8;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_adc_overflow_positive() {
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let base_value = 127;
        let add_value = 1;
        let program = vec![code, add_value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = base_value;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_adc_negative() {
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let base_value = 59;
        let add_value: i8 = -60;
        let program = vec![code, add_value as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = base_value;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a as i8, -1);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_adc_zero_and_carry() {
        let code = get_opcode_by_name_and_address_mode("ADC", AddressingMode::Immediate).unwrap().code;
        let base_value = 255;
        let add_value = 1;
        let program = vec![code, add_value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = base_value;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_sbc_no_flags_borrow() {
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let base_value = 5;
        let sub_value = 1;
        let program = vec![code, sub_value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = false;
        cpu.register_a = base_value;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, base_value - sub_value - 1);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_sbc_carry() {
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let base_value = 5;
        let sub_value = 3;
        let program = vec![code, sub_value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = true;
        cpu.register_a = base_value;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, base_value - sub_value);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_sbc_carry_borrow() {
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let base_value = 0;
        let sub_value = 1;
        let program = vec![code, sub_value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = false;
        cpu.register_a = base_value;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_sbc_overflow_negative() {
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let base_value:i8  = -120;
        let sub_value = 9;
        let program = vec![code, sub_value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = true;
        cpu.register_a = base_value as u8;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_sbc_overflow_positive() {
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let base_value = 120;
        let sub_value :i8 = -8;
        let program = vec![code, sub_value as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = true;
        cpu.register_a = base_value;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_sbc_negative() {
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let base_value: i8 = 5;
        let sub_value: i8 = 6;
        let program = vec![code, sub_value as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = true;
        cpu.register_a = base_value as u8;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, (base_value - sub_value) as u8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_sbc_zero() {
        let code = get_opcode_by_name_and_address_mode("SBC", AddressingMode::Immediate).unwrap().code;
        let base_value = 10;
        let sub_value = 10;
        let program = vec![code, sub_value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = true;
        cpu.register_a = base_value;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_inc_positive() {
        let code = get_opcode_by_name_and_address_mode("INC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0x05;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value as u8).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_inc_negative() {
        let code = get_opcode_by_name_and_address_mode("INC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0x7F;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value as u8).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, 0x80);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_inc_zero() {
        let code = get_opcode_by_name_and_address_mode("INC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0xFF;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value as u8).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_inx_positive() {
        let code = get_opcode_by_name_and_address_mode("INX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x05;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_inx_negative() {
        let code = get_opcode_by_name_and_address_mode("INX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x7F;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x80);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_inx_zero() {
        let code = get_opcode_by_name_and_address_mode("INX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0xFF;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_iny_positive() {
        let code = get_opcode_by_name_and_address_mode("INY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x05;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_iny_negative() {
        let code = get_opcode_by_name_and_address_mode("INY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x7F;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x80);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_iny_zero() {
        let code = get_opcode_by_name_and_address_mode("INY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0xFF;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dec_positive() {
        let code = get_opcode_by_name_and_address_mode("DEC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0x07;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value as u8).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dec_negative() {
        let code = get_opcode_by_name_and_address_mode("DEC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0x00;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value as u8).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_dec_zero() {
        let code = get_opcode_by_name_and_address_mode("DEC", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0x01;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value as u8).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dex_positive() {
        let code = get_opcode_by_name_and_address_mode("DEX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x07;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dex_negative() {
        let code = get_opcode_by_name_and_address_mode("DEX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x00;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_dex_zero() {
        let code = get_opcode_by_name_and_address_mode("DEX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x01;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dey_positive() {
        let code = get_opcode_by_name_and_address_mode("DEY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x07;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dey_negative() {
        let code = get_opcode_by_name_and_address_mode("DEY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x00;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_dey_zero() {
        let code = get_opcode_by_name_and_address_mode("DEY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x01;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tax_positive() {
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0x05;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tax_negative() {
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0x85;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tax_zero() {
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 1;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tay_positive() {
        let code = get_opcode_by_name_and_address_mode("TAY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0x05;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tay_negative() {
        let code = get_opcode_by_name_and_address_mode("TAY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0x85;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tay_zero() {
        let code = get_opcode_by_name_and_address_mode("TAY", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 1;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_y, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_txa_positive() {
        let code = get_opcode_by_name_and_address_mode("TXA", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x05;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_txa_negative() {
        let code = get_opcode_by_name_and_address_mode("TXA", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x85;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_txa_zero() {
        let code = get_opcode_by_name_and_address_mode("TXA", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 1;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tya_positive() {
        let code = get_opcode_by_name_and_address_mode("TYA", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x05;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tya_negative() {
        let code = get_opcode_by_name_and_address_mode("TYA", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x85;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tya_zero() {
        let code = get_opcode_by_name_and_address_mode("TYA", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 1;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_and_positive() {
        let code = get_opcode_by_name_and_address_mode("AND", AddressingMode::ZeroPage).unwrap().code;
        let address= 0x10;
        let memory_value = 0b1010_1010;
        let expected = 0b0010_1000;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, memory_value).unwrap();
        cpu.register_a = 0b0010_1100;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_and_negative() {
        let code = get_opcode_by_name_and_address_mode("AND", AddressingMode::ZeroPage).unwrap().code;
        let address= 0x10;
        let memory_value = 0b1010_1010;
        let expected = 0b1010_1000;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, memory_value).unwrap();
        cpu.register_a = 0b1110_1100;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_and_zero() {
        let code = get_opcode_by_name_and_address_mode("AND", AddressingMode::ZeroPage).unwrap().code;
        let address= 0x10;
        let memory_value = 0b0000_0011;
        let expected = 0b0000_0000;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, memory_value).unwrap();
        cpu.register_a = 0b1110_1100;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_eor_positive() {
        let code = get_opcode_by_name_and_address_mode("EOR", AddressingMode::Immediate).unwrap().code;
        let value = 0b1100_1010;
        let expected = 0b0110_0000;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1010_1010;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_eor_negative() {
        let code = get_opcode_by_name_and_address_mode("EOR", AddressingMode::Immediate).unwrap().code;
        let value = 0b0100_1010;
        let expected = 0b1110_0000;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1010_1010;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_eor_zero() {
        let code = get_opcode_by_name_and_address_mode("EOR", AddressingMode::Immediate).unwrap().code;
        let value = 0b0100_1010;
        let expected = 0b0000_0000;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0100_1010;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ora_positive() {
        let code = get_opcode_by_name_and_address_mode("ORA", AddressingMode::Immediate).unwrap().code;
        let value = 0b0100_1010;
        let expected = 0b0110_1010;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0110_1010;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_ora_negative() {
        let code = get_opcode_by_name_and_address_mode("ORA", AddressingMode::Immediate).unwrap().code;
        let value = 0b0100_1010;
        let expected = 0b1110_1011;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1010_1001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_ora_zero() {
        let code = get_opcode_by_name_and_address_mode("ORA", AddressingMode::Immediate).unwrap().code;
        let value = 0b0000_0000;
        let expected = 0b0000_0000;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0000_0000;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_cmp_carry() {
        let code = get_opcode_by_name_and_address_mode("CMP", AddressingMode::Immediate).unwrap().code;
        let value = 0x10;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cmp_zero() {
        let code = get_opcode_by_name_and_address_mode("CMP", AddressingMode::Immediate).unwrap().code;
        let value = 0x20;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_cmp_negative() {
        let code = get_opcode_by_name_and_address_mode("CMP", AddressingMode::Immediate).unwrap().code;
        let value = 0x21;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.carry);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cpx_carry() {
        let code = get_opcode_by_name_and_address_mode("CPX", AddressingMode::Immediate).unwrap().code;
        let value = 0x10;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cpx_zero() {
        let code = get_opcode_by_name_and_address_mode("CPX", AddressingMode::Immediate).unwrap().code;
        let value = 0x20;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_cpx_negative() {
        let code = get_opcode_by_name_and_address_mode("CPX", AddressingMode::Immediate).unwrap().code;
        let value = 0x21;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.carry);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cpy_carry() {
        let code = get_opcode_by_name_and_address_mode("CPY", AddressingMode::Immediate).unwrap().code;
        let value = 0x10;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_cpy_zero() {
        let code = get_opcode_by_name_and_address_mode("CPY", AddressingMode::Immediate).unwrap().code;
        let value = 0x20;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_cpy_negative() {
        let code = get_opcode_by_name_and_address_mode("CPY", AddressingMode::Immediate).unwrap().code;
        let value = 0x21;
        let program = vec![code, value, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_y = 0x20;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.carry);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_bit_positive() {
        let code = get_opcode_by_name_and_address_mode("BIT", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0011_1111;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.register_a = 0b1111_1111;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_bit_negative() {
        let code = get_opcode_by_name_and_address_mode("BIT", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b1000_0000;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.register_a = 0b1111_1111;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_bit_zero() {
        let code = get_opcode_by_name_and_address_mode("BIT", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0011_1100;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.register_a = 0b0000_0011;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_bit_overflow() {
        let code = get_opcode_by_name_and_address_mode("BIT", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0100_0000;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.register_a = 0;
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_asl_positive() {
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0010_0001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0100_0010);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_asl_zero() {
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1000_0000;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_asl_carry() {
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1000_0001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0010);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_asl_negative() {
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0100_0001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b1000_0010);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_asl_memory() {
        let code = get_opcode_by_name_and_address_mode("ASL", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0100_0001;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let store = cpu.read(address).unwrap();
        assert_eq!(store, 0b1000_0010);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_lsr_positive() {
        let code = get_opcode_by_name_and_address_mode("LSR", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0010_1000;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0001_0100);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_lsr_zero() {
        let code = get_opcode_by_name_and_address_mode("LSR", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0000_0001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_lsr_carry() {
        let code = get_opcode_by_name_and_address_mode("LSR", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1000_0001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0100_0000);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_lsr_memory() {
        let code = get_opcode_by_name_and_address_mode("LSR", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b1000_0010;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let store = cpu.read(address).unwrap();
        assert_eq!(store, 0b0100_0001);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_rol_positive() {
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0010_0001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0100_0010);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_rol_zero() {
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1000_0000;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_rol_carry() {
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1000_0001;
        cpu.status.carry = true;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0011);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_rol_negative() {
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0100_0001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b1000_0010);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_rol_memory() {
        let code = get_opcode_by_name_and_address_mode("ROL", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let value = 0b0100_0001;
        let program = vec![code, address as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = true;
        cpu.write(address, value).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let store = cpu.read(address).unwrap();
        assert_eq!(store, 0b1000_0011);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_ror_positive() {
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0010_1000;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0001_0100);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_ror_zero() {
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0000_0001;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0000);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_ror_negative() {
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b0000_0100;
        cpu.status.carry = true;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b1000_0010);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_ror_carry() {
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::Accumulator).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0b1000_0001;
        cpu.status.carry = true;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b1100_0000);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_ror_memory() {
        let code = get_opcode_by_name_and_address_mode("ROR", AddressingMode::ZeroPage).unwrap().code;
        let address = 0x10;
        let program = vec![code, address as u8, 0];
        let value = 0b1000_0011;
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let store = cpu.read(address).unwrap();
        assert_eq!(store, 0b0100_0001);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_jmp_absolute() {
        let code = get_opcode_by_name_and_address_mode("JMP", AddressingMode::Absolute).unwrap().code;
        let address = 0x1ABC;
        let program = vec![code, address as u8, (address >> 8) as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, address + brk_bytes);
    }

    #[test]
    fn test_jmp_indirect() {
        let code = get_opcode_by_name_and_address_mode("JMP", AddressingMode::Indirect).unwrap().code;
        let jump_address = 0x1EFF;
        let address = 0x1ABC;
        let program = vec![code, address as u8, (address >> 8) as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address + 1, (jump_address >> 8) as u8).unwrap();
        cpu.write(address, jump_address as u8).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, jump_address + brk_bytes);
    }

    #[test]
    fn test_bcc_true() {
        let opcode = get_opcode_by_name_and_address_mode("BCC", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        let branch = 0x05;
        let offset = 0x0F;
        let initial_pc = PRG_ROM_START + offset;
        let mut program = vec![0; offset as usize];
        program.extend_from_slice(&[code, branch as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = false;
        cpu.program_counter += offset;
        cpu.run(|_| Ok(())).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, initial_pc + branch as u16 + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_bcc_false() {
        let opcode = get_opcode_by_name_and_address_mode("BCC", AddressingMode::Relative).unwrap();
        let code = opcode.code;
        let branch = 0x05;
        let offset = 0x0F;
        let initial_pc = PRG_ROM_START + offset;
        let mut program = vec![0; offset as usize];
        program.extend_from_slice(&[code, branch as u8, 0]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = true;
        cpu.program_counter += offset;
        cpu.run(|_| Ok(())).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, initial_pc + opcode.bytes as u16 + brk_bytes);
    }

    #[test]
    fn test_tsx_positive() {
        let code = get_opcode_by_name_and_address_mode("TSX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.stack_pointer = 0x10;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x10);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tsx_negative() {
        let code = get_opcode_by_name_and_address_mode("TSX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.stack_pointer = 0xFF;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tsx_zero() {
        let code = get_opcode_by_name_and_address_mode("TSX", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.stack_pointer = 0x00;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_txs() {
        let code = get_opcode_by_name_and_address_mode("TXS", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_x = 0x10;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.stack_pointer, 0x10);
    }

    #[test]
    fn test_pha() {
        let code = get_opcode_by_name_and_address_mode("PHA", AddressingMode::Implied).unwrap().code;
        let address = STACK_POINTER_INIT as u16 + 0x0100;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0xAA;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.read(address).unwrap(), 0xAA);
        assert_eq!(cpu.stack_pointer, STACK_POINTER_INIT - 1);
    }

    #[test]
    fn test_pha_wrapping() {
        let code = get_opcode_by_name_and_address_mode("PHA", AddressingMode::Implied).unwrap().code;
        let address = 0x0100;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0xAA;
        cpu.stack_pointer = 0x00;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.read(address).unwrap(), 0xAA);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn test_php() {
        let code = get_opcode_by_name_and_address_mode("PHP", AddressingMode::Implied).unwrap().code;
        let address = STACK_POINTER_INIT as u16 + 0x0100;
        let status = ProcessorStatus::from_u8(0b1000_1010);
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status = status;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.read(address).unwrap(), status.to_u8() | 0b0001_0000);
        assert_eq!(cpu.stack_pointer, STACK_POINTER_INIT - 1);
    }

    #[test]
    fn test_php_wrapping() {
        let code = get_opcode_by_name_and_address_mode("PHP", AddressingMode::Implied).unwrap().code;
        let address = 0x0100;
        let status = ProcessorStatus::from_u8(0b1000_1010);
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status = status;
        cpu.stack_pointer = 0x00;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.read(address).unwrap(), status.to_u8() | 0b0001_0000);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn test_pla_positive() {
        let code = get_opcode_by_name_and_address_mode("PLA", AddressingMode::Implied).unwrap().code;
        let test_offset = 0x10;
        let test_value = 0x03;
        let address = STACK_POINTER_INIT as u16 + 0x0100 - test_offset;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, test_value).unwrap();
        cpu.stack_pointer -= test_offset as u8 + 1;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, test_value);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_pla_negative() {
        let code = get_opcode_by_name_and_address_mode("PLA", AddressingMode::Implied).unwrap().code;
        let test_offset = 0xFF;
        let test_value = 0xF0;
        let address = STACK_POINTER_INIT.wrapping_sub(test_offset) as u16 + STACK_START;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, test_value).unwrap();
        cpu.stack_pointer = address.wrapping_sub(1) as u8;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, test_value);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_pla_zero() {
        let code = get_opcode_by_name_and_address_mode("PLA", AddressingMode::Implied).unwrap().code;
        let test_offset = 0x00;
        let test_value = 0;
        let address = STACK_POINTER_INIT as u16 + 0x0100 + test_offset;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = 0x01;
        cpu.write(address, test_value).unwrap();
        cpu.stack_pointer = STACK_POINTER_INIT - 1;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, test_value);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_plp() {
        let code = get_opcode_by_name_and_address_mode("PLP", AddressingMode::Implied).unwrap().code;
        let mut status = ProcessorStatus::new();
        status.zero = true;
        status.negative = true;
        status.overflow = true;
        status.carry = true;
        status.interrupt_disable = true;
        let test_offset = 0x10;
        let test_value = status.to_u8();
        let address = STACK_POINTER_INIT as u16 + 0x0100 - test_offset;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, test_value).unwrap();
        cpu.stack_pointer -= test_offset as u8 + 1;
        cpu.run(|_| Ok(())).unwrap();
        let new = cpu.status.to_u8();
        assert_eq!(new, test_value);
    }

    #[test]
    fn test_clc() {
        let code = get_opcode_by_name_and_address_mode("CLC", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.carry = true;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.carry);
    }

    #[test]
    fn test_cli() {
        let code = get_opcode_by_name_and_address_mode("CLI", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.interrupt_disable = true;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.interrupt_disable);
    }

    #[test]
    fn test_cld() {
        let code = get_opcode_by_name_and_address_mode("CLD", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.decimal_mode = true;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.decimal_mode);
    }

    #[test]
    fn test_clv() {
        let code = get_opcode_by_name_and_address_mode("CLV", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.status.overflow = true;
        cpu.run(|_| Ok(())).unwrap();
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_sec() {
        let code = get_opcode_by_name_and_address_mode("SEC", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.carry);
    }

    #[test]
    fn test_sei() {
        let code = get_opcode_by_name_and_address_mode("SEI", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.interrupt_disable);
    }

    #[test]
    fn test_sed() {
        let code = get_opcode_by_name_and_address_mode("SED", AddressingMode::Implied).unwrap().code;
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.run(|_| Ok(())).unwrap();
        assert!(cpu.status.decimal_mode);
    }

    #[test]
    fn test_jsr() {
        let code = get_opcode_by_name_and_address_mode("JSR", AddressingMode::Absolute).unwrap().code;
        let offset = 0x0200;
        let initial_pc = PRG_ROM_START + offset;
        let initial_stack_pointer = 0xFF;
        let target_address_high = 0x1A;
        let target_address_low = 0xBC;
        let target_address = (target_address_high as u16) << 8 | target_address_low as u16;
        let return_address = initial_pc + 2;
        let mut program = vec![0; 0x0200];
        program.extend_from_slice(&[code, target_address_low, target_address_high]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        let mut cpu = initialize_cpu(program);
        cpu.program_counter = initial_pc;
        cpu.stack_pointer = initial_stack_pointer;
        cpu.run(|_| Ok(())).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.program_counter, target_address + brk_bytes);
        assert_eq!(cpu.stack_pointer, 0xFD);
        let high_byte = cpu.read(0x01FF).unwrap();
        let low_byte = cpu.read(0x01FE).unwrap();
        let pushed_address = (high_byte as u16) << 8 | low_byte as u16;
        assert_eq!(pushed_address, return_address);
    }

    #[test]
    fn test_rts() {
        let rts = get_opcode_by_name_and_address_mode("RTS", AddressingMode::Implied).unwrap().code;
        let jsr = get_opcode_by_name_and_address_mode("JSR", AddressingMode::Absolute).unwrap();
        let offset = 0x0200;
        let initial_pc = PRG_ROM_START + offset;
        let initial_stack_pointer = 0xFF;
        let target_address_high = 0x1A;
        let target_address_low = 0xBC;
        let target_address = (target_address_high as u16) << 8 | target_address_low as u16;
        let mut program = vec![0; 0x0200];
        program.extend_from_slice(&[jsr.code, target_address_low, target_address_high]);
        program.extend_from_slice(&[0; RAM_SIZE as usize]);
        let mut cpu = initialize_cpu(program);
        cpu.program_counter = initial_pc;
        cpu.stack_pointer = initial_stack_pointer;
        cpu.write(target_address, rts).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        let jsr_bytes = jsr.bytes as u16;
        assert_eq!(cpu.program_counter, initial_pc + jsr_bytes + brk_bytes);
    }

    #[test]
    fn test_rti() {
        let rti = get_opcode_by_name_and_address_mode("RTI", AddressingMode::Implied).unwrap().code;
        let initial_stack_pointer = 0xFC;
        let initial_status = ProcessorStatus::from_u8(0b0000_0000);
        let return_status = ProcessorStatus::from_u8(0b1000_0011);
        let return_status_u8 = return_status.to_u8();
        let return_address_high = 0x1A;
        let return_address_low = 0xBC;
        let return_address = (return_address_high as u16) << 8 | return_address_low as u16;
        let program = vec![rti, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.stack_pointer = initial_stack_pointer;
        cpu.status = initial_status;
        cpu.write(0x01FD, return_status_u8 | 0b0001_0000).unwrap();
        cpu.write(0x01FE, return_address_low).unwrap();
        cpu.write(0x01FF, return_address_high as u8).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let brk_bytes = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().bytes as u16;
        assert_eq!(cpu.status, return_status);
        assert_eq!(cpu.program_counter, return_address + brk_bytes);
    }

    #[test]
    fn test_brk() {
        let code = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().code;
        let offset = 0x0200;
        let initial_pc = PRG_ROM_START + offset;
        let initial_stack_pointer = 0x05;
        let initial_status = ProcessorStatus::from_u8(0b1000_0001);
        let program = vec![code, 0, 0];
        let mut cpu = initialize_cpu(program);
        cpu.tests = false;
        cpu.program_counter = initial_pc;
        cpu.stack_pointer = initial_stack_pointer;
        cpu.status = initial_status;
        cpu.write(IRQ_VECTOR, 0xBC).unwrap();
        cpu.write(IRQ_VECTOR+1, 0x1A).unwrap();
        cpu.run(|_| Ok(())).unwrap();
        let stored_status = cpu.read(0x0103).unwrap();
        let stored_pc_low = cpu.read(0x0104).unwrap();
        let stored_pc_high = cpu.read(0x0105).unwrap();
        let stored_pc = (stored_pc_high as u16) << 8 | stored_pc_low as u16;
        assert_eq!(cpu.stack_pointer, 0x02);
        assert_eq!(cpu.program_counter, 0x1ABC);
        assert_eq!(stored_status, initial_status.to_u8() | 0b0001_0000);
        assert_eq!(stored_pc, initial_pc);
    }

    #[test]
    fn test_aac_positive() {
        let code = get_opcode_by_name_and_address_mode("AAC", AddressingMode::Immediate).unwrap().code;
        let param = 0b0000_0111;
        let accumulator = 0b1011_0110;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0b0000_0110);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_aac_negative() {
        let code = get_opcode_by_name_and_address_mode("AAC", AddressingMode::Immediate).unwrap().code;
        let param = 0xFF;
        let accumulator = 0x80;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a & param, accumulator);
        assert!(cpu.status.carry);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_aac_zero() {
        let code = get_opcode_by_name_and_address_mode("AAC", AddressingMode::Immediate).unwrap().code;
        let param = 0;
        let accumulator = 0b1011_0110;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_aax_positive() {
        let code = get_opcode_by_name_and_address_mode("AAX", AddressingMode::Absolute).unwrap().code;
        let address = 0x1ABC;
        let a = 0b1011_0110;
        let x = 0b0000_0111;
        let expected = 0b0000_0110;
        let program = vec![code, address as u8, (address >> 8) as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = a;
        cpu.register_x = x;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.read(address).unwrap(), expected);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_aax_negative() {
        let code = get_opcode_by_name_and_address_mode("AAX", AddressingMode::Absolute).unwrap().code;
        let address = 0x1ABC;
        let a = 0b1011_0110;
        let x = 0b1101_0111;
        let expected = 0b1001_0110;
        let program = vec![code, address as u8, (address >> 8) as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = a;
        cpu.register_x = x;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.read(address).unwrap(), expected);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_aax_zero() {
        let code = get_opcode_by_name_and_address_mode("AAX", AddressingMode::Absolute).unwrap().code;
        let address = 0x1ABC;
        let a = 0b1011_0110;
        let x = 0b0000_0000;
        let expected = 0;
        let program = vec![code, address as u8, (address >> 8) as u8, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = a;
        cpu.register_x = x;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.read(address).unwrap(), expected);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_arr_positive() {
        let code = get_opcode_by_name_and_address_mode("ARR", AddressingMode::Immediate).unwrap().code;
        let param = 0b1111_1111;
        let accumulator = 0b1110_1010;
        let expected = 0b0111_0101;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_arr_positive_2() {
        let code = get_opcode_by_name_and_address_mode("ARR", AddressingMode::Immediate).unwrap().code;
        let param = 0b1111_1111;
        let accumulator = 0b1010_1010;
        let expected = 0b0101_0101;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.carry);
        assert!(cpu.status.overflow);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_arr_negative() {
        let code = get_opcode_by_name_and_address_mode("ARR", AddressingMode::Immediate).unwrap().code;
        let param = 0b0100_0111;
        let accumulator = 0b0101_0101;
        let expected = 0b1010_0010;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.carry);
        assert!(cpu.status.overflow);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_arr_negative_2() {
        let code = get_opcode_by_name_and_address_mode("ARR", AddressingMode::Immediate).unwrap().code;
        let param = 0b1010_0111;
        let accumulator = 0b1001_0100;
        let expected = 0b1100_0010;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.status.carry = true;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.carry);
        assert!(cpu.status.overflow);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_arr_zero() {
        let code = get_opcode_by_name_and_address_mode("ARR", AddressingMode::Immediate).unwrap().code;
        let param = 0;
        let accumulator = 0b1011_0101;
        let expected = 0;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_asr_positive() {
        let code = get_opcode_by_name_and_address_mode("ASR", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b1011_0110;
        let param = 0b1001_0111;
        let expected = 0b0100_1011;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_asr_carry() {
        let code = get_opcode_by_name_and_address_mode("ASR", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b0011_0111;
        let param = 0b0010_0111;
        let expected = 0b0001_0011;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_asr_zero() {
        let code = get_opcode_by_name_and_address_mode("ASR", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b0011_0111;
        let param = 0;
        let expected = 0;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_atx_positive() {
        let code = get_opcode_by_name_and_address_mode("ATX", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b1011_0111;
        let param = 0b0010_0011;
        let expected = 0b0010_0011;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert_eq!(cpu.register_x, expected);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_atx_negative() {
        let code = get_opcode_by_name_and_address_mode("ATX", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b1001_0110;
        let param = 0b1111_0011;
        let expected = 0b1001_0010;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert_eq!(cpu.register_x, expected);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_atx_zero() {
        let code = get_opcode_by_name_and_address_mode("ATX", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b1001_0110;
        let param = 0;
        let expected = 0;
        let program = vec![code, param, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, expected);
        assert_eq!(cpu.register_x, expected);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_axa() {
        let code = get_opcode_by_name_and_address_mode("AXA", AddressingMode::AbsoluteY).unwrap().code;
        let accumulator = 0b1101_1111;
        let x = 0b1111_1011;
        let expected = 0b0000_0011;
        let address_high = 0x1A;
        let address_low = 0xBC;
        let address = (address_high as u16) << 8 | address_low as u16;
        let program = vec![code, address_low, address_high, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.register_x = x;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(cpu.register_a, accumulator);
        assert_eq!(cpu.register_x, x);
        assert_eq!(stored, expected);
    }

    #[test]
    fn test_axs_positive() {
        let code = get_opcode_by_name_and_address_mode("AXS", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b1111_1111;
        let x = 0b0011_1100;
        let subtract = 0b0000_1100;
        let expected = 0b0011_0000;
        let program = vec![code, subtract, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.register_x = x;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, accumulator);
        assert_eq!(cpu.register_x, expected);
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_axs_borrow() {
        let code = get_opcode_by_name_and_address_mode("AXS", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b1010_0000;
        let x = 0b0011_0000;
        let subtract = 0b0100_0000;
        let expected = 0b0001_0000;
        let program = vec![code, subtract, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.register_x = x;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, accumulator);
        assert_eq!(cpu.register_x, expected);
        assert!(cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_axs_negative() {
        let code = get_opcode_by_name_and_address_mode("AXS", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b1010_0001;
        let x = 0b1011_0001;
        let subtract = 0b0000_0001;
        let expected = 0b1010_0000;
        let program = vec![code, subtract, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.register_x = x;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, accumulator);
        assert_eq!(cpu.register_x, expected);
        assert!(!cpu.status.carry);
        assert!(cpu.status.negative);
        assert!(!cpu.status.zero);
    }

    #[test]
    fn test_axs_zero() {
        let code = get_opcode_by_name_and_address_mode("AXS", AddressingMode::Immediate).unwrap().code;
        let accumulator = 0b1010_0001;
        let x = 0b1011_0001;
        let subtract = 0b1010_0001;
        let expected = 0;
        let program = vec![code, subtract, 0];
        let mut cpu = initialize_cpu(program);
        cpu.register_a = accumulator;
        cpu.register_x = x;
        cpu.run(|_| Ok(())).unwrap();
        assert_eq!(cpu.register_a, accumulator);
        assert_eq!(cpu.register_x, expected);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.negative);
        assert!(cpu.status.zero);
    }

    #[test]
    fn test_dcp_carry() {
        let code = get_opcode_by_name_and_address_mode("DCP", AddressingMode::Absolute).unwrap().code;
        let value = 0b0000_0011;
        let accumulator = 0b0010_0010;
        let address_high = 0x1A;
        let address_low = 0xBC;
        let address = (address_high as u16) << 8 | address_low as u16;
        let program = vec![code, address_low, address_high, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, value.wrapping_sub(1));
        assert!(cpu.status.carry);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_dcp_zero() {
        let code = get_opcode_by_name_and_address_mode("DCP", AddressingMode::Absolute).unwrap().code;
        let value = 0b1010_0011;
        let accumulator = 0b1010_0010;
        let address_high = 0x1A;
        let address_low = 0xBC;
        let address = (address_high as u16) << 8 | address_low as u16;
        let program = vec![code, address_low, address_high, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, value.wrapping_sub(1));
        assert!(cpu.status.carry);
        assert!(cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_dcp_negative() {
        let code = get_opcode_by_name_and_address_mode("DCP", AddressingMode::Absolute).unwrap().code;
        let value = 0b1110_0011;
        let accumulator = 0b0000_0010;
        let address_high = 0x1A;
        let address_low = 0xBC;
        let address = (address_high as u16) << 8 | address_low as u16;
        let program = vec![code, address_low, address_high, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address, value).unwrap();
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address).unwrap();
        assert_eq!(stored, value.wrapping_sub(1));
        assert!(!cpu.status.carry);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_isc_no_flags_borrow() {
        let code = get_opcode_by_name_and_address_mode("ISC", AddressingMode::ZeroPage).unwrap().code;
        let accumulator = 8;
        let memory_value = 2;
        let address = 0x0A;
        let program = vec![code, address, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address as u16, memory_value).unwrap();
        cpu.status.carry = false;
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address as u16).unwrap();
        assert_eq!(stored, 3);
        assert_eq!(cpu.register_a, 4);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_isc_negative() {
        let code = get_opcode_by_name_and_address_mode("ISC", AddressingMode::ZeroPage).unwrap().code;
        let accumulator = 0;
        let memory_value = 0xFF;
        let address = 0x0A;
        let program = vec![code, address, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address as u16, memory_value).unwrap();
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address as u16).unwrap();
        assert_eq!(stored, 0);
        assert_eq!(cpu.register_a, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_isc_zero() {
        let code = get_opcode_by_name_and_address_mode("ISC", AddressingMode::ZeroPage).unwrap().code;
        let accumulator = 8;
        let memory_value = 7;
        let address = 0x0A;
        let program = vec![code, address, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address as u16, memory_value).unwrap();
        cpu.status.carry = true;
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address as u16).unwrap();
        assert_eq!(stored, 8);
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_isc_carry() {
        let code = get_opcode_by_name_and_address_mode("ISC", AddressingMode::ZeroPage).unwrap().code;
        let accumulator = 2;
        let memory_value = 1;
        let address = 0x0A;
        let program = vec![code, address, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address as u16, memory_value).unwrap();
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address as u16).unwrap();
        assert_eq!(stored, 2);
        assert_eq!(cpu.register_a, 0xFF);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(!cpu.status.carry);
        assert!(!cpu.status.overflow);
    }

    #[test]
    fn test_isc_overflow_negative() {
        let code = get_opcode_by_name_and_address_mode("ISC", AddressingMode::ZeroPage).unwrap().code;
        let accumulator = 0x88;
        let memory_value = 0x08;
        let address = 0x0A;
        let program = vec![code, address, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address as u16, memory_value as u8).unwrap();
        cpu.status.carry = true;
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address as u16).unwrap();
        assert_eq!(stored, 0x09);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
        assert!(cpu.status.overflow);
    }

    #[test]
    fn test_isc_overflow_positive() {
        let code = get_opcode_by_name_and_address_mode("ISC", AddressingMode::ZeroPage).unwrap().code;
        let accumulator = 0x78;
        let memory_value = 0xF7;
        let address = 0x0A;
        let program = vec![code, address, 0];
        let mut cpu = initialize_cpu(program);
        cpu.write(address as u16, memory_value as u8).unwrap();
        cpu.status.carry = true;
        cpu.register_a = accumulator;
        cpu.run(|_| Ok(())).unwrap();
        let stored = cpu.read(address as u16).unwrap();
        assert_eq!(stored, 0xF8);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
        assert!(cpu.status.overflow);
    }
}
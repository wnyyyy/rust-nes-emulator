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
        let test_addr = 0xB0;
        let test_value = 0x05;
        cpu.register_a = test_value;
        let code = get_opcode_by_name_and_address_mode("STA", AddressingMode::ZeroPage).unwrap().code;
        cpu.load_and_run(vec![code, test_addr, 0]).unwrap();
        let stored = cpu.memory.read(test_addr as u16).unwrap();
        assert_eq!(stored, test_value);
    }


    #[test]
    fn test_tax_positive() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0x05;
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_stx_stores_in_memory() {
        let mut cpu = initialize_cpu();
        let test_addr = 0xB0;
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
        let test_addr = 0xB0;
        let test_value = 0x05;
        cpu.register_y = test_value;
        let code = get_opcode_by_name_and_address_mode("STY", AddressingMode::ZeroPage).unwrap().code;
        cpu.load_and_run(vec![code, test_addr, 0]).unwrap();
        let stored = cpu.memory.read(test_addr as u16).unwrap();
        assert_eq!(stored, test_value);
    }

    #[test]
    fn test_tax_negative() {
        let mut cpu = initialize_cpu();
        cpu.register_a = 0x85;
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tax_zero() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("TAX", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0);
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
    fn test_brk_flag() {
        let mut cpu = initialize_cpu();
        let code = get_opcode_by_name_and_address_mode("BRK", AddressingMode::Implied).unwrap().code;
        cpu.load_and_run(vec![code, 0, 0]).unwrap();
        assert!(cpu.status.break_command);
    }
}
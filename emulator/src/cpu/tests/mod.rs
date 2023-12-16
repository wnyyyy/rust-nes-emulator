#[cfg(test)]
mod test {
    use crate::cpu::opcode::get_opcode_by_name;
    use super::super::*;

    #[test]
    fn test_lda_immediate_positive() {
        let mut cpu = CPU::new();
        let code = get_opcode_by_name("LDA").unwrap().code;
        cpu.interpret(vec![code, 0x05, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_lda_immediate_negative() {
        let mut cpu = CPU::new();
        let code = get_opcode_by_name("LDA").unwrap().code;
        cpu.interpret(vec![code, 0x85, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_lda_immediate_zero() {
        let mut cpu = CPU::new();
        let code = get_opcode_by_name("LDA").unwrap().code;
        cpu.interpret(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tax_positive() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        let code = get_opcode_by_name("TAX").unwrap().code;
        cpu.interpret(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tax_negative() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x85;
        let code = get_opcode_by_name("TAX").unwrap().code;
        cpu.interpret(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tax_zero() {
        let mut cpu = CPU::new();
        let code = get_opcode_by_name("TAX").unwrap().code;
        cpu.interpret(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_inx_positive() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x05;
        let code = get_opcode_by_name("INX").unwrap().code;
        cpu.interpret(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x06);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_inx_negative() {
        let mut cpu = CPU::new();
        let code = get_opcode_by_name("INX").unwrap().code;
        cpu.register_x = 0x7F;
        cpu.interpret(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x80);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_inx_zero() {
        let mut cpu = CPU::new();
        let code = get_opcode_by_name("INX").unwrap().code;
        cpu.register_x = 0xFF;
        cpu.interpret(vec![code, 0, 0]).unwrap();
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_brk_flag() {
        let mut cpu = CPU::new();
        let code = get_opcode_by_name("BRK").unwrap().code;
        cpu.interpret(vec![code, 0, 0]).unwrap();
        assert!(cpu.status.break_command);
    }
}
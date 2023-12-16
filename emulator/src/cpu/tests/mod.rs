#[cfg(test)]
mod test {
    use crate::common::opcode::Opcode::{LdaImmediate, Tax};
    use super::super::*;

    #[test]
    fn test_lda_immediate_positive() {
        let mut cpu = CPU::new();
        let lda = LdaImmediate as u8;
        cpu.interpret(vec![LdaImmediate as u8, 0x05, 0]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_lda_immediate_negative() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![LdaImmediate as u8, 0x85, 0]);
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_lda_immediate_zero() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![LdaImmediate as u8, 0, 0]);
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tax_positive() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x05;
        cpu.interpret(vec![Tax as u8, 0, 0]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.zero);
        assert!(!cpu.status.negative);
    }

    #[test]
    fn test_tax_negative() {
        let mut cpu = CPU::new();
        cpu.register_a = 0x85;
        cpu.interpret(vec![Tax as u8, 0, 0]);
        assert_eq!(cpu.register_a, 0x85);
        assert!(!cpu.status.zero);
        assert!(cpu.status.negative);
    }

    #[test]
    fn test_tax_zero() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![Tax as u8, 0, 0]);
        assert_eq!(cpu.register_a, 0);
        assert!(cpu.status.zero);
        assert!(!cpu.status.negative);
    }
}
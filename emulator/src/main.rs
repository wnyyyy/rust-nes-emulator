use emulator::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xa9, 0x05, 0x00]);
    cpu.interpret(vec![0xa9, 0x05, 0x00]);
}

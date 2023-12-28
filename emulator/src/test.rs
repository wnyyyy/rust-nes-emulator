#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;
    use crate::cpu::CPU;
    use crate::common::logger::trace;
    use crate::memory::memory::Memory;

    fn initialize_cpu() -> CPU {
        let file = File::open("../test roms/snake.nes");
        let mut rom_bytes = Vec::new();
        file.unwrap().read_to_end(&mut rom_bytes).unwrap();
        let mut cpu = CPU::new();
        cpu.status.interrupt_disable = true;
        cpu.load(&rom_bytes).unwrap();
        cpu
    }

    #[test]
    fn test_format_trace() {
        let mut cpu = initialize_cpu();
        cpu.write(100, 0xa2).unwrap();
        cpu.write(101, 0x01).unwrap();
        cpu.write(102, 0xca).unwrap();
        cpu.write(103, 0x88).unwrap();
        cpu.write(104, 0x00).unwrap();
        cpu.program_counter = 0x64;
        cpu.register_a = 1;
        cpu.register_x = 2;
        cpu.register_y = 3;
        let mut result: Vec<String> = vec![];
        cpu.run(|cpu| Ok({
            result.push(trace(cpu)?);
        })).unwrap();
        assert_eq!(
            "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
            result[0]
        );
        assert_eq!(
            "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
            result[1]
        );
        assert_eq!(
            "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
            result[2]
        );
    }

    #[test]
    fn test_format_mem_access() {
        let mut cpu = initialize_cpu();
        cpu.write(100, 0x11).unwrap();
        cpu.write(101, 0x33).unwrap();
        cpu.write(0x33, 00).unwrap();
        cpu.write(0x34, 04).unwrap();
        cpu.write(0x400, 0xAA).unwrap();
        cpu.program_counter = 0x64;
        cpu.register_y = 0;
        let mut result: Vec<String> = vec![];
        cpu.run(|cpu| Ok({
            result.push(trace(cpu)?);
        })).unwrap();
        assert_eq!(
            "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
            result[0]
        );
    }
}
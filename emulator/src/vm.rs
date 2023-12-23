use crate::cpu;

pub struct VM {
    pub cpu: cpu::CPU,
}

impl VM {
    pub fn new() -> VM {
        VM {
            cpu: cpu::CPU::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.cpu.load(rom).unwrap();
        self.cpu.reset();
        self.cpu.run(move |cpu| Ok({

        })).unwrap();
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessorStatus {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal_mode: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl ProcessorStatus {
    pub fn new() -> ProcessorStatus {
        ProcessorStatus {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal_mode: false,
            break_command: false,
            overflow: false,
            negative: false,
        }
    }

    pub fn to_u8(&self) -> u8 {
        let mut status = 0b0010_0000;
        if self.carry {
            status |= 0b0000_0001;
        }
        if self.zero {
            status |= 0b0000_0010;
        }
        if self.interrupt_disable {
            status |= 0b0000_0100;
        }
        if self.decimal_mode {
            status |= 0b0000_1000;
        }
        if self.break_command {
            status |= 0b0001_0000;
        }
        if self.overflow {
            status |= 0b0100_0000;
        }
        if self.negative {
            status |= 0b1000_0000;
        }
        status
    }

    pub fn from_u8(status: u8) -> ProcessorStatus {
        ProcessorStatus {
            carry: status & 0b0000_0001 != 0,
            zero: status & 0b0000_0010 != 0,
            interrupt_disable: status & 0b0000_0100 != 0,
            decimal_mode: status & 0b0000_1000 != 0,
            break_command: status & 0b0001_0000 != 0,
            overflow: status & 0b0100_0000 != 0,
            negative: status & 0b1000_0000 != 0,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AddressingMode {
    Implied,
    Relative,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
    Accumulator
}
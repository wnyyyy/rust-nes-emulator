pub struct ProcessorStatus {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
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
            break_command: false,
            overflow: false,
            negative: false,
        }
    }
}

#[derive(Debug, PartialEq)]
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
    Accumulator,
    None,
}
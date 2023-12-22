pub const RAM_SIZE: u16 = 2048;
pub const MEMORY_SIZE: usize = 65536;

pub const STACK_START: u16 = 0x0100;
pub const STACK_POINTER_INIT: u8 = 0xFF;
pub const IRQ_VECTOR: u16 = 0xFFFE;
pub const RAM_SIZE: usize = 2048;
pub const MEMORY_SIZE: usize = 65536;
pub const RAM_START: u16 = 0x0000;
pub const RAM_END: u16 = 0x1FFF;
pub const PPU_START: u16 = 0x2000;
pub const PPU_END: u16 = 0x3FFF;
pub const PRG_ROM_START: u16 = 0x8000;
pub const PRG_ROM_END: u16 = 0xFFFF;

pub const STACK_START: u16 = 0x0100;
pub const STACK_POINTER_INIT: u8 = 0xFF;
pub const IRQ_VECTOR: u16 = 0xFFFE;
pub const PC_START_ADDRESS: u16 = 0xFFFC;

pub static DEBUG: bool = true;
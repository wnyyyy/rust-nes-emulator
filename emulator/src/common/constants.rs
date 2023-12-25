use lazy_static::lazy_static;
use std::collections::HashMap;

pub const RAM_SIZE: usize = 2048;
pub const MEMORY_SIZE: usize = 65536;
pub const RAM_START: u16 = 0x0000;
pub const RAM_END: u16 = 0x1FFF;
pub const PPU_START: u16 = 0x2000;
pub const PPU_END: u16 = 0x3FFF;

pub const STACK_START: u16 = 0x0100;
pub const STACK_POINTER_INIT: u8 = 0xFF;
pub const IRQ_VECTOR: u16 = 0xFFFE;

pub static DEBUG: bool = true;


lazy_static! {
    pub static ref SNAKE_GAME_ALIAS: HashMap<u16, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0x606, "init");
        map.insert(0x60D, "initSnake");
        map.insert(0x62A, "generateApplePosition");
        map.insert(0x638, "loop");
        map.insert(0x64D, "readKeys");
        map.insert(0x68D, "checkCollision");
        map.insert(0x694, "checkAppleCollision");
        map.insert(0x6A8, "checkSnakeCollision");
        map.insert(0x6C3, "updateSnake");
        map.insert(0x719, "drawApple");
        map.insert(0x720, "drawSnake");
        map.insert(0x72D, "spinWheels");
        map.insert(0x735, "gameOver");
        map
        };
}
pub fn get_alias(address: u16) -> &'static str {
    SNAKE_GAME_ALIAS.get(&address).unwrap_or(&"")
}

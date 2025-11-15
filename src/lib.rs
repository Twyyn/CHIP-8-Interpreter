pub mod core;
use crate::core::{CPU, Memory, display::Display, keyboard::Keyboard};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CHIP8 {
    cpu: CPU,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
}
impl CHIP8 {
    fn default() -> Self {
        Self {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: Display::new(),
            keyboard: Keyboard::default(),
        }
    }
}

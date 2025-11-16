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
    fn new() -> CHIP8 {
        CHIP8 {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }
    pub fn run(&mut self) {
        self.display.window.set_target_fps(60);

        while self.display.window.is_open() {
            
        }
    }
}

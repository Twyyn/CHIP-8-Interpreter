pub mod core;

use crate::core::{Audio, CPU, Memory, display::Display, keyboard::Keyboard};
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[allow(non_snake_case)]
pub struct CHIP8 {
    cpu: CPU,
    memory: Memory,
    display: Display,
    audio: Audio,
    keyboard: Keyboard,
}
impl CHIP8 {
    pub fn new() -> CHIP8 {
        CHIP8 {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: Display::new(),
            audio: Audio::new(),
            keyboard: Keyboard::new(),
        }
    }
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
        self.display.clear();
        self.keyboard.reset();
    }
    pub fn load(&mut self, path: &OsString) -> Result<(), std::io::Error> {
        let path = Path::new(path);
        let mut file = File::open(path)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let _ = self.memory.load(&buffer).unwrap();
        Ok(())
    }
    fn fetch_and_execute(
        cpu: &mut CPU,
        memory: &mut Memory,
        display: &mut Display,
        keyboard: &mut Keyboard,
    ) {
        let instruction = cpu.fetch(memory);
        let _ = cpu.decode_execute(memory, display, keyboard, instruction);
    }

    pub fn run(mut self) {
        const CPU_CYCLES_PER_FRAME: usize = 12;

        while self.display.window_is_open() {
            /* Limit FPS to 60Hz */
            self.display.set_target_fps();
            /* Poll keyboard input */
            self.keyboard.update(&self.display);
            /* CPU cycles */
            for _ in 0..CPU_CYCLES_PER_FRAME {
                CHIP8::fetch_and_execute(
                    &mut self.cpu,
                    &mut self.memory,
                    &mut self.display,
                    &mut self.keyboard,
                );
            }
            /* Update sound and delay timers at 60Hz */
            self.cpu.update_timers(&mut self.audio);
            /* Draw and update to window */
            self.display.update();
        }
    }
}

pub mod cli;
pub mod emulator;

use crate::emulator::errors::EmuError;
use crate::emulator::{Audio, CPU, Memory, display::Display, keyboard::Keypad};

#[allow(non_snake_case)]
pub struct CHIP8 {
    cpu: CPU,
    memory: Memory,
    display: Display,
    audio: Audio,
    keypad: Keypad,
}
impl CHIP8 {
    pub fn new() -> CHIP8 {
        CHIP8 {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: Display::new(),
            audio: Audio::new(),
            keypad: Keypad::new(),
        }
    }
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
        self.display.clear();
        self.keypad.reset();
    }
    pub fn load<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), EmuError> {
        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        use std::io::Read;
        file.read_to_end(&mut buffer)?;
        self.memory.load(&buffer)?;
        Ok(())
    }
    fn fetch_and_execute(
        cpu: &mut CPU,
        memory: &mut Memory,
        display: &mut Display,
        keyboard: &mut Keypad,
    ) -> Result<(), EmuError> {
        let instruction = cpu.fetch(memory);
        cpu.decode_execute(memory, display, keyboard, instruction)?;
        Ok(())
    }

    pub fn run(mut self) -> Result<(), EmuError> {
        const CPU_CYCLES_PER_FRAME: usize = 12;

        while self.display.window_is_open() {
            /* Limit FPS to 60Hz */
            self.display.set_target_fps();
            /* Poll keyboard input */
            self.keypad.update(&self.display);
            /* CPU cycles */
            for _ in 0..CPU_CYCLES_PER_FRAME {
                CHIP8::fetch_and_execute(
                    &mut self.cpu,
                    &mut self.memory,
                    &mut self.display,
                    &mut self.keypad,
                )?;
            }
            /* Update sound and delay timers at 60Hz */
            self.cpu.update_timers(&mut self.audio);
            /* Draw and update to window */
            self.display.update();
        }
        Ok(())
    }
}

pub mod core;
use crate::core::{Audio, CPU, CycleError, Memory, display::Display, keyboard::Keyboard};
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
    pub fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        let path = Path::new(path);
        let mut file = File::open(path)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let _ = self.memory.load(&buffer).unwrap();
        Ok(())
    }
    fn cycle(&mut self) -> Result<(), CycleError> {
        /* Fetch Instruction*/
        let instruction = self.cpu.fetch(&self.memory);
        /*  */
        /* Decode & Execute Instruction */
        let decode_execute = self.cpu.decode_execute(
            &mut self.memory,
            &mut self.display,
            &mut self.keyboard,
            instruction,
        );
        decode_execute.map_err(|_| CycleError::EmuCycleError)?;
        /* Update Sound and Delay Timers */
        let _ = self.cpu.update_timers(&mut self.audio);
        Ok(())
    }
    pub fn run(mut self) {
        while self.display.window_is_open() {
            /* FPS = 60Hz */
            self.display.set_target_fps();
            /* 60 Cycles per second  */
            self.cycle().unwrap();
            println!("{:?}", self.keyboard);
            /* Update display with each cycle */
            self.display.update();
        }
    }
}

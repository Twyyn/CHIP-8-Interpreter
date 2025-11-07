use crate::memory;

const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGS: usize = 16;
const START_ADDR: usize = 0x200;
const FONTSET_SIZE: usize = 80;
pub const FONT_BASE_ADDR: usize = 0x050;
pub const GLYPH_BYTES: usize = 5;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Memory {
    pub STACK: [u16; STACK_SIZE],
    pub RAM: [u8; RAM_SIZE],
    pub V: [u8; NUM_REGS],
    pub I: u16,
    pub S_TIMER: u8,
    pub D_TIMER: u8,
    pub PROGRAM_COUNTER: u16,
    pub STACK_POINTER: u8,
}
impl Default for Memory {
    fn default() -> Self {
        Self {
            STACK: [0; STACK_SIZE],
            RAM: [0; RAM_SIZE],
            V: [0; NUM_REGS],
            I: 0,
            S_TIMER: 0,
            D_TIMER: 0,
            PROGRAM_COUNTER: START_ADDR as u16,
            STACK_POINTER: 0,
        }
    }
}
impl Memory {
    pub fn new() -> Self {
        let mut memory = Self::default();
        // Load FONT in RAM
        memory.RAM[FONT_BASE_ADDR..FONT_BASE_ADDR + FONTSET_SIZE].copy_from_slice(&FONTSET);
        memory
    }
    pub fn load(&mut self, rom: &[u8; RAM_SIZE]) {
        match Some(rom.len()) {
            Some(length) => self.RAM[START_ADDR..START_ADDR + length].copy_from_slice(rom),
            _ => {}
        }
    }
    pub fn stack_pop(&mut self) -> u16 {
        self.STACK_POINTER -= 1;
        self.STACK[self.STACK_POINTER as usize]
    }
    pub fn stack_push(&mut self, value: u16) {
        self.STACK[self.STACK_POINTER as usize] = value;
        self.STACK_POINTER += 1;
    }
}

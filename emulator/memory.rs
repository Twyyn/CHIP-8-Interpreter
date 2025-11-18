use crate::emulator::MemoryError;

/* Public */
pub const FONT_BASE_ADDR: usize = 0x050;
pub const START_ADDR: usize = 0x200;

const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, /* 0 */
    0x20, 0x60, 0x20, 0x20, 0x70, /* 1 */
    0xF0, 0x10, 0xF0, 0x80, 0xF0, /* 2 */
    0xF0, 0x10, 0xF0, 0x10, 0xF0, /* 3 */
    0x90, 0x90, 0xF0, 0x10, 0x10, /* 4 */
    0xF0, 0x80, 0xF0, 0x10, 0xF0, /* 5 */
    0xF0, 0x80, 0xF0, 0x90, 0xF0, /* 6 */
    0xF0, 0x10, 0x20, 0x40, 0x40, /* 7 */
    0xF0, 0x90, 0xF0, 0x90, 0xF0, /* 8 */
    0xF0, 0x90, 0xF0, 0x10, 0xF0, /* 9 */
    0xF0, 0x90, 0xF0, 0x90, 0x90, /* A */
    0xE0, 0x90, 0xE0, 0x90, 0xE0, /* B */
    0xF0, 0x80, 0x80, 0x80, 0xF0, /* C */
    0xE0, 0x90, 0x90, 0x90, 0xE0, /* D */
    0xF0, 0x80, 0xF0, 0x80, 0xF0, /* E */
    0xF0, 0x80, 0xF0, 0x80, 0x80, /* F */
];

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Memory {
    pub STACK: Vec<u16>,
    pub RAM: [u8; RAM_SIZE],
}
impl Default for Memory {
    fn default() -> Memory {
        Memory {
            STACK: Vec::with_capacity(STACK_SIZE),
            RAM: [0; RAM_SIZE],
        }
    }
}

#[allow(non_snake_case)]
impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            ..Default::default()
        };
        /* Load Font in RAM */
        memory.RAM[FONT_BASE_ADDR..FONT_BASE_ADDR + FONTSET_SIZE].copy_from_slice(&FONTSET);
        memory
    }
    /* Reset memory to default */
    pub fn reset(&mut self) {
        *self = Memory::default();
    }
    /* Load ROM into RAM */
    pub fn load(&mut self, data: &[u8]) -> Result<(), MemoryError> {
        let data_len = data.len();
        if START_ADDR + data_len > self.RAM.len() {
            return Err(MemoryError::ROMLoadError);
        }
        self.RAM[START_ADDR..START_ADDR + data_len].copy_from_slice(data);
        Ok(())
    }
    /* Removes and returns top item from stack */
    pub fn stack_pop(&mut self) -> Result<u16, MemoryError> {
        if self.STACK.is_empty() {
            return Err(MemoryError::StackUnderflow);
        }
        Ok(self.STACK.pop().unwrap())
    }
    /* Adds item to top of stack *if possible*, then returns it's index */
    pub fn stack_push(&mut self, value: u16) -> Result<u8, MemoryError> {
        if self.STACK.len() >= STACK_SIZE {
            return Err(MemoryError::StackOverflow);
        }
        self.STACK.push(value);
        Ok(self.STACK.iter().position(|&x| x == value).unwrap() as u8)
    }
    /* Retuns length of stack */
    pub fn stack_len(&self) -> u8 {
        self.STACK.len() as u8
    }
}

use crate::core::MemoryError;

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
    pub STACK: [u16; STACK_SIZE],
    pub RAM: [u8; RAM_SIZE],
    pub STACK_POINTER: u8,
}
impl Default for Memory {
    fn default() -> Self {
        Self {
            STACK: [0; STACK_SIZE],
            RAM: [0; RAM_SIZE],
            STACK_POINTER: 0,
        }
    }
}
impl Memory {
    pub fn new(stack_pointer: u8) -> Memory {
        let mut memory = Memory {
            STACK_POINTER: stack_pointer,
            ..Memory::default()
        };
        // Load Font in RAM
        memory.RAM[FONT_BASE_ADDR..FONT_BASE_ADDR + FONTSET_SIZE].copy_from_slice(&FONTSET);
        memory
    }
    pub fn load_rom(&mut self, data: &[u8]) -> Result<(), MemoryError> {
        use hex;
        match hex::decode(data) {
            Ok(rom) => {
                self.RAM[START_ADDR..START_ADDR + rom.len()].copy_from_slice(&rom);
                rom
            }
            Err(_) => return Err(MemoryError::LoadError),
        };
        Ok(())
    }
    pub fn stack_pop(&mut self) -> Result<u16, MemoryError> {
        if self.STACK_POINTER == 0 {
            return Err(MemoryError::StackUnderflow);
        }
        self.STACK_POINTER -= 1;
        Ok(self.STACK[self.STACK_POINTER as usize])
    }
    pub fn stack_push(&mut self, value: u16) -> Result<(), MemoryError> {
        if self.STACK_POINTER + 1 <= STACK_SIZE as u8 {
            self.STACK_POINTER += 1;
            self.STACK[self.STACK_POINTER as usize] = value;
        } else {
            return Err(MemoryError::StackOverflow);
        }
        Ok(())
    }
}

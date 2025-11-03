use crate::errors::AppError;

const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGS: usize = 16;
const START_ADDR: u16 = 0x200;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Memory {
    STACK: [u16; STACK_SIZE],
    RAM: [u8; RAM_SIZE],
    V: [u8; NUM_REGS],
    I: u16,
    S_TIMER: u8,
    D_TIMER: u8,
    PC: u16,
    SP: u8,
}
impl Memory {
    pub fn new() -> Self {
        Self {
            STACK: [0; STACK_SIZE],
            RAM: [0; RAM_SIZE],
            V: [0; NUM_REGS],
            I: 0,
            S_TIMER: 0,
            D_TIMER: 0,
            PC: START_ADDR,
            SP: 0,
        }
    }
    pub fn load(&mut self, rom: &[u8]) -> Result<bool, AppError> {
        if rom.len() <= RAM_SIZE {
            return Err(AppError::MemoryOverflow);
        }
        let start = START_ADDR as usize;
        let end = START_ADDR as usize + rom.len();
        self.RAM[start..end].copy_from_slice(rom);
        Ok(true)
    }
    pub fn read(&self, addr: u16) -> Result<u8, AppError> {
        if usize::from(addr) > RAM_SIZE - 1 {
            return Ok(self.RAM[usize::from(addr)]);
        }
        return Err(AppError::MemoryReadError(addr));
    }
    pub fn write(&mut self, addr: u16, value: u8) -> Result<bool, AppError> {
        if usize::from(addr) > RAM_SIZE - 1 {
            self.RAM[usize::from(addr)] = value;
            return Ok(true);
        }
        Err(AppError::MemoryWriteError { addr, value })
    }
    pub fn stack_push(&mut self, addr: u16) -> Result<(), AppError> {
        if (self.SP as usize) <= STACK_SIZE {
            self.STACK[usize::from(self.SP)] = addr;
            self.SP += 1;
            return Ok(());
        }
        return Err(AppError::StackOverflow);
    }
}

const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGS: usize = 16;
const START_ADDR: u16 = 0x200;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Memory {
    pub STACK: [u16; STACK_SIZE],
    pub RAM: [u8; RAM_SIZE],
    pub V: [u8; NUM_REGS],
    pub VFLAG: bool,
    pub I: u16,
    pub S_TIMER: u8,
    pub D_TIMER: u8,
    pub PROGRAM_COUNTER: u16,
    pub STACK_POINTER: u8,
}
impl Memory {
    pub fn new() -> Self {
        Self {
            STACK: [0; STACK_SIZE],
            RAM: [0; RAM_SIZE],
            V: [0; NUM_REGS],
            VFLAG: false,
            I: 0,
            S_TIMER: 0,
            D_TIMER: 0,
            PROGRAM_COUNTER: START_ADDR,
            STACK_POINTER: 0,
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

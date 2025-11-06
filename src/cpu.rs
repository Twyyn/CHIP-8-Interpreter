use core::borrow;

use crate::memory::Memory;
use crate::opcodes::Opcode;

struct CPU {
    memory: Memory,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
        }
    }
    pub fn execute(&mut self, op: u16) {
        let instr = match Opcode::try_from(op).unwrap() {
            /* 00E0 - Clear the Display  */
            Opcode::CLEAR => {
                todo!()
            }
            Opcode::RETURN => {
                /* 00EE - Returns from subroutine. PC = Address popped from STACK */
                self.memory.PROGRAM_COUNTER = self.memory.stack_pop()
            }
            Opcode::JUMP { nnn } => {
                /* 1NNN - Jump (goto) address NNN */
                self.memory.PROGRAM_COUNTER = nnn
            }
            Opcode::CALL { nnn } => {
                /* 2NNN - Call subroutine at address NNN. Push current PC Address to STACK, then PC = NNN */
                self.memory.stack_push(self.memory.PROGRAM_COUNTER);
                self.memory.PROGRAM_COUNTER = nnn
            }
            Opcode::SE_Vx_NN { x, nn } => {
                /* 3XNN - Skip next instruction (PC += 2), if V[x] == NN */
                if self.memory.V[x as usize] == nn {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Opcode::SNE_Vx_NN { x, nn } => {
                /* 4XNN - Skip next instruction (PC += 2), if V[x] != NN */
                if self.memory.V[x as usize] != nn {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Opcode::SE_Vx_Vy { x, y } => {
                /* 5XY0 - Skip next instruction (PC += 2), if V[x] != V[Y] */
                if self.memory.V[x as usize] == self.memory.V[y as usize] {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Opcode::LOAD_Vx_NN { x, nn } => self.memory.RAM[x as usize] = nn,
            Opcode::ADD_Vx_NN { x, nn } => {
                self.memory.V[x as usize] = self.memory.V[x as usize].wrapping_add(nn)
            }
            Opcode::LOAD_Vx_Vy { x, y } => self.memory.V[x as usize] = self.memory.V[y as usize],
            Opcode::OR_Vx_Vy { x, y } => {
                todo!()
            }
            Opcode::AND_Vx_Vy { x, y } => {
                todo!()
            }
            Opcode::XOR_Vx_Vy { x, y } => {
                todo!()
            }
            Opcode::ADD_Vx_Vy { x, y } => {
                let (sum, carry) =
                    self.memory.V[x as usize].overflowing_add(self.memory.V[y as usize]);
                self.memory.V[x as usize] = sum;
                self.memory.V[0xF] = u8::from(carry);
            }
            Opcode::SUB_Vx_Vy { x, y } => {
                let (difference, borrow) =
                    self.memory.V[x as usize].overflowing_sub(self.memory.V[y as usize]);
                self.memory.V[0xF] = u8::from(!borrow);
                self.memory.V[x as usize] = difference;
            }
            Opcode::SHR_Vx_Vy { x, y } => {
                self.memory.V[0xF] = self.memory.V[y as usize] & 0x1;
                self.memory.V[x as usize] = self.memory.V[y as usize] >> 1;
            }
            Opcode::SUBN_Vx_Vy { x, y } => {
                let (difference, borrow) =
                    self.memory.V[y as usize].overflowing_sub(self.memory.V[x as usize]);
                self.memory.V[0xF] = u8::from(!borrow);
                self.memory.V[x as usize] = difference;
            }
            Opcode::SHL_Vx_Vy { x, y } => {
                self.memory.V[0xF] = (self.memory.V[y as usize] << (u8::BITS - 1)) & 1;
                self.memory.V[x as usize] = self.memory.V[y as usize] >> 1;
            }
        };
    }
}

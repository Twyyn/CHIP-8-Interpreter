use crate::core::errors::OpcodeError;
use crate::core::memory::{FONT_BASE_ADDR, GLYPH_BYTES, Memory};
use crate::core::mnemonics::Mnemonics;
//use crate::keyboard::Keyboard;

pub struct Instruction;
impl Instruction {}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CPU {
    memory: Memory,
}
impl CPU {
    pub fn new(memory: Memory) -> Self {
        Self { memory }
    }
    pub fn fetch(&self) -> u16 {
        ((self.memory.RAM[self.memory.PROGRAM_COUNTER as usize] as u16) << 8)
            | (self.memory.RAM[self.memory.PROGRAM_COUNTER as usize + 1] as u16)
    }
    pub fn execute(&mut self, opcode: u16) -> Result<(), OpcodeError> {
        let _ = match Mnemonics::try_from(opcode).unwrap() {
            /* 00E0 - Clear the Display  */
            Mnemonics::CLEAR => {
                todo!()
            }
            Mnemonics::RETURN => {
                /* 00EE - Returns from subroutine. PC = Address popped from STACK */
                self.memory.PROGRAM_COUNTER = self.memory.stack_pop().unwrap();
            }
            Mnemonics::JUMP { nnn } => {
                /* 1NNN - Jump (goto) address NNN */
                self.memory.PROGRAM_COUNTER = nnn
            }
            Mnemonics::CALL { nnn } => {
                /* 2NNN - Call subroutine at address NNN. Push current PC Address to STACK, then PC = NNN */
                self.memory.stack_push(self.memory.PROGRAM_COUNTER);
                self.memory.PROGRAM_COUNTER = nnn
            }
            Mnemonics::SE_Vx_NN { x, nn } => {
                /* 3XNN - Skip next instruction (PC += 2), if V[x] == NN */
                if self.memory.V[x as usize] == nn {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Mnemonics::SNE_Vx_NN { x, nn } => {
                /* 4XNN - Skip next instruction (PC += 2), if V[x] != NN */
                if self.memory.V[x as usize] != nn {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Mnemonics::SE_Vx_Vy { x, y } => {
                /* 5XY0 - Skip next instruction (PC += 2), if V[x] != V[Y] */
                if self.memory.V[x as usize] == self.memory.V[y as usize] {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Mnemonics::LOAD_Vx_NN { x, nn } => {
                /* 6XNN - V[x] = NN*/
                self.memory.V[x as usize] = nn
            }
            Mnemonics::ADD_Vx_NN { x, nn } => {
                /* 7XNN - V[x] = V[x] + nn */
                self.memory.V[x as usize] = self.memory.V[x as usize].wrapping_add(nn);
            }
            Mnemonics::LOAD_Vx_Vy { x, y } => {
                /* 8XY0 - V[x] = V[y] */
                self.memory.V[x as usize] = self.memory.V[y as usize];
            }
            Mnemonics::OR_Vx_Vy { x, y } => {
                /* 8XY1 - V[x] = V[x] |(OR) V[y] */
                self.memory.V[x as usize] = self.memory.V[x as usize] | self.memory.V[y as usize];
            }
            Mnemonics::AND_Vx_Vy { x, y } => {
                /* 8XY2 - V[x] = V[x] &(AND) V[y] */
                self.memory.V[x as usize] = self.memory.V[x as usize] & self.memory.V[y as usize];
            }
            Mnemonics::XOR_Vx_Vy { x, y } => {
                /* 8XY3 - V[x] = V[x] ^(XOR) V[y] */
                self.memory.V[x as usize] = self.memory.V[x as usize] ^ self.memory.V[y as usize];
            }
            Mnemonics::ADD_Vx_Vy { x, y } => {
                /* 8XY4 - V[x] = V[x] + [Vy](sum), set V[0xF] = Carry  */
                let (sum, carry) =
                    self.memory.V[x as usize].overflowing_add(self.memory.V[y as usize]);
                self.memory.V[x as usize] = sum;
                self.memory.V[0xF] = u8::from(carry);
            }
            Mnemonics::SUB_Vx_Vy { x, y } => {
                /* 8XY5 - V[x] = V[x] - V[y](difference), V[0xF] = (NOT) !borrow */
                let (difference, borrow) =
                    self.memory.V[x as usize].overflowing_sub(self.memory.V[y as usize]);
                self.memory.V[0xF] = u8::from(!borrow);
                self.memory.V[x as usize] = difference;
            }
            Mnemonics::SHR_Vx_Vy { x, y } => {
                /* 8XY6 - V[x] = V[y] >> (Shift Right) 1, then V[0xF] = LSB */
                let (result, carry) = (
                    self.memory.V[y as usize] & 0x1,
                    self.memory.V[y as usize] >> 1,
                );
                self.memory.V[x as usize] = result;
                self.memory.V[0xF] = carry;
            }
            Mnemonics::SUBN_Vx_Vy { x, y } => {
                /* 8XY7 - V[x] = V[y] - V[x](differnce), then V[0xF] = (NOT)!borrow */
                let (difference, borrow) =
                    self.memory.V[y as usize].overflowing_sub(self.memory.V[x as usize]);
                self.memory.V[0xF] = u8::from(!borrow);
                self.memory.V[x as usize] = difference;
            }
            Mnemonics::SHL_Vx_Vy { x, y } => {
                /* 8XYE -  V[x] = V[x] << (Shift Left) 1, then V[0xF] = MSB */
                let (result, carry) = (
                    (self.memory.V[y as usize] >> 7) & 0x01,
                    self.memory.V[y as usize] << 1,
                );
                self.memory.V[x as usize] = result;
                self.memory.V[0xF] = carry;
            }
            Mnemonics::SNE_Vx_Vy { x, y } => {
                /* 9XY0 - Skip next instruction (PC += 2) if V[x] (NOT) != V[y] */
                if self.memory.V[x as usize] != self.memory.V[y as usize] {
                    self.memory.PROGRAM_COUNTER += 2;
                }
            }
            Mnemonics::LOAD_I_NNN { nnn } => {
                /* ANNN - Load, [I] = NNN */
                self.memory.I = nnn;
            }
            Mnemonics::JUMP_V0_NNN { nnn } => {
                /* BNNN - Jump(goto) address V[0x0] + NNN */
                self.memory.PROGRAM_COUNTER = u16::from(self.memory.V[0x0]) + nnn
            }
            Mnemonics::RAND { x, nn } => {
                /* CXNN - V[x] = (Random u8 Byte) &&(AND) NN */
                use rand::Rng;
                let random_byte: u8 = rand::rng().random();
                self.memory.V[x as usize] = random_byte & nn;
            }
            Mnemonics::DRAW { x, y, n } => {
                /* DRAW -  Display N sprite, starting at [I] at (V[x], V[y]), then V[0xF] = collision */
                todo!()
            }
            Mnemonics::SKP_Vx { x } => {
                /* EX9E - Skip next instruction (PC += 2) if [KEY] == V[x] is pressed */
                // if let Ok(KeyState::PRESSED) =
                //     self.keyboard.get_key_state(self.memory.V[x as usize])
                // {
                //     self.memory.PROGRAM_COUNTER = self.memory.PROGRAM_COUNTER.wrapping_add(2);
                // }
                todo!()
            }
            Mnemonics::SKNP_Vx { x } => {
                /* EXA1 - Skip next instruction (PC += 2) if [KEY] == V[x] is NOT pressed */
                // if let Ok(KeyState::NOTPRESSED) =
                //     self.keyboard.get_key_state(self.memory.V[x as usize])
                // {
                //     self.memory.PROGRAM_COUNTER = self.memory.PROGRAM_COUNTER.wrapping_add(2);
                // }
                todo!()
            }
            Mnemonics::LOAD_Vx_DT { x } => {
                /* FX07 - Load, V[x] = [DELAY_TIMER] */
                self.memory.V[x as usize] = self.memory.D_TIMER;
            }
            Mnemonics::LOAD_Vx_K { x } => {
                /* FX0A - Wait.. for [KEY] pressed then V[x] = [KEY] */
                // self.keyboard.KEY_WAITING = true;
                // self.memory.V[x as usize] =
                todo!()
            }
            Mnemonics::LOAD_DT_Vx { x } => {
                /* FX15 - Load, [DELAY_TIMER] = V[x] */
                self.memory.D_TIMER = self.memory.V[x as usize];
            }
            Mnemonics::LOAD_ST_Vx { x } => {
                /* FX18 - Load, [SOUND_TIMER] = V[x] */
                self.memory.S_TIMER = self.memory.V[x as usize];
            }
            Mnemonics::ADD_I_Vx { x } => {
                /* FX1E - [I] = [I] = V[x] */
                self.memory.I = self
                    .memory
                    .I
                    .wrapping_add(u16::from(self.memory.V[x as usize]));
            }
            Mnemonics::LOAD_FONT { x } => {
                /* FX29 - [I] = address of sprite for digit V[x] */
                let digit = (self.memory.V[x as usize] & 0x0F) as usize;
                self.memory.I = (FONT_BASE_ADDR + digit * GLYPH_BYTES) as u16;
            }
            Mnemonics::LOAD_B_Vx { x } => {
                /* FX33 - Load, [I], [I + 1] and [I + 2]  = V[x] (as Binary) */
                self.memory.RAM[self.memory.I as usize] = self.memory.V[x as usize] / 100;
                self.memory.RAM[self.memory.I as usize + 1] = (self.memory.V[x as usize] / 10) % 10;
                self.memory.RAM[self.memory.I as usize + 2] = self.memory.V[x as usize] % 10;
            }
            Mnemonics::LOAD_I_Vx { x } => {
                /* FX55 - RAM[I] .. RAM[Ix] = V[0x0] .. V[x] */
                for idx in 0..=x as usize {
                    self.memory.RAM[self.memory.I as usize + idx] = self.memory.V[idx];
                }
            }
            Mnemonics::LOAD_Vx_I { x } => {
                /* FX65 - V[0x0] .. V[x] = RAM[I] .. RAM[Ix] */
                for idx in 0..=x as usize {
                    self.memory.V[idx] = self.memory.RAM[self.memory.I as usize + idx];
                }
            }
            Mnemonics::OpCodeError { op } => {
                /* Unknown Opcode  */
                return Err(OpcodeError::UnknownMnemonic(op));
            }
        };
        Ok(())
    }
}

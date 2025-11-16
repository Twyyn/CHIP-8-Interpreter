use crate::core::Audio;
use crate::core::display::{Display, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::core::errors::OpcodeError;
use crate::core::keyboard::Keyboard;
use crate::core::memory::{FONT_BASE_ADDR, Memory, START_ADDR};
use crate::core::mnemonics::Mnemonics;

const GLYPH_BYTES: usize = 5;
const NUM_V_REGS: usize = 16;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CPU {
    pub V: [u8; NUM_V_REGS],
    pub I: u16,
    pub S_TIMER: u8,
    pub D_TIMER: u8,
    pub PROGRAM_COUNTER: u16,
    pub STACK_POINTER: u8,
}
impl Default for CPU {
    fn default() -> CPU {
        CPU {
            V: [0; NUM_V_REGS],
            I: 0,
            S_TIMER: 0,
            D_TIMER: 0,
            PROGRAM_COUNTER: START_ADDR as u16,
            STACK_POINTER: 0,
        }
    }
}
#[allow(non_snake_case)]
impl CPU {
    pub fn new() -> CPU {
        CPU {
            ..Default::default()
        }
    }
    pub fn reset(&mut self) {
        self::CPU::default();
    }
    pub fn update_timers(&mut self, audio: &mut Audio) {
        if self.D_TIMER > 0 {
            self.D_TIMER -= 1;
        }
        if self.S_TIMER > 0 {
            if self.S_TIMER == 1 {
                audio.play_beep()
            } else {
                audio.stop_beep()
            }
            self.S_TIMER -= 1;
        }
    }
    fn increment_PC(&mut self) {
        self.PROGRAM_COUNTER += 2
    }
    pub fn fetch(&mut self, memory: &Memory) -> u16 {
        let opcode = ((memory.RAM[self.PROGRAM_COUNTER as usize] as u16) << 8)
            | (memory.RAM[self.PROGRAM_COUNTER as usize + 1] as u16);
        self.increment_PC();
        opcode
    }
    pub fn decode_execute(
        &mut self,
        memory: &mut Memory,
        display: &mut Display,
        keyboard: &mut Keyboard,
        instr: u16,
    ) -> Result<(), OpcodeError> {
        /* Decode & Execute */
        let _ = match Mnemonics::try_from(instr).unwrap() {
            /* 00E0 - Clear the Display  */
            Mnemonics::CLEAR => {
                let _ = &display.clear();
            }
            Mnemonics::RETURN => {
                /* 00EE - Returns from subroutine. PC = Address popped from STACK */
                self.PROGRAM_COUNTER = memory.stack_pop().unwrap();
                self.STACK_POINTER = memory.stack_len();
            }
            Mnemonics::JUMP { nnn } => {
                /* 1NNN - Jump (goto) address NNN */
                self.PROGRAM_COUNTER = nnn
            }
            Mnemonics::CALL { nnn } => {
                /* 2NNN - Call subroutine at address NNN. Push current PC Address to STACK, then PC = NNN */
                memory.stack_push(self.PROGRAM_COUNTER).unwrap();
                self.STACK_POINTER = memory.stack_len() + 1;
                self.PROGRAM_COUNTER = nnn
            }
            Mnemonics::SE_Vx_NN { x, nn } => {
                /* 3XNN - Skip next instruction (PC += 2), if V[x] == NN */
                if self.V[x as usize] == nn {
                    self.increment_PC();
                }
            }
            Mnemonics::SNE_Vx_NN { x, nn } => {
                /* 4XNN - Skip next instruction (PC += 2), if V[x] != NN */
                if self.V[x as usize] != nn {
                    self.increment_PC();
                }
            }
            Mnemonics::SE_Vx_Vy { x, y } => {
                /* 5XY0 - Skip next instruction (PC += 2), if V[x] != V[Y] */
                if self.V[x as usize] == self.V[y as usize] {
                    self.increment_PC();
                }
            }
            Mnemonics::LOAD_Vx_NN { x, nn } => {
                /* 6XNN - V[x] = NN */
                self.V[x as usize] = nn
            }
            Mnemonics::ADD_Vx_NN { x, nn } => {
                /* 7XNN - V[x] = V[x] + nn */
                self.V[x as usize] = self.V[x as usize].wrapping_add(nn);
            }
            Mnemonics::LOAD_Vx_Vy { x, y } => {
                /* 8XY0 - V[x] = V[y] */
                self.V[x as usize] = self.V[y as usize];
            }
            Mnemonics::OR_Vx_Vy { x, y } => {
                /* 8XY1 - V[x] = V[x] |(OR) V[y] */
                self.V[x as usize] = self.V[x as usize] | self.V[y as usize];
            }
            Mnemonics::AND_Vx_Vy { x, y } => {
                /* 8XY2 - V[x] = V[x] &(AND) V[y] */
                self.V[x as usize] = self.V[x as usize] & self.V[y as usize];
            }
            Mnemonics::XOR_Vx_Vy { x, y } => {
                /* 8XY3 - V[x] = V[x] ^(XOR) V[y] */
                self.V[x as usize] = self.V[x as usize] ^ self.V[y as usize];
            }
            Mnemonics::ADD_Vx_Vy { x, y } => {
                /* 8XY4 - V[x] = V[x] + [Vy](sum), set V[0xF] = Carry  */
                let (sum, carry) = self.V[x as usize].overflowing_add(self.V[y as usize]);
                self.V[x as usize] = sum;
                self.V[0xF] = u8::from(carry);
            }
            Mnemonics::SUB_Vx_Vy { x, y } => {
                /* 8XY5 - V[x] = V[x] - V[y](difference), V[0xF] = (NOT) !borrow */
                let (difference, borrow) = self.V[x as usize].overflowing_sub(self.V[y as usize]);
                self.V[0xF] = u8::from(!borrow);
                self.V[x as usize] = difference;
            }
            Mnemonics::SHR_Vx_Vy { x, y } => {
                /* 8XY6 - V[x] = V[y] >> (Shift Right) 1, then V[0xF] = LSB */
                let (result, carry) = (self.V[y as usize] & 0x1, self.V[y as usize] >> 1);
                self.V[x as usize] = result;
                self.V[0xF] = carry;
            }
            Mnemonics::SUBN_Vx_Vy { x, y } => {
                /* 8XY7 - V[x] = V[y] - V[x](differnce), then V[0xF] = (NOT)!borrow */
                let (difference, borrow) = self.V[y as usize].overflowing_sub(self.V[x as usize]);
                self.V[0xF] = u8::from(!borrow);
                self.V[x as usize] = difference;
            }
            Mnemonics::SHL_Vx_Vy { x, y } => {
                /* 8XYE -  V[x] = V[x] << (Shift Left) 1, then V[0xF] = MSB */
                let (result, carry) = ((self.V[y as usize] >> 7) & 0x01, self.V[y as usize] << 1);
                self.V[x as usize] = result;
                self.V[0xF] = carry;
            }
            Mnemonics::SNE_Vx_Vy { x, y } => {
                /* 9XY0 - Skip next instruction (PC += 2) if V[x] (NOT) != V[y] */
                if self.V[x as usize] != self.V[y as usize] {
                    self.increment_PC();
                }
            }
            Mnemonics::LOAD_I_NNN { nnn } => {
                /* ANNN - Load, [I] = NNN */
                self.I = nnn;
            }
            Mnemonics::JUMP_V0_NNN { nnn } => {
                /* BNNN - Jump(goto) address V[0x0] + NNN */
                self.PROGRAM_COUNTER = u16::from(self.V[0x0]) + nnn
            }
            Mnemonics::RAND { x, nn } => {
                /* CXNN - V[x] = (Random u8 Byte) &&(AND) NN */
                use rand::Rng;
                let random_byte: u8 = rand::rng().random();
                self.V[x as usize] = random_byte & nn;
            }
            Mnemonics::DRAW { x, y, n } => {
                /* DRAW -  Display N sprite, starting at [I] at (V[x], V[y]), then V[0xF] = collision */
                self.V[0xF] = 0; // Reset collision
                for row in 0..(n as usize) {
                    let sprite_byte = memory.RAM[self.I as usize + row];
                    for bit in 0..8 {
                        /* Check if current bit of the sprite is set */
                        let pixel_on = (sprite_byte & (0x80 >> bit)) != 0;
                        if !pixel_on {
                            continue;
                        }
                        /* Screen (X and Y) coordinates (idx) */
                        let idx = (display.get_x_postion(self.V[x as usize] as usize) + bit)
                            % WINDOW_WIDTH
                            + (display.get_y_postion(self.V[y as usize] as usize) + row)
                                % WINDOW_HEIGHT
                                * WINDOW_WIDTH;
                        /* If pixel is ON (1), collision detected, set VF = true (1) */
                        if display.get_pixel(idx) == 1 {
                            self.V[0xF] = 1;
                        }
                        /* If pixel is ON(1) flip (XOR) to OFF(0) */
                        display.set_pixels(idx);
                    }
                }
            }
            Mnemonics::SKP_Vx { x } => {
                /* EX9E - Skip next instruction (PC += 2) if [KEY] == V[x] is pressed */
                if keyboard.is_key_down(&display, self.V[x as usize] as usize) {
                    self.increment_PC();
                }
            }
            Mnemonics::SKNP_Vx { x } => {
                /* EXA1 - Skip next instruction (PC += 2) if [KEY] == V[x] is NOT pressed */
                if !keyboard.is_key_down(&display, self.V[x as usize] as usize) {
                    self.increment_PC();
                }
            }
            Mnemonics::LOAD_Vx_DT { x } => {
                /* FX07 - Load, V[x] = [DELAY_TIMER] */
                self.V[x as usize] = self.D_TIMER;
            }
            Mnemonics::LOAD_Vx_K { x } => {
                /* FX0A - Wait.. for [KEY] pressed then V[x] = [KEY] */
                if let Some(key) = keyboard.get_key_pressed(&display) {
                    self.V[x as usize] = key as u8;
                    self.increment_PC();
                }
            }
            Mnemonics::LOAD_DT_Vx { x } => {
                /* FX15 - Load, [DELAY_TIMER] = V[x] */
                self.D_TIMER = self.V[x as usize];
            }
            Mnemonics::LOAD_ST_Vx { x } => {
                /* FX18 - Load, [SOUND_TIMER] = V[x] */
                self.S_TIMER = self.V[x as usize];
            }
            Mnemonics::ADD_I_Vx { x } => {
                /* FX1E - [I] = [I] = V[x] */
                self.I = self.I.wrapping_add(u16::from(self.V[x as usize]));
            }
            Mnemonics::LOAD_FONT { x } => {
                /* FX29 - [I] = address of sprite for digit V[x] */
                let digit = (self.V[x as usize] & 0x0F) as usize;
                self.I = (FONT_BASE_ADDR + digit * GLYPH_BYTES) as u16;
            }
            Mnemonics::LOAD_B_Vx { x } => {
                /* FX33 - Load, [I], [I + 1] and [I + 2]  = V[x] (as Binary) */
                memory.RAM[self.I as usize] = self.V[x as usize] / 100;
                memory.RAM[self.I as usize + 1] = (self.V[x as usize] / 10) % 10;
                memory.RAM[self.I as usize + 2] = self.V[x as usize] % 10;
            }
            Mnemonics::LOAD_I_Vx { x } => {
                /* FX55 - RAM[I] .. RAM[Ix] = V[0x0] .. V[x] */
                for idx in 0..=x as usize {
                    memory.RAM[self.I as usize + idx] = self.V[idx];
                }
            }
            Mnemonics::LOAD_Vx_I { x } => {
                /* FX65 - V[0x0] .. V[x] = RAM[I] .. RAM[Ix] */
                for idx in 0..=x as usize {
                    self.V[idx] = memory.RAM[self.I as usize + idx];
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

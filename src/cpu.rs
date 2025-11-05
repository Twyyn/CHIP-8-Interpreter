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
            Opcode::CLS => {
                todo!()
            }
            Opcode::RETURN => self.memory.PROGRAM_COUNTER = self.memory.stack_pop(),
            Opcode::JMP { nnn } => self.memory.PROGRAM_COUNTER = nnn,
            Opcode::CALL { nnn } => {
                self.memory.stack_push(self.memory.PROGRAM_COUNTER);
                self.memory.PROGRAM_COUNTER = nnn
            }
            Opcode::XSKIP_NN { x, nn } => {
                if self.memory.RAM[x as usize] == nn {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Opcode::XSKIPN_NN { x, nn } => {
                if self.memory.RAM[x as usize] != nn {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Opcode::XSKIP_Y { x, y } => {
                if self.memory.RAM[x as usize] == self.memory.RAM[y as usize] {
                    self.memory.PROGRAM_COUNTER += 2
                }
            }
            Opcode::XLOAD_NN { x, nn } => self.memory.RAM[x as usize] = nn,
            Opcode::XADD_NN { x, nn } => {
                self.memory.RAM[x as usize] = self.memory.RAM[x as usize] + nn
            }
            Opcode::YLOAD_X { x, y } => self.memory.RAM[x as usize] = self.memory.RAM[y as usize],
        };
    }
}

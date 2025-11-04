use crate::errors::OpCodeError;

#[allow(non_camel_case_types)]
pub enum OpCode {
    //00E0 - Clear Screen
    CLS,
    //00EE - Return from a subroutine
    RETURN,
    //1NNN - Jump, PC = NNN
    JMP { addr: u16 },
    //2NNN - Execute subroutine starting at NNN
    CALL { addr: u16 },
    //3XNN - Skip if RAM[Vx] == NN
    VSKIP_NN { x: u8, nn: u8 },
    //4XNN - Skip if RAM[Vx] != NN
    VSKIPN_NN { x: u8, nn: u8 },
    //5XY0 - Skip if RAM[Vx] == RAM[Vy]
    VSKIP_Y { x: u8, y: u8 },
    //6XNN - RAM[Vx] = NN
    VLOAD_NN { x: u8, nn: u8 },
    //7XNN - RAM[Vx] += NN
    VADD_NN { x: u8, nn: u8 },
    //8XY0 - RAM[Vy] = RAM[Vx]
    YLOAD_X { x: u8, y: u8 },
    //8XY1 - RAM[Vx] = RAM[Vx] OR RAM[Xy]
    XSET_OR_Y { x: u8, y: u8 },
    //8XY2 - RAM[Vx] = RAM[Vx] AND RAM[Xy]
    XSET_AND_Y { x: u8, y: u8 },
    //8XY3 - RAM[Vx] = RAM[Vx] XOR RAM[Xy]
    XSET_XOR_Y { x: u8, y: u8 },
    //8XY4 - RAM[Vx] += RAM[Xy], Set VF
    XADD_Y { x: u8, y: u8 },
    //8XY5 - RAM[Vx] -= RAM[Xy], Set VF
    XSUB_Y { x: u8, y: u8 },
    //8XY6 - Set VF = LSB, RAM[Vx] = RAM[Vy] >> 1
    XSHR_Y { x: u8, y: u8 },
    //8XY7 - RAM[Vx] = RAM[Vy] - RAM[Vx], Set VF
    XSUB_XY { x: u8, y: u8 },
    //8XYE - RAM[Vx] = RAM[Vy] << 1, Set VF
    XSET_SHL_Y { x: u8, y: u8 },
    //9XY0 - Skip the following (PC +=2) if RAM[Vx] != RAM[Vy]
    SKIPX_N_Y { x: u8, y: u8 },
    //ANNN - RAM[I] = NNN
    LOADI { addr: u16 },
    //BNNN - Jump to NNN + V0
    JUMP_V0 { addr: u16 },
    //CVNN - RAM[Vx] = RandomNum with a mask of NN
    RAND { x: u8, nn: u8 },
    //DXYN - Draw sprite at position Vx, Vy with N bytes, starting at RAM[I], Set VF any pixels changes from 1 -> 0
    DRAW { x: u8, y: u8, n: u8 },
    //EX9E - Skip following (PC += 2) if KEY(Hex) is already = RAM[Vx]
    XSKIP_KP { x: u8 },
    //EXA1 - Skip following (PC += 2) if KEY(Hex) != Vx
    X_SKIP_KNP { x: u8 },
    //FX07 - RAM[Vx] = [DELAY_TIMER]
    VLOAD_DT { x: u8 },
    //FX0A - Wait for [KEY]press, RAM[Vx] = [KEY]
    VLOAD_KP { x: u8 },
    //FX15 - [DELAY_TIMER] = RAM[Vx]
    DTLOAD_V { x: u8 },
    //FX18 - [SOUND_TIMER] = RAM[Vx]
    STLOAD_V { x: u8 },
    //FX1E - RAM[I] += RAM[Vx]
    IADD_V { x: u8 },
    //FX29 - [I] =  VxRAM[SPRITE_DATA] ->= RAM[Vx]
    FONT { x: u8 },
    //FX33 - [I], [I + 1] [I + 2]  = RAM[Vx], Decimal form
    BCD { x: u8 },
    //FX55 - [I] ..[Ix] = RAM[V0]..RAM[Vx], [I] = I + X + 1
    DUMP { x: u8 },
    //FX65 - [V0] .. [Vx] = [I]..[Ix],  [I] = [I] + X + 1
    REG_LOAD { x: u8 },
    //Opcode Unkown,
    OpCodeError { op: u16 },
}
impl TryFrom<u16> for OpCode {
    type Error = OpCodeError;
    fn try_from(op: u16) -> Result<Self, Self::Error> {
        let (nnn, nn, n) = (op & 0x0FFF, (op & 0x00FF) as u8, (op & 0x000F) as u8);
        let (x, y) = (((op >> 8) & 0x000F) as u8, ((op >> 4) & 0x000F) as u8);

        let decoded = match op & 0xF000 {
            0x0000 => match op {
                0x00E0 => OpCode::CLS,
                0x00EE => OpCode::RETURN,
                _ => unimplemented!("{}", OpCodeError::UnknownOpcode(op)),
            },
            0x1000 => OpCode::JMP { addr: nnn },
            0x2000 => OpCode::CALL { addr: nnn },
            0x3000 => OpCode::VSKIP_NN { x, nn },
            0x4000 => OpCode::VSKIPN_NN { x, nn },
            0x5000 => {
                if n != 0 {
                    return Err(OpCodeError::UnknownOpcode(op));
                }
                OpCode::VSKIP_Y { x, y }
            }
            0x6000 => OpCode::VLOAD_NN { x, nn },
            0x7000 => OpCode::VADD_NN { x, nn },
            0x8000 => match n {
                0x0 => OpCode::YLOAD_X { x, y },
                0x1 => OpCode::XSET_OR_Y { x, y },
                0x2 => OpCode::XSET_AND_Y { x, y },
                0x3 => OpCode::XSET_XOR_Y { x, y },
                0x4 => OpCode::XADD_Y { x, y },
                0x5 => OpCode::XSUB_Y { x, y },
                0x6 => OpCode::XSHR_Y { x, y },
                0x7 => OpCode::XSUB_XY { x, y },
                0xE => OpCode::XSET_SHL_Y { x, y },
                _ => return Err(OpCodeError::UnknownOpcode(op)),
            },
            0x9000 => {
                if n != 0 {
                    return Err(OpCodeError::UnknownOpcode(op));
                }
                OpCode::SKIPX_N_Y { x, y }
            }
            0xA000 => OpCode::LOADI { addr: nnn },
            0xB000 => OpCode::JUMP_V0 { addr: nnn },
            0xC000 => OpCode::RAND { x, nn },
            0xD000 => OpCode::DRAW { x, y, n },
            0xE000 => match nn {
                0x9E => OpCode::XSKIP_KP { x },
                0xA1 => OpCode::X_SKIP_KNP { x },
                _ => return Err(OpCodeError::UnknownOpcode(op)),
            },
            0xF000 => match nn {
                0x07 => OpCode::VLOAD_DT { x },
                0x0A => OpCode::VLOAD_KP { x },
                0x15 => OpCode::DTLOAD_V { x },
                0x18 => OpCode::STLOAD_V { x },
                0x1E => OpCode::IADD_V { x },
                0x29 => OpCode::FONT { x },
                0x33 => OpCode::BCD { x },
                0x55 => OpCode::DUMP { x },
                0x65 => OpCode::REG_LOAD { x },
                _ => return Err(OpCodeError::UnknownOpcode(op)),
            },
            _ => return Err(OpCodeError::UnknownOpcode(op)),
        };
        Ok(decoded)
    }
}

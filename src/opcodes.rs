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
}
impl TryFrom<u16> for OpCode {
    type Error = OpCodeError;
    fn try_from(word: u16) -> Result<Self, Self::Error> {
        let (op, n, nn, nnn) = (
            ((word >> 12) & 0xF) as u8,
            (word & 0xF) as u8,
            ((word & 0xFF) & 0xF) as u8,
            word & 0x0FFF,
        );
        let (x, y) = (((word >> 8) & 0xF) as u8, ((word >> 4) & 0xF) as u8);
        todo!()
    }
}

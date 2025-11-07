use crate::errors::OpcodeError;

#[allow(non_camel_case_types)]
pub enum Opcode {
    //00E0 - Clear Screen
    CLEAR,
    //00EE - Return from a subroutine
    RETURN,
    //1NNN - Jump, PC = NNN
    JUMP { nnn: u16 },
    //2NNN - Execute subroutine starting at NNN
    CALL { nnn: u16 },
    //3XNN - Skip if RAM[Vx] == NN
    SE_Vx_NN { x: u8, nn: u8 },
    //4XNN - Skip if RAM[Vx] != NN
    SNE_Vx_NN { x: u8, nn: u8 },
    //5XY0 - Skip if RAM[Vx] == RAM[Vy]
    SE_Vx_Vy { x: u8, y: u8 },
    //6XNN - RAM[Vx] = NN
    LOAD_Vx_NN { x: u8, nn: u8 },
    //7XNN - RAM[Vx] += NN
    ADD_Vx_NN { x: u8, nn: u8 },
    //8XY0 - RAM[Vy] = RAM[Vx]
    LOAD_Vx_Vy { x: u8, y: u8 },
    //8XY1 - RAM[Vx] = RAM[Vx] OR RAM[Xy]
    OR_Vx_Vy { x: u8, y: u8 },
    //8XY2 - RAM[Vx] = RAM[Vx] AND RAM[Xy]
    AND_Vx_Vy { x: u8, y: u8 },
    //8XY3 - RAM[Vx] = RAM[Vx] XOR RAM[Xy]
    XOR_Vx_Vy { x: u8, y: u8 },
    //8XY4 - RAM[Vx] += RAM[Xy], Set VF
    ADD_Vx_Vy { x: u8, y: u8 },
    //8XY5 - RAM[Vx] -= RAM[Xy], Set VF
    SUB_Vx_Vy { x: u8, y: u8 },
    //8XY6 - Set VF = LSB, RAM[Vx] = RAM[Vy] >> 1
    SHR_Vx_Vy { x: u8, y: u8 },
    //8XY7 - RAM[Vx] = RAM[Vy] - RAM[Vx], Set VF
    SUBN_Vx_Vy { x: u8, y: u8 },
    //8XYE - RAM[Vx] = RAM[Vy] << 1, Set VF
    SHL_Vx_Vy { x: u8, y: u8 },
    //9XY0 - Skip the following (PC +=2) if RAM[Vx] != RAM[Vy]
    SNE_Vx_Vy { x: u8, y: u8 },
    //ANNN - RAM[I] = NNN
    LOAD_I_NNN { nnn: u16 },
    //BNNN - Jump to NNN + V0
    JUMP_V0_NNN { nnn: u16 },
    //CVNN - RAM[Vx] = RandomNum with a mask of NN
    RAND { x: u8, nn: u8 },
    //DXYN - Draw sprite at position Vx, Vy with N bytes, starting at RAM[I], Set VF any pixels changes from 1 -> 0
    DRAW { x: u8, y: u8, n: u8 },
    //EX9E - Skip following (PC += 2) if KEY(Hex) is already = RAM[Vx]
    SKP_Vx { x: u8 },
    //EXA1 - Skip following (PC += 2) if KEY(Hex) != Vx
    SKNP_Vx { x: u8 },
    //FX07 - RAM[Vx] = [DELAY_TIMER]
    LOAD_Vx_DT { x: u8 },
    //FX0A - Wait for [KEY]press, RAM[Vx] = [KEY]
    LOAD_Vx_K { x: u8 },
    //FX15 - [DELAY_TIMER] = RAM[Vx]
    LOAD_DT_Vx { x: u8 },
    //FX18 - [SOUND_TIMER] = RAM[Vx]
    LOAD_ST_Vx { x: u8 },
    //FX1E - RAM[I] += RAM[Vx]
    ADD_I_Vx { x: u8 },
    //FX29 - [I] =  VxRAM[SPRITE_DATA] ->= RAM[Vx]
    LOAD_FONT { x: u8 },
    //FX33 - [I], [I + 1] [I + 2]  = RAM[Vx], Decimal form
    LOAD_B_Vx { x: u8 },
    //FX55 - [I] ..[Ix] = RAM[V0]..RAM[Vx], [I] = I + X + 1
    LOAD_I_Vx { x: u8 },
    //FX65 - [V0] .. [Vx] = [I]..[Ix],  [I] = [I] + X + 1
    LOAD_Vx_I { x: u8 },
    //Opcode Unknown,
    OpCodeError { op: u16 },
}
impl TryFrom<u16> for Opcode {
    type Error = OpcodeError;
    fn try_from(instr: u16) -> Result<Self, Self::Error> {
        let first_byte = (instr & 0xFF00) >> 8 as u8;
        let second_byte = (instr & 0x00FF) as u8;

        let op = (first_byte & 0xF0) >> 4 as u8;

        let (x, y, n, nn, nnn) = (
            (first_byte & 0x0F) as u8,
            ((second_byte & 0xF0) >> 4) as u8,
            second_byte & 0x0F,
            second_byte,
            (op & 0x0FFF) as u16,
        );
        let op = match (op, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Opcode::CLEAR,
            (0x0, 0x0, 0xE, 0xE) => Opcode::RETURN,
            (0x1, _, _, _) => Opcode::JUMP { nnn },
            (0x2, _, _, _) => Opcode::CALL { nnn },
            (0x3, _, _, _) => Opcode::SE_Vx_NN { x, nn },
            (0x4, _, _, _) => Opcode::SNE_Vx_NN { x, nn },
            (0x5, _, _, 0x0) => Opcode::SE_Vx_Vy { x, y },
            (0x6, _, _, _) => Opcode::LOAD_Vx_NN { x, nn },
            (0x7, _, _, _) => Opcode::ADD_Vx_NN { x, nn },
            (0x8, _, _, 0x0) => Opcode::LOAD_Vx_Vy { x, y },
            (0x8, _, _, 0x1) => Opcode::OR_Vx_Vy { x, y },
            (0x8, _, _, 0x2) => Opcode::AND_Vx_Vy { x, y },
            (0x8, _, _, 0x3) => Opcode::XOR_Vx_Vy { x, y },
            (0x8, _, _, 0x4) => Opcode::ADD_Vx_Vy { x, y },
            (0x8, _, _, 0x5) => Opcode::SUB_Vx_Vy { x, y },
            (0x8, _, _, 0x6) => Opcode::SHR_Vx_Vy { x, y },
            (0x8, _, _, 0x7) => Opcode::SUBN_Vx_Vy { x, y },
            (0x8, _, _, 0xE) => Opcode::SHL_Vx_Vy { x, y },
            (0x9, _, _, 0x0) => Opcode::SNE_Vx_Vy { x, y },
            (0xA, _, _, _) => Opcode::LOAD_I_NNN { nnn },
            (0xB, _, _, _) => Opcode::JUMP_V0_NNN { nnn },
            (0xC, _, _, _) => Opcode::RAND { x, nn },
            (0xD, _, _, _) => Opcode::DRAW { x, y, n },
            (0xE, _, 0x9, 0xE) => Opcode::SKP_Vx { x },
            (0xE, _, 0xA, 0x1) => Opcode::SKNP_Vx { x },
            (0xF, _, 0x0, 0x7) => Opcode::LOAD_Vx_DT { x },
            (0xF, _, 0x0, 0xA) => Opcode::LOAD_Vx_K { x },
            (0xF, _, 0x1, 0x5) => Opcode::LOAD_DT_Vx { x },
            (0xF, _, 0x1, 0x8) => Opcode::LOAD_ST_Vx { x },
            (0xF, _, 0x1, 0xE) => Opcode::ADD_I_Vx { x },
            (0xF, _, 0x2, 0x9) => Opcode::LOAD_FONT { x },
            (0xF, _, 0x3, 0x3) => Opcode::LOAD_B_Vx { x },
            (0xF, _, 0x5, 0x5) => Opcode::LOAD_I_Vx { x },
            (0xF, _, 0x6, 0x5) => Opcode::LOAD_Vx_I { x },
            (_, _, _, _) => return Err(OpcodeError::UnknownOpcode(instr)),
        };
        Ok(op)
    }
}

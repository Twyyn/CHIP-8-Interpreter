use crate::core::OpcodeError;

#[allow(non_camel_case_types)]
pub enum Mnemonics {
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
impl TryFrom<u16> for Mnemonics {
    type Error = OpcodeError;
    fn try_from(instr: u16) -> Result<Self, Self::Error> {
        let (x, y, n, nn, nnn, op) = (
            ((instr & 0x0F00) >> 8) as u8,
            ((instr & 0x00F0) >> 4) as u8,
            (instr & 0x000F) as u8,
            (instr & 0x00FF) as u8,
            instr & 0x0FFF,
            (instr & 0xF000) >> 12,
        );

        let mnemontic = match (op, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Mnemonics::CLEAR,
            (0x0, 0x0, 0xE, 0xE) => Mnemonics::RETURN,
            (0x1, _, _, _) => Mnemonics::JUMP { nnn },
            (0x2, _, _, _) => Mnemonics::CALL { nnn },
            (0x3, _, _, _) => Mnemonics::SE_Vx_NN { x, nn },
            (0x4, _, _, _) => Mnemonics::SNE_Vx_NN { x, nn },
            (0x5, _, _, 0x0) => Mnemonics::SE_Vx_Vy { x, y },
            (0x6, _, _, _) => Mnemonics::LOAD_Vx_NN { x, nn },
            (0x7, _, _, _) => Mnemonics::ADD_Vx_NN { x, nn },
            (0x8, _, _, 0x0) => Mnemonics::LOAD_Vx_Vy { x, y },
            (0x8, _, _, 0x1) => Mnemonics::OR_Vx_Vy { x, y },
            (0x8, _, _, 0x2) => Mnemonics::AND_Vx_Vy { x, y },
            (0x8, _, _, 0x3) => Mnemonics::XOR_Vx_Vy { x, y },
            (0x8, _, _, 0x4) => Mnemonics::ADD_Vx_Vy { x, y },
            (0x8, _, _, 0x5) => Mnemonics::SUB_Vx_Vy { x, y },
            (0x8, _, _, 0x6) => Mnemonics::SHR_Vx_Vy { x, y },
            (0x8, _, _, 0x7) => Mnemonics::SUBN_Vx_Vy { x, y },
            (0x8, _, _, 0xE) => Mnemonics::SHL_Vx_Vy { x, y },
            (0x9, _, _, 0x0) => Mnemonics::SNE_Vx_Vy { x, y },
            (0xA, _, _, _) => Mnemonics::LOAD_I_NNN { nnn },
            (0xB, _, _, _) => Mnemonics::JUMP_V0_NNN { nnn },
            (0xC, _, _, _) => Mnemonics::RAND { x, nn },
            (0xD, _, _, _) => Mnemonics::DRAW { x, y, n },
            (0xE, _, 0x9, 0xE) => Mnemonics::SKP_Vx { x },
            (0xE, _, 0xA, 0x1) => Mnemonics::SKNP_Vx { x },
            (0xF, _, 0x0, 0x7) => Mnemonics::LOAD_Vx_DT { x },
            (0xF, _, 0x0, 0xA) => Mnemonics::LOAD_Vx_K { x },
            (0xF, _, 0x1, 0x5) => Mnemonics::LOAD_DT_Vx { x },
            (0xF, _, 0x1, 0x8) => Mnemonics::LOAD_ST_Vx { x },
            (0xF, _, 0x1, 0xE) => Mnemonics::ADD_I_Vx { x },
            (0xF, _, 0x2, 0x9) => Mnemonics::LOAD_FONT { x },
            (0xF, _, 0x3, 0x3) => Mnemonics::LOAD_B_Vx { x },
            (0xF, _, 0x5, 0x5) => Mnemonics::LOAD_I_Vx { x },
            (0xF, _, 0x6, 0x5) => Mnemonics::LOAD_Vx_I { x },
            (_, _, _, _) => return Err(OpcodeError::UnknownMnemonic(instr)),
        };
        Ok(mnemontic)
    }
}

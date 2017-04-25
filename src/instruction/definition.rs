use opcode;
use opcode::Opcode::*;

// Details about an instruction.
pub struct InstructionDefinition {
    pub len: u16,
    pub cycles: u8,
}

fn def(len: u16, cycles: u8) -> InstructionDefinition {
    InstructionDefinition {
        len: len,
        cycles: cycles,
    }
}

// Get an instruction definition, based on opcode.
pub fn lookup_instruction_definition(opcode: opcode::Opcode) -> InstructionDefinition {
    match opcode {
        // Flag (processor status)
        CLC => def(1, 2),
        SEC => def(1, 2),
        CLI => def(1, 2),
        SEI => def(1, 2),
        CLV => def(1, 2),
        CLD => def(1, 2),
        SED => def(1, 2),

        // DECrement memory
        DEC_Zero => def(2, 5),
        DEC_Zero_X => def(2, 6),
        DEC_Abs => def(3, 6),
        DEC_Abs_X => def(3, 7),

        // INCrement memory
        INC_Zero => def(2, 5),
        INC_Zero_X => def(2, 6),
        INC_Abs => def(3, 6),
        INC_Abs_X => def(3, 7),

        // LoaD Accumulator
        LDA_Imm => def(2, 2),
        LDA_Zero => def(2, 3),
        LDA_Zero_X => def(2, 4),
        LDA_Abs => def(3, 4),
        LDA_Abs_X => def(3, 4),
        LDA_Abs_Y => def(3, 4),
        LDA_Ind_X => def(2, 6), 
        LDA_Ind_Y => def(2, 5),

        // LoaD X register
        LDX_Imm => def(2, 2),
        LDX_Zero => def(2, 3),
        LDX_Zero_Y => def(2, 4),
        LDX_Abs => def(3, 4),
        LDX_Abs_Y => def(3, 4),

        // LoaD Y register
        LDY_Imm => def(2, 2),
        LDY_Zero => def(2, 3),
        LDY_Zero_X => def(2, 4),
        LDY_Abs => def(3, 4),
        LDY_Abs_X => def(3, 4),

        // STore Accumulator
        STA_Zero => def(2, 3),
        STA_Zero_X => def(2, 4),
        STA_Abs => def(3, 4),
        STA_Abs_X => def(3, 5),
        STA_Abs_Y => def(3, 5),
        STA_Ind_X => def(2, 6),
        STA_Ind_Y => def(2, 6),

        // STore X register
        STX_Zero => def(2, 3),
        STX_Zero_Y => def(2, 4),
        STX_Abs => def(3, 4),

        // STore Y register
        STY_Zero => def(2, 3),
        STY_Zero_X => def(2, 4),
        STY_Abs => def(3, 4),
    }
}

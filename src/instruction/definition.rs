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
        // ADd with Carry
        ADC_Imm => def(2, 2),
        ADC_Zero => def(2, 3),
        ADC_Zero_X => def(2, 4),
        ADC_Abs => def(3, 3),
        ADC_Abs_X => def(3, 4),
        ADC_Abs_Y => def(3, 4),
        ADC_Ind_X => def(2, 6),
        ADC_Ind_Y => def(2, 5),

        // bitwise AND with accumulator
        AND_Imm => def(2, 2),
        AND_Zero => def(2, 3),
        AND_Zero_X => def(2, 4),
        AND_Abs => def(3, 4),
        AND_Abs_X => def(3, 4),
        AND_Abs_Y => def(3, 4),
        AND_Ind_X => def(2, 6),
        AND_Ind_Y => def(2, 5),

        // Arithmetic Shift Left
        ASL_Acc => def(1, 2),
        ASL_Zero => def(2, 5),
        ASL_Zero_X => def(2, 6),
        ASL_Abs => def(3, 6),
        ASL_Abs_X => def(3, 7),

        // test BITs
        BIT_Zero => def(2, 3),
        BIT_Abs => def(3, 4),

        // Flag (processor status)
        CLC => def(1, 2),
        SEC => def(1, 2),
        CLI => def(1, 2),
        SEI => def(1, 2),
        CLV => def(1, 2),
        CLD => def(1, 2),
        SED => def(1, 2),

        // CoMPare accumulator
        CMP_Imm => def(2, 2),
        CMP_Zero => def(2, 3),
        CMP_Zero_X => def(2, 4),
        CMP_Abs => def(3, 4),
        CMP_Abs_X => def(3, 4),
        CMP_Abs_Y => def(3, 4),
        CMP_Ind_X => def(2, 6),
        CMP_Ind_Y => def(2, 5),

        // ComPare X register
        CPX_Imm => def(2, 2),
        CPX_Zero => def(2, 3),
        CPX_Abs => def(3, 4),

        // ComPare Y register
        CPY_Imm => def(2, 2),
        CPY_Zero => def(2, 3),
        CPY_Abs => def(3, 4),

        // DECrement memory
        DEC_Zero => def(2, 5),
        DEC_Zero_X => def(2, 6),
        DEC_Abs => def(3, 6),
        DEC_Abs_X => def(3, 7),

        // bitwise Exclusive OR
        EOR_Imm => def(2, 2),
        EOR_Zero => def(2, 3),
        EOR_Zero_X => def(2, 4),
        EOR_Abs => def(3, 4),
        EOR_Abs_X => def(3, 4),
        EOR_Abs_Y => def(3, 4),
        EOR_Ind_X => def(2, 6),
        EOR_Ind_Y => def(2, 5),

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

        // Logical Shift Right
        LSR_Acc => def(1, 2),
        LSR_Zero => def(2, 5),
        LSR_Zero_X => def(2, 6),
        LSR_Abs => def(3, 6),
        LSR_Abs_X => def(3, 7),

        // bitwise OR with Accumulator
        ORA_Imm => def(2, 2),
        ORA_Zero => def(2, 3),
        ORA_Zero_X => def(2, 4),
        ORA_Abs => def(3, 4),
        ORA_Abs_X => def(3, 4),
        ORA_Abs_Y => def(3, 4),
        ORA_Ind_X => def(2, 6),
        ORA_Ind_Y => def(2, 5),

        // No OPeration
        NOP => def(1, 2),

        // Register Instructions
        TAX => def(1, 2),
        TXA => def(1, 2),
        DEX => def(1, 2),
        INX => def(1, 2),
        TAY => def(1, 2),
        TYA => def(1, 2),
        DEY => def(1, 2),
        INY => def(1, 2),

        // ROtate Left
        ROL_Acc => def(1, 2),
        ROL_Zero => def(2, 5),
        ROL_Zero_X => def(2, 6),
        ROL_Abs => def(3, 6),
        ROL_Abs_X => def(3, 7),

        // ROtate Right
        ROR_Acc => def(1, 2),
        ROR_Zero => def(2, 5),
        ROR_Zero_X => def(2, 6),
        ROR_Abs => def(3, 6),
        ROR_Abs_X => def(3, 7),

        // SuBtract with Carry
        SBC_Imm => def(2, 2),
        SBC_Zero => def(2, 3),
        SBC_Zero_X => def(2, 4),
        SBC_Abs => def(3, 4),
        SBC_Abs_X => def(3, 4),
        SBC_Abs_Y => def(3, 4),
        SBC_Ind_X => def(2, 6),
        SBC_Ind_Y => def(2, 5),

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

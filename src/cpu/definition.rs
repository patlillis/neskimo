use cpu::opcode::Opcode;
use cpu::opcode::Opcode::*;

// Details about an instruction.
pub struct InstructionDefinition {
    pub len: u16,
    pub cycles: u8,
}

fn def(len: u16, cycles: u8) -> InstructionDefinition {
    InstructionDefinition { len, cycles }
}

// Get an instruction definition, based on opcode.
pub fn lookup_instruction_definition(opcode: Opcode) -> InstructionDefinition {
    match opcode {
        // ADd with Carry
        ADC_Imm => def(2, 2),
        ADC_Zero => def(2, 3),
        ADC_Zero_X => def(2, 4),
        ADC_Abs => def(3, 4),
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

        // Branch instructions
        BPL => def(2, 2),
        BMI => def(2, 2),
        BVC => def(2, 2),
        BVS => def(2, 2),
        BCC => def(2, 2),
        BCS => def(2, 2),
        BNE => def(2, 2),
        BEQ => def(2, 2),

        // BReaK
        // Note that the actual BRK instruction is only 1 bytes long,
        // but the return address pushed onto the stack is the address
        // of BRK + 2.
        BRK => def(2, 7),

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

        // JuMP
        JMP_Abs => def(3, 3),
        JMP_Ind => def(3, 5),

        // Jump to SubRoutine
        JSR => def(3, 6),

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

        // Stack Instructions
        TXS => def(1, 2),
        TSX => def(1, 2),
        PHA => def(1, 3),
        PLA => def(1, 4),
        PHP => def(1, 3),
        PLP => def(1, 4),

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

        // ReTurn from Interrupt
        RTI => def(1, 6),

        // ReTurn from Subroutine
        RTS => def(1, 6),

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

        // UNOFFICIAL OPCODES

        // No-ops
        _NOP_1 => def(1, 2),
        _NOP_2 => def(1, 2),
        _NOP_3 => def(1, 2),
        _NOP_4 => def(1, 2),
        _NOP_5 => def(1, 2),
        _NOP_6 => def(1, 2),

        // No-op reads
        _NOP_Imm_1 | _NOP_Imm_2 | _NOP_Imm_3 | _NOP_Imm_4 | _NOP_Imm_5 => {
            def(2, 2)
        }
        _NOP_Abs => def(3, 4),
        _NOP_Abs_X_1 | _NOP_Abs_X_2 | _NOP_Abs_X_3 | _NOP_Abs_X_4
        | _NOP_Abs_X_5 | _NOP_Abs_X_6 => def(3, 4),
        _NOP_Zero_1 | _NOP_Zero_2 | _NOP_Zero_3 => def(2, 3),
        _NOP_Zero_X_1 | _NOP_Zero_X_2 | _NOP_Zero_X_3 | _NOP_Zero_X_4
        | _NOP_Zero_X_5 | _NOP_Zero_X_6 => def(2, 4),

        // Load Accumulator into X register
        _LAX_Abs => def(3, 4),
        _LAX_Abs_Y => def(3, 4),
        _LAX_Zero => def(2, 3),
        _LAX_Zero_Y => def(2, 4),
        _LAX_Ind_X => def(2, 6),
        _LAX_Ind_Y => def(2, 5),

        // Store bitwise and of Accumulator and X register
        _SAX_Abs => def(3, 4),
        _SAX_Zero => def(2, 3),
        _SAX_Zero_Y => def(2, 4),
        _SAX_Ind_X => def(2, 6),

        // SuBtract with Carry
        _SBC_Imm => def(2, 2),

        // Decrement value, ComPare accumulator
        _DCP_Abs => def(3, 6),
        _DCP_Abs_X => def(3, 7),
        _DCP_Abs_Y => def(3, 7),
        _DCP_Zero => def(2, 5),
        _DCP_Zero_X => def(2, 6),
        _DCP_Ind_X => def(2, 8),
        _DCP_Ind_Y => def(2, 8),

        // Increment value, SuBtract with carry
        _ISB_Abs => def(3, 6),
        _ISB_Abs_X => def(3, 7),
        _ISB_Abs_Y => def(3, 7),
        _ISB_Zero => def(2, 5),
        _ISB_Zero_X => def(2, 6),
        _ISB_Ind_X => def(2, 8),
        _ISB_Ind_Y => def(2, 8),

        // arithmetic Shift Left, bitwise Or with accumulator
        _SLO_Abs => def(3, 6),
        _SLO_Abs_X => def(3, 7),
        _SLO_Abs_Y => def(3, 7),
        _SLO_Zero => def(2, 5),
        _SLO_Zero_X => def(2, 6),
        _SLO_Ind_X => def(2, 8),
        _SLO_Ind_Y => def(2, 8),

        // Rotate Left, And with accumulator
        _RLA_Abs => def(3, 6),
        _RLA_Abs_X => def(3, 7),
        _RLA_Abs_Y => def(3, 7),
        _RLA_Zero => def(2, 5),
        _RLA_Zero_X => def(2, 6),
        _RLA_Ind_X => def(2, 8),
        _RLA_Ind_Y => def(2, 8),

        // logical Shift Right, Exclusive or with accumulator
        _SRE_Abs => def(3, 6),
        _SRE_Abs_X => def(3, 7),
        _SRE_Abs_Y => def(3, 7),
        _SRE_Zero => def(2, 5),
        _SRE_Zero_X => def(2, 6),
        _SRE_Ind_X => def(2, 8),
        _SRE_Ind_Y => def(2, 8),

        // Rotate Right, Add with carry
        _RRA_Abs => def(3, 6),
        _RRA_Abs_X => def(3, 7),
        _RRA_Abs_Y => def(3, 7),
        _RRA_Zero => def(2, 5),
        _RRA_Zero_X => def(2, 6),
        _RRA_Ind_X => def(2, 8),
        _RRA_Ind_Y => def(2, 8),
    }
}

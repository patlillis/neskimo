use std;

use num::FromPrimitive;

// Decodes an opcode by converting an opcode number to an enum value.
pub fn decode(opcode: u8) -> Opcode {
    match Opcode::from_u8(opcode) {
        Some(opcode) => opcode,
        None => {
            panic!("Unexpected Opcode: {:#04x}", opcode);
        }
    }
}

impl std::fmt::Display for Opcode {
    // A little weird. This makes sure that unofficial opcodes (which start
    // with "_") show up as "*NOP", where as normal ops are " LSR", with a
    // space in front.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut name = format!("{:?}", self);
        if name.chars().nth(0).unwrap() != '_' {
            name.insert(0, ' ');
        }
        let replaced_name = name.replace("_", "*");
        write!(f, "{}", &replaced_name[..4])
    }
}

impl Opcode {
    pub fn val(&self) -> String {
        format!("{:#04x}", *self as u8)
    }
}

// Opcodes.
//
// "enum_from_primitive" allows for doing "Opcode::from_u8(0xab)",
// which is cool. Can also do "Opcode::NOP as u8", or use the "decode()" fn.
enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    #[allow(non_camel_case_types)]
    pub enum Opcode {
        // ADd with Carry
        ADC_Imm     = 0x69,
        ADC_Zero    = 0x65,
        ADC_Zero_X  = 0x75,
        ADC_Abs     = 0x6d,
        ADC_Abs_X   = 0x7d,
        ADC_Abs_Y   = 0x79,
        ADC_Ind_X   = 0x61,
        ADC_Ind_Y   = 0x71,

        // bitwise AND with accumulator
        AND_Imm     = 0x29,
        AND_Zero    = 0x25,
        AND_Zero_X  = 0x35,
        AND_Abs     = 0x2d,
        AND_Abs_X   = 0x3d,
        AND_Abs_Y   = 0x39,
        AND_Ind_X   = 0x21,
        AND_Ind_Y   = 0x31,

        // Arithmetic Shift Left
        ASL_Acc     = 0x0a,
        ASL_Zero    = 0x06,
        ASL_Zero_X  = 0x16,
        ASL_Abs     = 0x0e,
        ASL_Abs_X   = 0x1e,

        // test BITs
        BIT_Zero    = 0x24,
        BIT_Abs     = 0x2c,

        // Branch instructions
        BPL         = 0x10, // Branch on PLus
        BMI         = 0x30, // Branch on MInus
        BVC         = 0x50, // Branch on oVerflow Clear
        BVS         = 0x70, // Branch on oVerflow Set
        BCC         = 0x90, // Branch on Carry Clear
        BCS         = 0xb0, // Branch on Carry Set
        BNE         = 0xd0, // Branch on Not Equal
        BEQ         = 0xf0, // Branch on EQual

        // BReaK
        BRK       = 0x00,

        // CoMPare accumulator
        CMP_Imm     = 0xc9,
        CMP_Zero    = 0xc5,
        CMP_Zero_X  = 0xd5,
        CMP_Abs     = 0xcd,
        CMP_Abs_X   = 0xdd,
        CMP_Abs_Y   = 0xd9,
        CMP_Ind_X   = 0xc1,
        CMP_Ind_Y   = 0xd1,

        // ComPare X register
        CPX_Imm     = 0xe0,
        CPX_Zero    = 0xe4,
        CPX_Abs     = 0xec,

        // ComPare Y register
        CPY_Imm     = 0xc0,
        CPY_Zero    = 0xc4,
        CPY_Abs     = 0xcc,

        // Flag (processor status)
        CLC         = 0x18, // CLear Carry
        SEC         = 0x38, // SEt Carry
        CLI         = 0x58, // CLear Interrupt
        SEI         = 0x78, // SEt Interrupt
        CLV         = 0xB8, // CLear oVerflow
        CLD         = 0xD8, // CLear Decimal
        SED         = 0xF8, // SEt Decimal

        // DECrement memory
        DEC_Zero    = 0xc6,
        DEC_Zero_X  = 0xd6,
        DEC_Abs     = 0xce,
        DEC_Abs_X   = 0xde,

        // bitwise Exclusive OR
        EOR_Imm     = 0x49,
        EOR_Zero    = 0x45,
        EOR_Zero_X  = 0x55,
        EOR_Abs     = 0x4d,
        EOR_Abs_X   = 0x5d,
        EOR_Abs_Y   = 0x59,
        EOR_Ind_X   = 0x41,
        EOR_Ind_Y   = 0x51,

        // INCrement memory
        INC_Zero    = 0xe6,
        INC_Zero_X  = 0xf6,
        INC_Abs     = 0xee,
        INC_Abs_X   = 0xfe,

        // JuMP
        JMP_Abs     = 0x4c,
        JMP_Ind     = 0x6c,

        // Jump to SubRoutine
        JSR         = 0x20,

        // LoaD Accumulator
        LDA_Imm     = 0xa9,
        LDA_Zero    = 0xa5,
        LDA_Zero_X  = 0xb5,
        LDA_Abs     = 0xad,
        LDA_Abs_X   = 0xbd,
        LDA_Abs_Y   = 0xb9,
        LDA_Ind_X   = 0xa1,
        LDA_Ind_Y   = 0xb1,

        // LoaD X register
        LDX_Imm     = 0xa2,
        LDX_Zero    = 0xa6,
        LDX_Zero_Y  = 0xb6,
        LDX_Abs     = 0xae,
        LDX_Abs_Y   = 0xbe,

        // LoaD Y register
        LDY_Imm     = 0xa0,
        LDY_Zero    = 0xa4,
        LDY_Zero_X  = 0xb4,
        LDY_Abs     = 0xac,
        LDY_Abs_X   = 0xbc,

        // Logical Shift Right
        LSR_Acc     = 0x4a,
        LSR_Zero    = 0x46,
        LSR_Zero_X  = 0x56,
        LSR_Abs     = 0x4e,
        LSR_Abs_X   = 0x5e,

        // bitwise OR with Accumulator
        ORA_Imm     = 0x09,
        ORA_Zero    = 0x05,
        ORA_Zero_X  = 0x15,
        ORA_Abs     = 0x0d,
        ORA_Abs_X   = 0x1d,
        ORA_Abs_Y   = 0x19,
        ORA_Ind_X   = 0x01,
        ORA_Ind_Y   = 0x11,

        // No OPeration
        NOP         = 0xea,

        // Register Instructions
        TAX         = 0xaa, // Transfer A to X
        TXA         = 0x8a, // Transfer X to A
        DEX         = 0xca, // Decrement X
        INX         = 0xe8, // Increment X
        TAY         = 0xa8, // Transfer A to Y
        TYA         = 0x98, // Transfer Y to A
        DEY         = 0x88, // Decrement Y
        INY         = 0xc8, // Increment Y

        // Stack Instructions
        TXS         = 0x9a, // Transfer X to Stack ptr
        TSX         = 0xba, // Transfer Stack ptr to X
        PHA         = 0x48, // PusH Accumulator
        PLA         = 0x68, // PuLl Accumulator
        PHP         = 0x08, // PusH Processor status
        PLP         = 0x28, // PuLl Processor status

        // ROtate Left
        ROL_Acc     = 0x2a,
        ROL_Zero    = 0x26,
        ROL_Zero_X  = 0x36,
        ROL_Abs     = 0x2e,
        ROL_Abs_X   = 0x3e,

        // ROtate Right
        ROR_Acc     = 0x6a,
        ROR_Zero    = 0x66,
        ROR_Zero_X  = 0x76,
        ROR_Abs     = 0x6e,
        ROR_Abs_X   = 0x7e,

        // ReTurn from Interrupt
        RTI         = 0x40,

        // ReTurn from Subroutine
        RTS         = 0x60,

        // SuBtract with Carry
        SBC_Imm     = 0xe9,
        SBC_Zero    = 0xe5,
        SBC_Zero_X  = 0xf5,
        SBC_Abs     = 0xed,
        SBC_Abs_X   = 0xfd,
        SBC_Abs_Y   = 0xf9,
        SBC_Ind_X   = 0xe1,
        SBC_Ind_Y   = 0xf1,

        // STore Accumulator
        STA_Zero    = 0x85,
        STA_Zero_X  = 0x95,
        STA_Abs     = 0x8d,
        STA_Abs_X   = 0x9d,
        STA_Abs_Y   = 0x99,
        STA_Ind_X   = 0x81,
        STA_Ind_Y   = 0x91,

        // STore X register
        STX_Zero    = 0x86,
        STX_Zero_Y  = 0x96,
        STX_Abs     = 0x8e,

        // STore Y register
        STY_Zero    = 0x84,
        STY_Zero_X  = 0x94,
        STY_Abs     = 0x8c,

        // UNOFFICIAL OPCODES

        // No-ops
        _NOP_1          = 0x1a,
        _NOP_2          = 0x3a,
        _NOP_3          = 0x5a,
        _NOP_4          = 0x7a,
        _NOP_5          = 0xda,
        _NOP_6          = 0xfa,

        // No-op reads
        _NOP_Imm_1      = 0x80,
        _NOP_Imm_2      = 0x82,
        _NOP_Imm_3      = 0x89,
        _NOP_Imm_4      = 0xc2,
        _NOP_Imm_5      = 0xe2,
        _NOP_Abs        = 0x0c,
        _NOP_Abs_X_1    = 0x1c,
        _NOP_Abs_X_2    = 0x3c,
        _NOP_Abs_X_3    = 0x5c,
        _NOP_Abs_X_4    = 0x7c,
        _NOP_Abs_X_5    = 0xdc,
        _NOP_Abs_X_6    = 0xfc,
        _NOP_Zero_1     = 0x04,
        _NOP_Zero_2     = 0x44,
        _NOP_Zero_3     = 0x64,
        _NOP_Zero_X_1   = 0x14,
        _NOP_Zero_X_2   = 0x34,
        _NOP_Zero_X_3   = 0x54,
        _NOP_Zero_X_4   = 0x74,
        _NOP_Zero_X_5   = 0xd4,
        _NOP_Zero_X_6   = 0xf4,

        // Load Accumulator into X register
        _LAX_Abs        = 0xaf,
        _LAX_Abs_Y      = 0xbf,
        _LAX_Zero       = 0xa7,
        _LAX_Zero_Y     = 0xb7,
        _LAX_Ind_X      = 0xa3,
        _LAX_Ind_Y      = 0xb3,

        // Store bitwise and of Accumulator and X register
        _SAX_Abs        = 0x8f,
        _SAX_Zero       = 0x87,
        _SAX_Zero_Y     = 0x97,
        _SAX_Ind_X      = 0x83,

        // SuBtract with Carry
        _SBC_Imm        = 0xeb,

        // Decrement value, ComPare accumulator
        _DCP_Abs        = 0xcf,
        _DCP_Abs_X      = 0xdf,
        _DCP_Abs_Y      = 0xdb,
        _DCP_Zero       = 0xc7,
        _DCP_Zero_X     = 0xd7,
        _DCP_Ind_X      = 0xc3,
        _DCP_Ind_Y      = 0xd3,

        // Increment value, SuBtract with carry
        _ISB_Abs        = 0xef,
        _ISB_Abs_X      = 0xff,
        _ISB_Abs_Y      = 0xfb,
        _ISB_Zero       = 0xe7,
        _ISB_Zero_X     = 0xf7,
        _ISB_Ind_X      = 0xe3,
        _ISB_Ind_Y      = 0xf3,

        // arithmetic Shift Left, bitwise Or with accumulator
        _SLO_Abs        = 0x0f,
        _SLO_Abs_X      = 0x1f,
        _SLO_Abs_Y      = 0x1b,
        _SLO_Zero       = 0x07,
        _SLO_Zero_X     = 0x17,
        _SLO_Ind_X      = 0x03,
        _SLO_Ind_Y      = 0x13,

        // Rotate Left, And with accumulator
        _RLA_Abs        = 0x2f,
        _RLA_Abs_X      = 0x3f,
        _RLA_Abs_Y      = 0x3b,
        _RLA_Zero       = 0x27,
        _RLA_Zero_X     = 0x37,
        _RLA_Ind_X      = 0x23,
        _RLA_Ind_Y      = 0x33,

        // logical Shift Right, Exclusive or with accumulator
        _SRE_Abs        = 0x4f,
        _SRE_Abs_X      = 0x5f,
        _SRE_Abs_Y      = 0x5b,
        _SRE_Zero       = 0x47,
        _SRE_Zero_X     = 0x57,
        _SRE_Ind_X      = 0x43,
        _SRE_Ind_Y      = 0x53,

        // Rotate Right, Add with carry
        _RRA_Abs        = 0x6f,
        _RRA_Abs_X      = 0x7f,
        _RRA_Abs_Y      = 0x7b,
        _RRA_Zero       = 0x67,
        _RRA_Zero_X     = 0x77,
        _RRA_Ind_X      = 0x63,
        _RRA_Ind_Y      = 0x73,
    }
}

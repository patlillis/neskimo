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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = format!("{:?}", self);
        write!(f, "{}", &name[..3])
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

        // Arithmetic Shift Left
        ASL_Acc     = 0x0a,
        ASL_Zero    = 0x06,
        ASL_Zero_X  = 0x16,
        ASL_Abs     = 0x0e,
        ASL_Abs_X   = 0x1e,

        // test BITs
        BIT_Zero    = 0x24,
        BIT_Abs     = 0x2c,

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

        // INCrement memory
        INC_Zero    = 0xe6,
        INC_Zero_X  = 0xf6,
        INC_Abs     = 0xee,
        INC_Abs_X   = 0xfe,

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
    }
}

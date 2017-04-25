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

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
    #[allow(non_camel_case_types)]
    pub enum Opcode {
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
    }
}

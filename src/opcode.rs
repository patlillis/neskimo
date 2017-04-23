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

impl std::fmt::Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#04x}", *self as u8)
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Opcode {
        // LoaD Accumulator
        LDA_Imm     = 0xa9,
        // LDA_Zero    = 0xa5,
        // LDA_Zero_X  = 0xb5,
        // LDA_Abs     = 0xad,
        // LDA_Abs_X   = 0x9d,
        // LDA_Abs_Y   = 0x99,
        // LDA_Ind_X   = 0x81,
        // LDA_Ind_Y   = 0x91

        // STore Accumulator
        STA_Zero    = 0x85,
        STA_Zero_X  = 0x95,
        STA_Abs     = 0x8d,
        STA_Abs_X   = 0x9d,
        STA_Abs_Y   = 0x99,
        STA_Ind_X   = 0x81,
        STA_Ind_Y   = 0x91
    }
}

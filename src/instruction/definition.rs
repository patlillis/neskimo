use opcode;

// Details about an instruction.
pub struct InstructionDefinition {
    pub len: u16,
    pub cycles: u8,
}

// Get an instruction definition, based on opcode.
pub fn lookup_instruction_definition(opcode: opcode::Opcode) -> InstructionDefinition {
    use opcode::Opcode::*;
    use instruction::definition::InstructionDefinition as def;

    match opcode {
        LDA_Imm => def { len: 2, cycles: 2 },

        STA_Zero => def { len: 2, cycles: 3 },
        STA_Zero_X => def { len: 2, cycles: 4 },
        STA_Abs => def { len: 3, cycles: 4 },
        STA_Abs_X => def { len: 3, cycles: 5 },
        STA_Abs_Y => def { len: 3, cycles: 5 },
        STA_Ind_X => def { len: 2, cycles: 6 },
        STA_Ind_Y => def { len: 2, cycles: 6 },
    }
}

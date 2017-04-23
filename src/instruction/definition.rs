use opcode::Opcode;
use std::collections::HashMap;

// Details about an instruction.
pub struct InstructionDefinition {
    pub mneumonic: &'static str,
    pub len: u16,
    pub cycles: u8,
}

lazy_static! {
    static ref INSTRUCTION_DEFINITIONS: HashMap<Opcode, InstructionDefinition> = {
        let mut m = HashMap::new();

        m.insert(Opcode::LDA_Imm, InstructionDefinition {
            mneumonic: "LDA",
            len: 2,
            cycles: 2
        });

        m.insert(Opcode::STA_Zero, InstructionDefinition {
            mneumonic: "STA",
            len: 2,
            cycles: 3
        });

        m.insert(Opcode::STA_Zero_X, InstructionDefinition {
            mneumonic: "STA",
            len: 2,
            cycles: 4
        });

        m.insert(Opcode::STA_Abs, InstructionDefinition {
            mneumonic: "STA",
            len:3,
            cycles: 4
        });

        m.insert(Opcode::STA_Abs_X, InstructionDefinition {
            mneumonic: "STA",
            len: 3,
            cycles: 5
        });

        m.insert(Opcode::STA_Abs_Y, InstructionDefinition {
            mneumonic: "STA",
            len: 3,
            cycles: 5
        });

        m.insert(Opcode::STA_Ind_X, InstructionDefinition {
            mneumonic: "STA",
            len: 2,
            cycles: 6
        });

        m.insert(Opcode::STA_Ind_Y, InstructionDefinition {
            mneumonic: "STA",
            len: 2,
            cycles: 6
        });

        m
    };
}

// Get an instruction definition for a given opcode.
pub fn lookup_instruction_definition<'a, 'b>(opcode: &'a Opcode) -> &'b InstructionDefinition {
    match INSTRUCTION_DEFINITIONS.get(&opcode) {
        Some(i) => i,
        None => panic!("Unexpected opcode: {}", opcode),
    }
}

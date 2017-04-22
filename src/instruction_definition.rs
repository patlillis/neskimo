use opcode::Opcode;
use std::collections::HashMap;

pub struct InstructionDefinition {
    pub mneumonic: &'static str,
    pub len: u8,
    pub cycles: u8,
}

lazy_static! {
    static ref INSTRUCTION_DEFINITIONS: HashMap<Opcode, InstructionDefinition> = {
        let mut m = HashMap::new();

        m.insert(Opcode::LDA_Imm, InstructionDefinition {
            mneumonic: "LDA",
            len: 2,
            cycles: 3
        });

        m
    };
}

pub fn lookup_instruction_definition(opcode: &Opcode) -> &InstructionDefinition {
    match INSTRUCTION_DEFINITIONS.get(opcode) {
        Some(i) => i,
        None => panic!("Unexpected Opcode: {}", opcode)
    }
}
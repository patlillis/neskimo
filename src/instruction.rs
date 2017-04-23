use cpu;
use memory;
use opcode;
use std;
use utils;

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

// An instruction.
//
// First byte is opcode. Seconds and third are optional arguments.
pub struct Instruction(pub u8, pub u8, pub u8);

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:#04x}, {:#04x}, {:#04x})", self.0, self.1, self.2)
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Instruction {
    // Parse an instruction from a specifc point in memory.
    //
    // If the instruction takes arguments, they will be read from subsequent locations.
    pub fn parse<'a>(pc: u16, memory: &memory::Memory) -> (Instruction, &'a InstructionDefinition) {
        let raw_opcode = memory.fetch(pc);
        let opcode = opcode::decode(raw_opcode);
        let def = lookup_instruction_definition(&opcode);
        (match def.len {
             1 => Instruction(raw_opcode, 0, 0),
             2 => Instruction(raw_opcode, memory.fetch(pc + 1), 0),
             3 => Instruction(raw_opcode, memory.fetch(pc + 1), memory.fetch(pc + 2)),
             _ => panic!("Invalid instruction length far opcode {}", opcode),
         },
         def)
    }

    // Execute the instruction on the cpu.
    pub fn execute(&self, cpu: &mut cpu::Cpu) {
        let opcode = self.0;
        let arg1 = self.1;
        let arg2 = self.2;

        match opcode::decode(opcode) {
            Opcode::LDA_Imm => {
                cpu.lda(arg1);
            }
            Opcode::STA_Abs => {
                let address = utils::arithmetic::concat_bytes(arg1, arg2);
                cpu.sta(address);
            }
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }
}

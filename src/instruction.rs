use cpu;
use memory;
use opcode;
use std;

use opcode::Opcode;
use std::collections::HashMap;

// Details about an instruction.
pub struct InstructionDefinition {
    pub opcode: Opcode,
    pub mneumonic: &'static str,
    pub len: u8,
    pub cycles: u8,
}

lazy_static! {
    static ref INSTRUCTION_DEFINITIONS: HashMap<Opcode, InstructionDefinition> = {
        let mut m = HashMap::new();

        m.insert(Opcode::LDA_Imm, InstructionDefinition {
            mneumonic: "LDA",
            opcode: Opcode::LDA_Imm,
            len: 2,
            cycles: 3
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
pub struct Instruction(u8, u8, u8);

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
    pub fn parse(pc: u16, memory: &memory::Memory) -> (Instruction, &InstructionDefinition) {
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
    pub fn execute(&self, cpu: &cpu::Cpu) {
        let opcode = self.0;
        let arg1 = self.1;
        let arg2 = self.2;

        match opcode::decode(opcode) {
            Opcode::LDA_Imm => {
                println!("Executing LDA!");
            }
            _ => panic!("Unexpected opcode: {}", opcode),
        }

        //     return match inst.opcode {
        //                // LDA
        //                Opcode::LDA => {
        //         println!("Executing LDA!");
        //         15
        //     }
        //                // STA (for now only stores to 0xffff)
        //                Opcode::STA => {
        //         println!("Executing STA!");
        //         12
        //     }
        //                _ => panic!("Unexpected Opcode: {}", inst.opcode),
        //            };
        // }
    }
}
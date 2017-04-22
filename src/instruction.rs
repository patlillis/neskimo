use cpu;
use memory;
use std;

use opcode::Opcode;
use instruction_definition::{InstructionDefinition, lookup_instruction_definition};

// An instruction.
//
// First byte is opcode. Seconds and third are optional arguments.
pub struct Instruction(u8, u8, u8);


// pub fn new() -> InstructionTable {
//     let mut table = InstructionTable { opcodes: std::collections::HashMap::new() };

//     // LDA
//     table.add_instruction(Instruction {
//                               mneumonic: "LDA".to_string(),
//                               opcode: Opcode::LDA_Imm,
//                               cycles: 2,
//                               len: 2,
//                           });

//     // STA
//     table.add_instruction(Instruction {
//                               mneumonic: "STA".to_string(),
//                               opcode: Opcode::STA_Abs,
//                               cycles: 3,
//                               len: 4,
//                           });
//     return table;
// }

impl Instruction {
    pub fn parse(pc: usize, memory: &memory::Memory) -> (Instruction, &InstructionDefinition) {
        // let opcde = memory.fe
        (Instruction(0,0,0), lookup_instruction_definition(&Opcode::LDA_Imm))
    }

    pub fn execute(&self, cpu: &cpu::Cpu) {
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
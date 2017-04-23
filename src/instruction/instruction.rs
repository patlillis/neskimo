use cpu;
use memory;
use opcode;
use std;
use utils;
use instruction::definition::{InstructionDefinition, lookup_instruction_definition};

use opcode::Opcode;

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
    pub fn parse(pc: u16, memory: &memory::Memory) -> (Instruction, InstructionDefinition) {
        let raw_opcode = memory.fetch(pc);
        let opcode = opcode::decode(raw_opcode);
        let def = lookup_instruction_definition(opcode);
        (match def.len {
             1 => Instruction(raw_opcode, 0, 0),
             2 => Instruction(raw_opcode, memory.fetch(pc + 1), 0),
             3 => Instruction(raw_opcode, memory.fetch(pc + 1), memory.fetch(pc + 2)),
             _ => panic!("Invalid instruction length far opcode {}", opcode),
         },
         def)
    }

    fn opcode(&self) -> u8 {
        self.0
    }

    fn arg1(&self) -> u8 {
        self.1
    }

    fn arg2(&self) -> u8 {
        self.2
    }

    // Get the absolute address from the instruction args.
    fn absolute_address(&self) -> u16 {
        utils::arithmetic::concat_bytes(self.arg1(), self.arg2())
    }

    // Execute the instruction on the cpu.
    pub fn execute(&self, cpu: &mut cpu::Cpu) {
        let opcode = opcode::decode(self.opcode());

        match opcode {
            Opcode::LDA_Imm => {
                // Load arg1 directly into accumulator.
                cpu.lda(self.arg1());
            }
            Opcode::STA_Abs => {
                // Store accumulator into address in arguments.
                cpu.sta(self.absolute_address());
            }
            _ => panic!("Unexpected opcode: {}", self.opcode()),
        }
    }
}

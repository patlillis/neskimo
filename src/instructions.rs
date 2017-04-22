use cpu;
use std;

// The 8-bit op code corresponding to an instruction.
pub type Opcode = u8;

// How long an instruction takes.
pub type CycleCount = u16;

// The result of executing an instruction.
pub type InstructionStatus = u16;

// An instruction.
#[derive(Default)]
pub struct Instruction {
    pub mneumonic: String,
    pub opcode: Opcode,
    pub cycles: CycleCount,
    pub page_cross_cycles: CycleCount,
}

pub struct InstructionTable {
    opcodes: std::collections::HashMap<Opcode, Instruction>,
}

impl InstructionTable {
    pub fn get_instruction(&self, opcode: Opcode) -> Option<&Instruction> {
        self.opcodes.get(&opcode)
    }

    fn add_instruction(&mut self, inst: Instruction) {
        self.opcodes.insert(inst.opcode, inst);
    }

    fn remove_instruction(&mut self, inst: Instruction) {
        self.opcodes.remove(&inst.opcode);
    }

    fn remove_opcode(&mut self, opcode: Opcode) {
        self.opcodes.remove(&opcode);
    }

    pub fn new() -> InstructionTable {
        let mut table = InstructionTable { opcodes: std::collections::HashMap::new() };

        table.add_instruction(Instruction {
                                  mneumonic: "LDA".to_string(),
                                  opcode: 0xa1,
                                  cycles: 3,
                                  ..Default::default()
                              });

        return table;
    }

    pub fn execute_instruction(&self, cpu: &cpu::Cpu, inst: &Instruction) -> InstructionStatus {
        return match inst.opcode {
                   // LDA
                   0xa1 => {
            println!("Executing LDA!");
            15
        }
                   _ => panic!("Unexpected Opcode: {}", inst.opcode),
               };
    }
}
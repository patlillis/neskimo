use cpu;
use std;

// The 8-bit op code corresponding to an instruction.
pub type OpCode = u8;

// How long an instruction takes.
pub type CycleCount = u16;

// The result of executing an instruction.
pub type InstructionStatus = u16;

// An instruction.
pub struct Instruction {
    mneumonic: String,
    opCode: OpCode,
    exec: fn(&cpu::Cpu) -> InstructionStatus,
    cycles: CycleCount,
    pageCrossCycles: CycleCount,
}

pub struct InstructionTable {
    opCodes: std::collections::HashMap<OpCode, Instruction>,
}

impl InstructionTable {
    pub fn addInstruction(&mut self, inst: Instruction) {
        self.opCodes.insert(inst.opCode, inst);
    }

    pub fn removeInstruction(&mut self, inst: Instruction) {
        self.opCodes.remove(&inst.opCode);
    }

    pub fn removeOpCode(&mut self, opCode: OpCode) {
        self.opCodes.remove(&opCode);
    }

    pub fn execute(&self, cpu: &cpu::Cpu, opCode: OpCode) -> CycleCount {
        let inst = match self.opCodes.get(&opCode) {
            Some(i) => i,
            None => panic!("Unexpected OpCode: {}", opCode),
        };

        let status = (inst.exec)(cpu);

        // TODO: do something with status.

        // TODO: do something with page cross.

        return inst.cycles;
    }

    pub fn new() -> InstructionTable {
        let mut table = InstructionTable { opCodes: std::collections::HashMap::new() };


        table.addInstruction(Instruction {
            mneumonic: 'LDA',

        })

        return table;
    }
}
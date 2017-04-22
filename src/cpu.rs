use std;
use memory;
use instructions;

// The status of the system processor.
pub struct Status(pub u8);

impl Status {
    // Constructs a new status with all flags set to 0.
    pub fn new() -> Status {
        Status(0x0)
    }

    // Helper function for testing a mask against a status.
    fn matches_bits(&self, mask: u8) -> bool {
        self.0 & mask != 0
    }

    // Bit 0: Carry flag.
    pub fn c(&self) -> bool {
        self.matches_bits(1 << 0)
    }

    // Bit 1: Zero flag.
    pub fn z(&self) -> bool {
        self.matches_bits(1 << 1)
    }

    // Bit 2: Interrupt flag.
    pub fn i(&self) -> bool {
        self.matches_bits(1 << 2)
    }

    // Bit 3: Decimal mode.
    pub fn d(&self) -> bool {
        self.matches_bits(1 << 3)
    }

    // Bit 4: Break command.
    pub fn b(&self) -> bool {
        self.matches_bits(1 << 4)
    }

    // Bit 5: Unused.

    // Bit 6: Overflow flag.
    pub fn v(&self) -> bool {
        self.matches_bits(1 << 6)
    }

    // Bit 7: Negative flag.
    pub fn n(&self) -> bool {
        self.matches_bits(1 << 7)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status({:#04x})", self.0)
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Registers {
    // Accumulator.
    pub a: u8,
    // Index register X.
    pub x: u8,
    // Index register Y.
    pub y: u8,
    // Processor status.
    pub p: Status,
    // Stack pointer.
    pub sp: u8,
    // Program counter.
    pub pc: u16,
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "Registers(\
                a: {:#04x}, \
                x: {:#04x}, \
                y: {:#04x}, \
                p: {}, \
                sp: {:#04x}, \
                pc: {:#06x})",
               self.a,
               self.x,
               self.y,
               self.p,
               self.sp,
               self.pc)
    }
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x0,
            x: 0x0,
            y: 0x0,
            p: Status::new(),
            sp: 0x0,
            pc: 0x0,
        }
    }
}

pub struct Cpu {
    pub registers: Registers,
    pub instructions: instructions::InstructionTable,
    pub memory: memory::Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            instructions: instructions::InstructionTable::new(),
            memory: memory::Memory::new()
        }
    }

    // Executes the instruction at PC.
    pub fn execute(&self) {
        let opcode = self.memory.fetch(self.registers.pc);
        self.execute_opcode(opcode);

        //TODO: increment PC.
    }

    // Executes the given opcode.
    fn execute_opcode(&self, opcode: instructions::Opcode) -> instructions::CycleCount {
        let inst = match self.instructions.get_instruction(opcode) {
            Some(i) => i,
            None => panic!("Unexpected Opcode: {}", opcode),
        };

        let status = self.instructions.execute_instruction(self, &inst);

        // TODO: do something with status.

        // TODO: do something with page cross.

        return inst.cycles;
    }

}
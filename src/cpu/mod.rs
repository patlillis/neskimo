use cpu_state::{CpuFlags, CpuVectors};

use self::cpu_state::CpuState;
use self::instruction::Instruction;
use crate::nes::memory::Memory;
use crate::utils::arithmetic::is_negative;

pub mod cpu_operation;
pub mod cpu_state;
pub mod definition;
pub mod instruction;
pub mod memory;
pub mod opcode;

// Tests for the CPU.
#[cfg(test)]
mod cpu_test;

// Log from a single frame of execution.
#[derive(Debug, Default)]
pub struct Log {
    // The Program Counter.
    pub pc: u16,

    // This includes the opcode and any args.
    // E.g. "4C F5 C5"
    pub instruction: String,

    // The 3-letter mneumonic, e.g. "JMP".
    pub mneumonic: String,

    // Decoded args, e.g. "#$00".
    pub decoded_args: String,

    // Registers.
    pub registers: String,
}

impl Log {
    pub fn log(&self) -> String {
        format!(
            "{:04X}  {:8} {:3} {:26}  {}",
            self.pc,
            self.instruction,
            self.mneumonic,
            self.decoded_args,
            self.registers
        )
    }
}

pub struct Cpu {
    state: CpuState,
    pub frame_log: Log,
    mem_dump_pc: Option<u16>,
}

impl Cpu {
    pub fn new(
        memory: &Memory,
        program_counter: Option<u16>,
        mem_dump_counter: Option<u16>,
    ) -> Cpu {
        // Get the PC from the RESET vector pointer.
        let pc = match program_counter {
            Some(pc) => pc,
            None => memory.fetch_u16(CpuVectors::RESET),
        };

        Cpu {
            state: CpuState::new(pc),
            frame_log: Log {
                ..Default::default()
            },
            mem_dump_pc: mem_dump_counter,
        }
    }

    pub fn reset(&mut self) {
        self.reset_to_pc(0x0000);
    }

    pub fn reset_to_pc(&mut self, pc: u16) {
        self.registers.reset_to_pc(pc);
        self.irq = false;
        self.nmi = false;
        self.reset = false;
    }

    pub fn decode_operand_value(&mut self, operand: u8) {
        self.frame_log
            .decoded_args
            .push_str(format!(" = {:02X}", operand).as_str());
    }

    pub fn decode_operand_accumulator(&mut self) {
        self.frame_log.decoded_args.push('A');
    }

    // TODO: put somewhere else.
    // pub fn mem_dump(&self) {
    //     if let Ok(mut file) = File::create("mem-dump.bin") {
    //         self.memory.dump(&mut file).ok();
    //     }
    // }

    // Executes the instruction at PC and returns the number of cycles taken.
    pub fn execute(&mut self, memory: &Memory) -> u32 {
        self.frame_log = Log {
            pc: self.registers.pc,
            registers: self.registers.log(),
            ..Default::default()
        };

        // Dump memory, if option was passed in.
        // match self.mem_dump_pc {
        //     Some(pc) if pc == self.registers.pc => self.mem_dump(),
        //     _ => {}
        // }

        let instruction_location = self.registers.pc;
        let (instr, definition) =
            Instruction::parse(instruction_location, self);

        // Increment program counter.
        self.registers.pc += definition.len;

        // Execute the instruction.
        let cycles = instr.execute(self, instruction_location);

        // Check interrupts.
        self.check_interrupts(memory);

        u32::from(cycles)
    }

    // Checks the interrupt lines, and sets the pc to the
    // value in the correct interrupt vector if neccesary.
    fn check_interrupts(&mut self, memory: &Memory) {
        if self.irq && !self.registers.p.i() {
            self.handle_irq(memory);
            self.irq = false;
        } else if self.nmi {
            self.handle_nmi(memory);
            self.nmi = false;
        } else if self.reset {
            self.handle_reset(memory);
            self.reset = false;
        }
    }

    // Handle interrupt on the IRQ line.
    fn handle_irq(&mut self, memory: &Memory) {
        // Push return address and status onto stack. CpuFlags::U is 1, CpuFlags::B is 0.
        let pc = self.registers.pc;
        let status = (self.registers.p.0 | CpuFlags::U) & !CpuFlags::B;
        self.push_u16(pc);
        self.push(status);

        // Turn on interrupt disable.
        self.registers.p.set_i(true);

        // Fetch memory from IRQ vector.
        let vector = memory.fetch_u16(CpuVectors::IRQ);
        self.registers.pc = vector;
    }

    // Handle interrupt on the NMI line.
    fn handle_nmi(&mut self, memory: &Memory) {
        // Push return address and status onto stack. CpuFlags::U is 1, CpuFlags::B is 0.
        let pc = self.registers.pc;
        let status = (self.registers.p.0 | CpuFlags::U) & !CpuFlags::B;
        self.push_u16(pc);
        self.push(status);

        // Turn on interrupt disable.
        self.registers.p.set_i(true);

        // Fetch memory from NMI vector.
        let vector = memory.fetch_u16(CpuVectors::NMI);
        self.registers.pc = vector;
    }

    // Handle interrupt on the RESET line. Note that in the original 6502,
    // a RESET actually triggered the same sequence as IRQ and NMI, but with
    // the read/write bus set to "read", so no memory was modified. However,
    // the stack pointer was decremented 3 times, which is why the stack pointer
    // on startup is set to 0xfd (0x00 - 3).
    fn handle_reset(&mut self, memory: &Memory) {
        let vector = memory.fetch_u16(CpuVectors::RESET);
        self.registers.pc = vector;
    }

    fn set_z_flag(&mut self, value: u8) {
        match value {
            0 => self.registers.p.0 |= CpuFlags::Z,
            _ => self.registers.p.0 &= !CpuFlags::Z,
        };
    }

    fn set_n_flag(&mut self, value: u8) {
        if is_negative(value) {
            self.registers.p.0 |= CpuFlags::N;
        } else {
            self.registers.p.0 &= !CpuFlags::N;
        }
    }
}

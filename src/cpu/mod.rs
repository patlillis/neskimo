// Re-export all symbols so that consumers can write "nes::cpu::Cpu" instead of
// "nes::cpu::cpu::Cpu".
mod cpu;
pub use self::cpu::*;

mod definition;
pub use self::definition::*;

mod instruction;
pub use self::instruction::*;

mod opcode;
pub use self::opcode::*;

// Tests for the CPU.
#[cfg(test)]
mod cpu_test;

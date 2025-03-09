mod cpu;
pub use self::cpu::*;

pub mod definition;
pub mod instruction;
pub mod opcode;

// Tests for the CPU.
#[cfg(test)]
mod cpu_test;

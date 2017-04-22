#![allow(dead_code)]

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;
extern crate num;

mod clock;
mod cpu;
mod instruction;
mod instruction_definition;
mod memory;
mod opcode;
mod utils;

use opcode::Opcode;

fn main() {
    let mut x = cpu::Registers::new();
    x.a = 0xf;
    x.pc = 0xfffc;
    let mut cpu = cpu::Cpu::new();
    execute(&mut cpu);
}

fn execute(cpu: &mut cpu::Cpu) {
    // Simple program that shuffles data around, with 0xff ending
    // up in the accumulator.

    let val = 0x24;
    cpu.memory.store(0x0000, val);

    // Load from initial position.
    cpu.memory.store(0x0f00, Opcode::LDA_Imm as u8);

    // Store to other position (for now only stores to 0xffff).
    cpu.memory.store(0x0f01, Opcode::STA_Abs as u8);

    // Load from other position.
    cpu.memory.store(0x0f02, Opcode::LDA_Imm as u8);

    cpu.registers.pc = 0x0000;
    cpu.execute();
    cpu.execute();
    // Fetch opcode.
    // let opcode = Opcode(cpu.memory.fetch(cpu.registers.pc));
    // let instruction = None;

    // Execute instruction update on CPU, and get number of cycles.
    // let cycles = instruction.exec(cpu);

}
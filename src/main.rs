#![allow(dead_code)]

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;
extern crate num;

mod clock;
mod cpu;
mod instruction;
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

    cpu.memory.store(0x0000, Opcode::LDA_Imm as u8);
    cpu.registers.pc = 0x0000;
    cpu.execute();
}
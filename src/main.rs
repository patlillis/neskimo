#![allow(dead_code)]

// extern crate time;
// extern crate timer;
extern crate enum_primitive;

mod clock;
mod cpu;
mod instructions;
mod memory;

// use timer;
// use time;

// use cpu::{Status, Registers};

fn main() {
    let mut x = cpu::Registers::new();
    x.a = 0xf;
    x.pc = 0xfffc;
    let cpu = cpu::Cpu::new();
    execute(&cpu);
}

fn execute(cpu: &cpu::Cpu) {
    let op = 0xa1;

    cpu.execute(op);
    // Fetch opcode.
    // let opcode = Opcode(cpu.memory.fetch(cpu.registers.pc));
    // let instruction = None;

    // Execute instruction update on CPU, and get number of cycles.
    // let cycles = instruction.exec(cpu);

}
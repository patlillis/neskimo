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
    let mut cpu = cpu::Cpu::new();
    execute(&mut cpu);
}

fn execute(cpu: &mut cpu::Cpu) {
    // Store opcode and set pc to execute it.
    cpu.memory.store(0x0000, 0xa1);
    cpu.memory.store(0x00ff, 0x85);

    cpu.registers.pc = 0x0000;
    cpu.execute();

    cpu.registers.pc = 0x00ff;
    cpu.execute();
    // Fetch opcode.
    // let opcode = Opcode(cpu.memory.fetch(cpu.registers.pc));
    // let instruction = None;

    // Execute instruction update on CPU, and get number of cycles.
    // let cycles = instruction.exec(cpu);

}
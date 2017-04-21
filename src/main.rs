#![allow(dead_code)]
#![feature(io)]

// extern crate time;
// extern crate timer;

mod clock;
mod cpu;
mod memory;

use timer;
use time;

use cpu::{Status, Registers};

fn main() {
    let mut x = Registers::new();
    x.a = 0xf;
    x.pc = 0xfffc;
    println!("{:?}", x);
    println!("{}", x.p);
}

fn Execute(cpu: &cpu::Cpu) {
    // Fetch opcode.
    let opcode = Opcode(cpu.memory.fetch(cpu.registers.pc));
    let instruction = None;

    // Execute instruction update on CPU, and get number of cycles.
    let cycles = instruction.exec(cpu);

}
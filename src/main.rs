#![allow(dead_code)]

#[macro_use]
extern crate enum_primitive;

extern crate num;

mod clock;
mod cpu;
mod instructions;
mod memory;
mod opcode;
mod utils;

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
    cpu.memory.store(0x0001, 0x85);

    cpu.registers.pc = 0x0000;
    cpu.execute();
    cpu.execute();
    // Fetch opcode.
    // let opcode = Opcode(cpu.memory.fetch(cpu.registers.pc));
    // let instruction = None;

    // Execute instruction update on CPU, and get number of cycles.
    // let cycles = instruction.exec(cpu);

}
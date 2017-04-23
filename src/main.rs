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
    // Simple program that shuffles data around.

    let val = 18;
    let final_address = 0xab;

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[Opcode::LDA_Imm as u8,
                       val,
                       Opcode::STA_Abs as u8,
                       0x00,
                       final_address]);

    // Execute 2 instructions.
    cpu.execute();
    cpu.execute();

    // Check value in final_address.
    println!("Value in final address: {}", cpu.memory.fetch(final_address as u16));
}
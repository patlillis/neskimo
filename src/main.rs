#![allow(dead_code)]

#[macro_use]
extern crate enum_primitive;
extern crate num;

mod clock;
mod cpu;
mod instruction;
mod memory;
mod opcode;
mod utils;

fn main() {
    let mut x = cpu::Registers::new();
    x.a = 0xf;
    x.pc = 0xfffc;
    let mut cpu = cpu::Cpu::new();
    test(&mut cpu);
}

// Tests several instructions.
fn test(cpu: &mut cpu::Cpu) {
    // TODO: Actually load into X from memory.
    cpu.reset();
    cpu.registers.x = 0x01;

    // Store program in memory.
    use opcode::Opcode::*;
    let val = 18;
    cpu.memory
        .store_bytes(0x0000,
                     &[// Load value into accumulator.
                       LDA_Imm as u8,
                       val,

                       // Store to 0xffab
                       STA_Abs as u8,
                       0xff,
                       0xab,

                       // Store to 0x01ab
                       STA_Abs_X as u8,
                       0x01,
                       0xaa,

                       // Store to 0x00ab
                       STA_Zero as u8,
                       0xab,

                       // Store to 0x00ac
                       STA_Zero_X as u8,
                       0xab]);

    // Check value in final addresses.
    let final_addresses = [0xffab, 0x01ab, 0x00ab, 0x00ac];

    // Execute the load.
    cpu.execute();

    // Execute the correct number of stores.
    for _ in 0..final_addresses.len() {
        cpu.execute();
    }


    for addr in final_addresses.iter() {
        assert_eq!(val,
                   cpu.memory.fetch(*addr),
                   "Bad value at addr {:#06x}",
                   addr);
    }
}
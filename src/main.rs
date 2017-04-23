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
    cpu.reset();

    // TODO: Actually load into X from memory.
    cpu.registers.x = 0xff;
    cpu.registers.y = 0xfe;

    // For executing the indirect instruction lookups.
    cpu.memory.store_u16(0x00d0, 0xffd0);
    cpu.memory.store_u16(0x00e0, 0xffe0);

    // Store program in memory.
    use opcode::Opcode::*;
    let val = 18;
    cpu.memory
        .store_bytes(0x0000,
                     &[// Load value into accumulator.
                       LDA_Imm as u8,
                       val,

                       // Store to 0x00ab
                       STA_Zero as u8,
                       0xab,

                       // Store to 0x00ac
                       STA_Zero_X as u8,
                       0xad,

                       // Store to 0xffab
                       STA_Abs as u8,
                       0xff,
                       0xab,

                       // Store to 0x00aa
                       STA_Abs_X as u8,
                       0xff,
                       0xab,

                       // Store to 0x00a9
                       STA_Abs_Y as u8,
                       0xff,
                       0xab,

                       // Store to 0xffd0
                       STA_Ind_X as u8,
                       0xd1,

                       // Store to 0x00de
                       STA_Ind_Y as u8,
                       0xe0]);

    // Check value in final addresses.
    let final_addresses = [0xffab, 0x00aa, 0x00a9, 0x00ab, 0x00ac, 0xffd0, 0x00de];

    // Execute the load.
    cpu.execute();

    // Execute the correct number of stores.
    for _ in 0..final_addresses.len() {
        cpu.execute();
    }

    // Check the results in memory.
    for addr in final_addresses.iter() {
        assert_eq!(val,
                   cpu.memory.fetch(*addr),
                   "Bad value at addr {:#06x}",
                   addr);
    }
}
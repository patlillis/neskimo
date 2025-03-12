/*
Random thoughts

```rust
struct Memory {
    cpu: CPU,
    ppu: PPU,
    apu: APU,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Memory {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.cpu.ram[(address & 0x07FF) as usize], // RAM
            0x2000..=0x3FFF => self.ppu_read(address & 0x0007), // PPU Registers
            0x4000..=0x4017 => self.apu_read(address), // APU Registers
            0x8000..=0xFFFF => self.prg_rom[(address - 0x8000) as usize], // PRG ROM
            _ => 0, // Undefined address
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.cpu.ram[(address & 0x07FF) as usize] = value, // RAM
            0x2000..=0x3FFF => self.ppu_write(address & 0x0007, value), // PPU Registers
            0x4000..=0x4017 => self.apu_write(address, value), // APU Registers
            0x8000..=0xFFFF => { /* Handle PRG ROM, typically not writable */ },
            _ => {}
        }
    }

    fn ppu_read(&self, address: u16) -> u8 {
        // Implement PPU register reading logic
    }

    fn ppu_write(&mut self, address: u16, value: u8) {
        // Implement PPU register writing logic
    }

    fn apu_read(&self, address: u16) -> u8 {
        // Implement APU register reading logic
    }

    fn apu_write(&mut self, address: u16, value: u8) {
        // Implement APU register writing logic
    }
}
*/


// use crate::nes::memory::Memory;
// use crate::ppu::vram::Vram;
// use crate::rom::MirrorType;

pub struct PpuMemory {
    vram: 
}

// pub struct InternalMemory {
//     vram: Vram,
// }

// impl InternalMemory {
//     pub fn new(nametable_mirror_type: MirrorType) -> InternalMemory {
//         InternalMemory {
//             vram: Vram::new(nametable_mirror_type),
//         }
//     }
// }

// impl Memory for InternalMemory {
//     // Fetches a byte from the specified address in memory.
//     fn fetch(&self, address: u16) -> u8 {
//         match address {
//             // Pattern tables, normally mapped by the cartridge to a CHR-ROM or
//             // CHR-RAM.
//             0x0000..=0x1fff => 0x00,

//             // 2kB VRAM, with special mirroring configuration. Can be remapped
//             // to cartridge RAM, allowing up to 4 simultaneous nametables.
//             0x2000..=0x2fff => self.vram.fetch(address),

//             // Usually mirrored to $2000-$2eff.
//             0x3000..=0x3eff => self.vram.fetch(address - 0x1000),

//             // Not configurable, always mapped to the interal palette control.
//             0x3f00..=0x3fff => 0x03,

//             _ => 0xff,
//         }
//     }

//     // Stores value into memory at the specified address.
//     // Returns the previous value.
//     fn store(&mut self, address: u16, value: u8) -> u8 {
//         match address {
//             // Pattern tables, normally mapped by the cartridge to a CHR-ROM or
//             // CHR-RAM.
//             0x0000..=0x1fff => 0x00,

//             // 2kB VRAM, with special mirroring configuration. Can be remapped
//             // to cartridge RAM, allowing up to 4 simultaneous nametables.
//             0x2000..=0x2fff => self.vram.store(address, value),

//             // Usually mirrored to $2000-$2eff.
//             0x3000..=0x3eff => self.vram.store(address - 0x1000, value),

//             // Not configurable, always mapped to the interal palette control.
//             0x3f00..=0x3fff => 0x03,

//             _ => 0xff,
//         }
//     }
// }

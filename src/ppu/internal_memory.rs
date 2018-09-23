use nes::memory::Memory;
use ppu::vram::Vram;
use rom::MirrorType;

pub struct InternalMemory {
    vram: Vram,
}

impl InternalMemory {
    pub fn new(nametable_mirror_type: &MirrorType) -> InternalMemory {
        InternalMemory {
            vram: Vram::new(nametable_mirror_type),
        }
    }
}

impl Memory for InternalMemory {
    // Fetches a byte from the specified address in memory.
    fn fetch(&self, address: u16) -> u8 {
        match address {
            // Pattern tables, normally mapped by the cartridge to a CHR-ROM or
            // CHR-RAM.
            0x0000...0x1fff => 0x00,

            // 2kB VRAM, with special mirroring configuration. Can be remapped
            // to cartridge RAM, allowing up to 4 simultaneous nametables.
            0x2000...0x2fff => self.vram.fetch(address),

            // Usually mirrored to $2000-$2eff.
            0x3000...0x3eff => self.vram.fetch(address - 0x1000),

            // Not configurable, always mapped to the interal palette control.
            0x3f00...0x3fff => 0x03,

            _ => 0xff,
        }
    }

    // Stores value into memory at the specified address.
    // Returns the previous value.
    fn store(&mut self, address: u16, value: u8) -> u8 {
        match address {
            // Pattern tables, normally mapped by the cartridge to a CHR-ROM or
            // CHR-RAM.
            0x0000...0x1fff => 0x00,

            // 2kB VRAM, with special mirroring configuration. Can be remapped
            // to cartridge RAM, allowing up to 4 simultaneous nametables.
            0x2000...0x2fff => self.vram.store(address, value),

            // Usually mirrored to $2000-$2eff.
            0x3000...0x3eff => self.vram.store(address - 0x1000, value),

            // Not configurable, always mapped to the interal palette control.
            0x3f00...0x3fff => 0x03,

            _ => 0xff,
        }
    }
}

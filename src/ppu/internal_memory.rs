use nes::memory::{MappedMemory, Memory};

struct InternalMemory {
    mapped_memory: MappedMemory,
    // vram: VramArray,
}

impl InternalMemory {
    // pub fn new() -> InternalMemory {
    //     // Inter
    // }
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
            0x2000...0x2fff => 0x01,

            // Usually mirrored to $2000-$2eff.
            0x3000...0x3eff => 0x02,

            // Not configurable, always mapped to the interal palette control.
            0x3f00...0x3fff => 0x03,

            _ => 0xff,
        }
    }

    // Stores value into memory at the specified address.
    // Returns the previous value.
    fn store(&mut self, address: u16, value: u8) -> u8 {
        // let old_value = self.backing_store[address as usize];
        // self.backing_store[address as usize] = value;
        // old_value
        0x00
    }
}

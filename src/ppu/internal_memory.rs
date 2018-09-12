// use nes::memory::{MappedMemory, Memory};

// // 4kB of VRAM.
// const _VRAM_SIZE: usize = 4096;

// // Type alias for an array of size MEMORY_SIZE.
// type _VramArray = [u8; _VRAM_SIZE];

// struct _InternalMemory {
//     mapped_memory: MappedMemory,
//     vram: _VramArray,
// }

// impl _InternalMemory {
//     // pub fn new() -> InternalMemory {
//     //     // Inter
//     // }
// }

// impl Memory for _InternalMemory {
//     // Fetches a byte from the specified address in memory.
//     fn fetch(&self, _address: u16) -> u8 {
//         // match address {
//         //     // Pattern tables, normally mapped by the cartridge to a CHR-ROM or
//         //     // CHR-RAM.
//         //     0x0000...0x1fff => 0x00,
//         //     // 0x2000...0x2fff =>
//         // }
//         0x00
//     }

//     // Stores value into memory at the specified address.
//     // Returns the previous value.
//     fn store(&mut self, _address: u16, _value: u8) -> u8 {
//         // let old_value = self.backing_store[address as usize];
//         // self.backing_store[address as usize] = value;
//         // old_value
//         0x00
//     }
// }

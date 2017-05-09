use std::fs::File;
use std::io::{Result, Write};
use utils;

const MEMORY_SIZE: usize = 65536;

// Type alias for an array of 2^16 unsigned bytes.
type BasicMemory = [u8; MEMORY_SIZE];

pub struct Memory {
    memory: BasicMemory,
}

// TODO: Little-endian?
impl Memory {
    pub fn new() -> Memory {
        Memory { memory: [0; MEMORY_SIZE] }
    }

    // Resets the memory to an initial state.
    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE];
    }

    // Dumps the memory contents to a string (most likely
    // for writing to a dump file).
    pub fn dump(&self, file: &mut File) -> Result<()> {
        file.write_all(&self.memory)
    }

    // Fetches a byte from the specified address in memory.
    pub fn fetch(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    // Fetches two bytes stored consecutively in memory.
    pub fn fetch_u16(&self, address: u16) -> u16 {
        let low = self.fetch(address);
        let high = self.fetch(address + 1);
        utils::arithmetic::concat_bytes(high, low)
    }

    // Fetches two bytes from memory.
    //
    // This method implements a bug found in the original MOS6502 hardware,
    // where the two bytes read had to be on the same page. So if the low
    // byte is stored at 0x33ff, then the high byte would be fetched from
    // 0x3300 instead of 0x3400.
    pub fn fetch_u16_wrap_msb(&self, address: u16) -> u16 {
        let low = self.fetch(address);
        let high = if address & 0x00ff == 0x00ff {
            self.fetch(address & 0xff00)
        } else {
            self.fetch(address + 1)
        };
        utils::arithmetic::concat_bytes(high, low)
    }

    // Stores value into memory at the specified address.
    // Returns the previous value.
    pub fn store(&mut self, address: u16, value: u8) -> u8 {
        let old_value = self.memory[address as usize];
        self.memory[address as usize] = value;
        old_value
    }

    // Stores to bytes consecutively in memory, with the first byte at the
    // specified address.
    // Returns the previous value.
    pub fn store_u16(&mut self, address: u16, value: u16) -> u16 {
        let high = (value >> 8) as u8;
        let low = value as u8;
        let low_prev = self.store(address, low);
        let high_prev = self.store(address + 1, high);
        utils::arithmetic::concat_bytes(high_prev, low_prev)
    }

    // Store a slice of bytes consecutively in memory, starting at the
    // specified address.
    pub fn store_bytes(&mut self, address: u16, bytes: &[u8]) {
        for (offset, byte) in bytes.iter().enumerate() {
            self.store(address + offset as u16, *byte);
        }
    }
}

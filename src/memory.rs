const MEMORY_SIZE: usize = 65536;

// Type alias for an array of 2^16 unsigned bytes.
type BasicMemory = [u8; MEMORY_SIZE];

pub struct Memory {
    memory: BasicMemory,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: [0; MEMORY_SIZE] }
    }

    // Resets the memory to an initial state.
    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE];
    }

    // Fetches a byte from the specified address in memory.
    pub fn fetch(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    // Stores value into memory at the specified address.
    pub fn store(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn store_bytes(&mut self, address: u16, bytes: &[u8]) {
        for (offset, byte) in bytes.iter().enumerate() {
            self.store(address + offset as u16, *byte);
        }
    }
}
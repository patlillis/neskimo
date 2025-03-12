pub const CPU_MEMORY_SIZE: usize = 65536; // 2^16 bytes

pub struct CpuMemory {
    ram: [u8; CPU_MEMORY_SIZE],
}

impl CpuMemory {
    // Create a new instance with the default NES memory size.
    pub fn new() -> Self {
        Self {
            ram: [0; CPU_MEMORY_SIZE],
        }
    }

    // Reads a byte from the given memory address.
    pub fn read(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    // Writes a byte to the given memory address.
    pub fn write(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_memory_read_write() {
        let mut mem = super::CpuMemory::new();
        let addr = 0x1234;
        mem.write(addr, 42);
        assert_eq!(mem.read(addr), 42);
    }
}

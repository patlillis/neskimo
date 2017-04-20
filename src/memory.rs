// Type alias for an array of 2^16 unsigned bytes.
type BasicMemory = [u8; 65536];

pub trait Memory {
    // Resets the memory to an initial state.
    fn reset(&mut self);

    // Fetches a byte from the specified address in memory.
    fn fetch(&self, address: u16) -> u8;

    // Stores value into memory at the specified address,
    // and returns the old value that was there.
    fn store(&mut self, address: u16, value: u8) -> u8;
}
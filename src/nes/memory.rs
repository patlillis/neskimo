use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Result, Write};
use std::rc::Rc;
use utils;

// 2^16 unsigned bytes.
const MEMORY_SIZE: usize = 65536;

// Type alias for an array of size MEMORY_SIZE.
type MemoryArray = [u8; MEMORY_SIZE];

pub trait Memory {
    // Resets the memory to an initial state.
    fn reset(&mut self);

    // Fetches a byte from the specified address in memory.
    fn fetch(&self, address: u16) -> u8;

    // Stores value into memory at the specified address.
    // Returns the previous value.
    fn store(&mut self, address: u16, value: u8) -> u8;

    // Fetches two bytes stored consecutively in memory.
    fn fetch_u16(&self, address: u16) -> u16 {
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
    fn fetch_u16_wrap_msb(&self, address: u16) -> u16 {
        let low = self.fetch(address);
        let high = if address & 0x00ff == 0x00ff {
            self.fetch(address & 0xff00)
        } else {
            self.fetch(address + 1)
        };
        utils::arithmetic::concat_bytes(high, low)
    }

    // Stores to bytes consecutively in memory, with the first byte at the
    // specified address.
    // Returns the previous value.
    fn store_u16(&mut self, address: u16, value: u16) -> u16 {
        let high = (value >> 8) as u8;
        let low = value as u8;
        let low_prev = self.store(address, low);
        let high_prev = self.store(address + 1, high);
        utils::arithmetic::concat_bytes(high_prev, low_prev)
    }

    // Store a slice of bytes consecutively in memory, starting at the
    // specified address.
    fn store_bytes(&mut self, address: u16, bytes: &[u8]) {
        for (offset, byte) in bytes.iter().enumerate() {
            self.store(address + offset as u16, *byte);
        }
    }
}

pub struct BasicMemory {
    backing_store: MemoryArray,
}

impl BasicMemory {
    pub fn new() -> BasicMemory {
        BasicMemory { backing_store: [0; MEMORY_SIZE] }
    }

    // Dumps the memory contents to a string (most likely
    // for writing to a dump file).
    pub fn dump(&self, file: &mut File) -> Result<()> {
        file.write_all(&self.backing_store)
    }
}

// TODO: Little-endian?
impl Memory for BasicMemory {
    // Resets the memory to an initial state.
    fn reset(&mut self) {
        self.backing_store = [0; MEMORY_SIZE];
    }

    // Fetches a byte from the specified address in memory.
    fn fetch(&self, address: u16) -> u8 {
        self.backing_store[address as usize]
    }

    // Stores value into memory at the specified address.
    // Returns the previous value.
    fn store(&mut self, address: u16, value: u8) -> u8 {
        let old_value = self.backing_store[address as usize];
        self.backing_store[address as usize] = value;
        old_value
    }
}

pub enum MappingType {
    CPU,
    PPU,
}

// A definition of how to set up a memory mapping scheme.
pub trait MemoryMapping: Memory {
    fn fetch_mappings(&self) -> Vec<u16>;
    fn store_mappings(&self) -> Vec<u16>;
}

// A memory storage type that can defer memory operations to of memory
// implementations, with each fetch/store operation potentially mapped to a
// specific memory implementation. In addition, memory addresses can be
// mirrored to always point at another memory address.
pub struct MappedMemory {
    fallback_memory: Box<Memory>,
    mirrors: HashMap<u16, u16>,
    delegates: Vec<Rc<RefCell<Memory>>>,
    // Maps from memory address to index in "delegates" where the address is
    // mapped to.
    fetch: HashMap<u16, usize>,
    store: HashMap<u16, usize>,
}

impl MappedMemory {
    pub fn new(fallback_memory: Box<Memory>) -> MappedMemory {
        let mapped_memory = MappedMemory {
            fallback_memory: fallback_memory,
            mirrors: HashMap::new(),
            delegates: Vec::new(),
            fetch: HashMap::new(),
            store: HashMap::new(),
        };
        mapped_memory
    }

    pub fn add_mirrors(&mut self, mirrors: HashMap<u16, u16>) {
        for (from, to) in &mirrors {
            self.add_mirror(*from, *to);
        }
    }

    fn add_mirror(&mut self, from: u16, to: u16) {
        if from == to {
            warn!("Address {} cannot be mirrored to itself", from);
        }
        self.mirrors.insert(from, to);
    }

    // Helper function to get the mirrored address value for the passed in
    // address. If there is no mirror defined for the address, will return back
    // the passed in address instead.
    fn get_mirror(&self, address: u16) -> u16 {
        match self.mirrors.get(&address) {
            Some(mapped_address) => *mapped_address,
            None => address,
        }
    }

    pub fn add_mapping(&mut self,
                       memory: Rc<RefCell<Memory>>,
                       mapping: Rc<RefCell<MemoryMapping>>) {
        let fetch_addresses = mapping.borrow().fetch_mappings();
        let store_addresses = mapping.borrow().store_mappings();

        // If there's no mappings, just return without doing anything.
        if fetch_addresses.is_empty() && store_addresses.is_empty() {
            return;
        }

        self.delegates.push(memory.clone());
        let delegate_index = self.delegates.len() - 1;

        // Add fetch mappings.
        for fetch_address in &fetch_addresses {
            if self.fetch.contains_key(fetch_address) {
                warn!("Address {} is already mapped for fetch", *fetch_address);
                continue;
            }
            self.fetch.insert(*fetch_address, delegate_index);
        }

        // Add store mappings.
        for store_address in &store_addresses {
            if self.store.contains_key(store_address) {
                warn!("Address {} is already mapped for store", *store_address);
                continue;
            }
            self.store.insert(*store_address, delegate_index);
        }
    }
}

impl Memory for MappedMemory {
    // Resets the fallback memory, but does not reset the mappings.
    fn reset(&mut self) {
        self.fallback_memory.reset();
    }

    fn fetch(&self, address: u16) -> u8 {
        let mapped_address = self.get_mirror(address);

        // Use the mirrored fetch, or the backing memory.
        match self.fetch.get(&mapped_address) {
            Some(delegate_index) => {
                self.delegates[*delegate_index]
                    .borrow()
                    .fetch(mapped_address)
            }
            None => self.fallback_memory.fetch(mapped_address),
        }
    }

    fn store(&mut self, address: u16, value: u8) -> u8 {
        let mapped_address = self.get_mirror(address);

        // Use the mirrored store, or the backing memory.
        match self.store.get_mut(&mapped_address) {
            Some(delegate_index) => {
                self.delegates[*delegate_index]
                    .borrow_mut()
                    .store(mapped_address, value)
            }
            None => self.fallback_memory.store(mapped_address, value),
        }
    }
}
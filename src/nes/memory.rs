use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;
use utils;

// 2^16 unsigned bytes.
pub const DEFAULT_MEMORY_SIZE: usize = 65536;

pub trait Memory {
    // Fetches a byte from the specified address in memory.
    fn fetch(&self, address: u16) -> u8;

    // Stores value into memory at the specified address.
    // Returns the previous value.
    fn store(&mut self, address: u16, value: u8) -> u8;

    // Resets the memory to an initial state. Default implementation is a no-op.
    fn reset(&mut self) {}

    // Dumps the memory contents to a string (most likely
    // for writing to a dump file). Default implementation is no-op.
    fn dump(&self, _file: &mut File) -> io::Result<()> {
        Result::Ok(())
    }

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
    backing_store: Vec<u8>,
}

impl BasicMemory {
    pub fn new(size: usize) -> BasicMemory {
        BasicMemory {
            backing_store: vec![0x00; size],
        }
    }

    // Default size is 2^16 unsigned bytes.
    pub fn with_default_size() -> BasicMemory {
        BasicMemory::new(DEFAULT_MEMORY_SIZE)
    }

    pub fn len(&self) -> usize {
        self.backing_store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.backing_store.is_empty()
    }
}

// TODO: Little-endian?
impl Memory for BasicMemory {
    // Resets the memory to an initial state.
    fn reset(&mut self) {
        for i in 0..self.backing_store.len() {
            self.backing_store[i] = 0x00;
        }
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

    // Dumps the memory contents to a string (most likely
    // for writing to a dump file).
    fn dump(&self, file: &mut File) -> io::Result<()> {
        file.write_all(self.backing_store.as_ref())
    }
}

// A memory storage type that can defer memory operations to memory
// implementations, with each fetch/store operation potentially mapped to a
// specific memory implementation. In addition, memory addresses can be
// mirrored to always point at another memory address.
#[derive(Default)]
pub struct MappedMemory {
    mirrors: HashMap<u16, u16>,
    delegates: Vec<Rc<RefCell<Memory>>>,
    // Maps from memory address to index in "delegates" where the address is
    // mapped to.
    fetch: HashMap<u16, usize>,
    store: HashMap<u16, usize>,
}

impl MappedMemory {
    pub fn new() -> MappedMemory {
        MappedMemory {
            mirrors: HashMap::new(),
            delegates: Vec::new(),
            fetch: HashMap::new(),
            store: HashMap::new(),
        }
    }

    // Add new address mirrors. Note that this will override any previous
    // mirroring for those addresses.
    pub fn add_mirrors(&mut self, mirrors: &HashMap<u16, u16>) {
        for (from, to) in mirrors {
            self.add_mirror(*from, *to);
        }
    }

    // Add a new address mirror. Note that this will override any previous
    // mirroring for the "from" address.
    pub fn add_mirror(&mut self, from: u16, to: u16) {
        if from == to {
            warn!("Address {} cannot be mirrored to itself", from);
        }
        if let Some(old_mirror) = self.mirrors.insert(from, to) {
            warn!(
                concat!(
                    "Address {:#04x} is already mirrored to address {:#04x}. ",
                    "Overriding with new mirroring."
                ),
                from,
                old_mirror
            );
        }
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

    // Add new fetch & store mappings for all addresses specified by "mapping".
    // Note that this will override any previous mappings for the specified
    // addresses.
    pub fn add_mapping<I1, I2>(
        &mut self,
        memory: Rc<RefCell<Memory>>,
        fetch_addresses: I1,
        store_addresses: I2,
    ) where
        I1: IntoIterator<Item = u16>,
        I2: IntoIterator<Item = u16>,
    {
        // If we don't end up adding any mappings, then we don't need to keep
        // the delegate around.
        let mut mappings_added = false;

        self.delegates.push(memory.clone());
        let delegate_index = self.delegates.len() - 1;

        // Add fetch mappings.
        for fetch_address in fetch_addresses {
            if self.fetch.insert(fetch_address, delegate_index).is_some() {
                warn!(
                    concat!(
                        "Address {:#04x} is already mapped for fetch. ",
                        "Overriding with new mapping."
                    ),
                    fetch_address
                );
            }
            mappings_added = true;
        }

        // Add store mappings.
        for store_address in store_addresses {
            if self.store.insert(store_address, delegate_index).is_some() {
                warn!(
                    concat!(
                        "Address {:#04x} is already mapped for store. ",
                        "Overriding with new mapping."
                    ),
                    store_address
                );
            }
            mappings_added = true;
        }

        // If no mappings were added, remove delegate from list.
        if !mappings_added {
            self.delegates.remove(delegate_index);
        }
    }
}

impl Memory for MappedMemory {
    fn fetch(&self, address: u16) -> u8 {
        let mapped_address = self.get_mirror(address);

        // Use the mirrored fetch, or the backing memory.
        match self.fetch.get(&mapped_address) {
            Some(delegate_index) => self.delegates[*delegate_index]
                .borrow()
                .fetch(mapped_address),
            None => panic!(
                "No delegate memory for fetch at address {:#04x}",
                address
            ),
        }
    }

    fn store(&mut self, address: u16, value: u8) -> u8 {
        let mapped_address = self.get_mirror(address);

        // Use the mirrored store, or the backing memory.
        match self.store.get_mut(&mapped_address) {
            Some(delegate_index) => self.delegates[*delegate_index]
                .borrow_mut()
                .store(mapped_address, value),
            None => panic!(
                "No delegate memory for store at address {:#04x}",
                address
            ),
        }
    }
}

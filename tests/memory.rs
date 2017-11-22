#[macro_use]
extern crate maplit;
extern crate neskimolib;

use neskimolib::nes::memory::{BasicMemory, MappedMemory, Memory, MemoryMapping};

#[test]
fn test_mirror() {
    // Set up values in destination addresses of fallback memory.
    let mut memory = Box::new(BasicMemory::new());
    memory.store(0xa000, 0xaa);
    memory.store(0xb000, 0xbb);
    memory.store(0xc000, 0xcc);
    memory.store(0xd000, 0xdd);

    // Set up mirrors in mapped memory.
    let mut mapped_memory = MappedMemory::new(memory);
    mapped_memory.add_mirrors(hashmap!{
        0x0c00 => 0xc000,
        0x0d00 => 0xd000,
    });

    // Test non-mirrored address fetch.
    assert_eq!(mapped_memory.fetch(0x0000), 0x00);
    assert_eq!(mapped_memory.fetch(0xa000), 0xaa);

    // Test non-mirrored address store.
    assert_eq!(mapped_memory.store(0x0000, 0x01), 0x00);
    assert_eq!(mapped_memory.fetch(0x0000), 0x01);
    assert_eq!(mapped_memory.store(0xa000, 0x0a), 0xaa);
    assert_eq!(mapped_memory.fetch(0xa000), 0x0a);

    // Test mirrored address fetch.
    assert_eq!(mapped_memory.fetch(0x0c00), 0xcc);
    assert_eq!(mapped_memory.fetch(0x0d00), 0xdd);

    // Test mirrored address store.
    assert_eq!(mapped_memory.store(0x0c00, 0x0c), 0xcc);
    assert_eq!(mapped_memory.fetch(0x0c00), 0x0c);
    assert_eq!(mapped_memory.store(0x0d00, 0x0d), 0xdd);
    assert_eq!(mapped_memory.fetch(0x0d00), 0x0d);
}

struct TestMemoryMapping {
    last_stored_value: u8,
}

impl TestMemoryMapping {
    fn new() -> TestMemoryMapping {
        TestMemoryMapping { last_stored_value: 0x00 }
    }
}

impl Memory for TestMemoryMapping {
    fn reset(&mut self) {
        self.last_stored_value = 0x00;
    }

    // Test fetch just returns "address + 1", truncated to 8 bits.
    fn fetch(&self, address: u16) -> u8 {
        address.wrapping_add(1) as u8
    }

    // All calls to store just set "last_stored_value", and return the
    // previous last_stored_value.
    fn store(&mut self, _address: u16, value: u8) -> u8 {
        let result = self.last_stored_value;
        self.last_stored_value = value;
        result
    }
}

// Test mapping maps all addresses between 0x0000 and 0x1000 to test memory.
impl<'a> MemoryMapping<'a> for TestMemoryMapping {
    fn fetch_mappings(&self) -> Vec<u16> {
        let mut vec = Vec::new();
        for n in 0x0000..0x1000 {
            vec.push(n);
        }
        vec
    }

    fn store_mappings(&self) -> Vec<u16> {
        let mut vec = Vec::new();
        for n in 0x0000..0x1000 {
            vec.push(n);
        }
        vec
    }
}

#[test]
fn test_mappings() {
    // Set up values in destination addresses of fallback memory.
    let mut mappings = TestMemoryMapping::new();
    {
        let mut mapped_memory = MappedMemory::new(Box::new(BasicMemory::new()));
        mapped_memory.add_mappings(&mut mappings);

        assert_eq!(mapped_memory.fetch(0x0000), 0x0001);
        assert_eq!(mapped_memory.store(0x0100, 0xff), 0x00);
    }
    assert_eq!(mappings.last_stored_value, 0xff);
}
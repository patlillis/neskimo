use nes::memory::{BasicMemory, MappedMemory, Memory, DEFAULT_MEMORY_SIZE};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_fetch_store() {
    let mut memory = BasicMemory::with_default_size();

    // Test initial fetch.
    assert_eq!(memory.fetch(0x0000), 0x00);
    assert_eq!(memory.fetch(0xf000), 0x00);

    // Test store.
    assert_eq!(memory.store(0x0000, 0x01), 0x00);
    assert_eq!(memory.store(0x0000, 0x02), 0x01);
    assert_eq!(memory.store(0xf000, 0xf1), 0x00);
    assert_eq!(memory.store(0xf000, 0xf2), 0xf1);

    // Test fetch after store.
    assert_eq!(memory.fetch(0x0000), 0x02);
    assert_eq!(memory.fetch(0xf000), 0xf2);
}

#[test]
fn test_fetch_u16() {
    let mut memory = BasicMemory::with_default_size();

    // Test initial fetch.
    assert_eq!(memory.fetch_u16(0x0000), 0x0000);
    assert_eq!(memory.fetch_u16(0xf000), 0x0000);
    assert_eq!(memory.fetch_u16(0x33ff), 0x0000);

    memory.store(0x0001, 0x01);
    memory.store(0x0000, 0x02);
    memory.store(0xf001, 0xf1);
    memory.store(0xf000, 0xf2);
    memory.store(0x3400, 0xde);
    memory.store(0x33ff, 0xad);

    // Test fetch after store.
    assert_eq!(memory.fetch_u16(0x0000), 0x0102);
    assert_eq!(memory.fetch_u16(0xf000), 0xf1f2);
    assert_eq!(memory.fetch_u16(0x33ff), 0xdead);
}

#[test]
fn test_fetch_u16_wrap_msb() {
    let mut memory = BasicMemory::with_default_size();

    // Test initial fetch.
    assert_eq!(memory.fetch_u16_wrap_msb(0x0000), 0x0000);
    assert_eq!(memory.fetch_u16_wrap_msb(0xf000), 0x0000);
    assert_eq!(memory.fetch_u16_wrap_msb(0x33ff), 0x0000);

    memory.store(0x0001, 0x01);
    memory.store(0x0000, 0x02);
    memory.store(0xf001, 0xf1);
    memory.store(0xf000, 0xf2);
    memory.store(0x3400, 0xde);
    memory.store(0x3300, 0xed);
    memory.store(0x33ff, 0xad);

    // Test fetch after store.
    assert_eq!(memory.fetch_u16_wrap_msb(0x0000), 0x0102);
    assert_eq!(memory.fetch_u16_wrap_msb(0xf000), 0xf1f2);
    assert_eq!(memory.fetch_u16_wrap_msb(0x33ff), 0xedad);
}

#[test]
fn test_store_u16() {
    let mut memory = BasicMemory::with_default_size();

    // Test initial store.
    assert_eq!(memory.store_u16(0x0000, 0x0102), 0x0000);
    assert_eq!(memory.store_u16(0xf000, 0xf1f2), 0x0000);
    assert_eq!(memory.store_u16(0x33ff, 0xdead), 0x0000);

    // Test store to already-updated addresses.
    assert_eq!(memory.store_u16(0x0000, 0x0304), 0x0102);
    assert_eq!(memory.store_u16(0xf000, 0xf3f4), 0xf1f2);
    assert_eq!(memory.store_u16(0x33ff, 0xcafe), 0xdead);

    // Test storing offset from previous stores.
    assert_eq!(memory.store_u16(0x0001, 0x1234), 0x0003);
    assert_eq!(memory.store_u16(0xf001, 0xfedc), 0x00f3);
    assert_eq!(memory.store_u16(0x3400, 0xabcd), 0x00ca);
}

#[test]
fn test_store_bytes() {
    let mut memory = BasicMemory::with_default_size();

    // Initial store
    memory.store_bytes(0x0000, &vec![0xff, 0xee, 0xdd]);
    memory.store_bytes(0xf000, &vec![0x11, 0x22, 0x33]);
    memory.store_bytes(0x33ff, &vec![0x77, 0x88, 0x99]);

    assert_eq!(memory.fetch(0x0000), 0xff);
    assert_eq!(memory.fetch(0x0001), 0xee);
    assert_eq!(memory.fetch(0x0002), 0xdd);

    assert_eq!(memory.fetch(0xf000), 0x11);
    assert_eq!(memory.fetch(0xf001), 0x22);
    assert_eq!(memory.fetch(0xf002), 0x33);

    assert_eq!(memory.fetch(0x33ff), 0x77);
    assert_eq!(memory.fetch(0x3400), 0x88);
    assert_eq!(memory.fetch(0x3401), 0x99);
}

#[test]
fn test_basic_memory_new() {
    let memory1 = BasicMemory::new(1);
    assert_eq!(memory1.len(), 1);

    let memory2 = BasicMemory::new(1024);
    assert_eq!(memory2.len(), 1024);

    let memory3 = BasicMemory::new(555);
    assert_eq!(memory3.len(), 555);
}

#[test]
fn test_basic_memory_default_size() {
    let memory = BasicMemory::with_default_size();
    assert_eq!(memory.len(), DEFAULT_MEMORY_SIZE);
}

#[test]
fn test_mirror() {
    // Set up values in destination addresses of fallback memory.
    let memory = Rc::new(RefCell::new(BasicMemory::with_default_size()));
    memory.borrow_mut().store(0xa000, 0xaa);
    memory.borrow_mut().store(0xb000, 0xbb);
    memory.borrow_mut().store(0xc000, 0xcc);
    memory.borrow_mut().store(0xd000, 0xdd);

    // Set up mirrors in mapped memory.
    let mut mapped_memory = MappedMemory::new();
    mapped_memory.add_mapping(
        memory,
        (0x00..DEFAULT_MEMORY_SIZE).map(|x| x as u16),
        (0x00..DEFAULT_MEMORY_SIZE).map(|x| x as u16),
    );
    mapped_memory.add_mirrors(hashmap!{
        0x0c00 => 0xc000,
        0x0d00 => 0xd000,
    });

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
        TestMemoryMapping {
            last_stored_value: 0x00,
        }
    }
}

impl Memory for TestMemoryMapping {
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

#[test]
fn test_mappings() {
    // Set up values in destination addresses of fallback memory.
    let mappings = Rc::new(RefCell::new(TestMemoryMapping::new()));
    let mut mapped_memory = MappedMemory::new();
    mapped_memory.add_mapping(
        mappings.clone(),
        vec![0x0000, 0x0100],
        vec![0x0000, 0x0100],
    );

    assert_eq!(mapped_memory.fetch(0x0000), 0x0001);
    assert_eq!(mapped_memory.store(0x0100, 0xff), 0x00);
    assert_eq!(mappings.borrow().last_stored_value, 0xff);
}

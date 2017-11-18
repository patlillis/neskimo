#[allow(unused_imports)]
use nes::memory::*;

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
// Gets the page for addr, which is the upper byte. So for example, the address
// 0xf0cc is on page 0xf0. The address 0x00ff is on page 0x00. Each page
// consists of 256 addresses.
pub fn page(addr: u16) -> u8 {
    (addr >> 8) as u8
}

// Determines whether addr1 and addr2 are on the same page, or whether they
// cross a page boundary.
pub fn page_cross(addr1: u16, addr2: u16) -> bool {
    page(addr1) == page(addr2)
}
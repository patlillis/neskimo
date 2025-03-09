use std::fmt;

#[derive(PartialEq, Debug)]
pub enum PageCross {
    Same,
    Backwards,
    Forwards,
}

impl fmt::Display for PageCross {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PageCross::Same => write!(f, "Same"),
            PageCross::Backwards => write!(f, "Backwards"),
            PageCross::Forwards => write!(f, "Forwards"),
        }
    }
}

// Gets the page for addr, which is the upper byte. So for example, the address
// 0xf0cc is on page 0xf0. The address 0x00ff is on page 0x00. Each page
// consists of 256 addresses.
pub fn page(addr: u16) -> u8 {
    (addr >> 8) as u8
}

// Determines whether addr1 and addr2 are on the same page, or whether they
// cross a page boundary.
pub fn page_cross(addr1: u16, addr2: u16) -> PageCross {
    let page1 = page(addr1);
    let page2 = page(addr2);

    match page1.cmp(&page2) {
        std::cmp::Ordering::Greater => PageCross::Backwards,
        std::cmp::Ordering::Less => PageCross::Forwards,
        std::cmp::Ordering::Equal => PageCross::Same,
    }
}

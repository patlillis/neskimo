use nes::memory::{Memory, MemoryMapping};

/// Emulated screen width in pixels.
pub const SCREEN_WIDTH: usize = 256;
/// Emulated screen height in pixels.
pub const SCREEN_HEIGHT: usize = 240;
/// Number of pixels in the emulated screen.
pub const PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
/// Screen texture size in bytes.
pub const SCREEN_SIZE: usize = PIXEL_COUNT * 3;

pub struct Ppu {
    pub cycle: usize,
    pub screen: [u8; SCREEN_SIZE],
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
    oamdma: u8,
}

impl Default for Ppu {
    fn default() -> Ppu {
        Ppu {
            cycle: 0,
            screen: [0x00; SCREEN_SIZE],
            ppuctrl: 0x00,
            ppumask: 0x00,
            ppustatus: 0x00,
            oamaddr: 0x00,
            oamdata: 0x00,
            ppuscroll: 0x00,
            ppuaddr: 0x00,
            ppudata: 0x00,
            oamdma: 0x00,
        }
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            cycle: 0,
            screen: [0x00; SCREEN_SIZE],
            ..Default::default()
        }
    }

    pub fn step(&mut self) {
        for i in 0..(PIXEL_COUNT - 1) {
            self.screen[i * 3] = self.cycle as u8;
            self.screen[i * 3 + 1] = 0;
            self.screen[i * 3 + 2] = 0;
        }

        self.screen[115 * SCREEN_WIDTH + (self.cycle * 3)] = 255;
        self.screen[115 * SCREEN_WIDTH + (self.cycle * 3) + 1] = 255;
        self.screen[115 * SCREEN_WIDTH + (self.cycle * 3) + 2] = 255;

        self.cycle = self.cycle + 1;;
    }
}

impl Memory for Ppu {
    // Resets the memory to an initial state.
    fn reset(&mut self) {}

    // Fetches a byte from the specified address in memory.
    fn fetch(&self, address: u16) -> u8 {
        match address {
            0x2000 => self.ppuctrl,
            0x2001 => self.ppumask,
            0x2002 => self.ppustatus,
            0x2003 => self.oamaddr,
            0x2004 => self.oamdata,
            0x2005 => self.ppuscroll,
            0x2006 => self.ppuaddr,
            0x2007 => self.ppudata,
            0x4014 => self.oamdma,
            _ => {
                panic!("Tried to access non-existent PPU register at {:#04x}",
                       address)
            }
        }
    }

    // Stores value into memory at the specified address.
    // Returns the previous value.
    fn store(&mut self, address: u16, value: u8) -> u8 {
        let old_value = self.fetch(address);

        match address {
            0x2000 => self.ppuctrl = value,
            0x2001 => self.ppumask = value,
            0x2002 => self.ppustatus = value,
            0x2003 => self.oamaddr = value,
            0x2004 => self.oamdata = value,
            0x2005 => self.ppuscroll = value,
            0x2006 => self.ppuaddr = value,
            0x2007 => self.ppudata = value,
            0x4014 => self.oamdma = value,
            _ => {
                panic!("Tried to access non-existent PPU register at {:#04x}",
                       address)
            }
        };
        old_value
    }
}

impl MemoryMapping for Ppu {
    fn fetch_mappings(&self) -> Vec<u16> {
        vec![0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x4014]
    }

    fn store_mappings(&self) -> Vec<u16> {
        vec![0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x4014]
    }
}
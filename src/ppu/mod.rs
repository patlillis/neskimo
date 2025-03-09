pub mod internal_memory;
pub mod vram;

use crate::nes::memory::Memory;
use crate::ppu::internal_memory::InternalMemory;
use crate::rom::MirrorType;
use arrayvec::ArrayVec;

// Emulated screen width in pixels.
pub const SCREEN_WIDTH: usize = 256;
// Emulated screen height in pixels.
pub const SCREEN_HEIGHT: usize = 240;
// Number of pixels in the emulated screen.
pub const PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
// Screen texture size in bytes.
pub const SCREEN_SIZE: usize = PIXEL_COUNT * 3;
// Number of cycles for each scanline (pre-render scanline may skip a cycle).
pub const CYCLES_PER_SCANLINE: u32 = 341;
// First scanline that renders to the screen.
pub const FIRST_VISIBLE_SCANLINE: u16 = 0;
// Last scanline that renders to the screen.
pub const LAST_VISIBLE_SCANLINE: u16 = 239;
// Scanline on which to trigger the VBlank NMI.
pub const V_BLANK_SCANLINE: u16 = 241;
// Scanline at which to restart at 0.
pub const PRE_RENDER_SCANLINE: u16 = 261;
// Total number of scanlines, numbered 0 - 261.
pub const TOTAL_SCANLINE_COUNT: u16 = 262;

#[rustfmt::skip]
#[allow(dead_code)]
static PALETTE: [u8; 192] = [
    124,124,124,    0,0,252,        0,0,188,        68,40,188,
    148,0,132,      168,0,32,       168,16,0,       136,20,0,
    80,48,0,        0,120,0,        0,104,0,        0,88,0,
    0,64,88,        0,0,0,          0,0,0,          0,0,0,
    188,188,188,    0,120,248,      0,88,248,       104,68,252,
    216,0,204,      228,0,88,       248,56,0,       228,92,16,
    172,124,0,      0,184,0,        0,168,0,        0,168,68,
    0,136,136,      0,0,0,          0,0,0,          0,0,0,
    248,248,248,    60,188,252,     104,136,252,    152,120,248,
    248,120,248,    248,88,152,     248,120,88,     252,160,68,
    248,184,0,      184,248,24,     88,216,84,      88,248,152,
    0,232,216,      120,120,120,    0,0,0,          0,0,0,
    252,252,252,    164,228,252,    184,184,248,    216,184,248,
    248,184,248,    248,164,192,    240,208,176,    252,224,168,
    248,216,120,    216,248,120,    184,248,184,    184,248,216,
    0,252,252,      248,216,248,    0,0,0,          0,0,0
];

pub struct Ppu {
    pub cycle: u32,
    pub screen: Box<[u8; SCREEN_SIZE]>,
    current_scanline: u16,
    odd_frame: bool,

    // Fields for when the CPU access memory being mapped to the CPU.
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
    oamdma: u8,

    // Internal memory storage/access.
    #[allow(dead_code)]
    internal_memory: InternalMemory,
}

impl Ppu {
    pub fn new(nametable_mirror_type: MirrorType) -> Ppu {
        Ppu {
            cycle: 0,
            screen: Box::new([0x00; SCREEN_SIZE]),
            current_scanline: 241,
            odd_frame: false,
            ppuctrl: 0x00,
            ppumask: 0x00,
            ppustatus: 0x00,
            oamaddr: 0x00,
            oamdata: 0x00,
            ppuscroll: 0x00,
            ppuaddr: 0x00,
            ppudata: 0x00,
            oamdma: 0x00,
            internal_memory: InternalMemory::new(nametable_mirror_type),
        }
    }

    // Addresses that PPU can provide access to for CPU's mapped registers.
    // TODO: separate out registers that are read-only, write-only, etc.
    pub fn mapped_addresses() -> ArrayVec<u16, 9> {
        ArrayVec::from([
            0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007,
            0x4014,
        ])
    }

    pub fn write_to_screen(&mut self, x: usize, y: usize, color: [u8; 3]) {
        let base_index = (x + (y * SCREEN_WIDTH)) * 3;
        self.screen[base_index] = color[0];
        self.screen[base_index + 1] = color[1];
        self.screen[base_index + 2] = color[2];
    }

    pub fn read_from_screen(&self, x: usize, y: usize) -> [u8; 3] {
        let base_index = x + (y * SCREEN_WIDTH);
        [
            self.screen[base_index],
            self.screen[base_index + 1],
            self.screen[base_index + 2],
        ]
    }

    // Perform the number of PPU operations for the set number of cycles. Note
    // that cycles is already in PPU cycles. Returns true if on a new frame, and
    // true if entering v-blank.
    pub fn step(&mut self, cycles: u32) -> (bool, bool) {
        let mut new_frame = false;
        let mut v_blank = false;
        let skip_cycle =
            self.current_scanline == PRE_RENDER_SCANLINE && self.odd_frame;
        let cycles_per_scanline = if skip_cycle {
            CYCLES_PER_SCANLINE - 1
        } else {
            CYCLES_PER_SCANLINE
        };

        self.cycle += cycles;

        // Check for new scanline.
        if self.cycle >= cycles_per_scanline {
            self.cycle -= cycles_per_scanline;
            self.current_scanline += 1;
            if self.current_scanline >= TOTAL_SCANLINE_COUNT {
                new_frame = true;
                self.current_scanline -= TOTAL_SCANLINE_COUNT;
            }

            match self.current_scanline {
                FIRST_VISIBLE_SCANLINE..=LAST_VISIBLE_SCANLINE => {
                    self.render_scanline()
                }
                V_BLANK_SCANLINE => v_blank = true,
                _ => (),
            }
        }

        if new_frame {
            self.odd_frame = !self.odd_frame;
        }

        (new_frame, v_blank)
        // for i in 0..(PIXEL_COUNT - 1) {
        //     self.screen[i * 3] = self.cycle as u8;
        //     self.screen[i * 3 + 1] = 0;
        //     self.screen[i * 3 + 2] = 0;
        // }

        // let cycle_index = (self.cycle * 3) as usize;

        // self.screen[115 * SCREEN_WIDTH + cycle_index] = 255;
        // self.screen[115 * SCREEN_WIDTH + cycle_index + 1] = 255;
        // self.screen[115 * SCREEN_WIDTH + cycle_index + 2] = 255;
    }

    // Renders a scanline to the internal "screen".
    fn render_scanline(&mut self) {
        let y = self.current_scanline as usize;
        let (mut r, mut g, mut b) = match y % 3 {
            0 => (255, 0, 0),
            1 => (0, 255, 0),
            2 => (0, 0, 255),
            _ => (0, 0, 0),
        };
        if self.odd_frame {
            r = (f32::from(r) * 0.75) as u8;
            g = (f32::from(g) * 0.75) as u8;
            b = (f32::from(b) * 0.75) as u8;
        }
        for x in 0..SCREEN_WIDTH {
            self.write_to_screen(x, y, [r, g, b]);
        }
    }
}

impl Memory for Ppu {
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
            _ => panic!(
                "Tried to access non-existent PPU register at {:#04x}",
                address
            ),
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
            _ => panic!(
                "Tried to access non-existent PPU register at {:#04x}",
                address
            ),
        };
        old_value
    }
}

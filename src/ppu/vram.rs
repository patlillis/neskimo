// TODO: ADD TESTS!!!

// 960 bytes of CHR tile position data, one byte for each 8x8 pixel tile. There
// are 30 rows and 30 columns, which gives 960 total tiles.
use arrayvec::ArrayVec;
use nes::memory::Memory;

const TILE_DATA_SIZE: u16 = 960;

// 64 bytes of attribute (palette) data. Each byte controls a 32×32 pixel or 4×4
// tile area.
const ATTRIBUTE_DATA_SIZE: u16 = 64;

// Nametable addresses 1024 bytes of memory, divided between tile data and
// attribute data.
const MAX_NAMETABLE_ADDRESS: u16 = 1024 - 1;

struct Nametable {
    tile_data: [u8; TILE_DATA_SIZE as usize],
    attribute_data: [u8; ATTRIBUTE_DATA_SIZE as usize],
}

impl Nametable {
    pub fn new() -> Self {
        Nametable {
            tile_data: [0x00; TILE_DATA_SIZE as usize],
            attribute_data: [0x00; ATTRIBUTE_DATA_SIZE as usize],
        }
    }
}

impl Memory for Nametable {
    fn fetch(&self, address: u16) -> u8 {
        match address {
            _ if address < TILE_DATA_SIZE => self.tile_data[address as usize],
            _ if address < ATTRIBUTE_DATA_SIZE => {
                let attribute_address = (address - TILE_DATA_SIZE) as usize;
                self.attribute_data[attribute_address]
            }
            _ => panic!(
                "address {:#04x} is not within PPU VRAM addressable range",
                address
            ),
        }
    }

    fn store(&mut self, address: u16, value: u8) -> u8 {
        match address {
            _ if address < TILE_DATA_SIZE => {
                let old_value = self.tile_data[address as usize];
                self.tile_data[address as usize] = value;
                old_value
            }
            _ if address < ATTRIBUTE_DATA_SIZE => {
                let attribute_address = (address - TILE_DATA_SIZE) as usize;
                let old_value = self.attribute_data[attribute_address];
                self.attribute_data[attribute_address] = value;
                old_value
            }
            _ => panic!(
                "address {:#04x} is not within PPU VRAM addressable range",
                address
            ),
        }
    }
}

// PPU hash 4kB of addressable VRAM nametables,like follows:
//
//      (0,0)     (256,0)     (511,0)
//        +-----------+-----------+
//        |           |           |
//        |           |           |
//        |   $2000   |   $2400   |
//        |           |           |
//        |           |           |
// (0,240)+-----------+-----------+(511,240)
//        |           |           |
//        |           |           |
//        |   $2800   |   $2C00   |
//        |           |           |
//        |           |           |
//        +-----------+-----------+
//      (0,479)   (256,479)   (511,479)
//
// However, there is normally only 2kB of internal nametable memory. The other
// 2kB is mirrored. The type of mirroring is controlled by the cartridge.
#[derive(Copy, Clone)]
pub enum NametableMirroring {
    // $2000 equals $2400 and $2800 equals $2C00.
    Horizontal,

    // $2000 equals $2800, and $2400 equals $2C00.
    Vertical,
}

// PPU internal VRAM, used to store 2 nametables. These nametables are mirrored
// to make up 4kB of addressable memory. nametables for assigning CHR tiles to
// each screen position.
pub struct Vram {
    nametable_a: Nametable,
    nametable_b: Nametable,
    mirroring: NametableMirroring,
}

impl Vram {
    pub fn new(mirroring: &NametableMirroring) -> Self {
        Vram {
            nametable_a: Nametable::new(),
            nametable_b: Nametable::new(),
            mirroring: *mirroring,
        }
    }

    pub fn mapped_addresses() -> ArrayVec<[u16; 9]> {
        // TODO: replace this with the actual addresses that the VRAM can be
        // addressed on.
        ArrayVec::from([
            0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007,
            0x4014,
        ])
    }
}

// This is how VRAM is accessed by the PPU.
// See http://wiki.nesdev.com/w/index.php/Mirroring#Nametable_Mirroring for more
// details.
impl Memory for Vram {
    fn fetch(&self, address: u16) -> u8 {
        // This is a pain, but Rust doesn't support exclusive ranges on pattern
        // matches, so this is what we're left with.
        match address {
            // Address too small.
            _ if address < 0x2000 => panic!(
                "address {:#04x} is not within PPU VRAM addressable range",
                address
            ),

            // Top-left, always Nametable A.
            _ if address < 0x2400 => self.nametable_a.fetch(address - 0x2000),

            // Top-right, depends on mirroring configuration.
            _ if address < 0x2800 => match self.mirroring {
                NametableMirroring::Horizontal => {
                    self.nametable_a.fetch(address - 0x2400)
                }
                NametableMirroring::Vertical => {
                    self.nametable_b.fetch(address - 0x2400)
                }
            },

            // Bottom-left, depends on mirroring configuration.
            _ if address < 0x2c00 => match self.mirroring {
                NametableMirroring::Horizontal => {
                    self.nametable_b.fetch(address - 0x2c00)
                }
                NametableMirroring::Vertical => {
                    self.nametable_a.fetch(address - 0x2c00)
                }
            },

            // Bottom-right, always Nametable B.
            _ if address < 0x3000 => self.nametable_b.fetch(address - 0x2c00),

            // Address too large.
            _ => panic!(
                "address {:#04x} is not within PPU VRAM addressable range",
                address
            ),
        }
    }

    fn store(&mut self, address: u16, value: u8) -> u8 {
        // This is a pain, but Rust doesn't support exclusive ranges on pattern
        // matches, so this is what we're left with.
        match address {
            // Address too small.
            _ if address < 0x2000 => panic!(
                "address {:#04x} is not within PPU VRAM addressable range",
                address
            ),

            // Top-left, always Nametable A.
            _ if address < 0x2400 => {
                self.nametable_a.store(address - 0x2000, value)
            }

            // Top-right, depends on mirroring configuration.
            _ if address < 0x2800 => match self.mirroring {
                NametableMirroring::Horizontal => {
                    self.nametable_a.store(address - 0x2400, value)
                }
                NametableMirroring::Vertical => {
                    self.nametable_b.store(address - 0x2400, value)
                }
            },

            // Bottom-left, depends on mirroring configuration.
            _ if address < 0x2c00 => match self.mirroring {
                NametableMirroring::Horizontal => {
                    self.nametable_b.store(address - 0x2c00, value)
                }
                NametableMirroring::Vertical => {
                    self.nametable_a.store(address - 0x2c00, value)
                }
            },

            // Bottom-right, always Nametable B.
            _ if address < 0x3000 => {
                self.nametable_b.store(address - 0x2c00, value)
            }

            // Address too large.
            _ => panic!(
                "address {:#04x} is not within PPU VRAM addressable range",
                address
            ),
        }
    }
}

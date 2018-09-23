// TODO: ADD TESTS!!!

// 960 bytes of CHR tile position data, one byte for each 8x8 pixel tile. There
// are 30 rows and 30 columns, which gives 960 total tiles.
use nes::memory::Memory;
use rom::MirrorType;

const TILE_DATA_SIZE: u16 = 960;

// 64 bytes of attribute (palette) data. Each byte controls a 32×32 pixel or 4×4
// tile area.
const ATTRIBUTE_DATA_SIZE: u16 = 64;

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

// PPU internal VRAM, used to store 2 nametables. These nametables are mirrored
// to make up 4kB of addressable memory. nametables for assigning CHR tiles to
// each screen position.
//
// PPU has 4kB of addressable VRAM nametables,like follows:
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
//
// Horizontal Mirroring: $2000 equals $2400 and $2800 equals $2C00.
// Vertical Mirroring: $2000 equals $2800, and $2400 equals $2C00.
pub struct Vram {
    nametable_a: Nametable,
    nametable_b: Nametable,
    mirroring: MirrorType,
}

impl Vram {
    pub fn new(mirroring: &MirrorType) -> Self {
        Vram {
            nametable_a: Nametable::new(),
            nametable_b: Nametable::new(),
            mirroring: *mirroring,
        }
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
                MirrorType::Horizontal => {
                    self.nametable_a.fetch(address - 0x2400)
                }
                MirrorType::Vertical => {
                    self.nametable_b.fetch(address - 0x2400)
                }
                MirrorType::Both => panic!("MirrorType::Both is unsupported."),
            },

            // Bottom-left, depends on mirroring configuration.
            _ if address < 0x2c00 => match self.mirroring {
                MirrorType::Horizontal => {
                    self.nametable_b.fetch(address - 0x2c00)
                }
                MirrorType::Vertical => {
                    self.nametable_a.fetch(address - 0x2c00)
                }
                MirrorType::Both => panic!("MirrorType::Both is unsupported."),
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
                MirrorType::Horizontal => {
                    self.nametable_a.store(address - 0x2400, value)
                }
                MirrorType::Vertical => {
                    self.nametable_b.store(address - 0x2400, value)
                }
                MirrorType::Both => panic!("MirrorType::Both is unsupported."),
            },

            // Bottom-left, depends on mirroring configuration.
            _ if address < 0x2c00 => match self.mirroring {
                MirrorType::Horizontal => {
                    self.nametable_b.store(address - 0x2c00, value)
                }
                MirrorType::Vertical => {
                    self.nametable_a.store(address - 0x2c00, value)
                }
                MirrorType::Both => panic!("MirrorType::Both is unsupported."),
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

use std::fmt;
use std::io::{Error, ErrorKind, Result};
use utils::io::read_binary;
use std::path::Path;

pub const TRAINER_SIZE: usize = 0x0200;
pub const PRG_ROM_SIZE: usize = 0x4000;
pub const CHR_ROM_SIZE: usize = 0x2000;
pub const PRG_RAM_SIZE: usize = 0x2000;

#[derive(Debug)]
pub enum MirrorType {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug)]
pub enum TVSystem {
    NTSC,
    PAL,
    DualCompatible,
}

#[derive(Debug)]
pub enum Mapper {
    NROM,
}

pub struct INESFile {
    // From the name of the iNES file.
    game_name: String,

    // Whether 512 bytes of trainer data is present.
    trainer: bool,
    trainer_data: Option<[u8; TRAINER_SIZE]>,

    // Size of PRG ROM in 16 KB units.
    prg_rom_size: u8,
    prg_rom_data: Vec<[u8; PRG_ROM_SIZE]>,

    // Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM).
    chr_rom_size: u8,
    chr_rom_data: Vec<[u8; CHR_ROM_SIZE]>,

    // Size of PRG RAM in 8 KB units (Value 0 infers 8 KB for compatibility).
    prg_ram_size: u8,
    prg_ram_data: Vec<[u8; PRG_RAM_SIZE]>,

    mirror_type: MirrorType,

    tv_system: TVSystem,

    four_screen: bool,

    vs_cart: bool,

    mapper: u8,
}

// TODO: finish this up.
impl fmt::Debug for INESFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
                 "game_name: {}\
        trainer: {}\
        trainer_data: {}",
                 // prg_rom_size: {}\
                 // prg_rom: {}\
                 // chr_rom_size: {}\
                 // chr_rom: {}\
                 // prg_ram_size: {}\
                 // prg_ram: {}\
                 // mirror_type: {}\
                 // tv_system: {}\
                 // four_screen: {}\
                 // vs_cart: {}\
                 // mapper: {}",
                 self.game_name,
                 self.trainer,
                 self.trainer_data
                     .map_or("(none)".to_string(), |data| data.len().to_string()))
    }
}

impl fmt::Display for INESFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl INESFile {
    pub fn new(file_name: &String) -> Result<INESFile> {
        let bytes = try!(read_binary(file_name));
        let game_name = Path::new(&file_name)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        // Oh my god someone please kill me.
        INESFile::new_from_buffer(game_name, &bytes)
    }

    pub fn new_from_buffer(game_name: String, rom: &[u8]) -> Result<INESFile> {
        // File must have enough space to be a valid header.
        if rom.len() < 16 {
            return Err(Error::new(ErrorKind::InvalidData,
                                  "Invalid iNES header: file less than 16 bytes."));
        }

        // Check "NES" declaration.
        if &rom[0..3] != "NES".as_bytes() || rom[3] != 0x1a {
            return Err(Error::new(ErrorKind::InvalidData,
                                  "Invalid iNES header: missing \"NES\" declaration."));
        }

        Err(Error::new(ErrorKind::NotConnected,
                       format!("Successsfully read game {}.", game_name)))
    }
}
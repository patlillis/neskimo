#![allow(clippy::upper_case_acronyms)]

use crate::utils::io::read_binary;
use std::fmt;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

pub const TRAINER_SIZE: usize = 0x0200;
pub const PRG_ROM_SIZE: usize = 0x4000;
pub const CHR_ROM_SIZE: usize = 0x2000;
pub const PRG_RAM_SIZE: usize = 0x2000;

#[derive(Debug, Copy, Clone)]
pub enum MirrorType {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug)]
pub enum TVSystem {
    NTSC,
    PAL,
}

#[derive(Debug)]
pub enum Mapper {
    NROM,
}

pub struct RomFile {
    // From the name of the iNES file.
    pub game_name: String,

    // Possible 512 bytes of trainer data.
    pub trainer_data: Option<[u8; TRAINER_SIZE]>,

    // PRG ROM in 16 KB units.
    pub prg_rom_data: Vec<[u8; PRG_ROM_SIZE]>,

    // CHR ROM in 8 KB units (0 units means the board uses CHR RAM).
    pub chr_rom_data: Vec<[u8; CHR_ROM_SIZE]>,

    // Size of PRG RAM in 8 KB units.
    pub prg_ram_size: usize,

    pub mirror_type: MirrorType,

    pub tv_system: TVSystem,

    pub vs_cart: bool,

    pub play_choice: bool,

    pub mapper: u8,
}

// TODO: finish this up.
impl fmt::Debug for RomFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "\
Game name        : {:?}
Has trainer data : {:?}
PRG ROM size     : {:?} bytes
CHR ROM size     : {:?} bytes
PRG RAM size     : {:?} bytes
Mirror Type      : {:?}
TV System        : {:?}
VS Unisystem     : {:?}
PlayChoice-10    : {:?}
Mapper #         : {:?}",
            self.game_name,
            self.trainer_data.is_some(),
            self.prg_rom_data.len() * PRG_ROM_SIZE,
            self.chr_rom_data.len() * CHR_ROM_SIZE,
            self.prg_ram_size * PRG_RAM_SIZE,
            self.mirror_type,
            self.tv_system,
            self.vs_cart,
            self.play_choice,
            self.mapper
        )
    }
}

impl fmt::Display for RomFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl RomFile {
    pub fn new(file_name: &str) -> Result<RomFile> {
        let bytes = read_binary(file_name)?;
        let game_name = Path::new(&file_name)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        // Oh my god someone please kill me.
        RomFile::new_from_buffer(game_name, &bytes)
    }

    pub fn new_from_buffer(game_name: String, rom: &[u8]) -> Result<RomFile> {
        // File must have enough space to be a valid header.
        if rom.len() < 16 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid iNES header: file less than 16 bytes.",
            ));
        }

        // Bytes 0-3: Check "NES" declaration.
        if &rom[0..3] != b"NES" || rom[3] != 0x1a {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid iNES header: missing \"NES\" declaration.",
            ));
        }

        // Byte 4: PRG ROM size.
        let prg_rom_size = rom[4];

        // Byte 5: CHR ROM size. 0 implies that the cartridge uses CHR RAM.
        let chr_rom_size = rom[5];

        // Byte 6 flags:
        // 0: Mirroring (0 = horizontal, 1 = vertical)
        // 1: Cartridge contains battery-backed PRG RAM at $6000 - $7fff
        // 2: 512 trainer at $7000 - $71ff
        // 3: 4-screen VRAM (ignore above mirroring bit)
        // 4-7: lower nybble of mapper number
        let flags_6 = rom[6];
        let mirror_type = match flags_6 & 0x09 {
            0x08 | 0x09 => MirrorType::Both,
            0x01 => MirrorType::Vertical,
            _ => MirrorType::Horizontal,
        };
        #[allow(unused_variables)]
        let has_prg_ram = flags_6 & 0x02 == 0x02;
        let has_trainer = flags_6 & 0x04 == 0x04;
        let mapper_lower: u8 = (flags_6 & 0xf0) >> 4;

        // Byte 7 flags:
        // 0: System is VS Unisystem
        // 1: System is PlayChoice-10 (8KB of Hint Screen data after CHR data)
        // 2-3: If equal to 2, flags 8-15 are in NES 2.0 format
        // 4-7: Upper nybble of mapper number
        let flags_7 = rom[7];
        let vs_cart = flags_7 & 0x01 == 0x01;
        let play_choice = flags_7 & 0x02 == 0x02;
        #[allow(unused_variables)]
        let nes_20 = (flags_7 & 0x0c) >> 2 == 0x02;
        let mapper_upper: u8 = flags_7 & 0xf0;

        // Byte 8: PRG RAM size (0 implies 1 8KB bank).
        let prg_ram_size = match rom[8] {
            0x00 => 0x01,
            size => size,
        };

        // Byte 9 flags:
        // 0: TV system (0 = NTSC, 1 = PAL),
        // 1-7: Reserved, set to 0.
        let flags_9 = rom[9];
        let tv_system = match flags_9 & 0x01 {
            0x00 => TVSystem::NTSC,
            _ => TVSystem::PAL,
        };

        let mut rom_file = RomFile {
            game_name,
            trainer_data: None,
            prg_rom_data: Vec::new(),
            chr_rom_data: Vec::new(),
            prg_ram_size: prg_ram_size as usize,
            mirror_type,
            tv_system,
            vs_cart,
            play_choice,
            mapper: mapper_upper | mapper_lower,
        };

        // Copy data from buffer into data object.
        let mut cursor = 0x10;

        // Load trainer data.
        if has_trainer {
            let mut trainer_data = [0x00; TRAINER_SIZE];
            trainer_data.copy_from_slice(&rom[cursor..(cursor + TRAINER_SIZE)]);
            rom_file.trainer_data = Some(trainer_data);
            cursor += TRAINER_SIZE;
        }

        // Load PRG ROM data.
        for _ in 0..prg_rom_size {
            let mut prg_rom = [0x00; PRG_ROM_SIZE];
            prg_rom.copy_from_slice(&rom[cursor..(cursor + PRG_ROM_SIZE)]);
            rom_file.prg_rom_data.push(prg_rom);
            cursor += PRG_ROM_SIZE;
        }

        // Load CHR ROM data.
        for _ in 0..chr_rom_size {
            let mut chr_rom = [0x00; CHR_ROM_SIZE];
            chr_rom.copy_from_slice(&rom[cursor..(cursor + CHR_ROM_SIZE)]);
            rom_file.chr_rom_data.push(chr_rom);
            cursor += CHR_ROM_SIZE;
        }

        Ok(rom_file)
    }
}

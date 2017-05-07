use nes::cpu::Cpu;
use nes::memory::Memory;
use rom::RomFile;

pub struct Nes {
    pub cpu: Cpu,
    // pub ppu: Ppu,
}

impl Nes {
    pub fn new(rom: RomFile) -> Nes {
        let mut memory = Memory::new();

        // Copy trainer data to 0x7000.
        match rom.trainer_data {
            Some(data) => memory.store_bytes(0x7000, &data),
            None => {}
        }

        // Copy PRG ROM into 0x800.
        // TODO: map upper bank to lower bank instead of copying it twice.
        match rom.prg_rom_data.len() {
            1 => {
                memory.store_bytes(0x8000, &rom.prg_rom_data[0]);
                memory.store_bytes(0xc000, &rom.prg_rom_data[0]);
            }
            2 => {
                memory.store_bytes(0x8000, &rom.prg_rom_data[0]);
                memory.store_bytes(0xc000, &rom.prg_rom_data[1]);
            }
            _ => {}
        }

        Nes { cpu: Cpu::new(memory) }
    }
}
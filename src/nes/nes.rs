use nes::cpu::Cpu;
use nes::memory::Memory;
use rom::RomFile;

#[derive(Debug)]
pub struct Options {
    // File to write CPU log to,
    pub logfile: Option<String>,

    // Program counter to start execution at.
    pub program_counter: Option<u16>,
}

pub struct Nes {
    pub cpu: Cpu,
    // pub ppu: Ppu,
}

impl Nes {
    pub fn new(rom: RomFile, options: Options) -> Nes {
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


        let cpu = match options.program_counter {
            Some(pc) => Cpu::new_at_pc(memory, pc, options.logfile),
            None => Cpu::new(memory, options.logfile),
        };
        Nes { cpu: cpu }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.execute();
        }
    }
}
use std::cell::RefCell;
use nes::cpu::Cpu;
use nes::memory::{BasicMemory, MappedMemory, Memory};
use nes::ppu::Ppu;
use rom::PRG_ROM_SIZE;
use std::rc::Rc;
use rom::RomFile;

#[derive(Debug, Default)]
pub struct Options {
    // File to write CPU log to,
    pub logfile: Option<String>,

    // Program counter to start execution at.
    pub program_counter: Option<u16>,

    // Program counter to dump memory at.
    pub mem_dump_counter: Option<u16>,
}

pub struct Nes {
    pub cpu: Cpu,
    pub ppu: Rc<RefCell<Ppu>>,
}

impl Nes {
    pub fn new(rom: RomFile, options: Options) -> Nes {
        let mut memory = MappedMemory::new(Box::new(BasicMemory::new()));
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        memory.add_mapping(ppu.clone(), ppu.clone());

        // Copy trainer data to 0x7000.
        match rom.trainer_data {
            Some(data) => memory.store_bytes(0x7000, &data),
            None => {}
        }

        // Copy PRG ROM into 0x800.
        let lower_bank = 0x8000;
        let upper_bank = 0xc000;
        match rom.prg_rom_data.len() {
            1 => {
                // Store PRG ROM in lower bank.
                memory.store_bytes(lower_bank, &rom.prg_rom_data[0]);

                // Mirror the upper bank to point to the lower bank.
                for offset in 0x0000..PRG_ROM_SIZE {
                    let from = upper_bank + offset as u16;
                    let to = lower_bank + offset as u16;
                    memory.add_mirror(from, to);
                }
            }
            2 => {
                memory.store_bytes(lower_bank, &rom.prg_rom_data[0]);
                memory.store_bytes(upper_bank, &rom.prg_rom_data[1]);
            }
            _ => {}
        }

        Nes {
            cpu: Cpu::new(Box::new(memory), options),
            ppu: ppu,
        }
    }

    pub fn step(&mut self) {
        self.cpu.execute();
        self.ppu.borrow_mut().step();
    }
}
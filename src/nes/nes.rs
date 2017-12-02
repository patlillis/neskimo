use nes::cpu::Cpu;
use nes::memory::{BasicMemory, MappedMemory, Memory};
use nes::ppu::Ppu;
use rom::PRG_ROM_SIZE;
use rom::RomFile;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::rc::Rc;

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
    cycle: u32,
    logfile: Option<File>,
}

impl Nes {
    pub fn new(rom: RomFile, options: Options) -> Nes {
        // Set up log file.
        let buffer = options
            .logfile
            .and_then(|f| {
                          OpenOptions::new()
                              .write(true)
                              .create(true)
                              .truncate(true)
                              .open(f)
                              .ok()
                      });

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
            cpu: Cpu::new(Box::new(memory),
                          options.program_counter,
                          options.mem_dump_counter),
            ppu: ppu,
            cycle: 0,
            logfile: buffer,
        }
    }

    pub fn step(&mut self) {
        let cycles_taken = self.cpu.execute();
        self.ppu.borrow_mut().step();

        if self.logfile.is_some() {
            self.log();
        }

        self.cycle += cycles_taken * 3;
    }

    fn log(&mut self) {
        match self.logfile {
            Some(ref mut file) => {
                let current_cycle = self.cycle % 341;
                writeln!(file, "{} CYC:{:3}", self.cpu.frame_log.log(), current_cycle).ok();
            }
            _ => {}
        }
    }
}
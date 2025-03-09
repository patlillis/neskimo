pub mod memory;

// Tests for various NES stuff.
#[cfg(test)]
mod memory_test;

use crate::cpu::Cpu;
use crate::nes::memory::{
    BasicMemory, DEFAULT_MEMORY_SIZE, MappedMemory, Memory,
};
use crate::ppu::Ppu;
use crate::rom::PRG_ROM_SIZE;
use crate::rom::RomFile;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::rc::Rc;
use std::time::Instant;

const CPU_FREQ: u32 = 1_789_773; // 1.789773 MHz
const PPU_CYCLES_PER_CPU_CYCLE: u32 = 3; // PPU runs 3x faster than CPU
const FRAME_RATE: u32 = 60;
const CPU_CYCLES_PER_FRAME: u32 = CPU_FREQ / FRAME_RATE; // ~29780 cycles

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
    cycles: u32,
    last_frame_start: std::time::Instant,
    logfile: Option<File>,
}

impl Nes {
    pub fn new(rom: &RomFile, options: Options) -> Nes {
        // Set up log file.
        let buffer = options.logfile.and_then(|f| {
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(f)
                .ok()
        });

        let mut memory = MappedMemory::new();
        memory.add_mapping(
            Rc::new(RefCell::new(BasicMemory::with_default_size())),
            (0x00..DEFAULT_MEMORY_SIZE).map(|x| x as u16),
            (0x00..DEFAULT_MEMORY_SIZE).map(|x| x as u16),
        );
        let ppu = Rc::new(RefCell::new(Ppu::new(rom.mirror_type)));
        memory.add_mapping(
            ppu.clone(),
            Ppu::mapped_addresses(),
            Ppu::mapped_addresses(),
        );

        // Copy trainer data to 0x7000.
        if let Some(data) = rom.trainer_data {
            memory.store_bytes(0x7000, &data);
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
            cpu: Cpu::new(
                Box::new(memory),
                options.program_counter,
                options.mem_dump_counter,
            ),
            ppu,
            cycles: 0,
            last_frame_start: Instant::now(),
            logfile: buffer,
        }
    }

    // Returns true if we're on a new frame.
    pub fn run_frame(&mut self) {
        let mut cpu_cycles_this_frame = 0;

        while cpu_cycles_this_frame < CPU_CYCLES_PER_FRAME {
            let cpu_cycles = self.cpu.execute();

            let ppu_cycles = cpu_cycles * PPU_CYCLES_PER_CPU_CYCLE;
            let (_new_frame, _v_blank) = self.ppu.borrow_mut().step(ppu_cycles);

            // TODO: Handle NMI on VBlank
            // if v_blank {
            //     self.cpu.nmi = true;
            // }

            cpu_cycles_this_frame += cpu_cycles;
        }

        self.sync_frame();

        if self.logfile.is_some() {
            self.log();
        }
    }

    fn sync_frame(&mut self) {
        const FRAME_TIME: std::time::Duration =
            std::time::Duration::from_nanos(16_666_667); // 60Hz
        let elapsed = self.last_frame_start.elapsed();
        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        } else {
            // We're running behind, don't sleep
            // Log if significant drift occurs
            println!("Frame time drift: {:?}", elapsed - FRAME_TIME);
        }
        self.last_frame_start = std::time::Instant::now();
    }

    fn log(&mut self) {
        if let Some(ref mut file) = self.logfile {
            let current_cycle = self.cycles % 341;
            writeln!(
                file,
                "{} CYC:{:3}",
                self.cpu.frame_log.log(),
                current_cycle
            )
            .ok();
        }
    }
}

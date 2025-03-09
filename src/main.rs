extern crate clap;
extern crate sdl2;

mod cpu;
mod gfx;
mod nes;
mod ppu;
mod rom;
mod utils;

use clap::{ArgAction, arg, command};
use gfx::Gfx;
use nes::{Nes, Options};
use rom::RomFile;
use sdl2::event::Event;

// The version of neskimo that we're building.
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = command!("neskimo")
        .version(VERSION)
        .author("Pat Lillis <lillispm@gmail.com>")
        .about("A bare-bones NES emulator written in Rust.")
        .arg(arg!(<ROM> "The .nes ROM file to run"))
        .arg(arg!(-l --logfile <LOGFILE> "Writes the CPU log to a file"))
        .arg(
            arg!(-p --"program-counter" <PROGRAM_COUNTER> "Sets the initial program counter to the provided hex value")
        )
        .arg(
            arg!(-d --"mem-dump" <MEM_DUMP> "When execution reaches this point, contents of memory will be written to mem_dump.bin")
        )
        .arg(
            arg!(-f --fps "Print frames-per-second during emulator run")
                .action(ArgAction::SetTrue)
        )
        .after_help(
            "EXAMPLES:
    neskimo mario.nes
    neskimo -l=testing.log donkey_kong.nes
    neskimo -p=C000 castlevania.nes
    neskimo --logfile=testing.log --program-counter=0F00 my-cool-game.nes"
        )
        .get_matches();

    // .expect() is safe here ROM is required, so clap will crash if it's not
    // there.
    let file_name = matches.get_one::<String>("ROM").expect("ROM is required");
    let rom = RomFile::new(file_name);

    // Get logfile, program counter, and memory dump counter.
    let logfile = matches.get_one::<String>("logfile").cloned();
    let pc = matches
        .get_one::<String>("program-counter")
        .and_then(|s| u16::from_str_radix(s, 16).ok());
    let dump_pc = matches
        .get_one::<String>("mem-dump")
        .and_then(|s| u16::from_str_radix(s, 16).ok());

    // Get the FPS flag.
    let fps = *matches.get_one::<bool>("fps").unwrap_or(&false);

    let options = Options {
        logfile,
        program_counter: pc,
        mem_dump_counter: dump_pc,
    };

    let mut nes = match rom {
        Ok(rom) => Nes::new(&rom, options),
        Err(e) => panic!("{}", e),
    };

    // Test screen that fades from black to blue and has a single pixel moving
    // across it.
    let (mut gfx, _) = Gfx::new(fps);

    'run: loop {
        nes.run_frame();

        gfx.composite(&mut nes.ppu.borrow_mut().screen);

        for event in gfx.events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'run,
                _ => continue,
            }
        }
    }
}

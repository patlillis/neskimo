#![allow(dead_code)]

extern crate clap;
#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate sdl2;

mod gfx;
mod nes;
mod rom;
mod utils;

use clap::{Arg, App};
use gfx::{Gfx, SCREEN_SIZE};
use nes::nes::{Nes, Options};
use rom::RomFile;

// The version of neskimo that we're building.
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("neskimo")
        .version(VERSION)
        .author("Pat Lillis <lillispm@gmail.com>")
        .about("A bare-bones NES emulator written in Rust.")
        .arg(Arg::with_name("ROM")
                 .help("The .nes ROM file to run")
                 .required(true)
                 .index(1))
        .arg(Arg::with_name("LOGFILE")
                 .short("l")
                 .long("logfile")
                 .value_name("LOGFILE")
                 .takes_value(true)
                 .help("Writes the CPU log to a file"))
        .arg(Arg::with_name("PROGRAM_COUNTER")
                 .short("p")
                 .long("program-counter")
                 .value_name("PROGRAM COUNTER")
                 .takes_value(true)
                 .help("Sets the initial program counter to the provided hex value"))
        .arg(Arg::with_name("MEM_DUMP")
                .short("d")
                .long("mem-dump")
                .value_name("PROGRAM COUNTER")
                .takes_value(true)
                .help("When executaion reaches this point, contents of memory will be written to mem_dump.bin"))
        .after_help("EXAMPLES:
    neskimo mario.nes
    neskimo -l=testing.log donkey_kong.nes
    neskimo -p=C000 castlevania.nes
    neskimo --logfile=testing.log --program-counter=0F00 my-cool-game.nes")
        .get_matches();


    // .unwrap() is safe here ROM is required, so clap will crash if it's not there.
    let file_name = matches.value_of("ROM").unwrap();
    let rom = RomFile::new(&file_name.to_string());

    // Get logfile.
    let logfile = matches.value_of("LOGFILE").map(|s| s.to_string());

    // Get program counter.
    let pc = matches
        .value_of("PROGRAM_COUNTER")
        .and_then(|s| u16::from_str_radix(s, 16).ok());

    let dump_pc = matches
        .value_of("MEM_DUMP")
        .and_then(|s| u16::from_str_radix(s, 16).ok());

    let options = Options {
        logfile: logfile,
        program_counter: pc,
        mem_dump_counter: dump_pc,
    };

    let mut nes = match rom {
        Ok(rom) => Nes::new(rom, options),
        Err(e) => panic!(e),
    };


    // TEST SCREEN!
    // let (mut gfx, _) = Gfx::new();

    // let mut screen = [0_u8; SCREEN_SIZE];

    // for i in 0..(SCREEN_SIZE / 3) - 1 {
    //     screen[i * 3] = 255;
    // }

    // gfx.composite(&screen);


    nes.run();
}
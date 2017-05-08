#![allow(dead_code)]

extern crate clap;
#[macro_use]
extern crate enum_primitive;
extern crate num;

mod nes;
mod rom;
mod utils;

use clap::{Arg, App};
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

    let options = Options {
        logfile: logfile,
        program_counter: pc,
    };

    let mut nes = match rom {
        Ok(rom) => Nes::new(rom, options),
        Err(e) => panic!(e),
    };

    nes.run();
}
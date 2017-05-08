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
                 .help("Write the CPU log to a file"))
        .get_matches();


    // .unwrap() is safe here ROM is required, so clap will crash if it's not there.
    let file_name = matches.value_of("ROM").unwrap();
    let rom = RomFile::new(&file_name.to_string());

    let logfile = matches.value_of("LOGFILE").unwrap_or_default();
    let options = Options { logfile: logfile.to_string() };

    let mut nes = match rom {
        Ok(rom) => Nes::new(rom, options),
        Err(e) => panic!(e),
    };

    nes.run();
}
#![allow(dead_code)]

extern crate clap;
#[macro_use]
extern crate enum_primitive;
extern crate num;

mod nes;
mod utils;

use clap::{Arg, App};

// The version of neskimo that we're building.
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("neskimo")
        .version(VERSION)
        .author("Pat Lillis <lillispm@gmail.com>")
        .about("A bare-bones NES emulator written in Rust.")
        .arg(Arg::with_name("FILE")
                 .help("The .nes ROM file to run")
                 .required(true)
                 .index(1))
        .get_matches();

    // .unwrap() is safe here FILE is required, so clap will crash if it's not there.
    let file = matches.value_of("FILE").unwrap();
    println!("Running ROM {}...", file);
}
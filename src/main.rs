extern crate clap;
extern crate neskimolib;
extern crate sdl2;

use clap::{Arg, App};
use neskimolib::gfx::{Gfx, SCREEN_WIDTH, SCREEN_SIZE};
use neskimolib::nes::{Nes, Options};
use neskimolib::rom::RomFile;
use sdl2::event::Event;

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

    // Test screen that fades from black to blue and has a single pixel moving
    // across it.
    let (mut gfx, _) = Gfx::new();
    let mut screen = [0_u8; SCREEN_SIZE];

    let mut counter: usize = 0;

    'run: loop {
        nes.step();

        for i in 0..(SCREEN_SIZE / 3) - 1 {
            screen[i * 3] = counter as u8;
            screen[i * 3 + 1] = 0;
            screen[i * 3 + 2] = 0;
        }

        screen[115 * SCREEN_WIDTH + (counter * 3)] = 255;
        screen[115 * SCREEN_WIDTH + (counter * 3) + 1] = 255;
        screen[115 * SCREEN_WIDTH + (counter * 3) + 2] = 255;

        gfx.composite(&screen);
        for event in gfx.events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'run,
                _ => continue,
            }
        }

        counter = (counter + 1) % 255;
    }
}
# neskimo

> A bare-bones NES emulator written in [Rust](https://www.rust-lang.org).

## Notes

Notes on general NES system architecture can be found at
[NES_NOTES.md](NES_NOTES.md).

Notes on implementation options and Rust in general can be found at
[IMPLEMENTATION_NOTES.md](IMPLEMENTATION_NOTES.md).

## Running

Currently the only way to run [neskimo](https://github.com/patlillis/neskimo) is
to build it from source. Once the project is further along, I hope to provide
pre-built binaries or installers.

### Clone the GitHub repo

First, ensure that you have [Git](https://git-scm.com/) installed.

Next, run the following command to pull the source code from GitHub onto your
local machine:

```bash
git clone https://github.com/patlillis/neskimo
cd neskimo
```

### Install SDL

The windowing part of [neskimo](https://github.com/patlillis/neskimo) relies on
the SDL framework, which must be installed separately.

#### MacOS

If building on a Mac, SDL can be installed easily through
[homebrew](https://brew.sh/) using the following command:

```bash
brew install sdl2
```

#### Windows & Linux

Follow the installation instructions on the [`rust-sdl2` GitHub page](https://github.com/Rust-SDL2/rust-sdl2/blob/master/README.md).

### Install Cargo

Since [neskimo](https://github.com/patlillis/neskimo) is written using
[Rust](https://www.rust-lang.org), the easiest way to build it is using
[Cargo](crates.io/install).

### Build

You should now be ready to build the project. Run one of the following commands
from the `neskimo` directory:

```bash
cargo build  #Build with the "dev" profile, best suited for development or debugging.
cargo build --release  #Build with the "release" profile, optimized for release builds.
```

### Run!

Binaries from `cargo build` and `cargo build --release` are built to
`target/debug/neskimo` and `target/release/neskimo`, respectively.

Note that you need to pass a `.nes` ROM file for neskimo to run. A few test ROMs
are provided in the GitHub repo, in the `test_roms` directory.

Run `neskimo --help` to see full usage instructions:

```
neskimo (version)
Pat Lillis <lillispm@gmail.com>
A bare-bones NES emulator written in Rust.

USAGE:
    neskimo [FLAGS] [OPTIONS] <ROM>

FLAGS:
    -f, --fps        Print frames-per-second during emulator run
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --logfile <LOGFILE>                    Writes the CPU log to a file
    -d, --mem-dump <PROGRAM COUNTER>           When executaion reaches this point, contents of memory will be written to mem_dump.bin
    -p, --program-counter <PROGRAM COUNTER>    Sets the initial program counter to the provided hex value

ARGS:
    <ROM>    The .nes ROM file to run

EXAMPLES:
    neskimo mario.nes
    neskimo -l=testing.log donkey_kong.nes
    neskimo -p=C000 castlevania.nes
    neskimo --logfile=testing.log --program-counter=0F00 my-cool-game.nes
```

## Inspiration

Inspiration drawn from these fantastic projects:

- [Nintengo](https://github.com/nwidger/nintengo) by Niels Widger (written in
  Go)
- [nes-rs](https://github.com/Reshurum/nes-rs) by Walter Kuppens (written in
  Rust)
- [sprocketnes](https://github.com/pcwalton/sprocketnes) by Patrick Walton
  (written in Rust)
- [pinky](https://github.com/koute/pinky) by Koute (written in Rust)
- [oxidenes](https://github.com/iamsix/oxidenes) by iamsix (written in Rust)

## License

[Code licensed MIT](LICENSE). © 2018 Pat Lillis.

[![Built With
Love](http://forthebadge.com/images/badges/built-with-love.svg)](http://forthebadge.com)

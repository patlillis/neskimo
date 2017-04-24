#![allow(dead_code)]

#[macro_use]
extern crate enum_primitive;
extern crate num;

mod clock;
mod cpu;
mod instruction;
mod memory;
mod opcode;
mod utils;

#[cfg(test)]
mod test;

fn main() {}
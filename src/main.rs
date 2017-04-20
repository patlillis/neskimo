mod cpu;
mod memory;

use cpu::{Status, Registers};

fn main() {
    let mut x = Registers::new();
    x.a = 0xf;
    x.pc = 0xfffc;
    println!("{:?}", x);
    println!("{}", x.p);
}

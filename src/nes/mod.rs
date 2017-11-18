pub mod nes;

mod cpu;
mod definition;
mod instruction;
mod memory;
mod opcode;

#[cfg(test)]
mod test_cpu;
#[cfg(test)]
mod test_memory;
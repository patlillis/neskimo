use std;
use memory;
use utils;

use instruction::Instruction;
// The status of the system processor.
pub struct Status(pub u8);

const C_FLAG: u8 = 1 << 0;
const Z_FLAG: u8 = 1 << 1;
const I_FLAG: u8 = 1 << 2;
const D_FLAG: u8 = 1 << 3;
const B_FLAG: u8 = 1 << 4;
const V_FLAG: u8 = 1 << 6;
const N_FLAG: u8 = 1 << 7;

impl Status {
    // Constructs a new status with all flags set to 0.
    pub fn new() -> Status {
        Status(0x0)
    }

    // Helper function for testing a mask against a status.
    fn matches_bits(&self, mask: u8) -> bool {
        self.0 & mask != 0
    }

    // Bit 0: Carry flag.
    pub fn c(&self) -> bool {
        self.matches_bits(C_FLAG)
    }

    // Bit 1: Zero flag.
    pub fn z(&self) -> bool {
        self.matches_bits(Z_FLAG)
    }

    // Bit 2: Interrupt flag.
    pub fn i(&self) -> bool {
        self.matches_bits(I_FLAG)
    }

    // Bit 3: Decimal mode.
    pub fn d(&self) -> bool {
        self.matches_bits(D_FLAG)
    }

    // Bit 4: Break command.
    pub fn b(&self) -> bool {
        self.matches_bits(B_FLAG)
    }

    // Bit 5: Unused.

    // Bit 6: Overflow flag.
    pub fn v(&self) -> bool {
        self.matches_bits(V_FLAG)
    }

    // Bit 7: Negative flag.
    pub fn n(&self) -> bool {
        self.matches_bits(N_FLAG)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status({:#04x})", self.0)
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Registers {
    // Accumulator.
    pub a: u8,
    // Index register X.
    pub x: u8,
    // Index register Y.
    pub y: u8,
    // Processor status.
    pub p: Status,
    // Stack pointer.
    pub sp: u8,
    // Program counter.
    pub pc: u16,
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "Registers(\
                a: {:#04x}, \
                x: {:#04x}, \
                y: {:#04x}, \
                p: {}, \
                sp: {:#04x}, \
                pc: {:#06x})",
               self.a,
               self.x,
               self.y,
               self.p,
               self.sp,
               self.pc)
    }
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x0,
            x: 0x0,
            y: 0x0,
            p: Status::new(),
            sp: 0x0,
            pc: 0x0,
        }
    }
}

pub struct Cpu {
    pub registers: Registers,
    pub memory: memory::Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: memory::Memory::new(),
        }
    }

    pub fn reset(&mut self) {
        self.memory.reset();
        self.registers = Registers::new();
    }

    // Executes the instruction at PC.
    pub fn execute(&mut self) {
        let (instr, definition) = Instruction::parse(self.registers.pc, &self.memory);
        instr.execute(self);

        // Increment program counter.
        self.registers.pc = self.registers.pc + definition.len;
    }

    fn set_z_flag(&mut self, value: u8) {
        match value {
            0 => self.registers.p.0 |= Z_FLAG,
            _ => self.registers.p.0 &= !Z_FLAG,
        };
    }

    fn set_n_flag(&mut self, value: u8) {
        if utils::arithmetic::is_negative(value) {
            self.registers.p.0 |= N_FLAG;
        } else {
            self.registers.p.0 &= !N_FLAG;
        }
    }

    // Adds one to the value held at a specified memory location setting
    // the zero and negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if result is zero
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn inc(&mut self, address: u16) {
        let value = self.memory.fetch(address).wrapping_add(1);
        self.memory.store(address, value);
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Subtracts one from the value held at a specified memory location setting
    // the zero and negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if result is zero
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn dec(&mut self, address: u16) {
        let value = self.memory.fetch(address).wrapping_sub(1);
        self.memory.store(address, value);
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Loads a byte into the accumulator setting the zero and
    // negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if A = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of A is set
    pub fn lda(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.lda_value(value);
    }

    pub fn lda_value(&mut self, value: u8) {
        self.registers.a = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Loads a byte of memory into the X register setting the zero and
    // negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if X = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of X is set
    pub fn ldx(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.ldx_value(value);
    }

    pub fn ldx_value(&mut self, value: u8) {
        self.registers.x = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Loads a byte of memory into the Y register setting the zero and
    // negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if Y = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of Y is set
    pub fn ldy(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.ldy_value(value);
    }

    pub fn ldy_value(&mut self, value: u8) {
        self.registers.y = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Stores the contents of the accumulator into memory.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn sta(&mut self, address: u16) {
        self.memory.store(address, self.registers.a);
    }
}

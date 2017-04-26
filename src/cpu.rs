use std;
use memory;
use utils;

use instruction::Instruction;
// The status of the system processor.
pub struct Status(pub u8);

pub const C_FLAG: u8 = 1 << 0;
pub const Z_FLAG: u8 = 1 << 1;
pub const I_FLAG: u8 = 1 << 2;
pub const D_FLAG: u8 = 1 << 3;
pub const B_FLAG: u8 = 1 << 4;
pub const V_FLAG: u8 = 1 << 6;
pub const N_FLAG: u8 = 1 << 7;

impl Status {
    // Constructs a new status with all flags set to 0.
    pub fn new() -> Status {
        Status(0x0)
    }

    // Helper function for testing a mask against a status.
    fn matches_bits(&self, mask: u8) -> bool {
        self.0 & mask == mask
    }

    // Helper function for setting bits against a mask.
    fn set_bits(&mut self, mask: u8, value: bool) {
        if value {
            self.0 |= mask
        } else {
            self.0 &= !mask;
        }
    }

    // Bit 0: Carry flag.
    pub fn c(&self) -> bool {
        self.matches_bits(C_FLAG)
    }

    pub fn set_c(&mut self, value: bool) {
        self.set_bits(C_FLAG, value);
    }

    // Bit 1: Zero flag.
    pub fn z(&self) -> bool {
        self.matches_bits(Z_FLAG)
    }

    pub fn set_z(&mut self, value: bool) {
        self.set_bits(Z_FLAG, value);
    }

    // Bit 2: Interrupt flag.
    pub fn i(&self) -> bool {
        self.matches_bits(I_FLAG)
    }

    pub fn set_i(&mut self, value: bool) {
        self.set_bits(I_FLAG, value);
    }

    // Bit 3: Decimal mode.
    pub fn d(&self) -> bool {
        self.matches_bits(D_FLAG)
    }

    pub fn set_d(&mut self, value: bool) {
        self.set_bits(D_FLAG, value);
    }

    // Bit 4: Break command.
    pub fn b(&self) -> bool {
        self.matches_bits(B_FLAG)
    }

    pub fn set_b(&mut self, value: bool) {
        self.set_bits(B_FLAG, value);
    }

    // Bit 5: Unused.

    // Bit 6: Overflow flag.
    pub fn v(&self) -> bool {
        self.matches_bits(V_FLAG)
    }

    pub fn set_v(&mut self, value: bool) {
        self.set_bits(V_FLAG, value);
    }

    // Bit 7: Negative flag.
    pub fn n(&self) -> bool {
        self.matches_bits(N_FLAG)
    }

    pub fn set_n(&mut self, value: bool) {
        self.set_bits(N_FLAG, value);
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

    // Set the carry flag to zero.
    //
    //         C    Carry Flag          Set to 0
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn clc(&mut self) {
        self.registers.p.set_c(false);
    }

    // Set the decimal mode flag to zero.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Set to 0
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn cld(&mut self) {
        self.registers.p.set_d(false);
    }

    // Clears the interrupt disable flag allowing normal interrupt
    // requests to be serviced.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Set to 0
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn cli(&mut self) {
        self.registers.p.set_i(false);
    }

    // Clears the interrupt disable flag allowing normal interrupt
    // requests to be serviced.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Set to 0
    //         N    Negative Flag       Not affected
    pub fn clv(&mut self) {
        self.registers.p.set_v(false);
    }

    // Set the carry flag to one.
    //
    //         C    Carry Flag          Set to 1
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn sec(&mut self) {
        self.registers.p.set_c(true);
    }

    // Set the decimal mode flag to one.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Set to 1
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn sed(&mut self) {
        self.registers.p.set_d(true);
    }

    // Set the interrupt disable flag to one.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Set to 1
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn sei(&mut self) {
        self.registers.p.set_i(true);
    }

    // Shifts all bits of the value one bit left. Bit 0 is set to 0,
    // and bit 7 is placed in the carry flag. This multiplies the value
    // by 2, setting the carry if the result will not fit in 8 bits.
    // The shifted value is returned.
    //
    //         C    Carry Flag          Set to contents of old bit 7
    //         Z    Zero Flag           Set if value = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn asl(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let shifted_value = self.shift_l(value);
        self.memory.store(address, shifted_value);
    }

    pub fn asl_a(&mut self) {
        let value = self.registers.a;
        let shifted_value = self.shift_l(value);
        self.registers.a = shifted_value;
    }

    fn shift_l(&mut self, value: u8) -> u8 {
        let shifted_value = value << 1;

        let carry = value & 0x80 == 0x80;
        self.registers.p.set_c(carry);

        self.set_z_flag(shifted_value);
        self.set_n_flag(shifted_value);

        shifted_value
    }

    // Used to test if one or more bits are set at the specified memory
    // location. The value in A is ANDed with the value in memory to
    // set or unset the zero flag, but the result is not kept. Bits 6 and 7
    // of the value in memory are copied into the V and N flags respectively.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if (value & accumulator) = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Set to bit 6 of value
    //         N    Negative Flag       Set to bit 7 of value
    pub fn bit(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let zero_test = self.registers.a & value;
        self.set_z_flag(zero_test);
        self.registers.p.set_v(value & V_FLAG == V_FLAG);
        self.registers.p.set_n(value & N_FLAG == N_FLAG);
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

    // Compares the contents of the accumulator with another value, and sets
    // the carry, zero, and negative flags as appropriate.
    //
    //         C    Carry Flag          Set if A >= M
    //         Z    Zero Flag           Set if A = M
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if A < M
    pub fn cmp(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.cmp_value(value);
    }

    pub fn cmp_value(&mut self, value: u8) {
        let carry = self.registers.a >= value;
        self.registers.p.set_c(carry);

        let zero = self.registers.a == value;
        self.registers.p.set_z(zero);

        let comparison = self.registers.a.wrapping_sub(value);
        self.set_n_flag(comparison);
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

    // Causes no chage to the processor other than normal incrementing
    // of the program counter.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn nop(&mut self) {}

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

    // Stores the contents of the X register into memory.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn stx(&mut self, address: u16) {
        self.memory.store(address, self.registers.x);
    }

    // Stores the contents of the Y register into memory.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn sty(&mut self, address: u16) {
        self.memory.store(address, self.registers.y);
    }

    // Copies the contents of the accumulator into the X register,
    // setting the zero and negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if X = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of X is set
    pub fn tax(&mut self) {
        let value = self.registers.a;
        self.registers.x = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Copies the contents of the X register into the accumulator,
    // setting the zero and negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if A = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of A is set
    pub fn txa(&mut self) {
        let value = self.registers.x;
        self.registers.a = self.registers.x;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Subtracts one from the X register, setting the zero and negative
    // flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if X is zero
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the X is set
    pub fn dex(&mut self) {
        let value = self.registers.x.wrapping_sub(1);
        self.registers.x = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Adds one from the X register, setting the zero and negative
    // flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if X is zero
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the X is set
    pub fn inx(&mut self) {
        let value = self.registers.x.wrapping_add(1);
        self.registers.x = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Copies the contents of the accumulator into the Y register,
    // setting the zero and negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if Y = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of Y is set
    pub fn tay(&mut self) {
        let value = self.registers.a;
        self.registers.y = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Copies the contents of the Y register into the accumulator,
    // setting the zero and negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if A = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of A is set
    pub fn tya(&mut self) {
        let value = self.registers.y;
        self.registers.a = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Subtracts one from the Y register, setting the zero and negative
    // flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if Y is zero
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the Y is set
    pub fn dey(&mut self) {
        let value = self.registers.y.wrapping_sub(1);
        self.registers.y = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }

    // Adds one from the Y register, setting the zero and negative
    // flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if Y is zero
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the Y is set
    pub fn iny(&mut self) {
        let value = self.registers.y.wrapping_add(1);
        self.registers.y = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }
}

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
pub const U_FLAG: u8 = 1 << 5;
pub const V_FLAG: u8 = 1 << 6;
pub const N_FLAG: u8 = 1 << 7;

pub const NMI_VECTOR: u16 = 0xfffa;
pub const RESET_VECTOR: u16 = 0xfffc;
pub const IRQ_VECTOR: u16 = 0xfffe;

impl Status {
    // Constructs a new Status object, with only the I flag set.
    pub fn new() -> Status {
        Status(I_FLAG)
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
        write!(f, "Status({:08b})", self.0)
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
    // Constructs a new Registers object, with SP set to 0xfd,
    // PC set to 0xfffc (the RESET vector).
    pub fn new() -> Registers {
        Registers::new_at_pc(0x0000)
    }

    pub fn new_at_pc(pc: u16) -> Registers {
        Registers {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            p: Status::new(),
            sp: 0xfd,
            pc: pc,
        }
    }

    pub fn reset(&mut self) {
        self.reset_to_pc(0x0000);
    }

    pub fn reset_to_pc(&mut self, pc: u16) {
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.p = Status::new();
        self.sp = 0xfd;
        self.pc = pc;
    }
}

pub struct Cpu {
    pub registers: Registers,
    pub memory: memory::Memory,
    pub irq: bool,
    pub nmi: bool,
    pub reset: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: memory::Memory::new(),
            irq: false,
            nmi: false,
            reset: false,
        }
    }

    pub fn reset(&mut self) {
        self.reset_to_pc(0x0000);
    }

    pub fn reset_to_pc(&mut self, pc: u16) {
        self.memory.reset();
        self.registers.reset_to_pc(pc);
        self.irq = false;
        self.nmi = false;
        self.reset = false;
    }

    // Executes the instruction at PC.
    pub fn execute(&mut self) {
        let (instr, definition) = Instruction::parse(self.registers.pc, &self.memory);

        // Increment program counter.
        self.registers.pc = self.registers.pc + definition.len;

        // Execute the instruction.
        instr.execute(self);

        // Check interrupts.
        self.check_interrupts();
    }

    // Checks the interrupt lines, and sets the pc to the
    // value in the correct interrupt vector if neccesary.
    fn check_interrupts(&mut self) {
        if self.irq && !self.registers.p.i() {
            self.handle_irq();
            self.irq = false;
        } else if self.nmi {
            self.handle_nmi();
            self.nmi = false;
        } else if self.reset {
            self.handle_reset();
            self.reset = false;
        }
    }

    // Handle interrupt on the IRQ line.
    fn handle_irq(&mut self) {
        // Push return address and status onto stack. U_FLAG is 1, B_FLAG is 0.
        let pc = self.registers.pc;
        let status = (self.registers.p.0 | U_FLAG) & !B_FLAG;
        self.push_u16(pc);
        self.push(status);

        // Turn on interrupt disable.
        self.registers.p.set_i(true);

        // Fetch memory from IRQ vector.
        let vector = self.memory.fetch_u16(IRQ_VECTOR);
        self.registers.pc = vector;
    }

    // Handle interrupt on the NMI line.
    fn handle_nmi(&mut self) {
        // Push return address and status onto stack. U_FLAG is 1, B_FLAG is 0.
        let pc = self.registers.pc;
        let status = (self.registers.p.0 | U_FLAG) & !B_FLAG;
        self.push_u16(pc);
        self.push(status);

        // Turn on interrupt disable.
        self.registers.p.set_i(true);

        // Fetch memory from NMI vector.
        let vector = self.memory.fetch_u16(NMI_VECTOR);
        self.registers.pc = vector;
    }

    // Handle interrupt on the RESET line. Note that in the original 6502,
    // a RESET actually triggered the same sequence as IRQ and NMI, but with
    // the read/write bus set to "read", so no memory was modified. However,
    // the stack pointer was decremented 3 times, which is why the stack pointer
    // on startup is set to 0xfd (0x00 - 3).
    fn handle_reset(&mut self) {
        let vector = self.memory.fetch_u16(RESET_VECTOR);
        self.registers.pc = vector;
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


    // Copies the current contents of the X register into the stack
    // register and sets the zero and negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if SP = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of SP is set
    pub fn txs(&mut self) {
        let value = self.registers.x;
        self.registers.sp = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }


    // Copies the current contents of the stack register into the X
    // register and sets the zero and negative flags as appropriate.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if X = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of X is set
    pub fn tsx(&mut self) {
        let value = self.registers.sp;
        self.registers.x = value;
        self.set_z_flag(value);
        self.set_n_flag(value);
    }


    // Push value onto stack, and decrement stack pointer.
    pub fn push(&mut self, value: u8) {
        self.memory
            .store(utils::arithmetic::concat_bytes(0x01, self.registers.sp),
                   value);
        self.registers.sp = self.registers.sp - 1;
    }

    // Push value onto stack, high byte first then low byte.
    pub fn push_u16(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push(value as u8);
    }

    // Pull a value off of the stack, and increment stack pointer.
    pub fn pull(&mut self) -> u8 {
        self.registers.sp = self.registers.sp + 1;
        self.memory
            .fetch(utils::arithmetic::concat_bytes(0x01, self.registers.sp))
    }

    pub fn pull_u16(&mut self) -> u16 {
        let low = self.pull();
        let high = self.pull();

        utils::arithmetic::concat_bytes(high, low)
    }

    // Pushes a copy of the accumulator on to the stack.
    //
    // No processor status flags are affected.
    pub fn pha(&mut self) {
        let value = self.registers.a;
        self.push(value);
    }

    // Pushes a copy of the processor status flags on to the stack.
    //
    // No processor status flags are affected.
    pub fn php(&mut self) {
        let value = self.registers.p.0 | B_FLAG | U_FLAG;
        self.push(value);
    }

    // Pulls the value off of the stack, and puts it into the
    // accumulator.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if A = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of A is set
    pub fn pla(&mut self) {
        let value = self.pull();
        self.registers.a = value;
        self.set_n_flag(value);
        self.set_z_flag(value);
    }

    // Pulls the value off of the stack, and sets the processor flags
    // based on the result.
    //         C    Carry Flag          Set from stack
    //         Z    Zero Flag           Set from stack
    //         I    Interrupt Disable   Set from stack
    //         D    Decimal Mode Flag   Set from stack
    //         B    Break Command       Unset
    //         V    Overflow Flag       Set from stack
    //         N    Negative Flag       Set from stack
    pub fn plp(&mut self) {
        let value = self.pull();
        self.registers.p.0 = value & !B_FLAG & !U_FLAG;
    }


    // This instruction adds the contents of a memory location to the
    // accumulator together with the carry bit, and stores the sum
    // in the accumulator. If overflow occurs the carry bit is set.
    // This enables multiple byte addition to be performed.
    // TODO: Decimal mode.
    //
    //         C    Carry Flag          Set if overflow in bit 7
    //         Z    Zero Flag           Set if result = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Set if sign bit is incorrect
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn adc(&mut self, address: u16) {
        let arg = self.memory.fetch(address);
        self.adc_value(arg);
    }

    pub fn adc_value(&mut self, value: u8) {
        let initial_carry = if self.registers.p.c() { 1 } else { 0 };
        let (sum, carry) = self.registers
            .a
            .overflowing_add(value.wrapping_add(initial_carry));

        // The overflow flag is set when the sign of the addends is the same and
        // differs from the sign of the sum
        //  !(self.registers.a ^ value) ==> Do the addends sign match?
        // & (self.registers.a ^ sum) ====> Do A and result have different signs?
        // & 0x80                     ====> Extract sign bit.
        let overflow = !(self.registers.a ^ value) & (self.registers.a ^ sum) & 0x80 == 0x80;

        // Store result in accumulator.
        self.registers.a = sum;

        // Set Z and N flags.
        self.set_z_flag(sum);
        self.set_n_flag(sum);

        // Set carry flag if addition overflowed.
        self.registers.p.set_c(carry);

        // Set overflow flag if sign is incorrect.
        self.registers.p.set_v(overflow);
    }


    // Performs a bitwise and of the contents of a memory location
    // with the accumulator, storing the result back in the accumulator.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if result = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn and(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.and_value(value);
    }

    pub fn and_value(&mut self, value: u8) {
        let result = self.registers.a & value;
        self.registers.a = result;
        self.set_z_flag(result);
        self.set_n_flag(result);
    }


    // This instruction subtracts the contents a memory location from
    // the accumulator, adding the carry bit, and stores the result
    // in the accumulator. If overflow does not occur, the carry bit is set.
    // This enables multiple byte subtraction to be performed.
    // TODO: Decimal mode.
    //
    //         C    Carry Flag          Clear if overflow in bit 7
    //         Z    Zero Flag           Set if result = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Set if sign bit is incorrect
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn sbc(&mut self, address: u16) {
        let arg = self.memory.fetch(address);
        self.sbc_value(arg);
    }

    pub fn sbc_value(&mut self, arg: u8) {
        self.adc_value(!arg);
    }


    // Rotates bits in the memory address to the left. Bit 7 is placed in
    // the carry flag, and bit 0 is set to the old value of the carry flag.
    //
    //         C    Carry Flag          Set to contents of old bit 7
    //         Z    Zero Flag           Set if value = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn rol(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let rotated_value = self.rotate_l(value);
        self.memory.store(address, rotated_value);
    }

    pub fn rol_a(&mut self) {
        let value = self.registers.a;
        let rotated_value = self.rotate_l(value);
        self.registers.a = rotated_value;
    }

    // Rotates the value left, through the carry flag, and returns
    // the shifted value. C, Z and N flags are set appropriately.
    fn rotate_l(&mut self, value: u8) -> u8 {
        let c_flag_bit = self.registers.p.0 & C_FLAG;
        let rotated_value = value << 1 | c_flag_bit;

        let carry = value & 0x80 == 0x80;
        self.registers.p.set_c(carry);

        self.set_z_flag(rotated_value);
        self.set_n_flag(rotated_value);

        rotated_value
    }


    // Rotates bits in the memory address to the right. Bit 0 is placed in
    // the carry flag, and bit 7 is set to the old value of the carry flag.
    //
    //         C    Carry Flag          Set to contents of old bit 0
    //         Z    Zero Flag           Set if value = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn ror(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let rotated_value = self.rotate_r(value);
        self.memory.store(address, rotated_value);
    }

    pub fn ror_a(&mut self) {
        let value = self.registers.a;
        let rotated_value = self.rotate_r(value);
        self.registers.a = rotated_value;
    }

    // Rotates the value right, through the carry flag, and returns
    // the shifted value. C, Z and N flags are set appropriately.
    fn rotate_r(&mut self, value: u8) -> u8 {
        let c_flag_bit = (self.registers.p.0 & C_FLAG) << 7;
        let rotated_value = value >> 1 | c_flag_bit;

        let carry = value & 0x01 == 0x01;
        self.registers.p.set_c(carry);

        self.set_z_flag(rotated_value);
        self.set_n_flag(rotated_value);

        rotated_value
    }


    // Shifts all bits of the value one bit left. Bit 0 is set to 0,
    // and bit 7 is placed in the carry flag. This multiplies the value
    // by 2, setting the carry if the result will not fit in 8 bits.
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

    // Shifts the value, sets status flags, and returns the shifted value.
    // C, Z, and N flags are set appropriately.
    fn shift_l(&mut self, value: u8) -> u8 {
        let shifted_value = value << 1;

        let carry = value & 0x80 == 0x80;
        self.registers.p.set_c(carry);

        self.set_z_flag(shifted_value);
        self.set_n_flag(shifted_value);

        shifted_value
    }


    // Shifts all bits of the value one bit right. Bit 7 is set to 0,
    // and bit 0 is placed in the carry flag.
    //
    //         C    Carry Flag          Set to contents of old bit 0
    //         Z    Zero Flag           Set if value = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn lsr(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let shifted_value = self.shift_r(value);
        self.memory.store(address, shifted_value);
    }

    pub fn lsr_a(&mut self) {
        let value = self.registers.a;
        let shifted_value = self.shift_r(value);
        self.registers.a = shifted_value;
    }

    // Shifts the value, sets status flags, and returns the shifted value.
    fn shift_r(&mut self, value: u8) -> u8 {
        let shifted_value = value >> 1;

        let carry = value & 0x01 == 0x01;
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


    // Performs a bitwise exclusive or of the contents of a memory location
    // with the accumulator, storing the result back in the accumulator.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if result = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn eor(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.eor_value(value);
    }

    pub fn eor_value(&mut self, value: u8) {
        let result = self.registers.a ^ value;
        self.registers.a = result;
        self.set_z_flag(result);
        self.set_n_flag(result);
    }


    // Compares the contents of A, X, or Y register with another value,
    // and sets the carry, zero, and negative flags as appropriate.
    //
    //         C    Carry Flag          Set if register >= value
    //         Z    Zero Flag           Set if register = value
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if register < value
    fn compare(&mut self, register: u8, value: u8) {
        let carry = register >= value;
        self.registers.p.set_c(carry);

        let zero = register == value;
        self.registers.p.set_z(zero);

        let comparison = register.wrapping_sub(value);
        self.set_n_flag(comparison);
    }

    // Compare with Accumulator.
    pub fn cmp(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let register = self.registers.a;
        self.compare(register, value);
    }

    pub fn cmp_value(&mut self, value: u8) {
        let register = self.registers.a;
        self.compare(register, value);
    }

    // Compare with X register.
    pub fn cpx(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let register = self.registers.x;
        self.compare(register, value);
    }

    pub fn cpx_value(&mut self, value: u8) {
        let register = self.registers.x;
        self.compare(register, value);
    }

    // Compare with Y register.
    pub fn cpy(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        let register = self.registers.y;
        self.compare(register, value);
    }

    pub fn cpy_value(&mut self, value: u8) {
        let register = self.registers.y;
        self.compare(register, value);
    }


    // Sets the program counter to the address specified.
    //
    // No processor status flags are affected.
    pub fn jmp(&mut self, address: u16) {
        self.registers.pc = address;
    }


    // Pushes the program counter (minus one) of the return from
    // the subroutine onto the stack, then sets the program counter
    // to the address.
    //
    // No processor status flags are affected.
    pub fn jsr(&mut self, address: u16) {
        let return_addr = self.registers.pc.wrapping_sub(1);
        self.push_u16(return_addr);
        self.registers.pc = address;
    }


    // Address is pulled off the stack, and program counter is set to
    // that address + 1.
    //
    // No processor status flags are affected.
    pub fn rts(&mut self) {
        let address = self.pull_u16();
        self.registers.pc = address + 1;
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


    // Performs a bitwise or of the contents of a memory location
    // with the accumulator, storing the result back in the accumulator.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Set if result = 0
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Not affected
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Set if bit 7 of the result is set
    pub fn ora(&mut self, address: u16) {
        let value = self.memory.fetch(address);
        self.ora_value(value);
    }

    pub fn ora_value(&mut self, value: u8) {
        let result = self.registers.a | value;
        self.registers.a = result;
        self.set_z_flag(result);
        self.set_n_flag(result);
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


    // Forces an interrupt. The PC and status flags are pushed onto the stack,
    // then the PC is set to the value in the IRQ vector ($fffe) and the
    // break status flag is set to 1.
    //
    //         C    Carry Flag          Not affected
    //         Z    Zero Flag           Not affected
    //         I    Interrupt Disable   Not affected
    //         D    Decimal Mode Flag   Not affected
    //         B    Break Command       Set to 1
    //         V    Overflow Flag       Not affected
    //         N    Negative Flag       Not affected
    pub fn brk(&mut self) {
        // Push return address and status onto stack. U_FLAG is 1, B_FLAG is 1.
        let pc = self.registers.pc;
        let status = self.registers.p.0 | U_FLAG | B_FLAG;
        self.push_u16(pc);
        self.push(status);

        // Turn on interrupt disable.
        self.registers.p.set_i(true);

        // Fetch memory from IRQ vector.
        let vector = self.memory.fetch_u16(IRQ_VECTOR);
        self.registers.pc = vector;
    }

    // Branches to the specified address only if the Negative flag is cleared.
    //
    // No processor status flags are affected.
    pub fn bpl(&mut self, address: u16) {
        let condition = self.registers.p.n() == false;
        self.branch(condition, address);
    }

    // Branches to the specified address only if the Negative flag is set.
    //
    // No processor status flags are affected.
    pub fn bmi(&mut self, address: u16) {
        let condition = self.registers.p.n() == true;
        self.branch(condition, address);
    }

    // Branches to the specified address only if the Overflow flag is cleared.
    //
    // No processor status flags are affected.
    pub fn bvc(&mut self, address: u16) {
        let condition = self.registers.p.v() == false;
        self.branch(condition, address);
    }

    // Branches to the specified address only if the Overflow flag is set.
    //
    // No processor status flags are affected.
    pub fn bvs(&mut self, address: u16) {
        let condition = self.registers.p.v() == true;
        self.branch(condition, address);
    }

    // Branches to the specified address only if the Carry flag is cleared.
    //
    // No processor status flags are affected.
    pub fn bcc(&mut self, address: u16) {
        let condition = self.registers.p.c() == false;
        self.branch(condition, address);
    }

    // Branches to the specified address only if the Carry flag is set.
    //
    // No processor status flags are affected.
    pub fn bcs(&mut self, address: u16) {
        let condition = self.registers.p.c() == true;
        self.branch(condition, address);
    }

    // Branches to the specified address only if the Zero flag is cleared.
    //
    // No processor status flags are affected.
    pub fn bne(&mut self, address: u16) {
        let condition = self.registers.p.z() == false;
        self.branch(condition, address);
    }

    // Branches to the specified address only if the Zero flag is set.
    //
    // No processor status flags are affected.
    pub fn beq(&mut self, address: u16) {
        let condition = self.registers.p.z() == true;
        self.branch(condition, address);
    }

    // If condition is true, sets program counter to the
    // specified address.
    fn branch(&mut self, condition: bool, address: u16) {
        if condition {
            self.registers.pc = address;
        }
    }


    // Used to return from an interrupt handling routine. Pulls
    // the processor flags and PC from the stack.
    //
    //         C    Carry Flag          Set from stack
    //         Z    Zero Flag           Set from stack
    //         I    Interrupt Disable   Set from stack
    //         D    Decimal Mode Flag   Set from stack
    //         B    Break Command       Set to 0
    //         V    Overflow Flag       Set from stack
    //         N    Negative Flag       Set from stack
    pub fn rti(&mut self) {
        let status = self.pull() & !(B_FLAG | U_FLAG);
        let pc = self.pull_u16();
        self.registers.p.0 = status;
        self.registers.pc = pc;
    }
}

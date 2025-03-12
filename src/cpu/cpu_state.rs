pub struct CpuState {
    pub registers: Registers,
    pub irq: bool,
    pub nmi: bool,
    pub reset: bool,
}

impl CpuState {
    pub fn new(pc: u16) -> CpuState {
        CpuState {
            registers: Registers::new_at_pc(pc),
            irq: false,
            nmi: false,
            reset: false,
        }
    }
}

// The status of the system processor.
#[derive(Default)]
pub struct Status(pub u8);

#[repr(u8)]
pub enum CpuFlags {
    C = 1 << 0,
    Z = 1 << 1,
    I = 1 << 2,
    D = 1 << 3,
    B = 1 << 4,
    U = 1 << 5,
    V = 1 << 6,
    N = 1 << 7,
}

#[repr(u16)]
pub enum CpuVectors {
    NMI = 0xfffa,
    RESET = 0xfffc,
    IRQ = 0xfffe,
}

impl Status {
    // Constructs a new Status object, with only the I flag set.
    pub fn new() -> Status {
        Status(CpuFlags::I)
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
        self.matches_bits(CpuFlags::C)
    }

    pub fn set_c(&mut self, value: bool) {
        self.set_bits(CpuFlags::C, value);
    }

    // Bit 1: Zero flag.
    pub fn z(&self) -> bool {
        self.matches_bits(CpuFlags::Z)
    }

    pub fn set_z(&mut self, value: bool) {
        self.set_bits(CpuFlags::Z, value);
    }

    // Bit 2: Interrupt flag.
    pub fn i(&self) -> bool {
        self.matches_bits(CpuFlags::I)
    }

    pub fn set_i(&mut self, value: bool) {
        self.set_bits(CpuFlags::I, value);
    }

    // Bit 3: Decimal mode.
    pub fn d(&self) -> bool {
        self.matches_bits(CpuFlags::D)
    }

    pub fn set_d(&mut self, value: bool) {
        self.set_bits(CpuFlags::D, value);
    }

    // Bit 4: Break command.
    pub fn b(&self) -> bool {
        self.matches_bits(CpuFlags::B)
    }

    pub fn set_b(&mut self, value: bool) {
        self.set_bits(CpuFlags::B, value);
    }

    // Bit 5: Unused.

    // Bit 6: Overflow flag.
    pub fn v(&self) -> bool {
        self.matches_bits(CpuFlags::V)
    }

    pub fn set_v(&mut self, value: bool) {
        self.set_bits(CpuFlags::V, value);
    }

    // Bit 7: Negative flag.
    pub fn n(&self) -> bool {
        self.matches_bits(CpuFlags::N)
    }

    pub fn set_n(&mut self, value: bool) {
        self.set_bits(CpuFlags::N, value);
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
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
        write!(
            f,
            "Registers(\
             a: {:#04x}, \
             x: {:#04x}, \
             y: {:#04x}, \
             p: {}, \
             sp: {:#04x}, \
             pc: {:#06x})",
            self.a, self.x, self.y, self.p, self.sp, self.pc
        )
    }
}

impl Registers {
    // Constructs a new Registers object, with SP set to 0xfd.
    pub fn new_at_pc(pc: u16) -> Registers {
        Registers {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            p: Status::new(),
            sp: 0xfd,
            pc,
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

    pub fn log(&self) -> String {
        format!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.a,
            self.x,
            self.y,
            self.p.0 | CpuFlags::U,
            self.sp
        )
    }
}

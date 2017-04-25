use cpu;
use opcode::Opcode::*;

#[test]
fn test_bit() {
    let mut cpu = cpu::Cpu::new();

    // Z flag test is ANDed with the accumulator.
    cpu.registers.a = 0xc8;

    // For testing the Z flag.
    cpu.memory.store(0x0023, 0x00);
    cpu.memory.store(0x00f0, 0x0a);
    cpu.memory.store(0x0d23, 0x00);
    cpu.memory.store(0xf0f0, 0x0a);

    // For testing the V flag.
    cpu.memory.store(0x00dd, 0x40);
    cpu.memory.store(0x0099, 0x3f);
    cpu.memory.store(0xdddd, 0x40);
    cpu.memory.store(0x5599, 0x3f);

    // For testing the N flag.
    cpu.memory.store(0x006a, 0x80);
    cpu.memory.store(0x00ab, 0x3f);
    cpu.memory.store(0xaa6a, 0x80);
    cpu.memory.store(0x90ab, 0x3f);

    let expected_status_flags = [cpu::Z_FLAG,
                                 0x00,
                                 cpu::Z_FLAG,
                                 0x00,
                                 cpu::V_FLAG,
                                 0x00,
                                 cpu::V_FLAG,
                                 0x00,
                                 cpu::N_FLAG,
                                 0x00,
                                 cpu::N_FLAG,
                                 0x00];

    cpu.memory
        .store_bytes(0x0000,
                     &[// Test Z flag
                       BIT_Zero as u8,
                       0x23,
                       BIT_Zero as u8,
                       0xf0,
                       BIT_Abs as u8,
                       0x0d,
                       0x23,
                       BIT_Abs as u8,
                       0xf0,
                       0xf0,

                       // Test V flag
                       BIT_Zero as u8,
                       0xdd,
                       BIT_Zero as u8,
                       0x99,
                       BIT_Abs as u8,
                       0xdd,
                       0xdd,
                       BIT_Abs as u8,
                       0x55,
                       0x99,

                       // Test N flag
                       BIT_Zero as u8,
                       0x6a,
                       BIT_Zero as u8,
                       0xab,
                       BIT_Abs as u8,
                       0xaa,
                       0x6a,
                       BIT_Abs as u8,
                       0x90,
                       0xab]);

    // Check that the status flags are set properly.
    for flag in expected_status_flags.iter() {
        cpu.registers.p.0 = 0x00;

        // Execute and make sure flag was properly set.
        cpu.execute();
        assert!(cpu.registers.p.0 == *flag,
                "Bad flag: {:08b}",
                cpu.registers.p.0);
    }
}

#[test]
fn test_cmp() {
    assert!(false, "TODO: add tests for CMP instructions");
}

#[test]
fn test_flags() {
    let mut cpu = cpu::Cpu::new();

    cpu.memory
        .store_bytes(0x0000,
                     &[// Carry flag
                       SEC as u8,
                       CLC as u8,

                       // Interrupt flag
                       SEI as u8,
                       CLI as u8,

                       // Overflow flag
                       CLV as u8,

                       // Decimal flag
                       SED as u8,
                       CLD as u8]);

    // Carry flag
    cpu.execute();
    assert!(cpu.registers.p.c() == true, "Carry flag not set");
    cpu.execute();
    assert!(cpu.registers.p.c() == false, "Carry flag not cleared");

    // Interrupt flag
    cpu.execute();
    assert!(cpu.registers.p.i() == true, "Interrupt flag not set");
    cpu.execute();
    assert!(cpu.registers.p.i() == false, "Interrupt flag not cleared");

    // Overflow flag
    cpu.registers.p.set_v(true);
    cpu.execute();
    assert!(cpu.registers.p.v() == false, "Overflow flag not cleared");

    // Decimal flag
    cpu.execute();
    assert!(cpu.registers.p.d() == true, "Decimal flag not set");
    cpu.execute();
    assert!(cpu.registers.p.d() == false, "Decimal flag not cleared");
}

#[test]
fn test_dec() {
    let mut cpu = cpu::Cpu::new();

    // The value in memory before incrementing.
    let val = 18;

    // Set up CPU state for testing.
    cpu.registers.x = 0xfe;

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[// Increment at 0x00ab
                       DEC_Zero as u8,
                       0xab,

                       // Increment at 0x009f
                       DEC_Zero_X as u8,
                       0xa1,

                       // Increment at 0xffab
                       DEC_Abs as u8,
                       0xff,
                       0xab,

                       // Increment at 0x00a9
                       DEC_Abs_X as u8,
                       0xff,
                       0xab]);

    // Check value incremented at these addresses.
    let addresses = [0x00ab, 0x009f, 0xffab, 0x00a9];

    // Check that the results were loaded properly.
    for addr in addresses.iter() {
        cpu.memory.store(*addr as u16, val);

        // Execute and make sure value was incremented.
        cpu.execute();
        assert!(val - 1 == cpu.memory.fetch(*addr as u16),
                "Bad value loaded from addr {:#06x}",
                addr);
    }
}

#[test]
fn test_inc() {
    let mut cpu = cpu::Cpu::new();

    // The value in memory before incrementing.
    let val = 18;

    // Set up CPU state for testing.
    cpu.registers.x = 0xfe;

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[// Increment at 0x00ab
                       INC_Zero as u8,
                       0xab,

                       // Increment at 0x009f
                       INC_Zero_X as u8,
                       0xa1,

                       // Increment at 0xffab
                       INC_Abs as u8,
                       0xff,
                       0xab,

                       // Increment at 0x00a9
                       INC_Abs_X as u8,
                       0xff,
                       0xab]);

    // Check value incremented at these addresses.
    let addresses = [0x00ab, 0x009f, 0xffab, 0x00a9];

    // Check that the results were loaded properly.
    for addr in addresses.iter() {
        cpu.memory.store(*addr as u16, val);

        // Execute and make sure value was incremented.
        cpu.execute();
        assert!(val + 1 == cpu.memory.fetch(*addr as u16),
                "Bad value loaded from addr {:#06x}",
                addr);
    }
}

#[test]
fn test_lda() {
    let mut cpu = cpu::Cpu::new();

    // The value to load from various memory addresses.
    let val = 18;

    // Set up CPU state for testing.
    cpu.registers.x = 0xff;
    cpu.registers.y = 0xfe;

    // For executing the indirect instruction lookups.
    cpu.memory.store_u16(0x00d0, 0xffd0);
    cpu.memory.store_u16(0x00e0, 0xffe0);

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[// Load from instruction.
                       LDA_Imm as u8,
                       val,

                       // Load from  0x00ab
                       LDA_Zero as u8,
                       0xab,

                       // Load from 0x00a0
                       LDA_Zero_X as u8,
                       0xa1,

                       // Load from 0xffab
                       LDA_Abs as u8,
                       0xff,
                       0xab,

                       // Load from 0x00ca
                       LDA_Abs_X as u8,
                       0xff,
                       0xcb,

                       // Load from 0x00a9
                       LDA_Abs_Y as u8,
                       0xff,
                       0xab,

                       // Load from 0xffd0
                       LDA_Ind_X as u8,
                       0xd1,

                       // Load from 0x00de
                       LDA_Ind_Y as u8,
                       0xe0]);

    // Test once for immediate load.
    cpu.execute();
    assert!(val == cpu.registers.a, "Bad value from immediate load.");

    // Check value loaded from addresses.
    let addresses = [0x00ab, 0x00a0, 0xffab, 0x00ca, 0x00a9, 0xffd0, 0x00de];

    // Check that the results were loaded properly.
    for addr in addresses.iter() {
        // Reset accumulator, and store value in memory for test.
        cpu.registers.a = 0x00;
        cpu.memory.store(*addr as u16, val);

        // Execute and make sure accumulator was populated.
        cpu.execute();
        assert!(val == cpu.registers.a,
                "Bad value loaded from addr {:#06x}",
                addr);
    }
}

#[test]
fn test_ldx() {
    let mut cpu = cpu::Cpu::new();

    // The value to load from various memory addresses.
    let val = 18;

    // Set up CPU state for testing.
    cpu.registers.y = 0xfe;

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[// Load from instruction.
                       LDX_Imm as u8,
                       val,

                       // Load from  0x00ab
                       LDX_Zero as u8,
                       0xab,

                       // Load from 0x00b7
                       LDX_Zero_Y as u8,
                       0xb9,

                       // Load from 0xffab
                       LDX_Abs as u8,
                       0xff,
                       0xab,

                       // Load from 0x00cb
                       LDX_Abs_Y as u8,
                       0xff,
                       0xcd]);

    // Test once for immediate load.
    cpu.execute();
    assert!(val == cpu.registers.x, "Bad value from immediate load.");

    // Check value loaded from addresses.
    let addresses = [0x00ab, 0x00b7, 0xffab, 0x00cb];

    // Check that the results were loaded properly.
    for addr in addresses.iter() {
        // Reset accumulator, and store value in memory for test.
        cpu.registers.x = 0x00;
        cpu.memory.store(*addr as u16, val);

        // Execute and make sure accumulator was populated.
        cpu.execute();
        assert!(val == cpu.registers.x,
                "Bad value loaded from addr {:#06x}",
                addr);
    }
}

#[test]
fn test_ldy() {
    let mut cpu = cpu::Cpu::new();

    // The value to load from various memory addresses.
    let val = 18;

    // Set up CPU state for testing.
    cpu.registers.x = 0xfe;

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[// Load from instruction.
                       LDY_Imm as u8,
                       val,

                       // Load from  0x00ab
                       LDY_Zero as u8,
                       0xab,

                       // Load from 0x009f
                       LDY_Zero_X as u8,
                       0xa1,

                       // Load from 0xffab
                       LDY_Abs as u8,
                       0xff,
                       0xab,

                       // Load from 0x00a9
                       LDY_Abs_X as u8,
                       0xff,
                       0xab]);

    // Test once for immediate load.
    cpu.execute();
    assert!(val == cpu.registers.y, "Bad value from immediate load.");

    // Check value loaded from addresses.
    let addresses = [0x00ab, 0x009f, 0xffab, 0x00a9];

    // Check that the results were loaded properly.
    for addr in addresses.iter() {
        // Reset accumulator, and store value in memory for test.
        cpu.registers.y = 0x00;
        cpu.memory.store(*addr as u16, val);

        // Execute and make sure accumulator was populated.
        cpu.execute();
        assert!(val == cpu.registers.y,
                "Bad value loaded from addr {:#06x}",
                addr);
    }
}

#[test]
fn test_registers() {
    let mut cpu = cpu::Cpu::new();

    let val = 18;

    cpu.memory
        .store_bytes(0x0000,
                     &[// X register
                       TAX as u8,
                       TXA as u8,
                       DEX as u8,
                       INX as u8,

                       // Y register
                       TAY as u8,
                       TYA as u8,
                       DEY as u8,
                       INY as u8]);

    // TAX
    cpu.registers.a = val;
    cpu.registers.x = 0x00;
    cpu.execute();
    assert!(cpu.registers.x == val, "Bad TAX");

    // TXA
    cpu.registers.a = 0x00;
    cpu.registers.x = val;
    cpu.execute();
    assert!(cpu.registers.a == val, "Bad TXA");

    // DEX
    cpu.registers.x = val;
    cpu.execute();
    assert!(cpu.registers.x == val - 1, "Bad DEX");

    // INX
    cpu.registers.x = val;
    cpu.execute();
    assert!(cpu.registers.x == val + 1, "Bad INX");

    // TAY
    cpu.registers.a = val;
    cpu.registers.y = 0x00;
    cpu.execute();
    assert!(cpu.registers.y == val, "Bad TAY");

    // TYA
    cpu.registers.a = 0x00;
    cpu.registers.y = val;
    cpu.execute();
    assert!(cpu.registers.a == val, "Bad TYA");

    // DEY
    cpu.registers.y = val;
    cpu.execute();
    assert!(cpu.registers.y == val - 1, "Bad DEY");

    // INY
    cpu.registers.y = val;
    cpu.execute();
    assert!(cpu.registers.y == val + 1, "Bad INY");

}

#[test]
fn test_sta() {
    let mut cpu = cpu::Cpu::new();

    // The value to pass around and test.
    let val = 18;

    // Set up CPU state for testing.
    cpu.registers.x = 0xff;
    cpu.registers.y = 0xfe;
    cpu.registers.a = val;

    // For executing the indirect instruction lookups.
    cpu.memory.store_u16(0x00d0, 0xffd0);
    cpu.memory.store_u16(0x00e0, 0xffe0);

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[// Store to 0x00ab
                       STA_Zero as u8,
                       0xab,

                       // Store to 0x00ac
                       STA_Zero_X as u8,
                       0xad,

                       // Store to 0xffab
                       STA_Abs as u8,
                       0xff,
                       0xab,

                       // Store to 0x00aa
                       STA_Abs_X as u8,
                       0xff,
                       0xab,

                       // Store to 0x00a9
                       STA_Abs_Y as u8,
                       0xff,
                       0xab,

                       // Store to 0xffd0
                       STA_Ind_X as u8,
                       0xd1,

                       // Store to 0x00de
                       STA_Ind_Y as u8,
                       0xe0]);

    // Check value in addresses.
    let addresses = [0x00ab, 0x00ac, 0xffab, 0x00aa, 0x00a9, 0xffd0, 0x00de];

    // Check the results in memory.
    for addr in addresses.iter() {
        cpu.execute();
        assert!(val == cpu.memory.fetch(*addr),
                "Bad value at addr {:#06x}",
                addr);
    }
}

#[test]
fn test_stx() {
    let mut cpu = cpu::Cpu::new();

    // The value to pass around and test.
    let val = 18;

    // Set up CPU state for testing.
    cpu.registers.x = val;
    cpu.registers.y = 0xfe;

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[// Store to 0x00ab
                       STX_Zero as u8,
                       0xab,

                       // Store to 0x00ac
                       STX_Zero_Y as u8,
                       0xae,

                       // Store to 0xffab
                       STX_Abs as u8,
                       0xff,
                       0xab]);

    // Check value in addresses.
    let addresses = [0x00ab, 0x00ac, 0xffab];

    // Check the results in memory.
    for addr in addresses.iter() {
        cpu.execute();
        assert!(val == cpu.memory.fetch(*addr),
                "Bad value at addr {:#06x}",
                addr);
    }
}

#[test]
fn test_sty() {
    let mut cpu = cpu::Cpu::new();

    // The value to pass around and test.
    let val = 18;

    // Set up CPU state for testing.
    cpu.registers.x = 0xfe;
    cpu.registers.y = val;

    // Store program in memory.
    cpu.memory
        .store_bytes(0x0000,
                     &[// Store to 0x00ab
                       STY_Zero as u8,
                       0xab,

                       // Store to 0x00ac
                       STY_Zero_X as u8,
                       0xae,

                       // Store to 0xffab
                       STY_Abs as u8,
                       0xff,
                       0xab]);

    // Check value in addresses.
    let addresses = [0x00ab, 0x00ac, 0xffab];

    // Check the results in memory.
    for addr in addresses.iter() {
        cpu.execute();
        assert!(val == cpu.memory.fetch(*addr),
                "Bad value at addr {:#06x}",
                addr);
    }
}
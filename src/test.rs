use cpu;
use cpu::{C_FLAG, Z_FLAG, I_FLAG, D_FLAG, B_FLAG, U_FLAG, V_FLAG, N_FLAG};
use opcode::Opcode::*;

#[test]
fn test_adc() {
    let mut cpu = cpu::Cpu::new();

    // First entry is value to be added.
    // Second entry is value in accumulator before the add.
    // Third entry is carry bit.
    // Fourth entry is expected sum.
    // Fifth value is expected status register value.
    let adc_results = [(0x00, 0x00, false, 0x00, 0b00000010),
                       (0x01, 0x01, true, 0x03, 0b00000000),
                       (0x01, 0x7f, false, 0x80, 0b11000000),
                       (0xff, 0x80, false, 0x7F, 0b01000001)];

    for adc_result in adc_results.iter() {
        // Reset from the last round.
        cpu.reset();

        // Store value in memory locations for testing.
        let adc_addresses = [0x003a, 0x004a, 0x123a, 0x234a, 0x345a, 0xfada, 0xbeea];
        for addr in adc_addresses.iter() {
            cpu.memory.store(*addr, adc_result.0);
        }

        // Store extra memory values for indirect lookups.
        cpu.memory.store_u16(0x005a, 0xfada);
        cpu.memory.store_u16(0x006a, 0xbdec);

        // Store x and y registers.
        cpu.registers.x = 0xff;
        cpu.registers.y = 0xfe;

        cpu.memory
            .store_bytes(0x0000,
                         &[// Add immediate instruction arg.
                           ADC_Imm as u8,
                           adc_result.0,

                           // Add with 0x003a
                           ADC_Zero as u8,
                           0x3a,

                           // Add with 0x004a
                           ADC_Zero_X as u8,
                           0x4b,

                           // Add with 0x123a
                           ADC_Abs as u8,
                           0x3a,
                           0x12,

                           // Add with 0x234a
                           ADC_Abs_X as u8,
                           0x4b,
                           0x22,

                           // Add with 0x345a
                           ADC_Abs_Y as u8,
                           0x5c,
                           0x33,

                           // Add with 0xfada (thru 0x005a)
                           ADC_Ind_X as u8,
                           0x5b,

                           // Add with 0xbeea (thru 0x006a)
                           ADC_Ind_Y as u8,
                           0x6a]);

        for _ in 0..adc_addresses.len() + 1 {
            cpu.registers.a = adc_result.1;
            cpu.registers.p.0 = 0x00;
            cpu.registers.p.set_c(adc_result.2);
            cpu.execute();

            assert!(cpu.registers.a == adc_result.3,
                    "Bad result {:#04x} in A after ADC",
                    cpu.registers.a);
            assert!(cpu.registers.p.0 == adc_result.4,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
        }
    }
}

#[test]
fn test_and() {
    let mut cpu = cpu::Cpu::new();

    // First entry is value to be anded.
    // Second entry is value in accumulator before the and.
    // Third entry is expected result.
    // Fourth value is expected status register value.
    let and_results = [(0x00, 0x00, 0x00, 0b00000010),
                       (0xff, 0x80, 0x80, 0b10000000),
                       (0xc0, 0xfd, 0xc0, 0b10000000),
                       (0xa5, 0x5a, 0x00, 0b00000010)];

    for and_result in and_results.iter() {
        // Reset from the last round.
        cpu.reset();

        // Store value in memory locations for testing.
        let and_addresses = [0x003a, 0x004a, 0x123a, 0x234a, 0x345a, 0xfada, 0xbeea];
        for addr in and_addresses.iter() {
            cpu.memory.store(*addr, and_result.0);
        }

        // Store extra memory values for indirect lookups.
        cpu.memory.store_u16(0x005a, 0xfada);
        cpu.memory.store_u16(0x006a, 0xbdec);

        // Store x and y registers.
        cpu.registers.x = 0xff;
        cpu.registers.y = 0xfe;

        cpu.memory
            .store_bytes(0x0000,
                         &[// And immediate instruction arg.
                           AND_Imm as u8,
                           and_result.0,

                           // And with 0x003a
                           AND_Zero as u8,
                           0x3a,

                           // And with 0x004a
                           AND_Zero_X as u8,
                           0x4b,

                           // And with 0x123a
                           AND_Abs as u8,
                           0x3a,
                           0x12,

                           // And with 0x234a
                           AND_Abs_X as u8,
                           0x4b,
                           0x22,

                           // And with 0x345a
                           AND_Abs_Y as u8,
                           0x5c,
                           0x33,

                           // And with 0xfada (thru 0x005a)
                           AND_Ind_X as u8,
                           0x5b,

                           // And with 0xbeea (thru 0x006a)
                           AND_Ind_Y as u8,
                           0x6a]);

        for _ in 0..and_addresses.len() + 1 {
            cpu.registers.a = and_result.1;
            cpu.registers.p.0 = 0x00;
            cpu.execute();

            assert!(cpu.registers.a == and_result.2,
                    "Bad result {:#04x} in A after AND",
                    cpu.registers.a);
            assert!(cpu.registers.p.0 == and_result.3,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
        }
    }
}

#[test]
fn test_asl() {
    let mut cpu = cpu::Cpu::new();

    // First entry in tuple is value that should be shifted.
    // Second entry is expected processor status flags after that shift.
    let asl_results = [(0x80, I_FLAG | C_FLAG | Z_FLAG),
                       (0xc0, I_FLAG | C_FLAG | N_FLAG),
                       (0x4f, I_FLAG | N_FLAG)];

    for asl_result in asl_results.iter() {
        // Reset from last round.
        cpu.reset();

        // Test accumulator shift.
        cpu.registers.a = asl_result.0;
        cpu.memory.store(0x0000, ASL_Acc as u8);
        cpu.execute();
        assert!(cpu.registers.p.0 == asl_result.1,
                "Bad flags after ASL on accumulator: {:08b}",
                cpu.registers.p.0);
        assert!(cpu.registers.a == (asl_result.0 << 1),
                "Bad shift result {:#04x} in accumulator",
                cpu.registers.a);

        // Test asl on memory.
        cpu.reset();

        // Store the value in several memory locations for lookup during ASL
        // instructions.
        let asl_addresses = [0x002a, 0x003a, 0x123a, 0x234a];
        for addr in asl_addresses.iter() {
            cpu.memory.store(*addr as u16, asl_result.0);
        }

        // X register for zero_x and abs_x.
        cpu.registers.x = 0xff;

        cpu.memory
            .store_bytes(0x0000,
                         &[// Shift location 0x002a
                           ASL_Zero as u8,
                           0x2a,

                           // Shift location 0x003a
                           ASL_Zero_X as u8,
                           0x3b,

                           // Shift location 0x123a
                           ASL_Abs as u8,
                           0x3a,
                           0x12,

                           // Shift location 0x234a
                           ASL_Abs_X as u8,
                           0x4b,
                           0x22]);

        // Test that processor status is set correctly after each instruction.
        for addr in asl_addresses.iter() {
            cpu.registers.p = cpu::Status::new();
            cpu.execute();
            assert!(cpu.registers.p.0 == asl_result.1,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
            let shift_result = cpu.memory.fetch(*addr);
            assert!(shift_result == (asl_result.0 << 1),
                    "Bad shift result {:#04x} in memory {:#06x}",
                    shift_result,
                    addr);
        }
    }
}

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

    let expected_status_flags = [// Expected flags.
                                 I_FLAG | Z_FLAG,
                                 I_FLAG,
                                 I_FLAG | Z_FLAG,
                                 I_FLAG,
                                 I_FLAG | V_FLAG,
                                 I_FLAG,
                                 I_FLAG | V_FLAG,
                                 I_FLAG,
                                 I_FLAG | N_FLAG,
                                 I_FLAG,
                                 I_FLAG | N_FLAG,
                                 I_FLAG];

    cpu.memory
        .store_bytes(0x0000,
                     &[// Test Z flag
                       BIT_Zero as u8, // 0x0023
                       0x23,
                       BIT_Zero as u8, // 0x00f0
                       0xf0,
                       BIT_Abs as u8, // 0x0d23
                       0x23,
                       0x0d,
                       BIT_Abs as u8, // 0xf0f0
                       0xf0,
                       0xf0,

                       // Test V flag
                       BIT_Zero as u8, // 0x00dd
                       0xdd,
                       BIT_Zero as u8, // 0x0099
                       0x99,
                       BIT_Abs as u8, // 0xdddd
                       0xdd,
                       0xdd,
                       BIT_Abs as u8, // 0x5599
                       0x99,
                       0x55,

                       // Test N flag
                       BIT_Zero as u8, // 0x006a
                       0x6a,
                       BIT_Zero as u8, // 0x00ab
                       0xab,
                       BIT_Abs as u8, // 0xaa6a
                       0x6a,
                       0xaa,
                       BIT_Abs as u8, // 0x90ab+
                       0xab,
                       0x90]);

    // Check that the status flags are set properly.
    for flag in expected_status_flags.iter() {
        cpu.registers.p = cpu::Status::new();

        // Execute and make sure flag was properly set.
        cpu.execute();
        assert!(cpu.registers.p.0 == *flag,
                "Bad flag: {:08b}",
                cpu.registers.p.0);
    }
}

#[test]
fn test_cmp() {
    let mut cpu = cpu::Cpu::new();

    let accumulator_value = 15;
    // First entry in tuple is value that should be used for comparison.
    // Second entry is expected processor status flags after that comparison.
    let cmp_results = [(5, I_FLAG | C_FLAG),
                       (15, I_FLAG | C_FLAG | Z_FLAG),
                       (100, I_FLAG | N_FLAG)];

    for cmp_result in cmp_results.iter() {
        // Reset from previous tests.
        cpu.reset();

        // Set accumulator for comparisons.
        cpu.registers.a = accumulator_value;

        // Set index registers for zero, abs, and ind instructions.
        cpu.registers.x = 0xff;
        cpu.registers.y = 0xfe;

        // Store the value in several memory locations for lookup during CMP instructions.
        for addr in [0x002a, 0x003a, 0x004a, 0x005a, 0x123a, 0x223a, 0x423a].iter() {
            cpu.memory.store(*addr as u16, cmp_result.0);
        }

        // For indirect instructions.
        cpu.memory.store_u16(0x00ff, 0x004a);
        cpu.memory.store_u16(0x00af, 0xff5c);

        cpu.memory
            .store_bytes(0x0000,
                         &[// Compare with value 0x00
                           CMP_Imm as u8,
                           cmp_result.0,

                           // Compare with 0x002a
                           CMP_Zero as u8,
                           0x2a,

                           // Compare with 0x003a
                           CMP_Zero_X as u8,
                           0x3b,

                           // Compare with 0x123a
                           CMP_Abs as u8,
                           0x3a,
                           0x12,

                           // Compare with 0x223a
                           CMP_Abs_X as u8,
                           0x3b,
                           0x21,

                           // Compare with 0x423a
                           CMP_Abs_Y as u8,
                           0x3c,
                           0x41,

                           // Compare with 0x004a (thru 0x00ff)
                           CMP_Ind_X as u8,
                           0x00,

                           // Compare with 0x005a (thru 0x00af)
                           CMP_Ind_Y as u8,
                           0xaf]);

        // Test that processor status is set correctly after each instruction.
        for _ in 0..8 {
            cpu.registers.p = cpu::Status::new();
            cpu.execute();
            assert!(cpu.registers.p.0 == cmp_result.1,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
        }
    }
}

#[test]
fn test_cpx() {
    let mut cpu = cpu::Cpu::new();

    let x_value = 15;
    // First entry in tuple is value that should be used for comparison.
    // Second entry is expected processor status flags after that comparison.
    let cpx_results = [(5, I_FLAG | C_FLAG),
                       (15, I_FLAG | Z_FLAG | C_FLAG),
                       (100, I_FLAG | N_FLAG)];

    for cpx_result in cpx_results.iter() {
        // Reset from previous tests.
        cpu.reset();

        // Set X register for comparisons.
        cpu.registers.x = x_value;

        // Store the value in several memory locations for lookup during CMP instructions.
        for addr in [0x002a, 0x423a].iter() {
            cpu.memory.store(*addr as u16, cpx_result.0);
        }

        cpu.memory
            .store_bytes(0x0000,
                         &[// Compare with value
                           CPX_Imm as u8,
                           cpx_result.0,

                           // Compare with 0x002a
                           CPX_Zero as u8,
                           0x2a,

                           // Compare with 0x423a
                           CPX_Abs as u8,
                           0x3a,
                           0x42]);

        // Test that processor status is set correctly after each instruction.
        for _ in 0..3 {
            cpu.registers.p = cpu::Status::new();
            cpu.execute();
            assert!(cpu.registers.p.0 == cpx_result.1,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
        }
    }
}

#[test]
fn test_cpy() {
    let mut cpu = cpu::Cpu::new();

    let y_value = 15;
    // First entry in tuple is value that should be used for comparison.
    // Second entry is expected processor status flags after that comparison.
    let cpy_results = [(5, I_FLAG | C_FLAG),
                       (15, I_FLAG | Z_FLAG | C_FLAG),
                       (100, I_FLAG | N_FLAG)];

    for cpy_result in cpy_results.iter() {
        // Reset from previous tests.
        cpu.reset();

        // Set Y register for comparisons.
        cpu.registers.y = y_value;

        // Store the value in several memory locations for lookup during CMP instructions.
        for addr in [0x002a, 0x423a].iter() {
            cpu.memory.store(*addr as u16, cpy_result.0);
        }

        cpu.memory
            .store_bytes(0x0000,
                         &[// Compare with value
                           CPY_Imm as u8,
                           cpy_result.0,

                           // Compare with 0x002a
                           CPY_Zero as u8,
                           0x2a,

                           // Compare with 0x423a
                           CPY_Abs as u8,
                           0x3a,
                           0x42]);

        // Test that processor status is set correctly after each instruction.
        for _ in 0..3 {
            cpu.registers.p = cpu::Status::new();
            cpu.execute();
            assert!(cpu.registers.p.0 == cpy_result.1,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
        }
    }
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
                       0xab,
                       0xff,

                       // Increment at 0x00a9
                       DEC_Abs_X as u8,
                       0xab,
                       0xff]);

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
fn test_eor() {
    let mut cpu = cpu::Cpu::new();

    // First entry is value to be xor.
    // Second entry is value in accumulator before the xor.
    // Third entry is expected result.
    // Fourth value is expected status register value.
    let eor_results = [(0x00, 0x00, 0x00, I_FLAG | Z_FLAG),
                       (0xff, 0x80, 0x7f, I_FLAG),
                       (0xc0, 0xfd, 0x3d, I_FLAG),
                       (0xa5, 0x5a, 0xff, I_FLAG | N_FLAG),
                       (0x33, 0x33, 0x00, I_FLAG | Z_FLAG)];

    for eor_result in eor_results.iter() {
        // Reset from the last round.
        cpu.reset();

        // Store value in memory locations for testing.
        let eor_addresses = [0x003a, 0x004a, 0x123a, 0x234a, 0x345a, 0xfada, 0xbeea];
        for addr in eor_addresses.iter() {
            cpu.memory.store(*addr, eor_result.0);
        }

        // Store extra memory values for indirect lookups.
        cpu.memory.store_u16(0x005a, 0xfada);
        cpu.memory.store_u16(0x006a, 0xbdec);

        // Store x and y registers.
        cpu.registers.x = 0xff;
        cpu.registers.y = 0xfe;

        cpu.memory
            .store_bytes(0x0000,
                         &[// XOR immediate instruction arg.
                           EOR_Imm as u8,
                           eor_result.0,

                           // XOR with 0x003a
                           EOR_Zero as u8,
                           0x3a,

                           // XOR with 0x004a
                           EOR_Zero_X as u8,
                           0x4b,

                           // XOR with 0x123a
                           EOR_Abs as u8,
                           0x3a,
                           0x12,

                           // XOR with 0x234a
                           EOR_Abs_X as u8,
                           0x4b,
                           0x22,

                           // XOR with 0x345a
                           EOR_Abs_Y as u8,
                           0x5c,
                           0x33,

                           // XOR with 0xfada (thru 0x005a)
                           EOR_Ind_X as u8,
                           0x5b,

                           // XOR with 0xbeea (thru 0x006a)
                           EOR_Ind_Y as u8,
                           0x6a]);

        for _ in 0..eor_addresses.len() + 1 {
            cpu.registers.a = eor_result.1;
            cpu.registers.p = cpu::Status::new();
            cpu.execute();

            assert!(cpu.registers.a == eor_result.2,
                    "Bad result {:#04x} in A after EOR",
                    cpu.registers.a);
            assert!(cpu.registers.p.0 == eor_result.3,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
        }
    }
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
                       0xab,
                       0xff,

                       // Increment at 0x00a9
                       INC_Abs_X as u8,
                       0xab,
                       0xff]);

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
fn test_jmp() {
    let mut cpu = cpu::Cpu::new();

    // Store indirect ones.
    cpu.memory.store_u16(0x4545, 0x2fff);
    cpu.memory.store(0x3fff, 0xdc);
    cpu.memory.store(0x3f00, 0xfe);

    // Jump to 0x1234.
    cpu.memory
        .store_bytes(0x0000, &[JMP_Abs as u8, 0x34, 0x12]);

    // Jump to 0x2fff (thru 0x4545).
    cpu.memory
        .store_bytes(0x1234, &[JMP_Ind as u8, 0x45, 0x45]);

    // Jump to 0x00fa.
    cpu.memory
        .store_bytes(0x2fff, &[JMP_Abs as u8, 0xfa, 0x00]);

    // Jump to 0xfedc (thru 0x3fff).
    cpu.memory
        .store_bytes(0x00fa, &[JMP_Ind as u8, 0xff, 0x3f]);

    for addr in [0x1234, 0x2fff, 0x00fa, 0xfedc].iter() {
        cpu.execute();
        assert!(cpu.registers.pc == *addr,
                "Jumped to wrong address. At {:#06x}, should be at {:#06x}",
                cpu.registers.pc,
                *addr);
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
                       0xab,
                       0xff,

                       // Load from 0x00ca
                       LDA_Abs_X as u8,
                       0xcb,
                       0xff,

                       // Load from 0x00a9
                       LDA_Abs_Y as u8,
                       0xab,
                       0xff,

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
                       0xab,
                       0xff,

                       // Load from 0x00cb
                       LDX_Abs_Y as u8,
                       0xcd,
                       0xff]);

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
                       0xab,
                       0xff,

                       // Load from 0x00a9
                       LDY_Abs_X as u8,
                       0xab,
                       0xff]);

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
fn test_lsr() {
    let mut cpu = cpu::Cpu::new();

    // First entry in tuple is value that should be shifted.
    // Second entry is expected processor status flags after that shift.
    let lsr_results = [(0x01, I_FLAG | Z_FLAG | C_FLAG),
                       (0x03, I_FLAG | C_FLAG),
                       (0xf2, I_FLAG)];

    for lsr_result in lsr_results.iter() {
        // Reset from last round.
        cpu.reset();

        // Test accumulator shift.
        cpu.registers.a = lsr_result.0;
        cpu.memory.store(0x0000, LSR_Acc as u8);
        cpu.execute();
        assert!(cpu.registers.p.0 == lsr_result.1,
                "Bad flags after LSR on accumulator: {:08b}",
                cpu.registers.p.0);
        assert!(cpu.registers.a == (lsr_result.0 >> 1),
                "Bad shift result {:#04x} in accumulator",
                cpu.registers.a);

        // Test asl on memory.
        cpu.reset();

        // Store the value in several memory locations for lookup during LSR instructions.
        let lsr_addresses = [0x002a, 0x003a, 0x123a, 0x234a];
        for addr in lsr_addresses.iter() {
            cpu.memory.store(*addr as u16, lsr_result.0);
        }

        // X register for zero_x and abs_x.
        cpu.registers.x = 0xff;

        cpu.memory
            .store_bytes(0x0000,
                         &[// Shift location 0x002a
                           LSR_Zero as u8,
                           0x2a,

                           // Shift location 0x003a
                           LSR_Zero_X as u8,
                           0x3b,

                           // Shift location 0x123a
                           LSR_Abs as u8,
                           0x3a,
                           0x12,

                           // Shift location 0x234a
                           LSR_Abs_X as u8,
                           0x4b,
                           0x22]);

        // Test that processor status is set correctly after each instruction.
        for addr in lsr_addresses.iter() {
            cpu.registers.p = cpu::Status::new();
            cpu.execute();
            assert!(cpu.registers.p.0 == lsr_result.1,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
            let shift_result = cpu.memory.fetch(*addr);
            assert!(shift_result == (lsr_result.0 >> 1),
                    "Bad shift result {:#04x} in memory {:#06x}",
                    shift_result,
                    addr);
        }
    }
}

#[test]
fn test_ora() {
    let mut cpu = cpu::Cpu::new();

    // First entry is value to be or.
    // Second entry is value in accumulator before the or.
    // Third entry is expected result.
    // Fourth value is expected status register value.
    let ora_results = [(0x00, 0x00, 0x00, I_FLAG | Z_FLAG),
                       (0xff, 0x80, 0xff, I_FLAG | N_FLAG),
                       (0xc0, 0xfd, 0xfd, I_FLAG | N_FLAG),
                       (0xa5, 0x5a, 0xff, I_FLAG | N_FLAG)];

    for ora_result in ora_results.iter() {
        // Reset from the last round.
        cpu.reset();

        // Store value in memory locations for testing.
        let ora_addresses = [0x003a, 0x004a, 0x123a, 0x234a, 0x345a, 0xfada, 0xbeea];
        for addr in ora_addresses.iter() {
            cpu.memory.store(*addr, ora_result.0);
        }

        // Store extra memory values for indirect lookups.
        cpu.memory.store_u16(0x005a, 0xfada);
        cpu.memory.store_u16(0x006a, 0xbdec);

        // Store x and y registers.
        cpu.registers.x = 0xff;
        cpu.registers.y = 0xfe;

        cpu.memory
            .store_bytes(0x0000,
                         &[// XOR immediate instruction arg.
                           ORA_Imm as u8,
                           ora_result.0,

                           // XOR with 0x003a
                           ORA_Zero as u8,
                           0x3a,

                           // XOR with 0x004a
                           ORA_Zero_X as u8,
                           0x4b,

                           // XOR with 0x123a
                           ORA_Abs as u8,
                           0x3a,
                           0x12,

                           // XOR with 0x234a
                           ORA_Abs_X as u8,
                           0x4b,
                           0x22,

                           // XOR with 0x345a
                           ORA_Abs_Y as u8,
                           0x5c,
                           0x33,
                           // XOR with 0xfada (thru 0x005a)
                           ORA_Ind_X as u8,
                           0x5b,

                           // XOR with 0xbeea (thru 0x006a)
                           ORA_Ind_Y as u8,
                           0x6a]);

        for _ in 0..ora_addresses.len() + 1 {
            cpu.registers.a = ora_result.1;
            cpu.registers.p = cpu::Status::new();
            cpu.execute();

            assert!(cpu.registers.a == ora_result.2,
                    "Bad result {:#04x} in A after ORA",
                    cpu.registers.a);
            assert!(cpu.registers.p.0 == ora_result.3,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
        }
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
fn test_rol() {
    let mut cpu = cpu::Cpu::new();

    // First entry in tuple is value that should be rotated.
    // Second entry is initial carry flag.
    // Third enty is expected value after rotating.
    // Fourth entry is expected processor status flags after rotating.
    let rol_results = [(0x80, true, 0x01, I_FLAG | C_FLAG),
                       (0x00, false, 0x00, I_FLAG | Z_FLAG),
                       (0xff, false, 0xfe, I_FLAG | N_FLAG | C_FLAG)];

    for rol_result in rol_results.iter() {
        // Reset from last round.
        cpu.reset();

        // Test accumulator rotate.
        cpu.registers.a = rol_result.0;
        cpu.registers.p.set_c(rol_result.1);
        cpu.memory.store(0x0000, ROL_Acc as u8);
        cpu.execute();
        assert!(cpu.registers.p.0 == rol_result.3,
                "Bad flags after ROL on accumulator: {:08b}",
                cpu.registers.p.0);
        assert!(cpu.registers.a == (rol_result.2),
                "Bad rotate result {:#04x} in accumulator",
                cpu.registers.a);

        // Test asl on memory.
        cpu.reset();

        // Store the value in several memory locations for lookup during ROL instructions.
        let rol_addresses = [0x002a, 0x003a, 0x123a, 0x234a];
        for addr in rol_addresses.iter() {
            cpu.memory.store(*addr as u16, rol_result.0);
        }

        // X register for zero_x and abs_x.
        cpu.registers.x = 0xff;

        cpu.memory
            .store_bytes(0x0000,
                         &[// Rotate location 0x002a
                           ROL_Zero as u8,
                           0x2a,

                           // Rotate location 0x003a
                           ROL_Zero_X as u8,
                           0x3b,

                           // Rotate location 0x123a
                           ROL_Abs as u8,
                           0x3a,
                           0x12,

                           // Rotate location 0x234a
                           ROL_Abs_X as u8,
                           0x4b,
                           0x22]);

        // Test that processor status is set correctly after each instruction.
        for addr in rol_addresses.iter() {
            // Carry flag for rotating in to value.
            cpu.registers.p = cpu::Status::new();
            cpu.registers.p.set_c(rol_result.1);
            cpu.execute();
            assert!(cpu.registers.p.0 == rol_result.3,
                    "Bad flags after ROL: {:08b} in address {:#06x}",
                    cpu.registers.p.0,
                    *addr);
            let value = cpu.memory.fetch(*addr);
            assert!(value == (rol_result.2),
                    "Bad rotate result {:#04x} in address {:#06x}",
                    value,
                    *addr);
        }
    }
}

#[test]
fn test_ror() {
    let mut cpu = cpu::Cpu::new();

    // First entry in tuple is value that should be rotated.
    // Second entry is initial carry flag.
    // Third enty is expected value after rotating.
    // Fourth entry is expected processor status flags after rotating.
    let ror_results = [(0x01, true, 0x80, I_FLAG | N_FLAG | C_FLAG),
                       (0x00, false, 0x00, I_FLAG | Z_FLAG),
                       (0xff, false, 0x7f, I_FLAG | C_FLAG)];

    for ror_result in ror_results.iter() {
        // Reset from last round.
        cpu.reset();

        // Test accumulator rotate.
        cpu.registers.a = ror_result.0;
        cpu.registers.p.set_c(ror_result.1);
        cpu.memory.store(0x0000, ROR_Acc as u8);
        cpu.execute();
        assert!(cpu.registers.p.0 == ror_result.3,
                "Bad flags after ROR on accumulator: {:08b}",
                cpu.registers.p.0);
        assert!(cpu.registers.a == (ror_result.2),
                "Bad rotate result {:#04x} in accumulator",
                cpu.registers.a);

        // Test asl on memory.
        cpu.reset();

        // Store the value in several memory locations for lookup during ROR instructions.
        let ror_addresses = [0x002a, 0x003a, 0x123a, 0x234a];
        for addr in ror_addresses.iter() {
            cpu.memory.store(*addr as u16, ror_result.0);
        }

        // X register for zero_x and abs_x.
        cpu.registers.x = 0xff;

        cpu.memory
            .store_bytes(0x0000,
                         &[// Rotate location 0x002a
                           ROR_Zero as u8,
                           0x2a,

                           // Rotate location 0x003a
                           ROR_Zero_X as u8,
                           0x3b,

                           // Rotate location 0x123a
                           ROR_Abs as u8,
                           0x3a,
                           0x12,

                           // Rotate location 0x234a
                           ROR_Abs_X as u8,
                           0x4b,
                           0x22]);

        // Test that processor status is set correctly after each instruction.
        for addr in ror_addresses.iter() {
            // Carry flag for rotating in to value.
            cpu.registers.p = cpu::Status::new();
            cpu.registers.p.set_c(ror_result.1);
            cpu.execute();
            assert!(cpu.registers.p.0 == ror_result.3,
                    "Bad flags after ROR: {:08b} in address {:#06x}",
                    cpu.registers.p.0,
                    *addr);
            let value = cpu.memory.fetch(*addr);
            assert!(value == (ror_result.2),
                    "Bad rotate result {:#04x} in address {:#06x}",
                    value,
                    *addr);
        }
    }
}

#[test]
fn test_sbc() {
    let mut cpu = cpu::Cpu::new();

    // First entry is value to be subtracted.
    // Second entry is value in accumulator before the subtraction.
    // Third entry is carry bit.
    // Fourth entry is expected outcome.
    // Fifth value is expected status register value.
    let sbc_results = [(0x01, 0x00, true, 0xff, I_FLAG | N_FLAG),
                       (0x01, 0x80, true, 0x7f, I_FLAG | V_FLAG | C_FLAG),
                       (0xff, 0x7f, false, 0x7f, I_FLAG)];

    for sbc_result in sbc_results.iter() {
        // Reset from the last round.
        cpu.reset();

        // Store value in memory locations for testing.
        let sbc_addresses = [0x003a, 0x004a, 0x123a, 0x234a, 0x345a, 0xfada, 0xbeea];
        for addr in sbc_addresses.iter() {
            cpu.memory.store(*addr, sbc_result.0);
        }

        // Store extra memory values for indirect lookups.
        cpu.memory.store_u16(0x005a, 0xfada);
        cpu.memory.store_u16(0x006a, 0xbdec);

        // Store x and y registers.
        cpu.registers.x = 0xff;
        cpu.registers.y = 0xfe;

        cpu.memory
            .store_bytes(0x0000,
                         &[// Subtract immediate instruction arg.
                           SBC_Imm as u8,
                           sbc_result.0,

                           // Subtract 0x003a
                           SBC_Zero as u8,
                           0x3a,

                           // Subtract 0x004a
                           SBC_Zero_X as u8,
                           0x4b,

                           // Subtract 0x123a
                           SBC_Abs as u8,
                           0x3a,
                           0x12,

                           // Subtract 0x234a
                           SBC_Abs_X as u8,
                           0x4b,
                           0x22,

                           // Subtract 0x345a
                           SBC_Abs_Y as u8,
                           0x5c,
                           0x33,

                           // Subtract 0xfada (thru 0x005a)
                           SBC_Ind_X as u8,
                           0x5b,

                           // Subtract 0xbeea (thru 0x006a)
                           SBC_Ind_Y as u8,
                           0x6a]);

        for _ in 0..sbc_addresses.len() + 1 {
            cpu.registers.a = sbc_result.1;
            cpu.registers.p = cpu::Status::new();
            cpu.registers.p.set_c(sbc_result.2);
            cpu.execute();

            assert!(cpu.registers.a == sbc_result.3,
                    "Bad result {:#04x} in A after SBC",
                    cpu.registers.a);
            assert!(cpu.registers.p.0 == sbc_result.4,
                    "Bad flag: {:08b}",
                    cpu.registers.p.0);
        }
    }
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
                       0xab,
                       0xff,

                       // Store to 0x00aa
                       STA_Abs_X as u8,
                       0xab,
                       0xff,

                       // Store to 0x00a9
                       STA_Abs_Y as u8,
                       0xab,
                       0xff,

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
                       0xab,
                       0xff]);

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
                       0xab,
                       0xff]);

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
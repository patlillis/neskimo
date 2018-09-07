use nes::cpu;
use nes::opcode;
use std;
use utils::arithmetic::{add_relative, concat_bytes};
use utils::paging::{page_cross, PageCross};

use nes::definition::*;

// Whether a branch was taken. This is used to figure out whether we need to
// take an extra cycle when executing a branch instruction.
pub type BranchTaken = bool;

// An instruction.
//
// First byte is opcode. Seconds and third are optional arguments.
pub struct Instruction(pub u8, pub u8, pub u8);

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:#04x}, {:#04x}, {:#04x})", self.0, self.1, self.2)
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Instruction {
    // Parse an instruction from a specifc point in memory.
    //
    // If the instruction takes arguments, they will be read from
    // subsequent locations. Also sets CPU's decoded args.
    pub fn parse(pc: u16, cpu: &mut cpu::Cpu) -> (Instruction, InstructionDefinition) {
        let raw_opcode = cpu.memory.fetch(pc);
        let opcode = opcode::decode(raw_opcode);
        let def = lookup_instruction_definition(opcode);
        let arg1 = if def.len > 1 {
            cpu.memory.fetch(pc + 1)
        } else {
            0
        };
        let arg2 = if def.len > 2 {
            cpu.memory.fetch(pc + 2)
        } else {
            0
        };

        let instr_str = match def.len {
            1 => format!("{:02X}", raw_opcode),
            2 => format!("{:02X} {:02X}", raw_opcode, arg1),
            3 => format!("{:02X} {:02X} {:02X}", raw_opcode, arg1, arg2),
            _ => "".to_string(),
        };
        cpu.frame_log.instruction = instr_str;
        cpu.frame_log.mneumonic = format!("{}", opcode);

        (Instruction(raw_opcode, arg1, arg2), def)
    }

    pub fn opcode(&self) -> u8 {
        self.0
    }

    pub fn arg1(&self) -> u8 {
        self.1
    }

    pub fn arg2(&self) -> u8 {
        self.2
    }

    fn immediate_value(&self, cpu: &mut cpu::Cpu) -> u8 {
        let value = self.arg1();
        cpu.frame_log.decoded_args = format!("#${:02X}", value);
        value
    }

    // Get the absolute address from the instruction args.
    fn absolute_address(&self, cpu: &mut cpu::Cpu) -> u16 {
        let address = concat_bytes(self.arg2(), self.arg1());
        cpu.frame_log
            .decoded_args
            .push_str(format!("${:04X}", address).as_str());
        address
    }

    // Get the absolute address from the instruction args, and add an offset
    // from the X index register. Also returns whether a page boundary was
    // crossed.
    fn absolute_address_x(&self, cpu: &mut cpu::Cpu) -> (u16, PageCross) {
        let base_addr = self.absolute_address(cpu);
        let address = base_addr.wrapping_add(cpu.registers.x as u16);
        cpu.frame_log
            .decoded_args
            .push_str(format!(",X @ {:04X}", address).as_str());
        (address, page_cross(base_addr, address))
    }

    // Get the absolute address from the instruction args, and add an offset
    // from the Y index register. Also returns whether a page boundary was
    // crossed.
    fn absolute_address_y(&self, cpu: &mut cpu::Cpu) -> (u16, PageCross) {
        let base_addr = self.absolute_address(cpu);
        let address = base_addr.wrapping_add(cpu.registers.y as u16);
        cpu.frame_log
            .decoded_args
            .push_str(format!(",Y @ {:04X}", address).as_str());
        (address, page_cross(base_addr, address))
    }

    // Uses a signed variation of the instruction args, plus the current PC.
    //
    // This is used for branch operations, which uses a signed offset from the
    // current program counter. In other words, branches can jump forward or
    // back.
    fn relative_address(&self, cpu: &mut cpu::Cpu) -> u16 {
        let arg = self.arg1() as i8;
        let address = add_relative(cpu.registers.pc, arg);
        cpu.frame_log.decoded_args = format!("${:04X}", address);
        address
    }

    // Get the zero page address from the instruction args.
    fn zero_page_address(&self, cpu: &mut cpu::Cpu) -> u16 {
        let address = self.arg1();
        cpu.frame_log.decoded_args = format!("${:02X}", address);
        address as u16
    }

    // Get the zero page address from the instruciton args, and add an offset
    // from the X index register. Note that this add wraps around to always be
    // on the zero page.
    fn zero_page_address_x(&self, cpu: &mut cpu::Cpu) -> u16 {
        let arg1 = self.arg1();
        let result = arg1.wrapping_add(cpu.registers.x);
        cpu.frame_log
            .decoded_args
            .push_str(format!("${:02X},X @ {:02X}", arg1, result).as_str());
        result as u16
    }

    // Get the zero page address from the instruction args, and add an offset
    // from the Y index register. Note that this add wraps around to always be
    // on the zero page.
    fn zero_page_address_y(&self, cpu: &mut cpu::Cpu) -> u16 {
        let arg1 = self.arg1();
        let result = arg1.wrapping_add(cpu.registers.y);
        cpu.frame_log
            .decoded_args
            .push_str(format!("${:02X},Y @ {:02X}", arg1, result).as_str());
        result as u16
    }

    // Get the absolute address from the instruction args, and return the value
    // that is stored in that address in memory.
    //
    // This method implements a bug found in the original MOS6502 hardware,
    // where the two bytes read had to be on the same page. So if the low
    // byte is stored at 0x33ff, then the high byte would be fetched from
    // 0x3300 instead of 0x3400.
    fn indirect_address(&self, cpu: &mut cpu::Cpu) -> u16 {
        cpu.frame_log.decoded_args.push_str("(");
        let address = self.absolute_address(cpu);
        let result = cpu.memory.fetch_u16_wrap_msb(address);
        cpu.frame_log
            .decoded_args
            .push_str(format!(") = {:04X}", result).as_str());
        result
    }

    // Calculates a memory address using by adding X to the 8-bit value in the
    // instruction, THEN use that address to find ANOTHER address, then return
    // THAT address.
    fn indirect_address_x(&self, cpu: &mut cpu::Cpu) -> u16 {
        let address = self.zero_page_address_x(cpu);
        let result = cpu.memory.fetch_u16_wrap_msb(address);
        cpu.frame_log.decoded_args = format!(
            "(${:02X},X) @ {:02X} = {:04X}",
            self.arg1(),
            address,
            result
        );
        result
    }

    // Similar to indirect_address_x, except that the y register value is added
    // after dereferencing the 8-bit value. Also returns whether a page
    // boundary was crossed.
    fn indirect_address_y(&self, cpu: &mut cpu::Cpu) -> (u16, PageCross) {
        let address = self.zero_page_address(cpu);
        let intermediate = cpu.memory.fetch_u16_wrap_msb(address);
        let result = intermediate.wrapping_add(cpu.registers.y as u16);
        cpu.frame_log.decoded_args = format!(
            "(${:02X}),Y = {:04X} @ {:04X}",
            self.arg1(),
            intermediate,
            result
        );
        (result, page_cross(intermediate, result))
    }

    // Execute the instruction on the cpu. Returns the number of cycles taken.
    pub fn execute(&self, cpu: &mut cpu::Cpu, instruction_location: u16) -> u8 {
        use nes::opcode::Opcode::*;
        let opcode = opcode::decode(self.opcode());
        let def = lookup_instruction_definition(opcode);
        let mut cycles = def.cycles;

        match opcode {
            // ADd with Carry
            ADC_Imm => {
                let value = self.immediate_value(cpu);
                cpu.adc_value(value);
                cycles += 0;
            }
            ADC_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.adc(address);
            }
            ADC_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.adc(address);
            }
            ADC_Abs => {
                let address = self.absolute_address(cpu);
                cpu.adc(address);
            }
            ADC_Abs_X => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                cpu.adc(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            ADC_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu.adc(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            ADC_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.adc(address);
            }
            ADC_Ind_Y => {
                let (address, page_cross) = self.indirect_address_y(cpu);
                cpu.adc(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // bitwise AND with accumulator
            AND_Imm => {
                let value = self.immediate_value(cpu);
                cpu.and_value(value);
            }
            AND_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.and(address);
            }
            AND_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.and(address);
            }
            AND_Abs => {
                let address = self.absolute_address(cpu);
                cpu.and(address);
            }
            AND_Abs_X => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                cpu.and(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            AND_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu.and(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            AND_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.and(address);
            }
            AND_Ind_Y => {
                let (address, page_cross) = self.indirect_address_y(cpu);
                cpu.and(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // Arithmetic Shift Left
            ASL_Acc => {
                cpu.asl_a();
            }
            ASL_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.asl(address);
            }
            ASL_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.asl(address);
            }
            ASL_Abs => {
                let address = self.absolute_address(cpu);
                cpu.asl(address);
            }
            ASL_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu.asl(address);
            }

            // test BITs
            BIT_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.bit(address);
            }
            BIT_Abs => {
                let address = self.absolute_address(cpu);
                cpu.bit(address);
            }

            // Branch instructions
            BPL => {
                let address = self.relative_address(cpu);
                let branch_taken = cpu.bpl(address);
                let page_cross = page_cross(instruction_location + def.len, address);
                if branch_taken {
                    match page_cross {
                        PageCross::Same => cycles += 1,
                        _ => cycles += 2,
                    }
                }
            }
            BMI => {
                let address = self.relative_address(cpu);
                let branch_taken = cpu.bmi(address);
                let page_cross = page_cross(instruction_location + def.len, address);
                if branch_taken {
                    match page_cross {
                        PageCross::Same => cycles += 1,
                        _ => cycles += 2,
                    }
                }
            }
            BVC => {
                let address = self.relative_address(cpu);
                let branch_taken = cpu.bvc(address);
                let page_cross = page_cross(instruction_location + def.len, address);
                if branch_taken {
                    match page_cross {
                        PageCross::Same => cycles += 1,
                        _ => cycles += 2,
                    }
                }
            }
            BVS => {
                let address = self.relative_address(cpu);
                let branch_taken = cpu.bvs(address);
                let page_cross = page_cross(instruction_location + def.len, address);
                if branch_taken {
                    match page_cross {
                        PageCross::Same => cycles += 1,
                        _ => cycles += 2,
                    }
                }
            }
            BCC => {
                let address = self.relative_address(cpu);
                let branch_taken = cpu.bcc(address);
                let page_cross = page_cross(instruction_location + def.len, address);
                if branch_taken {
                    match page_cross {
                        PageCross::Same => cycles += 1,
                        _ => cycles += 2,
                    }
                }
            }
            BCS => {
                let address = self.relative_address(cpu);
                let branch_taken = cpu.bcs(address);
                let page_cross = page_cross(instruction_location + def.len, address);
                if branch_taken {
                    match page_cross {
                        PageCross::Same => cycles += 1,
                        _ => cycles += 2,
                    }
                }
            }
            BNE => {
                let address = self.relative_address(cpu);
                let branch_taken = cpu.bne(address);
                let page_cross = page_cross(instruction_location + def.len, address);
                if branch_taken {
                    match page_cross {
                        PageCross::Same => cycles += 1,
                        _ => cycles += 2,
                    }
                }
            }
            BEQ => {
                let address = self.relative_address(cpu);
                let branch_taken = cpu.beq(address);
                let page_cross = page_cross(instruction_location + def.len, address);
                if branch_taken {
                    match page_cross {
                        PageCross::Same => cycles += 1,
                        _ => cycles += 2,
                    }
                }
            }

            // BReaK
            BRK => cpu.brk(),

            // Flag (processor status)
            CLC => cpu.clc(),
            SEC => cpu.sec(),
            CLI => cpu.cli(),
            SEI => cpu.sei(),
            CLV => cpu.clv(),
            CLD => cpu.cld(),
            SED => cpu.sed(),

            // CoMPare accumulator
            CMP_Imm => {
                let value = self.immediate_value(cpu);
                cpu.cmp_value(value);
            }
            CMP_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.cmp(address);
            }
            CMP_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.cmp(address);
            }
            CMP_Abs => {
                let address = self.absolute_address(cpu);
                cpu.cmp(address);
            }
            CMP_Abs_X => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                cpu.cmp(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            CMP_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu.cmp(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            CMP_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.cmp(address);
            }
            CMP_Ind_Y => {
                let (address, page_cross) = self.indirect_address_y(cpu);
                cpu.cmp(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // ComPare X register
            CPX_Imm => {
                let value = self.immediate_value(cpu);
                cpu.cpx_value(value);
            }
            CPX_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.cpx(address);
            }
            CPX_Abs => {
                let address = self.absolute_address(cpu);
                cpu.cpx(address);
            }

            // ComPare Y register
            CPY_Imm => {
                let value = self.immediate_value(cpu);
                cpu.cpy_value(value);
            }
            CPY_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.cpy(address);
            }
            CPY_Abs => {
                let address = self.absolute_address(cpu);
                cpu.cpy(address);
            }

            // DECrement memory
            DEC_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.dec(address);
            }
            DEC_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.dec(address);
            }
            DEC_Abs => {
                let address = self.absolute_address(cpu);
                cpu.dec(address);
            }
            DEC_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu.dec(address);
            }

            // bitwise Exclusive OR
            EOR_Imm => {
                let value = self.immediate_value(cpu);
                cpu.eor_value(value);
            }
            EOR_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.eor(address);
            }
            EOR_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.eor(address);
            }
            EOR_Abs => {
                let address = self.absolute_address(cpu);
                cpu.eor(address);
            }
            EOR_Abs_X => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                cpu.eor(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            EOR_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu.eor(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            EOR_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.eor(address);
            }
            EOR_Ind_Y => {
                let (address, page_cross) = self.indirect_address_y(cpu);
                cpu.eor(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // INCrement memory
            INC_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.inc(address);
            }
            INC_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.inc(address);
            }
            INC_Abs => {
                let address = self.absolute_address(cpu);
                cpu.inc(address);
            }
            INC_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu.inc(address);
            }

            // JuMP
            JMP_Abs => {
                let address = self.absolute_address(cpu);
                cpu.jmp(address);
            }
            JMP_Ind => {
                let address = self.indirect_address(cpu);
                cpu.jmp(address);
            }

            // Jump to SubRoutine
            JSR => {
                let address = self.absolute_address(cpu);
                cpu.jsr(address);
            }

            // LoaD Accumulator
            LDA_Imm => {
                let value = self.immediate_value(cpu);
                cpu.lda_value(value);
            }
            LDA_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.lda(address);
            }
            LDA_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.lda(address);
            }
            LDA_Abs => {
                let address = self.absolute_address(cpu);
                cpu.lda(address);
            }
            LDA_Abs_X => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                cpu.lda(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            LDA_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu.lda(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            LDA_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.lda(address);
            }
            LDA_Ind_Y => {
                let (address, page_cross) = self.indirect_address_y(cpu);
                cpu.lda(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // LoaD X register
            LDX_Imm => {
                let value = self.immediate_value(cpu);
                cpu.ldx_value(value);
            }
            LDX_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.ldx(address);
            }
            LDX_Zero_Y => {
                let address = self.zero_page_address_y(cpu);
                cpu.ldx(address);
            }
            LDX_Abs => {
                let address = self.absolute_address(cpu);
                cpu.ldx(address);
            }
            LDX_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu.ldx(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // LoaD Y register
            LDY_Imm => {
                let value = self.immediate_value(cpu);
                cpu.ldy_value(value);
            }
            LDY_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.ldy(address);
            }
            LDY_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.ldy(address);
            }
            LDY_Abs => {
                let address = self.absolute_address(cpu);
                cpu.ldy(address);
            }
            LDY_Abs_X => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                cpu.ldy(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // Logical Shift Right
            LSR_Acc => {
                cpu.lsr_a();
            }
            LSR_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.lsr(address);
            }
            LSR_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.lsr(address);
            }
            LSR_Abs => {
                let address = self.absolute_address(cpu);
                cpu.lsr(address);
            }
            LSR_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu.lsr(address);
            }

            // bitwise OR with Accumulator
            ORA_Imm => {
                let value = self.immediate_value(cpu);
                cpu.ora_value(value);
            }
            ORA_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.ora(address);
            }
            ORA_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.ora(address);
            }
            ORA_Abs => {
                let address = self.absolute_address(cpu);
                cpu.ora(address);
            }
            ORA_Abs_X => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                cpu.ora(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            ORA_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu.ora(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            ORA_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.ora(address);
            }
            ORA_Ind_Y => {
                let (address, page_cross) = self.indirect_address_y(cpu);
                cpu.ora(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // No OPeration
            NOP => {
                cpu.nop();
            }

            // ROtate Left
            ROL_Acc => {
                cpu.rol_a();
            }
            ROL_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.rol(address);
            }
            ROL_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.rol(address);
            }
            ROL_Abs => {
                let address = self.absolute_address(cpu);
                cpu.rol(address);
            }
            ROL_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu.rol(address);
            }

            // ROtate Right
            ROR_Acc => {
                cpu.ror_a();
            }
            ROR_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.ror(address);
            }
            ROR_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.ror(address);
            }
            ROR_Abs => {
                let address = self.absolute_address(cpu);
                cpu.ror(address);
            }
            ROR_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu.ror(address);
            }

            // ReTurn from Interrupt
            RTI => cpu.rti(),

            // ReTurn from Subroutine
            RTS => cpu.rts(),

            // SuBtract with Carry
            SBC_Imm => {
                let value = self.immediate_value(cpu);
                cpu.sbc_value(value);
            }
            SBC_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.sbc(address);
            }
            SBC_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.sbc(address);
            }
            SBC_Abs => {
                let address = self.absolute_address(cpu);
                cpu.sbc(address);
            }
            SBC_Abs_X => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                cpu.sbc(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            SBC_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu.sbc(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            SBC_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.sbc(address);
            }
            SBC_Ind_Y => {
                let (address, page_cross) = self.indirect_address_y(cpu);
                cpu.sbc(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // STore Accumulator
            STA_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.sta(address);
            }
            STA_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.sta(address);
            }
            STA_Abs => {
                let address = self.absolute_address(cpu);
                cpu.sta(address);
            }
            STA_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu.sta(address);
            }
            STA_Abs_Y => {
                let (address, _) = self.absolute_address_y(cpu);
                cpu.sta(address);
            }
            STA_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.sta(address);
            }
            STA_Ind_Y => {
                let (address, _) = self.indirect_address_y(cpu);
                cpu.sta(address);
            }

            // STore X register
            STX_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.stx(address);
            }
            STX_Zero_Y => {
                let address = self.zero_page_address_y(cpu);
                cpu.stx(address);
            }
            STX_Abs => {
                let address = self.absolute_address(cpu);
                cpu.stx(address);
            }

            // Register Instructions
            TAX => cpu.tax(),
            TXA => cpu.txa(),
            DEX => cpu.dex(),
            INX => cpu.inx(),
            TAY => cpu.tay(),
            TYA => cpu.tya(),
            DEY => cpu.dey(),
            INY => cpu.iny(),

            // Stack Instructions
            TXS => cpu.txs(),
            TSX => cpu.tsx(),
            PHA => cpu.pha(),
            PLA => cpu.pla(),
            PHP => cpu.php(),
            PLP => cpu.plp(),

            // STore Y register
            STY_Zero => {
                let address = self.zero_page_address(cpu);
                cpu.sty(address);
            }
            STY_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.sty(address);
            }
            STY_Abs => {
                let address = self.absolute_address(cpu);
                cpu.sty(address);
            }

            // UNOFFICIAL OPCODES

            // No-ops
            _NOP_1 | _NOP_2 | _NOP_3 | _NOP_4 | _NOP_5 | _NOP_6 => {}

            // No-op reads
            _NOP_Imm_1 | _NOP_Imm_2 | _NOP_Imm_3 | _NOP_Imm_4 | _NOP_Imm_5 => {
                #[allow(unused_variables)]
                let address = self.immediate_value(cpu);
            }
            _NOP_Abs => {
                let address = self.absolute_address(cpu);
                let value = cpu.memory.fetch(address);
                cpu.decode_operand_value(value);
            }
            _NOP_Abs_X_1 | _NOP_Abs_X_2 | _NOP_Abs_X_3 | _NOP_Abs_X_4 | _NOP_Abs_X_5
            | _NOP_Abs_X_6 => {
                let (address, page_cross) = self.absolute_address_x(cpu);
                let value = cpu.memory.fetch(address);
                cpu.decode_operand_value(value);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            _NOP_Zero_1 | _NOP_Zero_2 | _NOP_Zero_3 => {
                let address = self.zero_page_address(cpu);
                let value = cpu.memory.fetch(address);
                cpu.decode_operand_value(value);
            }
            _NOP_Zero_X_1 | _NOP_Zero_X_2 | _NOP_Zero_X_3 | _NOP_Zero_X_4 | _NOP_Zero_X_5
            | _NOP_Zero_X_6 => {
                let address = self.zero_page_address_x(cpu);
                let value = cpu.memory.fetch(address);
                cpu.decode_operand_value(value);
            }

            // Load Accumulator into X register
            _LAX_Abs => {
                let address = self.absolute_address(cpu);
                cpu._lax(address);
            }
            _LAX_Abs_Y => {
                let (address, page_cross) = self.absolute_address_y(cpu);
                cpu._lax(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }
            _LAX_Zero => {
                let address = self.zero_page_address(cpu);
                cpu._lax(address);
            }
            _LAX_Zero_Y => {
                let address = self.zero_page_address_y(cpu);
                cpu._lax(address);
            }
            _LAX_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu._lax(address);
            }
            _LAX_Ind_Y => {
                let (address, page_cross) = self.indirect_address_y(cpu);
                cpu._lax(address);
                if page_cross != PageCross::Same {
                    cycles += 1;
                }
            }

            // Store bitwise and of Accumulator and X register
            _SAX_Abs => {
                let address = self.absolute_address(cpu);
                cpu._sax(address);
            }
            _SAX_Zero => {
                let address = self.zero_page_address(cpu);
                cpu._sax(address);
            }
            _SAX_Zero_Y => {
                let address = self.zero_page_address_y(cpu);
                cpu._sax(address);
            }
            _SAX_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu._sax(address);
            }

            // SuBtract with Carry
            _SBC_Imm => {
                let value = self.immediate_value(cpu);
                cpu.sbc_value(value);
            }

            // Decrement value, ComPare accumulator
            _DCP_Abs => {
                let address = self.absolute_address(cpu);
                cpu._dcp(address);
            }
            _DCP_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu._dcp(address);
            }
            _DCP_Abs_Y => {
                let (address, _) = self.absolute_address_y(cpu);
                cpu._dcp(address);
            }
            _DCP_Zero => {
                let address = self.zero_page_address(cpu);
                cpu._dcp(address);
            }
            _DCP_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu._dcp(address);
            }
            _DCP_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu._dcp(address);
            }
            _DCP_Ind_Y => {
                let (address, _) = self.indirect_address_y(cpu);
                cpu._dcp(address);
            }

            // Increment value, Subtract with Carry
            _ISB_Abs => {
                let address = self.absolute_address(cpu);
                cpu._isb(address);
            }
            _ISB_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu._isb(address);
            }
            _ISB_Abs_Y => {
                let (address, _) = self.absolute_address_y(cpu);
                cpu._isb(address);
            }
            _ISB_Zero => {
                let address = self.zero_page_address(cpu);
                cpu._isb(address);
            }
            _ISB_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu._isb(address);
            }
            _ISB_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu._isb(address);
            }
            _ISB_Ind_Y => {
                let (address, _) = self.indirect_address_y(cpu);
                cpu._isb(address);
            }

            // arithmetic Shift Left, bitwise Or with accumulator
            _SLO_Abs => {
                let address = self.absolute_address(cpu);
                cpu._slo(address);
            }
            _SLO_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu._slo(address);
            }
            _SLO_Abs_Y => {
                let (address, _) = self.absolute_address_y(cpu);
                cpu._slo(address);
            }
            _SLO_Zero => {
                let address = self.zero_page_address(cpu);
                cpu._slo(address);
            }
            _SLO_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu._slo(address);
            }
            _SLO_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu._slo(address);
            }
            _SLO_Ind_Y => {
                let (address, _) = self.indirect_address_y(cpu);
                cpu._slo(address);
            }

            // Rotate Left, And with accumulator
            _RLA_Abs => {
                let address = self.absolute_address(cpu);
                cpu._rla(address);
            }
            _RLA_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu._rla(address);
            }
            _RLA_Abs_Y => {
                let (address, _) = self.absolute_address_y(cpu);
                cpu._rla(address);
            }
            _RLA_Zero => {
                let address = self.zero_page_address(cpu);
                cpu._rla(address);
            }
            _RLA_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu._rla(address);
            }
            _RLA_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu._rla(address);
            }
            _RLA_Ind_Y => {
                let (address, _) = self.indirect_address_y(cpu);
                cpu._rla(address);
            }

            // logical Shift Right, Exclusive or with accumulator
            _SRE_Abs => {
                let address = self.absolute_address(cpu);
                cpu._sre(address);
            }
            _SRE_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu._sre(address);
            }
            _SRE_Abs_Y => {
                let (address, _) = self.absolute_address_y(cpu);
                cpu._sre(address);
            }
            _SRE_Zero => {
                let address = self.zero_page_address(cpu);
                cpu._sre(address);
            }
            _SRE_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu._sre(address);
            }
            _SRE_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu._sre(address);
            }
            _SRE_Ind_Y => {
                let (address, _) = self.indirect_address_y(cpu);
                cpu._sre(address);
            }

            // Rotate Right, Add with carry
            _RRA_Abs => {
                let address = self.absolute_address(cpu);
                cpu._rra(address);
            }
            _RRA_Abs_X => {
                let (address, _) = self.absolute_address_x(cpu);
                cpu._rra(address);
            }
            _RRA_Abs_Y => {
                let (address, _) = self.absolute_address_y(cpu);
                cpu._rra(address);
            }
            _RRA_Zero => {
                let address = self.zero_page_address(cpu);
                cpu._rra(address);
            }
            _RRA_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu._rra(address);
            }
            _RRA_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu._rra(address);
            }
            _RRA_Ind_Y => {
                let (address, _) = self.indirect_address_y(cpu);
                cpu._rra(address);
            }
        }

        cycles
    }
}

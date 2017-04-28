use cpu;
use memory;
use opcode;
use std;
use utils;

use instruction::definition::*;

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
    // If the instruction takes arguments, they will be read from subsequent locations.
    pub fn parse(pc: u16, memory: &memory::Memory) -> (Instruction, InstructionDefinition) {
        let raw_opcode = memory.fetch(pc);
        let opcode = opcode::decode(raw_opcode);
        let def = lookup_instruction_definition(opcode);
        (match def.len {
             1 => Instruction(raw_opcode, 0, 0),
             2 => Instruction(raw_opcode, memory.fetch(pc + 1), 0),
             3 => Instruction(raw_opcode, memory.fetch(pc + 1), memory.fetch(pc + 2)),
             _ => panic!("Invalid instruction length far opcode {}", opcode),
         },
         def)
    }

    fn opcode(&self) -> u8 {
        self.0
    }

    fn arg1(&self) -> u8 {
        self.1
    }

    fn arg2(&self) -> u8 {
        self.2
    }

    fn immediate_value(&self) -> u8 {
        self.arg1()
    }

    // Get the absolute address from the instruction args.
    fn absolute_address(&self) -> u16 {
        utils::arithmetic::concat_bytes(self.arg1(), self.arg2())
    }

    // Get the absolute address from the instruction args, and add an offset
    // from the X index register.
    fn absolute_address_x(&self, cpu: &cpu::Cpu) -> u16 {
        self.absolute_address()
            .wrapping_add(cpu.registers.x as u16)
    }

    // Get the absolute address from the instruction args, and add an offset
    // from the Y index register.
    fn absolute_address_y(&self, cpu: &cpu::Cpu) -> u16 {
        self.absolute_address()
            .wrapping_add(cpu.registers.y as u16)
    }

    // Get the zero page address from the instruction args.
    fn zero_page_address(&self) -> u16 {
        self.arg1() as u16
    }

    // Get the zero page address from the instruciton args, and add an offset
    // from the X index register. Note that this add wraps around to always be
    // on the zero page.
    fn zero_page_address_x(&self, cpu: &cpu::Cpu) -> u16 {
        self.arg1().wrapping_add(cpu.registers.x) as u16
    }

    // Get the zero page address from the instruciton args, and add an offset
    // from the Y index register. Note that this add wraps around to always be
    // on the zero page.
    fn zero_page_address_y(&self, cpu: &cpu::Cpu) -> u16 {
        self.arg1().wrapping_add(cpu.registers.y) as u16
    }

    // Calculates a memory address using by adding X to the 8-bit value in the
    // instruction, THEN use that address to find ANOTHER address, then return
    // THAT address.
    fn indirect_address_x(&self, cpu: &cpu::Cpu) -> u16 {
        let address = self.zero_page_address_x(cpu);
        cpu.memory.fetch_u16(address)
    }

    fn indirect_address_y(&self, cpu: &cpu::Cpu) -> u16 {
        let address = self.zero_page_address();
        cpu.memory
            .fetch_u16(address)
            .wrapping_add(cpu.registers.y as u16)
    }

    // Execute the instruction on the cpu.
    pub fn execute(&self, cpu: &mut cpu::Cpu) {
        use opcode::Opcode::*;
        let opcode = opcode::decode(self.opcode());

        match opcode {
            // ADd with Carry
            ADC_Imm => {
                let value = self.immediate_value();
                cpu.adc_value(value);
            }
            ADC_Zero => {
                let address = self.zero_page_address();
                cpu.adc(address);
            }
            ADC_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.adc(address);
            }
            ADC_Abs => {
                let address = self.absolute_address();
                cpu.adc(address);
            }
            ADC_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.adc(address);
            }
            ADC_Abs_Y => {
                let address = self.absolute_address_y(cpu);
                cpu.adc(address);
            }
            ADC_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.adc(address);
            }
            ADC_Ind_Y => {
                let address = self.indirect_address_y(cpu);
                cpu.adc(address);
            }

            // Arithmetic Shift Left
            ASL_Acc => {
                cpu.asl_a();
            }
            ASL_Zero => {
                let address = self.zero_page_address();
                cpu.asl(address);
            }
            ASL_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.asl(address);
            }
            ASL_Abs => {
                let address = self.absolute_address();
                cpu.asl(address);
            }
            ASL_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.asl(address);
            }

            // test BITs
            BIT_Zero => {
                let address = self.zero_page_address();
                cpu.bit(address);
            }
            BIT_Abs => {
                let address = self.absolute_address();
                cpu.bit(address);
            }

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
                let value = self.immediate_value();
                cpu.cmp_value(value);
            }
            CMP_Zero => {
                let address = self.zero_page_address();
                cpu.cmp(address);
            }
            CMP_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.cmp(address);
            }
            CMP_Abs => {
                let address = self.absolute_address();
                cpu.cmp(address);
            }
            CMP_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.cmp(address);
            }
            CMP_Abs_Y => {
                let address = self.absolute_address_y(cpu);
                cpu.cmp(address);
            }
            CMP_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.cmp(address);
            }
            CMP_Ind_Y => {
                let address = self.indirect_address_y(cpu);
                cpu.cmp(address);
            }

            // ComPare X register
            CPX_Imm => {
                let value = self.immediate_value();
                cpu.cpx_value(value);
            }
            CPX_Zero => {
                let address = self.zero_page_address();
                cpu.cpx(address);
            }
            CPX_Abs => {
                let address = self.absolute_address();
                cpu.cpx(address);
            }

            // ComPare Y register
            CPY_Imm => {
                let value = self.immediate_value();
                cpu.cpy_value(value);
            }
            CPY_Zero => {
                let address = self.zero_page_address();
                cpu.cpy(address);
            }
            CPY_Abs => {
                let address = self.absolute_address();
                cpu.cpy(address);
            }

            // DECrement memory
            DEC_Zero => {
                let address = self.zero_page_address();
                cpu.dec(address);
            }
            DEC_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.dec(address);
            }
            DEC_Abs => {
                let address = self.absolute_address();
                cpu.dec(address);
            }
            DEC_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.dec(address);
            }

            // INCrement memory
            INC_Zero => {
                let address = self.zero_page_address();
                cpu.inc(address);
            }
            INC_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.inc(address);
            }
            INC_Abs => {
                let address = self.absolute_address();
                cpu.inc(address);
            }
            INC_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.inc(address);
            }

            // LoaD Accumulator
            LDA_Imm => {
                let value = self.immediate_value();
                cpu.lda_value(value);
            }
            LDA_Zero => {
                let address = self.zero_page_address();
                cpu.lda(address);
            }
            LDA_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.lda(address);
            }
            LDA_Abs => {
                let address = self.absolute_address();
                cpu.lda(address);
            }
            LDA_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.lda(address);
            }
            LDA_Abs_Y => {
                let address = self.absolute_address_y(cpu);
                cpu.lda(address);
            }
            LDA_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.lda(address);
            }
            LDA_Ind_Y => {
                let address = self.indirect_address_y(cpu);
                cpu.lda(address);
            }

            // LoaD X register
            LDX_Imm => {
                let value = self.immediate_value();
                cpu.ldx_value(value);
            }
            LDX_Zero => {
                let address = self.zero_page_address();
                cpu.ldx(address);
            }
            LDX_Zero_Y => {
                let address = self.zero_page_address_y(cpu);
                cpu.ldx(address);
            }
            LDX_Abs => {
                let address = self.absolute_address();
                cpu.ldx(address);
            }
            LDX_Abs_Y => {
                let address = self.absolute_address_y(cpu);
                cpu.ldx(address);
            }

            // LoaD Y register
            LDY_Imm => {
                let value = self.immediate_value();
                cpu.ldy_value(value);
            }
            LDY_Zero => {
                let address = self.zero_page_address();
                cpu.ldy(address);
            }
            LDY_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.ldy(address);
            }
            LDY_Abs => {
                let address = self.absolute_address();
                cpu.ldy(address);
            }
            LDY_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.ldy(address);
            }

            // Logical Shift Right
            LSR_Acc => {
                cpu.lsr_a();
            }
            LSR_Zero => {
                let address = self.zero_page_address();
                cpu.lsr(address);
            }
            LSR_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.lsr(address);
            }
            LSR_Abs => {
                let address = self.absolute_address();
                cpu.lsr(address);
            }
            LSR_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.lsr(address);
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
                let address = self.zero_page_address();
                cpu.rol(address);
            }
            ROL_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.rol(address);
            }
            ROL_Abs => {
                let address = self.absolute_address();
                cpu.rol(address);
            }
            ROL_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.rol(address);
            }

            // ROtate Right
            ROR_Acc => {
                cpu.ror_a();
            }
            ROR_Zero => {
                let address = self.zero_page_address();
                cpu.ror(address);
            }
            ROR_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.ror(address);
            }
            ROR_Abs => {
                let address = self.absolute_address();
                cpu.ror(address);
            }
            ROR_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.ror(address);
            }

            // SuBtract with Carry
            SBC_Imm => {
                let value = self.immediate_value();
                cpu.sbc_value(value);
            }
            SBC_Zero => {
                let address = self.zero_page_address();
                cpu.sbc(address);
            }
            SBC_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.sbc(address);
            }
            SBC_Abs => {
                let address = self.absolute_address();
                cpu.sbc(address);
            }
            SBC_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.sbc(address);
            }
            SBC_Abs_Y => {
                let address = self.absolute_address_y(cpu);
                cpu.sbc(address);
            }
            SBC_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.sbc(address);
            }
            SBC_Ind_Y => {
                let address = self.indirect_address_y(cpu);
                cpu.sbc(address);
            }

            // STore Accumulator
            STA_Zero => {
                let address = self.zero_page_address();
                cpu.sta(address);
            }
            STA_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.sta(address);
            }
            STA_Abs => {
                let address = self.absolute_address();
                cpu.sta(address);
            }
            STA_Abs_X => {
                let address = self.absolute_address_x(cpu);
                cpu.sta(address);
            }
            STA_Abs_Y => {
                let address = self.absolute_address_y(cpu);
                cpu.sta(address);
            }
            STA_Ind_X => {
                let address = self.indirect_address_x(cpu);
                cpu.sta(address);
            }
            STA_Ind_Y => {
                let address = self.indirect_address_y(cpu);
                cpu.sta(address);
            }

            // STore X register
            STX_Zero => {
                let address = self.zero_page_address();
                cpu.stx(address);
            }
            STX_Zero_Y => {
                let address = self.zero_page_address_y(cpu);
                cpu.stx(address);
            }
            STX_Abs => {
                let address = self.absolute_address();
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

            // STore Y register
            STY_Zero => {
                let address = self.zero_page_address();
                cpu.sty(address);
            }
            STY_Zero_X => {
                let address = self.zero_page_address_x(cpu);
                cpu.sty(address);
            }
            STY_Abs => {
                let address = self.absolute_address();
                cpu.sty(address);
            }
        }
    }
}

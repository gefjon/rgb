#![allow(non_camel_case_types)]

use std::num::Wrapping;

#[cfg(test)]
mod test;

pub mod registers;
use self::registers::*;

mod addition;
use self::addition::Addition;

mod subtraction;
use self::subtraction::Subtraction;

mod bit_ops;
use self::bit_ops::BitInstructions;

mod increment;
use self::increment::Increment;

mod load_ops;
use self::load_ops::Ld;

mod stack;
use self::stack::Stack;

mod function_calls;
use self::function_calls::FunCall;

mod jump_relative;
use self::jump_relative::JumpRelative;

mod jump;
use self::jump::Jump;

use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;
use memory::Memory;

#[derive(Debug, Copy, Clone)]
pub enum CpuMode {
    DMG,
    MGB,
    CGB,
}

macro_rules! bad_inst {
    () => (
        panic!("Bad instruction!")
    )
}

pub struct Cpu {
    gp_registers: Registers,
    stack_pointer: d16,
    program_counter: d16,
    cycle_count: u64,
    memory: Memory,
}

impl Cpu {
    pub fn new(mode: CpuMode) -> Self {
        let stack_pointer = d16(Wrapping(0xfffe));
        let program_counter = d16(Wrapping(0x0100));

        let gp_registers = Registers::new(mode);

        Cpu {
            gp_registers,
            stack_pointer,
            program_counter,
            cycle_count: 0,
            memory: Memory::new_zeros(),
        }
    }

    pub fn process_instruction(&mut self, ins: ::instructions::RawOpcode) {
        use instructions::RawOpcode::*;
        self.program_counter += d16(Wrapping(1)); // inc the program counter before doing work so that loading subsequent bytes will work
        match ins {
            NOP => self.nop(),
            LD_BC_d16 => self.ld_r16_d16(r16::BC),
            LD_BC_A => self.ld_r16_r8(r16::BC, r8::A),
            INC_BC => self.inc_r16(r16::BC),
            INC_B => self.inc_r8(r8::B),
            DEC_B => self.dec_r8(r8::B),
            LD_B_d8 => self.ld_r8_d8(r8::B),
            RLCA => self.rotate_left_carry(r8::A),
            
            LD_a16_SP => self.ld_a16_SP(),
            ADD_HL_BC => self.add_r16_r16(r16::HL, r16::BC),
            LD_A_ptrBC => self.ld_r8_ptrr16(r8::A, r16::BC),
            DEC_BC => self.dec_r16(r16::BC),
            INC_C => self.inc_r8(r8::C),
            DEC_C => self.dec_r8(r8::C),
            LD_C_d8 => self.ld_r8_d8(r8::C),
            RRCA => self.rotate_right_carry(r8::A),

            
            STOP_0 => unimplemented!(),
            LD_DE_d16 => self.ld_r16_d16(r16::DE),
            LD_DE_A => self.ld_r16_r8(r16::DE, r8::A),
            INC_DE => self.inc_r16(r16::BC),
            INC_D => self.inc_r8(r8::D),
            DEC_D => self.dec_r8(r8::D),
            LD_D_d8 => self.ld_r8_d8(r8::D),
            RLA => self.rotate_left(r8::A),
            
            JR_d8 => self.jr_d8(),
            ADD_HL_DE => self.add_r16_r16(r16::HL, r16::DE),
            LD_A_ptrDE => self.ld_r8_ptrr16(r8::A, r16::DE),
            DEC_DE => self.dec_r16(r16::DE),
            INC_E => self.inc_r8(r8::E),
            DEC_E => self.dec_r8(r8::E),
            LD_E_d8 => self.ld_r8_d8(r8::E),
            RRA => self.rotate_right(r8::A),


            JR_NZ_d8 => self.jr_cond_d8(Conditions::NZ),
            LD_HL_d16 => self.ld_r16_d16(r16::HL),
            LD_ptrHLp_A => unimplemented!(),
            INC_HL => self.inc_r16(r16::HL),
            INC_H => self.inc_r8(r8::H),
            DEC_H => self.dec_r8(r8::H),
            LD_H_d8 => self.ld_r8_d8(r8::H),
            DAA => unimplemented!(),

            JR_Z_d8 => self.jr_cond_d8(Conditions::Z),
            ADD_HL_HL => self.add_r16_r16(r16::HL, r16::HL),
            LD_A_ptrHLm => unimplemented!(),
            DEC_HL => self.dec_r16(r16::HL),
            INC_L => self.inc_r8(r8::L),
            DEC_L => self.dec_r8(r8::L),
            LD_L_d8 => self.ld_r8_d8(r8::L),
            CPL => self.compliment_r8(r8::A),


            JR_NC_d8 => self.jr_cond_d8(Conditions::NC),
            LD_SP_d16 => self.ld_sp_d16(),
            LD_HLm_A => unimplemented!(),
            INC_SP => self.inc_sp(),
            INC_ptrHL => self.inc_ptrr16(r16::HL),
            DEC_ptrHL => self.dec_ptrr16(r16::HL),
            LD_ptrHL_d8 => unimplemented!(),
            SCF => self.set_carry(true),

            JR_C_d8 => self.jr_cond_d8(Conditions::C),
            ADD_HL_SP => self.add_sp_into(r16::HL),
            LD_A_HLm => unimplemented!(),
            DEC_SP => self.dec_sp(),
            INC_A => self.inc_r8(r8::A),
            DEC_A => self.dec_r8(r8::A),
            LD_A_d8 => unimplemented!(),
            CCF => { let v = !self.gp_registers.get_flag(Flags::C); self.set_carry(v); },
            // the borrow checker won't let me write this one the way I want to;
            // hence this ugly expression

            LD_B_B => self.ld_r8_r8(r8::B, r8::B),
            LD_B_C => self.ld_r8_r8(r8::B, r8::C),
            LD_B_D => self.ld_r8_r8(r8::B, r8::D),
            LD_B_E => self.ld_r8_r8(r8::B, r8::E),
            LD_B_H => self.ld_r8_r8(r8::B, r8::H),
            LD_B_L => self.ld_r8_r8(r8::B, r8::L),
            LD_B_ptrHL => self.ld_r8_ptrr16(r8::B, r16::HL),
            LD_B_A => self.ld_r8_r8(r8::B, r8::A),

            LD_C_B => self.ld_r8_r8(r8::C, r8::B),
            LD_C_C => self.ld_r8_r8(r8::C, r8::C),
            LD_C_D => self.ld_r8_r8(r8::C, r8::D),
            LD_C_E => self.ld_r8_r8(r8::C, r8::E),
            LD_C_H => self.ld_r8_r8(r8::C, r8::H),
            LD_C_L => self.ld_r8_r8(r8::C, r8::L),
            LD_C_ptrHL => self.ld_r8_ptrr16(r8::C, r16::HL),
            LD_C_A => self.ld_r8_r8(r8::C, r8::A),


            LD_D_B => self.ld_r8_r8(r8::D, r8::B),
            LD_D_C => self.ld_r8_r8(r8::D, r8::C),
            LD_D_D => self.ld_r8_r8(r8::D, r8::D),
            LD_D_E => self.ld_r8_r8(r8::D, r8::E),
            LD_D_H => self.ld_r8_r8(r8::D, r8::H),
            LD_D_L => self.ld_r8_r8(r8::D, r8::L),
            LD_D_ptrHL => self.ld_r8_ptrr16(r8::D, r16::HL),
            LD_D_A => self.ld_r8_r8(r8::D, r8::A),

            LD_E_B => self.ld_r8_r8(r8::E, r8::B),
            LD_E_C => self.ld_r8_r8(r8::E, r8::C),
            LD_E_D => self.ld_r8_r8(r8::E, r8::D),
            LD_E_E => self.ld_r8_r8(r8::E, r8::E),
            LD_E_H => self.ld_r8_r8(r8::E, r8::H),
            LD_E_L => self.ld_r8_r8(r8::E, r8::L),
            LD_E_ptrHL => self.ld_r8_ptrr16(r8::E, r16::HL),
            LD_E_A => self.ld_r8_r8(r8::E, r8::A),

            LD_H_B => self.ld_r8_r8(r8::H, r8::B),
            LD_H_C => self.ld_r8_r8(r8::H, r8::C),
            LD_H_D => self.ld_r8_r8(r8::H, r8::D),
            LD_H_E => self.ld_r8_r8(r8::H, r8::E),
            LD_H_H => self.ld_r8_r8(r8::H, r8::H),
            LD_H_L => self.ld_r8_r8(r8::H, r8::L),
            LD_H_ptrHL => self.ld_r8_ptrr16(r8::H, r16::HL),
            LD_H_A => self.ld_r8_r8(r8::H, r8::A),

            LD_L_B => self.ld_r8_r8(r8::L, r8::B),
            LD_L_C => self.ld_r8_r8(r8::L, r8::C),
            LD_L_D => self.ld_r8_r8(r8::L, r8::D),
            LD_L_E => self.ld_r8_r8(r8::L, r8::E),
            LD_L_H => self.ld_r8_r8(r8::L, r8::H),
            LD_L_L => self.ld_r8_r8(r8::L, r8::L),
            LD_L_ptrHL => self.ld_r8_ptrr16(r8::L, r16::HL),
            LD_L_A => self.ld_r8_r8(r8::L, r8::A),


            LD_ptrHL_B => self.ld_ptrr16_r8(r16::HL, r8::B),
            LD_ptrHL_C => self.ld_ptrr16_r8(r16::HL, r8::C),
            LD_ptrHL_D => self.ld_ptrr16_r8(r16::HL, r8::D),
            LD_ptrHL_E => self.ld_ptrr16_r8(r16::HL, r8::E),
            LD_ptrHL_H => self.ld_ptrr16_r8(r16::HL, r8::H),
            LD_ptrHL_L => self.ld_ptrr16_r8(r16::HL, r8::L),
            HALT => unimplemented!(),
            LD_ptrHL_A => self.ld_ptrr16_r8(r16::HL, r8::A),

            LD_A_B => self.ld_r8_r8(r8::A, r8::B),
            LD_A_C => self.ld_r8_r8(r8::A, r8::C),
            LD_A_D => self.ld_r8_r8(r8::A, r8::D),
            LD_A_E => self.ld_r8_r8(r8::A, r8::E),
            LD_A_H => self.ld_r8_r8(r8::A, r8::H),
            LD_A_L => self.ld_r8_r8(r8::A, r8::L),
            LD_A_ptrHL => self.ld_r8_ptrr16(r8::A, r16::HL),
            LD_A_A => self.ld_r8_r8(r8::A, r8::A),


            ADD_A_B => self.add_r8_r8(r8::A, r8::B),
            ADD_A_C => self.add_r8_r8(r8::A, r8::C),
            ADD_A_D => self.add_r8_r8(r8::A, r8::D),
            ADD_A_E => self.add_r8_r8(r8::A, r8::E),
            ADD_A_H => self.add_r8_r8(r8::A, r8::H),
            ADD_A_L => self.add_r8_r8(r8::A, r8::L),
            ADD_A_ptrHL => unimplemented!(),
            ADD_A_A => self.add_r8_r8(r8::A, r8::A),

            ADC_A_B => self.adc_r8_r8(r8::A, r8::B),
            ADC_A_C => self.adc_r8_r8(r8::A, r8::C),
            ADC_A_D => self.adc_r8_r8(r8::A, r8::D),
            ADC_A_E => self.adc_r8_r8(r8::A, r8::E),
            ADC_A_H => self.adc_r8_r8(r8::A, r8::H),
            ADC_A_L => self.adc_r8_r8(r8::A, r8::L),
            ADC_A_ptrHL => unimplemented!(),
            ADC_A_A => self.adc_r8_r8(r8::A, r8::A),


            SUB_B => self.sub_r8(r8::B),
            SUB_C => self.sub_r8(r8::C),
            SUB_D => self.sub_r8(r8::D),
            SUB_E => self.sub_r8(r8::E),
            SUB_H => self.sub_r8(r8::H),
            SUB_L => self.sub_r8(r8::L),
            SUB_ptrHL => unimplemented!(),
            SUB_A => self.sub_r8(r8::A),

            SBC_B => self.sbc_r8(r8::B),
            SBC_C => self.sbc_r8(r8::C),
            SBC_D => self.sbc_r8(r8::D),
            SBC_E => self.sbc_r8(r8::E),
            SBC_H => self.sbc_r8(r8::H),
            SBC_L => self.sbc_r8(r8::L),
            SBC_ptrHL => unimplemented!(),
            SBC_A => self.sbc_r8(r8::A),


            AND_B => self.and_r8(r8::B),
            AND_C => self.and_r8(r8::C),
            AND_D => self.and_r8(r8::D),
            AND_E => self.and_r8(r8::E),
            AND_H => self.and_r8(r8::H),
            AND_L => self.and_r8(r8::L),
            AND_ptrHL => unimplemented!(),
            AND_A => self.and_r8(r8::A),

            XOR_B => self.xor_r8(r8::B),
            XOR_C => self.xor_r8(r8::C),
            XOR_D => self.xor_r8(r8::D),
            XOR_E => self.xor_r8(r8::E),
            XOR_H => self.xor_r8(r8::H),
            XOR_L => self.xor_r8(r8::L),
            XOR_ptrHL => unimplemented!(),
            XOR_A => self.xor_r8(r8::A),


            OR_B => self.or_r8(r8::B),
            OR_C => self.or_r8(r8::C),
            OR_D => self.or_r8(r8::D),
            OR_E => self.or_r8(r8::E),
            OR_H => self.or_r8(r8::H),
            OR_L => self.or_r8(r8::L),
            OR_ptrHL => unimplemented!(),
            OR_A => self.or_r8(r8::A),

            CP_B => self.cp_r8(r8::B),
            CP_C => self.cp_r8(r8::C),
            CP_D => self.cp_r8(r8::D),
            CP_E => self.cp_r8(r8::E),
            CP_H => self.cp_r8(r8::H),
            CP_L => self.cp_r8(r8::L),
            CP_ptrHL => unimplemented!(),
            CP_A => self.cp_r8(r8::A),


            RET_NZ => self.return_if(Conditions::NZ),
            POP_BC => self.pop_r16(r16::BC),
            JP_NZ => self.jp_cond_a16(Conditions::NZ),
            JP => self.jp_a16(),
            CALL_NZ => self.call_if(Conditions::NZ),
            PUSH_BC => self.push_r16(r16::BC),
            ADD_A_d8 => unimplemented!(),
            RST_00H => unimplemented!(),

            RET_Z => self.return_if(Conditions::Z),
            RET => self.ret(),
            JP_Z => self.jp_cond_a16(Conditions::Z),
            PREFIX_CB => unimplemented!(),
            CALL_Z => self.call_if(Conditions::Z),
            CALL => self.call(),
            ADC_A_d8 => unimplemented!(),
            RST_08H => unimplemented!(),


            RET_NC => self.return_if(Conditions::NC),
            POP_DE => self.pop_r16(r16::DE),
            JP_NC => self.jp_cond_a16(Conditions::NC),
            BAD_0 => panic!("Bad instruction!"),
            CALL_NC => self.call_if(Conditions::NC),
            PUSH_DE => self.push_r16(r16::DE),
            SUB_A_d8 => unimplemented!(),
            RST_10H => unimplemented!(),

            RET_C => self.return_if(Conditions::C),
            RETI => unimplemented!(),
            JP_C => self.jp_cond_a16(Conditions::C),
            BAD_1 => panic!("Bad instruction!"),
            CALL_C => self.call_if(Conditions::C),
            BAD_2 => panic!("Bad instruction!"),
            SBC_A_d8 => unimplemented!(),
            RST_18H => unimplemented!(),


            LDH_a8_A => self.ldh_a8_r8(r8::A),
            POP_HL => self.pop_r16(r16::HL),
            LDH_C_A => self.ldh_c_r8(r8::A),
            BAD_3 => panic!("Bad instruction!"),
            BAD_4 => panic!("Bad instruction!"),
            PUSH_HL => self.push_r16(r16::HL),
            AND_d8 => unimplemented!(),
            RST_20H => unimplemented!(),
            
            ADD_SP_d8 => self.add_sp_d8(),
            JP_ptrHL => unimplemented!(),
            LD_a16_A => self.ld_a16_r8(r8::A),
            BAD_5 => bad_inst!(),
            BAD_6 => bad_inst!(),
            BAD_7 => bad_inst!(),
            XOR_d8 => unimplemented!(),
            RST_28H => unimplemented!(),

            
            LDH_A_a8 => self.ldh_r8_a8(r8::A),
            POP_AF => self.pop_r16(r16::AF),
            LDH_A_C => self.ldh_r8_c(r8::A),
            DI => unimplemented!(),
            BAD_8 => bad_inst!(),
            PUSH_AF => self.push_r16(r16::AF),
            OR_d8 => unimplemented!(),
            RST_30H => unimplemented!(),

            LD_HL_SPpd8 => unimplemented!(),
            LD_SP_HL => self.ld_sp_hl(),
            LD_A_a16 => unimplemented!(),
            EI => unimplemented!(),
            BAD_9 => bad_inst!(),
            BAD_a => bad_inst!(),
            CP_d8 => unimplemented!(),
            RST_38H => unimplemented!(),
        }
    }

    fn cycle(&mut self, count: u64) {
        debug_assert_eq!(count % 4, 0);
        // The GameBoy processor ops all take an amount of time that is a multiple of 4
        // Some sources use the cycle count / 4 instead, so if you see a resource that
        // says some methods have a time of 1 or 2, that's why
        self.cycle_count += count;
    }

    fn read_next_d8(&mut self) -> d8 {
        let val = self.memory.read_d8(self.stack_pointer.into())
            .unwrap_or(d8::ZERO);
        self.stack_pointer += 1;
        val
    }

    fn read_next_d16(&mut self) -> d16 {
        let val = self.memory.read_d16(self.stack_pointer.into())
            .unwrap_or(d16::ZERO);
        self.stack_pointer += 2;
        val
    }

    fn read_d16_from_ptrr16(&self, reg: r16) -> d16 {
        let val = self.memory.read_d16(self.gp_registers[reg].into())
            .unwrap_or(d16::ZERO);
        val
    }

    fn write_d16_to_ptrr16(&mut self, reg: r16, val: d16) {
        self.memory.put_d16(self.gp_registers[reg].into(), val);
    }

    fn read_d8_from_ptrr16(&self, reg: r16) -> d8 {
        let val = self.memory.read_d8(self.gp_registers[reg].into())
            .unwrap_or(d8::ZERO);
        val
    }

    fn write_d8_to_ptrr16(&mut self, reg: r16, val: d8) {
        self.memory.put_d8(self.gp_registers[reg].into(), val);
    }
    
    fn set_carry(&mut self, value: bool) {
        self.gp_registers.set_flag(Flags::C, value);
        self.cycle(4);
    }
    fn nop(&mut self) {
        self.cycle(4);
    }
}

#![allow(non_camel_case_types)]

use std::num::Wrapping;

#[cfg(test)]
mod test;

pub mod registers;
use self::registers::*;

use number_types::d8_type::d8;
use number_types::d16_type::d16;
use memory::Memory;

#[derive(Debug, Copy, Clone)]
pub enum CpuMode {
    DMG,
    MGB,
    CGB,
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
            
            JR_d8 => unimplemented!(),
            ADD_HL_DE => self.add_r16_r16(r16::HL, r16::DE),
            LD_A_ptrDE => self.ld_r8_ptrr16(r8::A, r16::DE),
            DEC_DE => self.dec_r16(r16::DE),
            INC_E => self.inc_r8(r8::E),
            DEC_E => self.dec_r8(r8::E),
            LD_E_d8 => self.ld_r8_d8(r8::E),
            RRA => self.rotate_right(r8::A),


            JR_NZ_r8 => unimplemented!(),
            LD_HL_d16 => self.ld_r16_d16(r16::HL),
            LD_ptrHL_A => self.ld_ptrr16_r8(r16::HL, r8::A),
            INC_HL => self.inc_r16(r16::HL),
            INC_H => self.inc_r8(r8::H),
            DEC_H => self.dec_r8(r8::H),
            LD_H_d8 => self.ld_r8_d8(r8::H),
            DAA => unimplemented!(),

            JR_Z_d8 => unimplemented!(),
            ADD_HL_HL => self.add_r16_r16(r16::HL, r16::HL),
            LD_A_ptrHL => self.ld_r8_ptrr16(r8::A, r16::HL),
            DEC_HL => self.dec_r16(r16::HL),
            INC_L => self.inc_r8(r8::L),
            DEC_L => self.dec_r8(r8::L),
            CPL => self.compliment_r8(r8::A),


            JR_NC_r8 => unimplemented!(),
            LD_SP_d16 => self.ld_sp_d16(),
            LD_HLm_A => unimplemented!(),
            INC_SP => self.inc_sp(),
            INC_ptrHL => unimplemented!(),
            DEC_ptrHL => unimplemented!(),
            LD_ptrHL_d8 => unimplemented!(),
            SCF => self.set_carry(true),

            JR_C_d8 => unimplemented!(),
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

            _ => unimplemented!(),
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
        let val = self.memory.read_d8(self.stack_pointer)
            .unwrap_or(d8::ZERO);
        self.stack_pointer += 1;
        val
    }

    fn read_next_d16(&mut self) -> d16 {
        let val = self.memory.read_d16(self.stack_pointer)
            .unwrap_or(d16::ZERO);
        self.stack_pointer += 2;
        val
    }

    fn ld_r8_r8(&mut self, target: r8, src: r8) {
        self.gp_registers[target] = self.gp_registers[src];
        self.cycle(4);
    }

    fn ld_r16_d16(&mut self, reg: r16) {
        self.gp_registers[reg] = self.read_next_d16();
        self.cycle(12);
    }

    fn ld_sp_d16(&mut self) {
        self.stack_pointer = self.read_next_d16();
        self.cycle(12);
    }

    fn ld_r8_d8(&mut self, reg: r8) {
        self.gp_registers[reg] = self.read_next_d8();
        self.cycle(8);
    }

    fn ld_a16_SP(&mut self) {
        let ptr = self.read_next_d16();
        let sp = self.stack_pointer;
        self.memory.put_d16(ptr, sp);
        self.cycle(20);
    }

    fn ld_r8_ptrr16(&mut self, target: r8, source: r16) {
        if let Some(val) = self.memory.read_d8(
            self.gp_registers[source]
        ) {
            self.gp_registers[target] = val;
        }
    }

    fn ld_ptrr16_r8(&mut self, target: r16, source: r8) {
        let val = self.gp_registers[source];
        let idx = self.gp_registers[target];
        self.memory.put_d8(idx, val);
    }

    fn set_carry(&mut self, value: bool) {
        self.gp_registers.set_flag(Flags::C, value);
        self.cycle(4);
    }

    fn inc_sp(&mut self) {
        self.stack_pointer += 1;
        self.cycle(8);
    }

    fn dec_sp(&mut self) {
        self.stack_pointer -= 1;
        self.cycle(8);
    }

    fn inc_r16(&mut self, reg: r16) {
        // for some reason, the inc/dec r16 instructions don't affect any flags
        // it's weird, but I'm not complaining
        self.gp_registers[reg] += 1;
        self.cycle(8);
    }

    fn dec_r16(&mut self, reg: r16) {
        // for some reason, the inc/dec r16 instructions don't affect any flags
        // it's weird, but I'm not complaining
        self.gp_registers[reg] -= 1;
        self.cycle(8);
    }

    fn inc_r8(&mut self, reg: r8) {
        let old_value: d8 = self.gp_registers[reg]; // We store this so we can compare it for flags
        let new_value = old_value + 1;
        self.gp_registers[reg] = new_value;

        let flags: [Option<bool>; 4] = [
            Some(new_value == 0),
            Some(false),
            Some(new_value.upper_nibble() != old_value.upper_nibble()),
            Some(new_value < old_value)
        ];

        self.gp_registers.set_maybe_flags(flags);
        
        self.cycle(4);
    }

    fn dec_r8(&mut self, reg: r8) {
        let old_value: d8 = self.gp_registers[reg];

        let new_value = old_value - 1;

        self.gp_registers[reg] = new_value;

        let flags: [Option<bool>; 4] = [
            Some(new_value == 0),
            Some(true),
            Some(new_value.upper_nibble() != old_value.upper_nibble()),
            Some(new_value > old_value)
        ];
        
        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }

    fn add_sp_into(&mut self, target: r16) {
        // strangely, the Z flag is unaffected by these operations
        // but the other three are used
        let lhs = self.gp_registers[target];
        let rhs = self.stack_pointer;

        let nibble_overflow = d16::check_nibble_overflow(rhs, lhs);        
        let (result, carry_flag) = d16::add_and_check_overflow(lhs, rhs);
        
        self.gp_registers[target] = result;

        let flags: [Option<bool>; 4] = [
            None,
            Some(false),
            Some(nibble_overflow),
            Some(carry_flag)
        ];
        self.gp_registers.set_maybe_flags(flags);
        self.cycle(8);
    }

    fn add_r16_r16(&mut self, target: r16, source: r16) {
        // strangely, the Z flag is unaffected by these operations
        // but the other three are used
        let lhs: d16 = self.gp_registers[target];
        let rhs: d16 = self.gp_registers[source];

        let nibble_overflow = d16::check_nibble_overflow(lhs, rhs);
        let (result, carry_flag) = d16::add_and_check_overflow(lhs, rhs);
        
        self.gp_registers[target] = result;

        let flags: [Option<bool>; 4] = [
            None,
            Some(false),
            Some(nibble_overflow),
            // I'm not actually sure if this is correct, but I'm assuming that the half-carry
            // on 16-bit ops cares about the LSB
            Some(carry_flag)
        ];

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(8);
    }

    fn add_r8_r8(&mut self, target: r8, source: r8) {
        let lhs: d8 = self.gp_registers[target];
        let rhs: d8 = self.gp_registers[source];

        let nibble_overflow = d8::check_nibble_overflow(lhs, rhs);
        let (result, carry_flag) = d8::add_and_check_overflow(lhs, rhs);

        self.gp_registers[target] = result;

        let flags: [Option<bool>; 4] = [
            Some(result == 0),
            Some(false),
            Some(nibble_overflow),
            Some(carry_flag)
        ];

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }

    fn ld_r16_r8(&mut self, target: r16, source: r8) {
        self.gp_registers[target] = self.gp_registers[source].into();
        self.cycle(8);
    }

    fn rotate_left_carry(&mut self, reg: r8) {
        // it seems kinda weird that rotations set the zero flag to f,
        // no matter the result of the rotation
        let mut flags: [Option<bool>; 4] = [Some(false); 4];
        flags[3] = Some((self.gp_registers[reg] & d8::HIGHEST_BIT_MASK) != 0);
        // carry occurs iff the most significant bit is a 1

        self.gp_registers[reg] <<= 1;
        
        self.gp_registers.set_maybe_flags(flags);
        
        self.cycle(4);
    }

    fn rotate_right_carry(&mut self, reg: r8) {
        let mut flags: [Option<bool>; 4] = [Some(false); 4];
        flags[3] = Some((self.gp_registers[reg] & d8::LOWEST_BIT_MASK) != 0);
        // carry occurs iff the least significant bit is a 1

        self.gp_registers[reg] >>= 1;

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }

    fn rotate_left(&mut self, reg: r8) {
        let mut flags: [Option<bool>; 4] = [Some(false); 4];
        flags[3] = Some((self.gp_registers[reg] & d8::HIGHEST_BIT_MASK) != 0);
        self.gp_registers[reg] <<= 1;
        self.gp_registers[reg] += self.gp_registers.get_flag(Flags::C) as u8;

        self.gp_registers.set_maybe_flags(flags);
        
        self.cycle(4);
    }

    fn rotate_right(&mut self, reg: r8) {
        let mut flags: [Option<bool>; 4] = [Some(false); 4];
        flags[3] = Some((self.gp_registers[reg] & d8::LOWEST_BIT_MASK) != 0);
        self.gp_registers[reg] >>= 1;
        self.gp_registers[reg] += (self.gp_registers.get_flag(Flags::C) as u8) << 7;

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }

    fn compliment_r8(&mut self, reg: r8) {
        let flags: [Option<bool>; 4] = [None, Some(true), Some(true), None];

        self.gp_registers.set_maybe_flags(flags);

        self.gp_registers[reg] = !(self.gp_registers[reg]);

        self.cycle(4);
    }

    fn nop(&mut self) {
        self.cycle(4);
    }
}

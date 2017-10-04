#![allow(non_camel_case_types)]

use std::num::Wrapping;

#[cfg(test)]
mod test;

pub mod registers;
use self::registers::*;

use number_types::d8_type::d8;
use number_types::d16_type::d16;

const HIGHEST_BIT_MASK: d16 = d16(Wrapping(0b1000000000000000));

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
        }
    }

    pub fn process_instruction(&mut self, ins: ::instructions::RawOpcode) {
        use instructions::RawOpcode::*;
        self.program_counter += d16(Wrapping(1)); // inc the program counter before doing work so that loading subsequent bytes will work
        match ins {
            NOP => self.nop(),
            LD_BC_d16 => unimplemented!(),
            LD_BC_A => self.ld_r16_r8(r16::BC, r8::A),
            INC_BC => self.inc_r16(r16::BC),
            INC_B => self.inc_r8(r8::B),
            DEC_B => self.dec_r8(r8::B),
            LD_B_d8 => unimplemented!(),
            RLCA => self.rotate_left_carry(r8::A),
            LD_a16_SP => unimplemented!(),
            ADD_HL_BC => self.add_r16_r16(r16::HL, r16::BC),
            LD_A_ptrBC => unimplemented!(),
            DEC_BC => self.dec_r16(r16::BC),
            _ => unimplemented!(),
        }
    }

    fn cycle(&mut self, count: u64) {
        self.cycle_count += count;
    }

    fn inc_r16(&mut self, reg: r16) {
        // for some reason, the inc/dec r16 instructions don't affect any flags
        // it's weird, but I'm not complaining
        self.gp_registers[reg] += d16(Wrapping(1));
        self.cycle(8);
    }

    fn dec_r16(&mut self, reg: r16) {
        self.gp_registers[reg] -= d16(Wrapping(1));
        self.cycle(8);
    }

    fn inc_r8(&mut self, reg: r8) {
        let old_value: d8 = self.gp_registers[reg];
        self.gp_registers[reg] += d8(Wrapping(1));
        let new_value: d8 = self.gp_registers[reg];

        self.gp_registers.set_flag(
            Flags::Z,
            new_value == d8(Wrapping(0))
        );

        self.gp_registers.set_flag(Flags::N, false);
        // this is an addition op, so N is false

        self.gp_registers.set_flag(
            Flags::H,
            new_value.upper_nibble() != old_value.upper_nibble()
            // half-carry occured if top nibbles do not match
        );

        self.gp_registers.set_flag(
            Flags::C,
            new_value < old_value
            // a carry in addition occurs on overflow, so the new value
            // will be less than the old one
        );
            
        self.cycle(4);
    }

    fn dec_r8(&mut self, reg: r8) {
        let old_value: d8 = self.gp_registers[reg];
        self.gp_registers[reg] -= d8(Wrapping(1));
        let new_value: d8 = self.gp_registers[reg];
        
        self.gp_registers.set_flag(
            Flags::Z,
            new_value == d8(Wrapping(0))
        );
        
        self.gp_registers.set_flag(Flags::N, true);
        // this is a subtraction op, so N is true
        
        self.gp_registers.set_flag(
            Flags::H,
            new_value.upper_nibble() != old_value.upper_nibble()
            // the half-carry flag is set if the top nibbles of the new
            //and old values do not match
        );
        
        self.gp_registers.set_flag(
            Flags::C,
            new_value > old_value
            // a carry in subtraction occurs on underflow, so the new value
            //will be greater than the old one
        );
        
        self.cycle(4);
    }

    fn add_r16_r16(&mut self, target: r16, source: r16) {
        // strangely, the Z flag is unaffected by these operations
        // but the other three are used
        let old_value: d16 = self.gp_registers[target];
        let to_add: d16 = self.gp_registers[source];
        self.gp_registers[target] += to_add;
        let new_value: d16 = self.gp_registers[target];

        self.gp_registers.set_flag(Flags::N, false);

        self.gp_registers.set_flag(
            Flags::H,
            new_value.lsb().upper_nibble() != old_value.lsb().upper_nibble()
        );

        self.gp_registers.set_flag(
            Flags::C,
            HIGHEST_BIT_MASK & new_value & old_value != d16(Wrapping(0))
            // addition will overflow iff the most significant bit
            // of each operand is 1
        );

        self.cycle(8);
    }

    fn ld_r16_r8(&mut self, target: r16, source: r8) {
        self.gp_registers[target] = self.gp_registers[source].into();
        self.cycle(8);
    }

    fn rotate_left_carry(&mut self, reg: r8) {
        let d8(Wrapping(tmp_value)) = self.gp_registers[reg];
        let mut tmp_value: u16 = tmp_value as _;
        tmp_value <<= 1;
        self.gp_registers.set_flag(Flags::C, (tmp_value >> 8) == 1);
        self.cycle(4);
    }

    fn nop(&mut self) {
        self.cycle(4);
    }
}
/*
impl ::std::ops::Index<r8> for Cpu {
    type Output = d8;

    fn index(&self, register: r8) -> &Self::Output {
        &self.gp_registers.0[register as usize]
    }
}

impl ::std::ops::IndexMut<r8> for Cpu {
    fn index_mut(&mut self, register: r8) -> &mut <Self as ::std::ops::Index<r8>>::Output {
        &mut self.gp_registers.0[register as usize]
    }
}
*/

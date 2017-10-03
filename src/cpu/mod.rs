#![allow(non_camel_case_types)]

use std::num::Wrapping;

#[cfg(test)]
mod test;

pub mod registers;
use self::registers::*;

use number_types::d8_type::d8;
use number_types::d16_type::d16;


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
            LD_BC_d16 => panic!("Unimplemented instruction"),
            LD_BC_A => self.ld_r16_r8(r16::BC, r8::A),
            INC_BC => self.inc_r16(r16::BC),
            INC_B => self.inc_r8(r8::B),
            DEC_B => self.dec_r8(r8::B),
            LD_B_d8 => panic!("Unimplemented instruction"),
            RLCA => self.rotate_left_carry(r8::A),
            _ => panic!("Unimplemented instruction"),
        }
    }

    fn inc_r16(&mut self, reg: r16) {
        self.gp_registers[reg] += d16(Wrapping(1));
        self.cycle_count += 8;
    }

    fn inc_r8(&mut self, reg: r8) {
        self.gp_registers[reg] += d8(Wrapping(1));
        self.cycle_count += 4;
    }

    fn dec_r8(&mut self, reg: r8) {
        self.gp_registers[reg] -= d8(Wrapping(1));
        self.cycle_count += 4;
    }

    fn ld_r16_r8(&mut self, target: r16, source: r8) {
        self.gp_registers[target] = self.gp_registers[source].into();
        self.cycle_count += 8;
    }

    fn rotate_left_carry(&mut self, reg: r8) {
        let d8(Wrapping(tmp_value)) = self.gp_registers[reg];
        let mut tmp_value: u16 = tmp_value as _;
        tmp_value <<= 1;
        self.gp_registers.set_flag(Flags::C, (tmp_value >> 8) == 1);
        self.cycle_count += 4;
    }

    fn nop(&mut self) {
        self.cycle_count += 4;
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

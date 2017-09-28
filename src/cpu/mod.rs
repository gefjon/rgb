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
        }
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

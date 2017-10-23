#![allow(non_snake_case, non_camel_case_types, unused_variables)]

use number_types::d8_type::d8;
use number_types::d16_type::d16;
use super::CpuMode;
use std::num::Wrapping;
use std::ops::{Index, IndexMut};

pub struct Registers {
    pub registers: [d16; 4]
}

impl Registers {
    pub fn new(mode: CpuMode) -> Self {
        let AF: d16 = match mode {
            CpuMode::DMG => d16(Wrapping(0x01b0)),
            CpuMode::MGB => d16(Wrapping(0xffb0)),
            CpuMode::CGB => d16(Wrapping(0x1180)),
        };
        let BC: d16 = match mode {
            CpuMode::CGB => d16(Wrapping(0x0000)),
            _ => d16(Wrapping(0x0013)),
        };
        let DE: d16 = match mode {
            CpuMode::CGB => d16(Wrapping(0x0008)),
            _ => d16(Wrapping(0x00d8)),
        };
        let HL: d16 = match mode {
            CpuMode::CGB => d16(Wrapping(0x007c)),
            _ => d16(Wrapping(0x014d)),
        };
        Registers { registers: [AF, BC, DE, HL] }
    }

    fn flags_register_mut(&mut self) -> &mut d8 {
        let reg_8s: &mut [d8; 8] = unsafe {
            ::std::mem::transmute(&mut self.registers)
        };
        &mut reg_8s[r8::F as usize]
    }

    pub fn flags_register(&self) -> &d8 {
        let reg_8s: &[d8; 8] = unsafe {
            ::std::mem::transmute(&self.registers)
        };
        &reg_8s[r8::F as usize]
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        let mut flags: &mut d8 = self.flags_register_mut();
        let value = Wrapping(value as u8);
        let flag = flag as usize;
        flags.0 = flags.0 & !(Wrapping(1u8) << flag) | (value << flag);
    }

    pub fn set_maybe_flags(&mut self, flags: [Option<bool>; 4]) {
        // this method takes the flags in most significant to least significant
        // order, meaning: [Z, N, H, C]
        if let Some(val) = flags[0] {
            self.set_flag(Flags::Z, val);
        }
        if let Some(val) = flags[1] {
            self.set_flag(Flags::N, val);
        }
        if let Some(val) = flags[2] {
            self.set_flag(Flags::H, val);
        }
        if let Some(val) = flags[3] {
            self.set_flag(Flags::C, val);
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        let d8(Wrapping(val)) = *self.flags_register() & d8(Wrapping(1 << flag as usize));
        val > 0
    }

    pub fn check_condition(&self, cond: Conditions) -> bool {
        match cond {
            Conditions::Z => self.get_flag(Flags::Z),
            Conditions::NZ => !(self.get_flag(Flags::Z)),
            Conditions::C => self.get_flag(Flags::C),
            Conditions::NC => !(self.get_flag(Flags::C)),
        }
    }
}

impl Index<r16> for Registers {
    type Output = d16;

    fn index(&self, index: r16) -> &<Self as Index<r16>>::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<r16> for Registers {
    fn index_mut(&mut self, index: r16) -> &mut <Self as Index<r16>>::Output {
        &mut self.registers[index as usize]
    }
}

impl Index<r8> for Registers {
    type Output = d8;

    fn index(&self, index: r8) -> &<Self as Index<r8>>::Output {
        debug_assert_ne!(index, r8::F); // F is not a valid single register
        
        let reg_8s: &[d8; 8] = unsafe {
            ::std::mem::transmute(&self.registers)
        };
        &reg_8s[index as usize]
    }
}

impl IndexMut<r8> for Registers {
    fn index_mut(&mut self, index: r8) -> &mut <Self as Index<r8>>::Output {
        debug_assert_ne!(index, r8::F);

        let reg_8s: &mut [d8; 8] = unsafe {
            ::std::mem::transmute(&mut self.registers)
        };
        &mut reg_8s[index as usize]
    }
}

#[allow(warnings)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum r8 { // 8-bit registers
    // note the ordering of these: the register that is written first
    // in the pair is the more significant byte, but the GB
    // (and modern PCs, more importantly) are little-endian
    // so the more-significant bytes come later
    F, // Flags
    A, // the Accumulator
    C, // General-purpose registers
    B,
    E,
    D,
    L,
    H,
}

#[allow(warnings)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum r16 { // 16-bit registers
    AF, // Acc and flags
    BC, // General-purpose registers
    DE,
    HL,
}

#[allow(warnings)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Flags {
    // src: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
    Z = 0x7, // set when a math operation results in 0 or by the CP instruction
    N = 0x6, // set if the last math instruction was subtraction
    H = 0x5, // set if a carry occurred from the lower nibble in the last math operation
    C = 0x4, // set if a carry occurred from the last math operation or if A is the smaller value when using the CP instruction
}

#[allow(warnings)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Conditions {
    Z,
    NZ,
    C,
    NC,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn index_with_r8() {
        let registers = Registers::new(super::CpuMode::DMG);
        assert_eq!(
            registers[r8::A],
            d8(Wrapping(0x01)),
            "failed to index r8::A"
        );
    }

    #[test]
    fn index_flag() {
        let registers = Registers::new(super::CpuMode::DMG);
        assert!(registers.get_flag(Flags::Z));
        assert!(!(registers.get_flag(Flags::N)));
        assert!(registers.get_flag(Flags::H));
        assert!(registers.get_flag(Flags::C));
    }

    #[test]
    fn flags_in_right_places() {
        let mut registers = Registers::new(super::CpuMode::DMG);
        registers.set_flag(Flags::N, true);
        assert_eq!(*registers.flags_register(), d8(Wrapping(0b11110000)));
    }

    #[test]
    fn assign_flag() {
        let mut registers = Registers::new(super::CpuMode::DMG);
        assert!(!(registers.get_flag(Flags::N)));
        registers.set_flag(Flags::N, true);
        assert!(registers.get_flag(Flags::N));

        assert!(registers.get_flag(Flags::C));
        registers.set_flag(Flags::C, false);
        assert!(!(registers.get_flag(Flags::C)));
    }
}

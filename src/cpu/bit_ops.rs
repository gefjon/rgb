use super::Cpu;
use super::registers::*;
use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;

pub trait BitInstructions {
    type Register_8;
    fn and_r8(&mut self, source: Self::Register_8);
    fn xor_r8(&mut self, source: Self::Register_8);
    fn or_r8(&mut self, source: Self::Register_8);
    fn rotate_left_carry(&mut self, reg: r8);
    fn rotate_right_carry(&mut self, reg: r8);
    fn rotate_left(&mut self, reg: Self::Register_8);
    fn rotate_right(&mut self, reg: Self::Register_8);
    fn compliment_r8(&mut self, reg: Self::Register_8);
}

impl BitInstructions for Cpu {
    type Register_8 = r8;
    fn and_r8(&mut self, source: r8) {
        let lhs: d8 = self.gp_registers[r8::A];
        let rhs: d8 = self.gp_registers[source];

        let result = lhs & rhs;

        self.gp_registers[r8::A] = result;

        let flags: [Option<bool>; 4] = [
            Some(result == 0),
            Some(false),
            Some(true),
            Some(false)
        ];

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }
    fn xor_r8(&mut self, source: r8) {
        let lhs: d8 = self.gp_registers[r8::A];
        let rhs: d8 = self.gp_registers[source];

        let result = lhs ^ rhs;

        self.gp_registers[r8::A] = result;

        let flags: [Option<bool>; 4] = [
            Some(result == 0),
            Some(false),
            Some(false),
            Some(false)
        ];

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }
    fn or_r8(&mut self, source: r8) {
        let lhs: d8 = self.gp_registers[r8::A];
        let rhs: d8 = self.gp_registers[source];

        let result = lhs | rhs;

        self.gp_registers[r8::A] = result;

        let flags: [Option<bool>; 4] = [
            Some(result == 0),
            Some(false),
            Some(false),
            Some(false)
        ];

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
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
}

use super::Cpu;
use super::registers::*;
use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;

pub trait Subtraction {
    type Register_8;
    type Register_16;
    fn sub_r8(&mut self, source: Self::Register_8);
    fn sbc_r8(&mut self, source: Self::Register_8);
    fn cp_r8(&mut self, source: Self::Register_8);
}

impl Subtraction for Cpu {
    type Register_8 = r8;
    type Register_16 = r16;
    fn sub_r8(&mut self, source: r8) {
        let lhs: d8 = self.gp_registers[r8::A];
        let rhs: d8 = self.gp_registers[source];

        let nibble_overflow = d8::sub_nibble_overflow(lhs, rhs);
        let (result, carry_flag) = d8::sub_and_check_overflow(lhs, rhs);

        self.gp_registers[r8::A] = result;

        let flags: [Option<bool>; 4] = [
            Some(result == 0),
            Some(true),
            Some(nibble_overflow),
            Some(carry_flag)
        ];

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }
    fn sbc_r8(&mut self, source: r8) {
        let lhs: d8 = self.gp_registers[r8::A];
        let rhs: d8 = self.gp_registers[source] +
            ((self.gp_registers.get_flag(Flags::C) as u8) << 7);

        let nibble_overflow = d8::sub_nibble_overflow(lhs, rhs);
        let (result, carry_flag) = d8::sub_and_check_overflow(lhs, rhs);

        self.gp_registers[r8::A] = result;

        let flags: [Option<bool>; 4] = [
            Some(result == 0),
            Some(true),
            Some(nibble_overflow),
            Some(carry_flag)
        ];

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }
    fn cp_r8(&mut self, source: r8) {
        let lhs: d8 = self.gp_registers[r8::A];
        let rhs: d8 = self.gp_registers[source];

        let nibble_overflow = d8::sub_nibble_overflow(lhs, rhs);
        let (result, carry_flag) = d8::sub_and_check_overflow(lhs, rhs);

        let flags: [Option<bool>; 4] = [
            Some(result == 0),
            Some(true),
            Some(nibble_overflow),
            Some(carry_flag)
        ];

        self.gp_registers.set_maybe_flags(flags);

        self.cycle(4);
    }
}

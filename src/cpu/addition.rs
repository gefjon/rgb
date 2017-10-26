use super::Cpu;
use super::registers::*;
use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;

pub trait Addition {
    type Register_8;
    type Register_16;
    fn add_sp_d8(&mut self);
    fn add_r16_r16(&mut self, target: Self::Register_16, source: Self::Register_16);
    fn add_r8_r8(&mut self, target: Self::Register_8, source: Self::Register_8);
    fn adc_r8_r8(&mut self, target: Self::Register_8, source: Self::Register_8);
    fn add_sp_into(&mut self, target: Self::Register_16);
}

impl Addition for Cpu {
    type Register_8 = r8;
    type Register_16 = r16;
    fn add_sp_d8(&mut self) {
        let rhs: d8 = self.read_next_d8();
        let lhs: a16 = self.stack_pointer.into();
        let (result, carry_flag, nibble_carry) = a16::add_and_check_carry(lhs, rhs);

        let flags: [Option<bool>; 4] = [
            Some(false),
            Some(false),
            Some(nibble_carry),
            Some(carry_flag)
        ];
        self.stack_pointer = result.into();
        self.cycle(16);
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
    fn adc_r8_r8(&mut self, target: r8, source: r8) {
        let lhs: d8 = self.gp_registers[target];
        let rhs: d8 = self.gp_registers[source] +
            (self.gp_registers.get_flag(Flags::C) as u8);

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
}

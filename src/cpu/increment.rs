use super::Cpu;
use super::registers::*;
use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;

pub trait Increment {
    type Register_8;
    type Register_16;
    fn inc_sp(&mut self);
    fn dec_sp(&mut self);
    fn inc_r16(&mut self, reg: Self::Register_16);
    fn dec_r16(&mut self, reg: Self::Register_16);
    fn inc_r8(&mut self, reg: Self::Register_8);
    fn dec_r8(&mut self, reg: Self::Register_8);
}

impl Increment for Cpu {
    type Register_8 = r8;
    type Register_16 = r16;
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
}

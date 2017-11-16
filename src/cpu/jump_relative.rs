use super::Cpu;
use super::registers::*;
use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;

pub trait JumpRelative {
    fn jr_d8(&mut self);
    fn jr_cond_d8(&mut self, cond: Conditions);
    fn jump_relative(&mut self, ptr: d8);
}

impl JumpRelative for Cpu {
    fn jr_d8(&mut self) {
        let ptr = self.read_next_d8();
        self.stack_pointer -= 2;
        self.jump_relative(ptr);
        self.cycle(4);
    }

    fn jr_cond_d8(&mut self, cond: Conditions) {
        let ptr = self.read_next_d8();
        if self.gp_registers.check_condition(cond) {
            self.jump_relative(ptr);
        }
        self.cycle(8);
    }

    fn jump_relative(&mut self, ptr: d8) {
        let mut sp: a16 = self.stack_pointer.into();
        sp += ptr;
        self.stack_pointer = sp.into();
        self.cycle(4);
    }
}

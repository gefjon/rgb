use super::Cpu;
use super::registers::*;
use number_types::a16_type::a16;

pub trait Jump {
    fn jp_a16(&mut self);
    fn jp_cond_a16(&mut self, cond: Conditions);
    fn jump_absolute(&mut self, ptr: a16);
    fn jp_ptrr16(&mut self, reg: r16);
}

impl Jump for Cpu {
    fn jp_a16(&mut self) {
        let ptr: a16 = self.read_next_d16().into();
        self.jump_absolute(ptr);
        self.cycle(16);
    }
    fn jp_cond_a16(&mut self, cond: Conditions) {
        let ptr: a16 = self.read_next_d16().into();
        if self.gp_registers.check_condition(cond) {
            self.jump_absolute(ptr);
            self.cycle(16);
        } else {
            self.cycle(12);
        }
    }
    fn jump_absolute(&mut self, ptr: a16) {
        self.stack_pointer = ptr.into();
    }
    fn jp_ptrr16(&mut self, reg: r16) {
        let ptr: a16 = self.read_d16_from_ptrr16(reg).into();
        self.jump_absolute(ptr);
        self.cycle(4)
    }
}

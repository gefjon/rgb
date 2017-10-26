use super::Cpu;
use super::registers::*;
use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;
use super::stack::Stack;

pub trait FunCall: Stack {
    fn call_addr(&mut self, addr: Self::Pointer_16);
    fn call(&mut self);
    fn call_if(&mut self, cond: Conditions);
    fn ret(&mut self);
    fn return_if(&mut self, cond: Conditions);
}

impl FunCall for Cpu {
    fn call_addr(&mut self, addr: a16) {
        let sp = self.stack_pointer;
        self.d16_to_stack(sp);
        self.stack_pointer = addr.into();
    }

    fn call(&mut self) {
        let address: a16 = self.read_next_d16().into();
        self.call_addr(address);
        self.cycle(24);
    }

    fn call_if(&mut self, cond: Conditions) {
        let address: a16 = self.read_next_d16().into();
        if self.gp_registers.check_condition(cond) {
            self.call_addr(address);
            self.cycle(24);
        } else {
            self.cycle(12);
        }
    }

    fn ret(&mut self) {
        let address = self.d16_from_stack();
        self.stack_pointer = address;
        self.cycle(16);
    }

    fn return_if(&mut self, cond: Conditions) {
        if self.gp_registers.check_condition(cond) {
            self.ret();
            self.cycle(4);
            // RET cycles 16, and these methods take 20 cycles
            // when they return, or 8 otherwise
            // because I wanted to use the same method to return,
            // cycle the extra 4 here
        } else {
            self.cycle(8);
        }
    }
}

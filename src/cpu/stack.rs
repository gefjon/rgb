use super::Cpu;
use super::registers::*;
use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;

pub trait Stack {
    type Pointer_16;
    type Pointer_8;
    type Int_16;
    type Int_8;
    type Register_16;
    type Register_8;

    fn d16_from_stack(&mut self) -> Self::Int_16;
    fn d16_to_stack(&mut self, val: Self::Int_16);
    fn d8_from_stack(&mut self) -> Self::Int_8;
    fn d8_to_stack(&mut self, val: Self::Int_8);
    fn pop_r16(&mut self, target: Self::Register_16);
    fn push_r16(&mut self, source: Self::Register_16);
}

impl Stack for Cpu {
    type Pointer_16 = a16;
    type Pointer_8 = a8;
    type Int_16 = d16;
    type Int_8 = d8;
    type Register_16 = r16;
    type Register_8 = r8;
    fn d16_from_stack(&mut self) -> d16 {
        // the GB stack lives at a very positive address
        // and grows towards 0
        let ret_val = self.memory.read_d16(self.stack_pointer.into()).unwrap();
        self.stack_pointer += 2;
        ret_val
    }

    fn d16_to_stack(&mut self, val: d16) {
        // the GB stack lives at a very positive address
        // and grows towards 0
        let _ = self.memory.put_d16(self.stack_pointer.into(), val);
        self.stack_pointer -= 2;
    }

    fn d8_from_stack(&mut self) -> d8 {
        let ret_val = self.memory.read_d8(self.stack_pointer.into()).unwrap();
        self.stack_pointer += 1;
        ret_val
    }

    fn d8_to_stack(&mut self, val: d8) {
        let _ = self.memory.put_d8(self.stack_pointer.into(), val);
        self.stack_pointer -= 1;
    }
    
    fn pop_r16(&mut self, target: r16) {
        self.gp_registers[target] = self.d16_from_stack();
        self.cycle(12);
    }

    fn push_r16(&mut self, source: r16) {
        // why do the push/pop ops not take the same amount of time?
        let val = self.gp_registers[source];
        self.d16_to_stack(val);
        self.cycle(16);
    }
}

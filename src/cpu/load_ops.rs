use super::Cpu;
use super::registers::*;
use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use number_types::a8_type::a8;
use std::num::Wrapping;

pub trait Ld {
    type Register_8;
    type Register_16;
    fn ldh_a8_r8(&mut self, source: Self::Register_8);
    fn ldh_r8_a8(&mut self, target: Self::Register_8);
    fn ldh_c_r8(&mut self, source: Self::Register_8);
    fn ldh_r8_c(&mut self, target: Self::Register_8);
    fn ld_a16_r8(&mut self, source: Self::Register_8);
    fn ld_r8_ptrr16(&mut self, target: Self::Register_8, source: Self::Register_16);
    fn ld_ptrr16_r8(&mut self, target: Self::Register_16, target: Self::Register_8);
    fn ld_a16_SP(&mut self);
    fn ld_r8_r8(&mut self, target: Self::Register_8, source: Self::Register_8);
    fn ld_r16_d16(&mut self, target: Self::Register_16);
    fn ld_sp_d16(&mut self);
    fn ld_sp_hl(&mut self);
    fn ld_r8_d8(&mut self, target: Self::Register_8);
    fn ld_r16_r8(&mut self, target: Self::Register_16, source: Self::Register_8);
}

impl Ld for Cpu {
    type Register_8 = r8;
    type Register_16 = r16;
    fn ldh_a8_r8(&mut self, source: r8) {
        let adr: a8 = self.read_next_d8().into();
        let val = self.gp_registers[source];
        let _ = self.memory.put_d8(adr.into(), val);
        self.cycle(12);
    }
    fn ldh_r8_a8(&mut self, target: r8) {
        let adr: a8 = self.read_next_d8().into();
        let val: d8 = self.memory.read_d8(adr.into()).unwrap_or(d8::ZERO);
        self.gp_registers[target] = val;
        self.cycle(12);
    }
    fn ldh_c_r8(&mut self, source: r8) {
        let adr: a8 = a8(Wrapping(self.gp_registers.get_flag(Flags::C) as u8));
        let val = self.gp_registers[source];
        let _ = self.memory.put_d8(adr.into(), val);
        self.cycle(8);
    }
    fn ldh_r8_c(&mut self, target: r8) {
        let adr: a8 = a8(Wrapping(self.gp_registers.get_flag(Flags::C) as u8));
        let val = self.memory.read_d8(adr.into()).unwrap_or(d8::ZERO);
        self.gp_registers[target] = val;
        self.cycle(8);
    }
    fn ld_r8_ptrr16(&mut self, target: r8, source: r16) {
        if let Some(val) = self.memory.read_d8(
            self.gp_registers[source].into()
        ) {
            self.gp_registers[target] = val;
        }
    }
    fn ld_ptrr16_r8(&mut self, target: r16, source: r8) {
        let val = self.gp_registers[source];
        let idx = self.gp_registers[target];
        self.memory.put_d8(idx.into(), val);
    }
        fn ld_a16_SP(&mut self) {
        let ptr = self.read_next_d16();
        let sp = self.stack_pointer;
        self.memory.put_d16(ptr.into(), sp);
        self.cycle(20);
    }

    fn ld_a16_r8(&mut self, reg: r8) {
        let ptr = self.read_next_d16();
        let val = self.gp_registers[reg];
        self.memory.put_d8(ptr.into(), val);
        self.cycle(16);
    }
        fn ld_r8_r8(&mut self, target: r8, src: r8) {
        self.gp_registers[target] = self.gp_registers[src];
        self.cycle(4);
    }

    fn ld_r16_d16(&mut self, reg: r16) {
        self.gp_registers[reg] = self.read_next_d16();
        self.cycle(12);
    }

    fn ld_sp_d16(&mut self) {
        self.stack_pointer = self.read_next_d16();
        self.cycle(12);
    }

    fn ld_sp_hl(&mut self) {
        self.stack_pointer = self.gp_registers[r16::HL];
        self.cycle(4);
    }

    fn ld_r8_d8(&mut self, reg: r8) {
        self.gp_registers[reg] = self.read_next_d8();
        self.cycle(8);
    }
    
    fn ld_r16_r8(&mut self, target: r16, source: r8) {
        self.gp_registers[target] = self.gp_registers[source].into();
        self.cycle(8);
    }
}

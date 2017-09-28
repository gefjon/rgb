use super::{Cpu, d8, d16};
use super::registers::*;
use std::num::Wrapping;

#[test]
fn make_a_cpu() {
    let cpu = Cpu {
        gp_registers: Registers { registers: [d16(Wrapping(0)); 4] },
        stack_pointer: d16(Wrapping(0)),
        program_counter: d16(Wrapping(0)),
    };
}

#[test]
fn index_an_r16() {
    let cpu = Cpu {
        gp_registers: Registers { registers: [d16(Wrapping(0)); 4] },
        stack_pointer: d16(Wrapping(0)),
        program_counter: d16(Wrapping(0)),
    };

    assert_eq!(cpu.gp_registers[r16::AF], d16(Wrapping(0)));
}

#[test]
fn initial_register_values_dmg() {
    let cpu = Cpu::new(super::CpuMode::DMG);
    assert_eq!(cpu.gp_registers[r16::AF], d16(Wrapping(0x01b0)));
    assert_eq!(cpu.gp_registers[r16::BC], d16(Wrapping(0x0013)));
    assert_eq!(cpu.gp_registers[r16::DE], d16(Wrapping(0x00d8)));
    assert_eq!(cpu.gp_registers[r16::HL], d16(Wrapping(0x014d)));
    assert_eq!(cpu.stack_pointer, d16(Wrapping(0xfffe)));
    assert_eq!(cpu.program_counter, d16(Wrapping(0x0100)));
}

#[test]
fn initial_register_values_mgb() {
    let cpu = Cpu::new(super::CpuMode::MGB);
    assert_eq!(cpu.gp_registers[r16::AF], d16(Wrapping(0xffb0)));
    assert_eq!(cpu.gp_registers[r16::BC], d16(Wrapping(0x0013)));
    assert_eq!(cpu.gp_registers[r16::DE], d16(Wrapping(0x00d8)));
    assert_eq!(cpu.gp_registers[r16::HL], d16(Wrapping(0x014d)));
    assert_eq!(cpu.stack_pointer, d16(Wrapping(0xfffe)));
    assert_eq!(cpu.program_counter, d16(Wrapping(0x0100)));
}

#[test]
fn initial_register_values_cgb() {
    let cpu = Cpu::new(super::CpuMode::CGB);
    assert_eq!(cpu.gp_registers[r16::AF], d16(Wrapping(0x1180)));
    assert_eq!(cpu.gp_registers[r16::BC], d16(Wrapping(0x0000)));
    assert_eq!(cpu.gp_registers[r16::DE], d16(Wrapping(0x0008)));
    assert_eq!(cpu.gp_registers[r16::HL], d16(Wrapping(0x007c)));
    assert_eq!(cpu.stack_pointer, d16(Wrapping(0xfffe)));
    assert_eq!(cpu.program_counter, d16(Wrapping(0x0100)));
}

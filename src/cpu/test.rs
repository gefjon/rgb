use super::{Cpu, d8, d16};
use super::registers::*;
use std::num::Wrapping;

#[test]
fn make_a_cpu() {
    let cpu = Cpu {
        gp_registers: Registers { registers: [d16(Wrapping(0)); 4] },
        stack_pointer: d16(Wrapping(0)),
        program_counter: d16(Wrapping(0)),
        cycle_count: 0,
    };
}

#[test]
fn index_an_r16() {
    let cpu = Cpu {
        gp_registers: Registers { registers: [d16(Wrapping(0)); 4] },
        stack_pointer: d16(Wrapping(0)),
        program_counter: d16(Wrapping(0)),
        cycle_count: 0,
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


#[test]
fn process_basic_instruction() {
    let mut cpu = Cpu::new(super::CpuMode::DMG);
    cpu.process_instruction(::instructions::RawOpcode::INC_BC);
    assert_eq!(cpu.gp_registers[r16::BC], d16(Wrapping(0x0013 + 1)));
    assert_eq!(cpu.gp_registers[r8::C], d8(Wrapping(0x13 + 1)));
    assert_eq!(cpu.gp_registers[r8::B], d8(Wrapping(0x00)));
}

#[test]
fn inc_r8s() {
    let mut cpu = Cpu::new(super::CpuMode::DMG);
    cpu.process_instruction(::instructions::RawOpcode::INC_B);
    assert_eq!(cpu.gp_registers[r8::B], d8(Wrapping(0x00 + 1)));
    assert_eq!(cpu.gp_registers[r16::BC], d16(Wrapping(0x0013 + 0x0100)));
    cpu.process_instruction(::instructions::RawOpcode::DEC_B);
    assert_eq!(cpu.gp_registers[r8::B], d8(Wrapping(0x00)));
    assert_eq!(cpu.gp_registers[r16::BC], d16(Wrapping(0x0013)));
}

#[test]
fn flags_from_dec_r8() {
    let mut cpu = Cpu::new(super::CpuMode::DMG);
    // the initial value of B in DMG mode is 0x00
    // so after a dec, Carry, Half-carry, and Nsubtraction should be t
    // but Zero will be f
    cpu.process_instruction(::instructions::RawOpcode::DEC_B);
    assert_eq!(*cpu.gp_registers.flags_register(), d8(Wrapping(0b01110000)));
}

#[test]
fn flags_from_inc_r8() {
    let mut cpu = Cpu::new(super::CpuMode::DMG);
    // the initial value of B in DMG mode is 0x00
    // so after an inc, all flags will be false
    cpu.process_instruction(::instructions::RawOpcode::INC_B);
    assert_eq!(*cpu.gp_registers.flags_register(), d8(Wrapping(0b00000000)));
}

#[test]
fn mov_between_register_instructions() {
    let mut cpu = Cpu::new(super::CpuMode::DMG);
    cpu.process_instruction(::instructions::RawOpcode::LD_BC_A);
    assert_eq!(cpu.gp_registers[r16::BC], d16(Wrapping(0x0001)));
}

#[test]
fn addition_instructions() {
    let mut cpu = Cpu::new(super::CpuMode::DMG);
    cpu.process_instruction(::instructions::RawOpcode::ADD_HL_BC);
    assert_eq!(
        cpu.gp_registers[r16::HL],
        d16(Wrapping(0x014du16) + Wrapping(0x0013u16))
    );
}

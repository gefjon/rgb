#![allow(non_snake_case)]
enum r8 { // 8-bit registers
    A, // the Accumulator
    F, // Flags
    B, // General-purpose registers
    C,
    D,
    E,
    H,
    L,
}

#![allow(non_snake_case)]
enum r16 { // 16-bit registers
    AF, // General-purpose registers
    BC,
    DE,
    HL,
    SP, // the Stack Pointer
    PC, // the Program Counter
}

#![allow(non_snake_case)]
enum Flags {
    // src: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
    Z = 0x7, // set when a math operation results in 0 or by the CP instruction
    N = 0x6, // set if the last math instruction was subtraction
    H = 0x5, // set if a carry occurred from the lower nibble in the last math operation
    C = 0x4, // set if a carry occurred from the last math operation or if A is the smaller value when using the CP instruction
}

#![allow(non_snake_case)]
enum RawOpcode {
    // src: http://pastraiser.com/cpu/gameboy/gameboy_opcodes.html
    NOP = 0x00, // No-op
    LD_BC_d16 = 0x01, // Load the next two bytes into BC
    LD_BC_A = 0x02, // Load the value in A into BC
    INC_BC = 0x03, // Increment BC by 1
    INC_B = 0x04, // Increment B by 1
    DEC_B = 0x05, // Decrement B by 1
    LD_B_d8 = 0x06, // Load the next byte into B
    RLCA = 0x07, // Rotate register A left by 1 bit, treating the C flag as the leftmost bit
    LD_a16_SP = 0x08, // Store SP into addresses a16 (LSB) and a16+1 (MSB)
    ADD_HL_BC = 0x09, // Add the value in BC to HL
    LD_A_BC = 0x0a, // Treat BC as a pointer and load the value from memory into A
    DEC_BC = 0x0b, // Decrement BC by 1
    INC_C = 0x0c, // Increment C by 1
    DEC_C = 0x0d, // Decrement C by 1
    LD_C_d8 = 0x0e, // Load the next byte into C
    RRCA = 0x0f, // Rotate A right by 1 bit, treating the C flag as the rightmost bit
}

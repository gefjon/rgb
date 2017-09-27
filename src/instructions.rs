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
    //                             len cycles| z n h c | description
    NOP = 0x00, //                  1     4  | - - - - | No-op
    LD_BC_d16 = 0x01, //            3    12  | - - - - | Load the next two bytes into BC
    LD_BC_A = 0x02, //              1     8  | - - - - | Load the value in A into BC
    INC_BC = 0x03, //               1     8  | - - - - | Increment BC by 1
    INC_B = 0x04, //                1     4  | Z 0 H - | Increment B by 1
    DEC_B = 0x05, //                1     4  | Z 1 H - | Decrement B by 1
    LD_B_d8 = 0x06, //              2     8  | - - - - | Load the next byte into B
    RLCA = 0x07, //                 1     4  | 0 0 0 C | Rotate register A left by 1 bit, treating the C flag as the leftmost bit
    LD_a16_SP = 0x08, //            3    20  | - - - - | Store SP into addresses a16 (LSB) and a16+1 (MSB)
    ADD_HL_BC = 0x09, //            1     8  | - 0 H C | Add the value in BC to HL
    LD_A_BC = 0x0a, //              1     8  | - - - - | Treat BC as a pointer and load the value from memory into A
    DEC_BC = 0x0b, //               1     8  | - - - - | Decrement BC by 1
    INC_C = 0x0c, //                1     4  | Z 0 H - | Increment C by 1
    DEC_C = 0x0d, //                1     4  | Z 1 H - | Decrement C by 1
    LD_C_d8 = 0x0e, //              2     8  | - - - - | Load the next byte into C
    RRCA = 0x0f, //                 1     4  | 0 0 0 C | Rotate A right by 1 bit, treating the C flag as the rightmost bit
    
    STOP_0 = 0x10, //               2     4  | - - - - | Enter low-power mode. Next byte is expected to be 0x00
    LD_DE_d16 = 0x11, //            3    12  | - - - - | Load the next two bytes into DE
    LD_DE_A = 0x12, //              1     8  | - - - - | Load the value in A into BC
    INC_DE = 0x13, //               1     4  | - - - - | Increment DE by 1
    INC_D = 0x14, //                1     4  | Z 0 H - | Increment D by 1
    DEC_D = 0x15, //                1     4  | Z 1 H - | Decrement D by 1
    LD_D_d8 = 0x16, //              2     8  | - - - - | Load the next byte into D
    RLA = 0x17, //                  1     4  | 0 0 0 C | Rotate A left by 1 bit, moving the C flag into the rightmost bit and the leftmost into the C flag
    JR_d8 = 0x18, //                2     8  | - - - - | With the next byte as a signed int, add it to the current address and jump to it
    ADD_HL_DE = 0x19, //            1     8  | - 0 H C | Add the value in DE to HL
    LD_A_DE = 0x1a, //              1     8  | - - - - | Treat the value in DE as a pointer and load the value from memory into A
    DEC_DE = 0x1b, //               1     8  | - - - - | Decrement DE by 1
    INC_E = 0x1c, //                1     4  | Z 0 H - | Increment E by 1
    DEC_E = 0x1d, //                1     4  | Z 1 H - | Decrement E by 1
    LD_E_d8 = 0x1e, //              2     8  | - - - - | Load the next byte into E
    RRA = 0x1f, //                  1     4  | 0 0 0 C | Rotate A right by 1 bit, moving the C flag into the rightmost bit and the leftmost into the C flag

    JR_NZ_r8 = 0x20, //             2  12/8  | - - - - | Jump relative (see 0x18 JR_d8) if the Z flag is not set
    LD_HL_d16 = 0x21, //            3    12  | - - - - | Load the next two bytes into HL
    LD_HLp_A = 0x22, //             1     8  | - - - - | Store value in register A into byte pointed by HL and post-increment HL
    INC_HL = 0x23, //               1     4  | - - - - | Increment DE by 1
    INC_H = 0x24, //                1     4  | Z 0 H - | Increment H by 1
    DEC_H = 0x25, //                1     4  | Z 1 H - | Decrement H by 1
    LD_H_d8 = 0x26, //              2     8  | - - - - | Load the next byte into H
    DAA = 0x27, //                  1     4  | Z - 0 C | Decimal adjust register A to get a correct BCD representation after an arithmetic instruction
    JR_Z_d8 = 0x28, //              2  12/8  | - - - - | Jump relative (see 0x18 JR_d8) if the Z flag is set
    ADD_HL_HL = 0x29, //            1     8  | - 0 H C | Add the value in HL to itself
    LD_A_HLp = 0x2a, //             1     8  | - - - - | Load value into register A from byte pointed by HL and post-increment HL
    DEC_HL = 0x2b, //               1     8  | - - - - | Decrement HL by 1
    INC_L = 0x2c, //                1     4  | Z 0 H - | Increment L by 1
    DEC_L = 0x2d, //                1     4  | Z 1 H - | Decrement L by 1
    LD_L_d8 = 0x2e, //              2     8  | - - - - | Load the next byte into L
    CPL = 0x2f, //                  1     4  | - 1 1 - | Take the bitwise compliment of A (A = ~A)

    JR_NC_r8 = 0x30, //             2  12/8  | - - - - | Jump relative (see 0x18 JR_d8) if the C flag is not set
    LD_SP_d16 = 0x31, //            3    12  | - - - - | Load the next two bytes into SP
    LD_HLm_A = 0x32, //             1     8  | - - - - | Store value in register A into byte pointed by HL and post-decrement HL
    INC_SP = 0x33, //               1     4  | - - - - | Increment SP by 1
    INC_ptrHL = 0x34, //            1    12  | Z 0 H - | Treat HL as a pointer and increment the value by 1
    DEC_ptrHL = 0x35, //            1    12  | Z 1 H - | Treat HL as a pointer and decrement the value by 1
    LD_ptrHL_d8 = 0x36, //          2    12  | - - - - | Treat HL as a pointer and load the next byte into its value
    SCF = 0x37, //                  1     4  | - 0 0 1 | Set the carry flag
    JR_C_d8 = 0x38, //              2  12/8  | - - - - | Jump relative (see 0x18 JR_d8) if the C flag is set
    ADD_HL_SP = 0x29, //            1     8  | - 0 H C | Add the value in SP to HL
    LD_A_HLm = 0x2a, //             1     8  | - - - - | Load value into register A from byte pointed by HL and post-decrement HL
    DEC_SP = 0x2b, //               1     8  | - - - - | Decrement SP by 1
    INC_A = 0x2c, //                1     4  | Z 0 H - | Increment A by 1
    DEC_A = 0x2d, //                1     4  | Z 1 H - | Decrement A by 1
    LD_A_d8 = 0x2e, //              2     8  | - - - - | Load the next byte into A
    CCF = 0x2f, //                  1     4  | - 0 0 C | Take the bitwise compliment of the C flag (C = ~C)
}

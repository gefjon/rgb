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
    ADD_HL_SP = 0x39, //            1     8  | - 0 H C | Add the value in SP to HL
    LD_A_HLm = 0x3a, //             1     8  | - - - - | Load value into register A from byte pointed by HL and post-decrement HL
    DEC_SP = 0x3b, //               1     8  | - - - - | Decrement SP by 1
    INC_A = 0x3c, //                1     4  | Z 0 H - | Increment A by 1
    DEC_A = 0x3d, //                1     4  | Z 1 H - | Decrement A by 1
    LD_A_d8 = 0x3e, //              2     8  | - - - - | Load the next byte into A
    CCF = 0x3f, //                  1     4  | - 0 0 C | Take the bitwise compliment of the C flag (C = ~C)

    LD_B_B = 0x40, //               1     4  | - - - - | Load the value from B into B (?)
    LD_B_C = 0x41, //               1     4  | - - - - | Load the value from C into B
    LD_B_D = 0x42, //               1     4  | - - - - | Load the value from D into B
    LD_B_E = 0x43, //               1     4  | - - - - | Load the value from E into B
    LD_B_H = 0x44, //               1     4  | - - - - | Load the value from H into B
    LD_B_L = 0x45, //               1     4  | - - - - | Load the value from L into B
    LD_B_ptrHL = 0x46, //           1     8  | - - - - | Load the value pointed to by HL into B
    LD_B_A = 0x47, //               1     4  | - - - - | Load the value from A into B
    LD_C_B = 0x48, //               1     4  | - - - - | Load the value from B into C
    LD_C_C = 0x49, //               1     4  | - - - - | Load the value from C into C (?)
    LD_C_D = 0x4a, //               1     4  | - - - - | Load the value from D into C
    LD_C_E = 0x4b, //               1     4  | - - - - | Load the value from E into C
    LD_C_H = 0x4c, //               1     4  | - - - - | Load the value from H into C
    LD_C_L = 0x4d, //               1     4  | - - - - | Load the value from L into C
    LD_C_ptrHL = 0x4e, //           1     8  | - - - - | Load the value pointed to by HL into C
    LD_C_A = 0x4f, //               1     4  | - - - - | Load the value from A into C

    LD_D_B = 0x50, //               1     4  | - - - - | Load the value from B into D
    LD_D_C = 0x51, //               1     4  | - - - - | Load the value from C into D
    LD_D_D = 0x52, //               1     4  | - - - - | Load the value from D into D
    LD_D_E = 0x53, //               1     4  | - - - - | Load the value from E into D
    LD_D_H = 0x54, //               1     4  | - - - - | Load the value from H into D
    LD_D_L = 0x55, //               1     4  | - - - - | Load the value from L into D
    LD_D_ptrHL = 0x56, //           1     8  | - - - - | Load the value pointed to by HL into D
    LD_D_A = 0x57, //               1     4  | - - - - | Load the value from A into D
    LD_E_B = 0x58, //               1     4  | - - - - | Load the value from B into E
    LD_E_C = 0x59, //               1     4  | - - - - | Load the value from C into E
    LD_E_D = 0x5a, //               1     4  | - - - - | Load the value from D into E
    LD_E_E = 0x5b, //               1     4  | - - - - | Load the value from E into E
    LD_E_H = 0x5c, //               1     4  | - - - - | Load the value from H into E
    LD_E_L = 0x5d, //               1     4  | - - - - | Load the value from L into E
    LD_E_ptrHL = 0x5e, //           1     8  | - - - - | Load the value pointed to by HL into E
    LD_E_A = 0x5f, //               1     4  | - - - - | Load the value from A into E

    LD_H_B = 0x60, //               1     4  | - - - - | Load the value from B into H
    LD_H_C = 0x61, //               1     4  | - - - - | Load the value from C into H
    LD_H_D = 0x62, //               1     4  | - - - - | Load the value from D into H
    LD_H_E = 0x63, //               1     4  | - - - - | Load the value from E into H
    LD_H_H = 0x64, //               1     4  | - - - - | Load the value from H into H
    LD_H_L = 0x65, //               1     4  | - - - - | Load the value from L into H
    LD_H_ptrHL = 0x66, //           1     8  | - - - - | Load the value pointed to by HL into H
    LD_H_A = 0x67, //               1     4  | - - - - | Load the value from A into H
    LD_L_B = 0x68, //               1     4  | - - - - | Load the value from B into L
    LD_L_C = 0x69, //               1     4  | - - - - | Load the value from C into L
    LD_L_D = 0x6a, //               1     4  | - - - - | Load the value from D into L
    LD_L_E = 0x6b, //               1     4  | - - - - | Load the value from E into L
    LD_L_H = 0x6c, //               1     4  | - - - - | Load the value from H into L
    LD_L_L = 0x6d, //               1     4  | - - - - | Load the value from L into L
    LD_L_ptrHL = 0x6e, //           1     8  | - - - - | Load the value pointed to by HL into L
    LD_L_A = 0x6f, //               1     4  | - - - - | Load the value from A into L

    LD_ptrHL_B = 0x70, //           1     8  | - - - - | Load the value from B into the location pointed to by HL
    LD_ptrHL_C = 0x71, //           1     8  | - - - - | Load the value from C into the location pointed to by HL
    LD_ptrHL_D = 0x72, //           1     8  | - - - - | Load the value from D into the location pointed to by HL
    LD_ptrHL_E = 0x73, //           1     8  | - - - - | Load the value from E into the location pointed to by HL
    LD_ptrHL_H = 0x74, //           1     8  | - - - - | Load the value from H into the location pointed to by HL
    LD_ptrHL_L = 0x75, //           1     8  | - - - - | Load the value from L into the location pointed to by HL
    HALT = 0x76, //                 1     4  | - - - - | Enter CPU low power mode
    LD_A_A = 0x77, //               1     8  | - - - - | Load the value from A into the location pointed to by HL
    LD_A_B = 0x78, //               1     4  | - - - - | Load the value from B into A
    LD_A_C = 0x79, //               1     4  | - - - - | Load the value from C into A
    LD_A_D = 0x7a, //               1     4  | - - - - | Load the value from D into A
    LD_A_E = 0x7b, //               1     4  | - - - - | Load the value from E into A
    LD_A_H = 0x7c, //               1     4  | - - - - | Load the value from H into A
    LD_A_L = 0x7d, //               1     4  | - - - - | Load the value from L into A
    LD_A_ptrHL = 0x7e, //           1     8  | - - - - | Load the value pointed to by HL into A
    LD_A_A = 0x7f, //               1     4  | - - - - | Load the value from A into A

    ADD_A_B = 0x80, //              1     4  | Z 0 H C | A += B
    ADD_A_C = 0x81, //              1     4  | Z 0 H C | A += C
    ADD_A_D = 0x82, //              1     4  | Z 0 H C | A += D
    ADD_A_E = 0x83, //              1     4  | Z 0 H C | A += E
    ADD_A_H = 0x84, //              1     4  | Z 0 H C | A += H
    ADD_A_L = 0x85, //              1     4  | Z 0 H C | A += L
    ADD_A_ptrHL = 0x86, //          1     8  | Z 0 H C | A += *HL
    ADC_A_A = 0x87, //              1     4  | Z 0 H C | A += A
    ADC_A_B = 0x88, //              1     4  | Z 0 H C | A += B + Cflag
    ADC_A_C = 0x89, //              1     4  | Z 0 H C | A += C + Cflag
    ADC_A_D = 0x8a, //              1     4  | Z 0 H C | A += D + Cflag
    ADC_A_E = 0x8b, //              1     4  | Z 0 H C | A += E + Cflag
    ADC_A_H = 0x8c, //              1     4  | Z 0 H C | A += H + Cflag
    ADC_A_L = 0x8d, //              1     4  | Z 0 H C | A += L + Cflag
    ADC_A_ptrHL = 0x8e, //          1     8  | Z 0 H C | A += *HL + Cflag
    ADC_A_A = 0x8f, //              1     4  | Z 0 H C | A += A + Cflag

    SUB_B = 0x90, //                1     4  | Z 1 H C | A -= B
    SUB_C = 0x91, //                1     4  | Z 1 H C | A -= C
    SUB_D = 0x92, //                1     4  | Z 1 H C | A -= D
    SUB_E = 0x93, //                1     4  | Z 1 H C | A -= E
    SUB_H = 0x94, //                1     4  | Z 1 H C | A -= H
    SUB_L = 0x95, //                1     4  | Z 1 H C | A -= L
    SUB_ptrHL = 0x96, //            1     8  | Z 1 H C | A -= *HL
    SBC_A = 0x97, //                1     4  | Z 1 H C | A -= A
    SBC_B = 0x98, //                1     4  | Z 1 H C | A -= B + Cflag
    SBC_C = 0x99, //                1     4  | Z 1 H C | A -= C + Cflag
    SBC_D = 0x9a, //                1     4  | Z 1 H C | A -= D + Cflag
    SBC_E = 0x9b, //                1     4  | Z 1 H C | A -= E + Cflag
    SBC_H = 0x9c, //                1     4  | Z 1 H C | A -= H + Cflag
    SBC_L = 0x9d, //                1     4  | Z 1 H C | A -= L + Cflag
    SBC_ptrHL = 0x9e, //            1     8  | Z 1 H C | A -= *HL + Cflag
    SBC_A = 0x9f, //                1     4  | Z 1 H C | A -= A + Cflag

    AND_B = 0xa0, //                1     4  | Z 0 1 0 | A &= B
    AND_C = 0xa1, //                1     4  | Z 0 1 0 | A &= C
    AND_D = 0xa2, //                1     4  | Z 0 1 0 | A &= D
    AND_E = 0xa3, //                1     4  | Z 0 1 0 | A &= E
    AND_H = 0xa4, //                1     4  | Z 0 1 0 | A &= H
    AND_L = 0xa5, //                1     4  | Z 0 1 0 | A &= L
    AND_ptrHL = 0xa6, //            1     8  | Z 0 1 0 | A &= *HL
    AND_A = 0xa7, //                1     4  | Z 0 1 0 | A &= A
    XOR_B = 0xa8, //                1     4  | Z 0 0 0 | A xor= B
    XOR_C = 0xa9, //                1     4  | Z 0 0 0 | A xor= C
    XOR_D = 0xaa, //                1     4  | Z 0 0 0 | A xor= D
    XOR_E = 0xab, //                1     4  | Z 0 0 0 | A xor= E
    XOR_H = 0xac, //                1     4  | Z 0 0 0 | A xor= H
    XOR_L = 0xad, //                1     4  | Z 0 0 0 | A xor= L
    XOR_ptrHL = 0xae, //            1     8  | Z 0 0 0 | A xor= *HL
    XOR_A = 0xaf, //                1     4  | Z 0 0 0 | A xor= A

    OR_B = 0xb0, //                 1     4  | Z 0 0 0 | A |= B
    OR_C = 0xb1, //                 1     4  | Z 0 0 0 | A |= C
    OR_D = 0xb2, //                 1     4  | Z 0 0 0 | A |= D
    OR_E = 0xb3, //                 1     4  | Z 0 0 0 | A |= E
    OR_H = 0xb4, //                 1     4  | Z 0 0 0 | A |= H
    OR_L = 0xb5, //                 1     4  | Z 0 0 0 | A |= L
    OR_ptrHL = 0xb6, //             1     8  | Z 0 0 0 | A |= *HL
    OR_A = 0xb7, //                 1     4  | Z 0 0 0 | A |= A
    CP_B = 0xb8, //                 1     4  | Z 1 H C | A - B, do not store
    CP_C = 0xb9, //                 1     4  | Z 1 H C | A - C, do not store
    CP_D = 0xba, //                 1     4  | Z 1 H C | A - D, do not store
    CP_E = 0xbb, //                 1     4  | Z 1 H C | A - E, do not store
    CP_H = 0xbc, //                 1     4  | Z 1 H C | A - H, do not store
    CP_L = 0xbd, //                 1     4  | Z 1 H C | A - L, do not store
    CP_ptrHL = 0xbe, //             1     8  | Z 1 H C | A - *HL, do not store
    CP_A = 0xbf, //                 1     4  | Z 1 H C | A - A, do not store

    RET_NZ = 0xc0, //               1  20/8  | - - - - | Return from subroutine if the Z flag is not set
    POP_BC = 0xc1, //               1    12  | - - - - | Pop from stack into BC
    JP_NZ = 0xc2, //                3 16/12  | - - - - | Absolute jump to the next two bytes if the Z flag is not set
    JP = 0xc3, //                   3    16  | - - - - | Absolute jump to the next two bytes
    CALL_NZ = 0xc4, //              3 24/12  | - - - - | Call the next two bytes if the Z flag is not set
    PUSH_BC = 0xc5, //              1    16  | - - - - | Push from BC onto the stack
    ADD_A_d8 = 0xc6, //             2     8  | Z 0 H C | Add the next byte to A
    RST_00H = 0xc7, //              1    16  | - - - - | Call restart vector 00H
    RET_Z = 0xc8, //                1  20/8  | - - - - | Return from subroutine if the Z flag is set
    RET = 0xc9, //                  1    16  | - - - - | Return from subroutine
    JP_Z = 0xca, //                 3 16/12  | - - - - | Absolute jump to the next two bytes if the Z flag is set
    PREFIX_CB = 0xcb, //            1     4  | - - - - | Read the next instruction from the other table of ops
    CALL_Z = 0xcc, //               3 24/12  | - - - - | Call the next two bytes if the Z flag is set
    CALL = 0xcd, //                 3    24  | - - - - | Call the subroutine pointed to by the next two bytes
    ADC_A_d8 = 0xce, //             2     8  | Z 0 H C | A += d8 + Cflag
    RST_08H = 0xcf, //              1    16  | - - - - | Call restart vector 08H

    RET_NC = 0xd0, //               1  20/8  | - - - - | Return from subroutine if the C flag is not set
    POP_DE = 0xd1, //               1    12  | - - - - | Pop from stack into BC
    JP_NC = 0xd2, //                3 16/12  | - - - - | Absolute jump to the next two bytes if the C flag is not set
    BAD_0 = 0xd3,
    CALL_NC = 0xd4, //              3 24/12  | - - - - | Call the next two bytes if the C flag is not set
    PUSH_DE = 0xd5, //              1    16  | - - - - | Push from BC onto the stack
    SUB_A_d8 = 0xd6, //             2     8  | Z 1 H C | Sub the next byte from A
    RST_10H = 0xd7, //              1    16  | - - - - | Call restart vector 10H
    RET_C = 0xd8, //                1  20/8  | - - - - | Return from subroutine if the C flag is set
    RETI = 0xd9, //                 1    16  | - - - - | Return from subroutine and enable interrupts
    JP_C = 0xda, //                 3 16/12  | - - - - | Absolute jump to the next two bytes if the C flag is set
    BAD_1 = 0xdb,
    CALL_C = 0xdc, //               3 24/12  | - - - - | Call the next two bytes if the C flag is set
    BAD02 = 0xdd,
    SBC_A_d8 = 0xde, //             2     8  | Z 1 H C | A -= d8 + Cflag
    RST_18H = 0xdf, //              1    16  | - - - - | Call restart vector 18H

    LDH_d8_A = 0xe0, //             2    12  | - - - - | Write the value in A to memory at (FF00 + d8)
    POP_HL = 0xe1, //               1    12  | - - - - | Pop from stack into HL
    LDH_C_A = 0xe2, //              1     8  | - - - - | Write the value in A to memory at (FF00 + C)
    BAD_3 = 0xe3,
    BAD_4 = 0xe4,
    PUSH_HL = 0xe5, //              1    16  | - - - - | Push from HL onto the stack
    AND_d8 = 0xe6, //               2     8  | Z 0 1 0 | A &= d8
    RST_20H = 0xe7, //              1    16  | - - - - | Call restart vector 20H
    ADD_SP_d8 = 0xe8, //            2    16  | 0 0 H C | Add the signed value d8 to SP
    JP_ptrHL = 0xe9, //             1     4  | - - - - | Absolute jump to the location pointed to by HL
    LD_d16_A = 0xea, //             3    16  | - - - - | Load the value in A into the location pointed to by the next two bytes
    BAD_5 = 0xeb,
    BAD_6 = 0xec,
    BAD_7 = 0xed,
    XOR_d8 = 0xee, //               2     8  | Z 0 1 0 | A xor= d8
    RST_28H = 0xef, //              1    16  | - - - - | Call restart vector 28H

    LDH_A_d8 = 0xf0, //             2    12  | - - - - | Load the value in memory at (FF00 + d8) into A
    POP_AF = 0xf1, //               1    12  | - - - - | Pop from stack into AF
    LDH_A_C = 0xf2, //              1     8  | - - - - | Load the value in memory at (FF00 + C) into A
    DI = 0xf3, //                   1     4  | - - - - | Disable interrupts
    BAD_8 = 0xf4,
    PUSH_AF = 0xf5, //              1    16  | - - - - | Push from AF onto the stack
    OR_d8 = 0xf6, //                2     8  | Z 0 0 0 | A |= d8
    RST_30H = 0xf7, //              1    16  | - - - - | Call restart vector 30H
    LD_HL_SPpd8 = 0xf8, //          2    16  | 0 0 H C | Add the signed value d8 to SP and store the result in HL
    LD_SP_HL = 0xf9, //             1     4  | - - - - | Load HL into SP
    LD_A_d16 = 0xfa, //             3    16  | - - - - | Load the value pointed to by the next two bytes into A
    EI = 0xfb, //                   1     4  | - - - - | Enable interrupts
    BAD_9 = 0xfc,
    BAD_a = 0xfd,
    CP_d8 = 0xfe, //                2     8  | Z 1 H C | A - d8, do not store the result
    RST_38H = 0xff, //              1    16  | - - - - | Call restart vector 38H
}

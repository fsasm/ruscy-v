/*
 * constants.rs
 * Author: Fabjan Sukalia <fsukalia@gmail.com>
 * Date: 2016-10-01
 */
#![allow(dead_code)]

/* The opcodes without the two lower bits that indicate the instruction length.
 * The lower two bits must be "11". */
pub mod opcode {
    pub const LOAD      : u8 = 0b00000;
    pub const LOAD_FP   : u8 = 0b00001;
    pub const MISC_MEM  : u8 = 0b00011;
    pub const OP_IMM    : u8 = 0b00100;
    pub const AUIPC     : u8 = 0b00101;
    pub const OP_IMM_32 : u8 = 0b00110;

    pub const STORE     : u8 = 0b01000;
    pub const STORE_FP  : u8 = 0b01001;
    pub const AMO       : u8 = 0b01011;
    pub const OP        : u8 = 0b01100;
    pub const LUI       : u8 = 0b01101;
    pub const OP_32     : u8 = 0b01110;

    pub const MADD      : u8 = 0b10000;
    pub const MSUB      : u8 = 0b10001;
    pub const NMSUB     : u8 = 0b10010;
    pub const NMADD     : u8 = 0b10011;
    pub const OP_FP     : u8 = 0b10100;

    pub const BRANCH    : u8 = 0b11000;
    pub const JALR      : u8 = 0b11001;
    pub const JAL       : u8 = 0b11011;
    pub const SYSTEM    : u8 = 0b11100;
}

pub mod funct3 {
    pub const JALR    : u8 = 0b000;

    pub const BEQ     : u8 = 0b000;
    pub const BNE     : u8 = 0b001;
    pub const BLT     : u8 = 0b100;
    pub const BGE     : u8 = 0b101;
    pub const BLTU    : u8 = 0b110;
    pub const BGEU    : u8 = 0b111;

    pub const LB      : u8 = 0b000;
    pub const LH      : u8 = 0b001;
    pub const LW      : u8 = 0b010;
    pub const LBU     : u8 = 0b100;
    pub const LHU     : u8 = 0b101;

    pub const SB      : u8 = 0b000;
    pub const SH      : u8 = 0b001;
    pub const SW      : u8 = 0b010;

    pub const ADDI    : u8 = 0b000;
    pub const SLTI    : u8 = 0b010;
    pub const SLTIU   : u8 = 0b011;
    pub const XORI    : u8 = 0b100;
    pub const ORI     : u8 = 0b110;
    pub const ANDI    : u8 = 0b111;
    pub const SLLI    : u8 = 0b001;
    pub const SRLI    : u8 = 0b101;
    pub const SRAI    : u8 = 0b101;

    pub const ADD     : u8 = 0b000;
    pub const SUB     : u8 = 0b000;
    pub const SLL     : u8 = 0b001;
    pub const SLT     : u8 = 0b010;
    pub const SLTU    : u8 = 0b011;
    pub const XOR     : u8 = 0b100;
    pub const SRL     : u8 = 0b101;
    pub const SRA     : u8 = 0b101;
    pub const OR      : u8 = 0b110;
    pub const AND     : u8 = 0b111;

    pub const FENCE   : u8 = 0b000;
    pub const FENCE_I : u8 = 0b001;
    pub const ECALL   : u8 = 0b000;
    pub const EBREAK  : u8 = 0b000;
    pub const CSRRW   : u8 = 0b001;
    pub const CSRRS   : u8 = 0b010;
    pub const CSRRC   : u8 = 0b011;
    pub const CSRRWI  : u8 = 0b101;
    pub const CSRRSI  : u8 = 0b110;
    pub const CSRRCI  : u8 = 0b111;
}

pub mod funct7 {
    pub const SLLI : u8 = 0b0000000;
    pub const SRLI : u8 = 0b0000000;
    pub const SRAI : u8 = 0b0100000;
    pub const ADD  : u8 = 0b0000000;
    pub const SUB  : u8 = 0b0100000;
    pub const SLL  : u8 = 0b0000000;
    pub const SLT  : u8 = 0b0000000;
    pub const SLTU : u8 = 0b0000000;
    pub const XOR  : u8 = 0b0000000;
    pub const SRL  : u8 = 0b0000000;
    pub const SRA  : u8 = 0b0100000;
    pub const OR   : u8 = 0b0000000;
    pub const AND  : u8 = 0b0000000;
}

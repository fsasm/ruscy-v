/*
 * constants.rs
 * Author: Fabjan Sukalia <fsukalia@gmail.com>
 * Date: 2016-10-01
 * Copyright 2016-2017 Fabjan Sukalia
 *
 * This file is part of ruscy-v
 *
 * ruscy-v is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * 
 * ruscy-v is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with ruscy-v.  If not, see <http://www.gnu.org/licenses/>.
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
    pub const LD      : u8 = 0b011;
    pub const LBU     : u8 = 0b100;
    pub const LHU     : u8 = 0b101;
    pub const LWU     : u8 = 0b110;

    pub const SB      : u8 = 0b000;
    pub const SH      : u8 = 0b001;
    pub const SW      : u8 = 0b010;
    pub const SD      : u8 = 0b011;

    pub const ADDI    : u8 = 0b000;
    pub const ADDIW   : u8 = 0b000;
    pub const SLTI    : u8 = 0b010;
    pub const SLTIU   : u8 = 0b011;
    pub const XORI    : u8 = 0b100;
    pub const ORI     : u8 = 0b110;
    pub const ANDI    : u8 = 0b111;
    pub const SLLI    : u8 = 0b001;
    pub const SLLIW   : u8 = 0b001;
    pub const SRLI    : u8 = 0b101;
    pub const SRLIW   : u8 = 0b101;
    pub const SRAI    : u8 = 0b101;
    pub const SRAIW   : u8 = 0b101;

    pub const ADD     : u8 = 0b000;
    pub const ADDW    : u8 = 0b000;
    pub const SUB     : u8 = 0b000;
    pub const SUBW    : u8 = 0b000;
    pub const SLL     : u8 = 0b001;
    pub const SLLW    : u8 = 0b001;
    pub const SLT     : u8 = 0b010;
    pub const SLTU    : u8 = 0b011;
    pub const XOR     : u8 = 0b100;
    pub const SRL     : u8 = 0b101;
    pub const SRLW    : u8 = 0b101;
    pub const SRA     : u8 = 0b101;
    pub const SRAW    : u8 = 0b101;
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
    pub const SLLI  : u8 = 0b0000000;
    pub const SLLIW : u8 = 0b0000000;
    pub const SRLI  : u8 = 0b0000000;
    pub const SRLIW : u8 = 0b0000000;
    pub const SRAI  : u8 = 0b0100000;
    pub const SRAIW : u8 = 0b0100000;
    pub const ADD   : u8 = 0b0000000;
    pub const ADDW  : u8 = 0b0000000;
    pub const SUB   : u8 = 0b0100000;
    pub const SUBW  : u8 = 0b0100000;
    pub const SLL   : u8 = 0b0000000;
    pub const SLLW  : u8 = 0b0000000;
    pub const SLT   : u8 = 0b0000000;
    pub const SLTU  : u8 = 0b0000000;
    pub const XOR   : u8 = 0b0000000;
    pub const SRL   : u8 = 0b0000000;
    pub const SRLW  : u8 = 0b0000000;
    pub const SRA   : u8 = 0b0100000;
    pub const SRAW  : u8 = 0b0100000;
    pub const OR    : u8 = 0b0000000;
    pub const AND   : u8 = 0b0000000;
}

pub mod mul_div {
    pub mod funct3 {
        pub const MUL    : u8 = 0b000;
        pub const MULH   : u8 = 0b001;
        pub const MULHSU : u8 = 0b010;
        pub const MULHU  : u8 = 0b011;
        pub const DIV    : u8 = 0b100;
        pub const DIVU   : u8 = 0b101;
        pub const REM    : u8 = 0b110;
        pub const REMU   : u8 = 0b111;
        pub const MULW   : u8 = 0b000;
        pub const DIVW   : u8 = 0b100;
        pub const DIVUW  : u8 = 0b101;
        pub const REMW   : u8 = 0b110;
        pub const REMUW  : u8 = 0b111;

    }

    pub mod funct7 {
        pub const MUL    : u8 = 0b0000001;
        pub const MULH   : u8 = 0b0000001;
        pub const MULHSU : u8 = 0b0000001;
        pub const MULHU  : u8 = 0b0000001;
        pub const DIV    : u8 = 0b0000001;
        pub const DIVU   : u8 = 0b0000001;
        pub const REM    : u8 = 0b0000001;
        pub const REMU   : u8 = 0b0000001;
        pub const MULW   : u8 = 0b0000001;
        pub const DIVW   : u8 = 0b0000001;
        pub const DIVUW  : u8 = 0b0000001;
        pub const REMW   : u8 = 0b0000001;
        pub const REMUW  : u8 = 0b0000001;
    }
}

pub mod amo {
    pub mod funct3 {
        pub const LR_W      : u8 = 0b010;
        pub const SC_W      : u8 = 0b010;
        pub const AMOSWAP_W : u8 = 0b010;
        pub const AMOADD_W  : u8 = 0b010;
        pub const AMOXOR_W  : u8 = 0b010;
        pub const AMOAND_W  : u8 = 0b010;
        pub const AMOOR_W   : u8 = 0b010;
        pub const AMOMIN_W  : u8 = 0b010;
        pub const AMOMAX_W  : u8 = 0b010;
        pub const AMOMINU_W : u8 = 0b010;
        pub const AMOMAXU_W : u8 = 0b010;
    }

    pub mod funct5 {
        pub const LR_W      : u8 = 0b00010;
        pub const SC_W      : u8 = 0b00011;
        pub const AMOSWAP_W : u8 = 0b00001;
        pub const AMOADD_W  : u8 = 0b00000;
        pub const AMOXOR_W  : u8 = 0b00100;
        pub const AMOAND_W  : u8 = 0b01100;
        pub const AMOOR_W   : u8 = 0b01000;
        pub const AMOMIN_W  : u8 = 0b10000;
        pub const AMOMAX_W  : u8 = 0b10100;
        pub const AMOMINU_W : u8 = 0b11000;
        pub const AMOMAXU_W : u8 = 0b11100;
    }

}

pub mod single_fp {
    pub mod funct3 {
        pub const FLW      : u8 = 0b010;
        pub const FSW      : u8 = 0b010;
        pub const FSGNJ_S  : u8 = 0b000;
        pub const FSGNJN_S : u8 = 0b001;
        pub const FSGNJX_S : u8 = 0b010;
        pub const FMIN_S   : u8 = 0b000;
        pub const FMAX_S   : u8 = 0b001;
        pub const FMV_X_S  : u8 = 0b000;
        pub const FEQ_S    : u8 = 0b010;
        pub const FLT_S    : u8 = 0b001;
        pub const FLE_S    : u8 = 0b000;
        pub const FCLASS_S : u8 = 0b001;
        pub const FMV_S_X  : u8 = 0b000;
    }

    pub mod rs2 {
        pub const FSQRT_S   : u8 = 0b00000;
        pub const FCVT_W_S  : u8 = 0b00000;
        pub const FCVT_WU_S : u8 = 0b00001;
        pub const FMV_X_S   : u8 = 0b00000;
        pub const FCLASS_S  : u8 = 0b00000;
        pub const FCVT_S_W  : u8 = 0b00000;
        pub const FCVT_S_WU : u8 = 0b00001;
        pub const FMV_S_X   : u8 = 0b00000;
    }

    pub mod funct7 {
        pub const FADD_S    : u8 = 0b0000000;
        pub const FSUB_S    : u8 = 0b0000100;
        pub const FMUL_S    : u8 = 0b0001000;
        pub const FDIV_S    : u8 = 0b0001100;
        pub const FSQRT_S   : u8 = 0b0101100;
        pub const FSGNJ_S   : u8 = 0b0010000;
        pub const FSGNJN_S  : u8 = 0b0010000;
        pub const FSGNJX_S  : u8 = 0b0010000;
        pub const FMIN_S    : u8 = 0b0010100;
        pub const FMAX_S    : u8 = 0b0010100;
        pub const FCVT_W_S  : u8 = 0b1100000;
        pub const FCVT_WU_S : u8 = 0b1100000;
        pub const FMV_X_S   : u8 = 0b1110000;
        pub const FEQ_S     : u8 = 0b1010000;
        pub const FLT_S     : u8 = 0b1010000;
        pub const FLE_S     : u8 = 0b1010000;
        pub const FCLASS_S  : u8 = 0b1110000;
        pub const FCVT_S_W  : u8 = 0b1101000;
        pub const FCVT_S_WU : u8 = 0b1101000;
        pub const FMV_S_X   : u8 = 0b1111000;
    }

}

pub mod double_fp {
    pub mod funct3 {
        pub const FLD      : u8 = 0b011;
        pub const FSD      : u8 = 0b011;
        pub const FSGNJ_D  : u8 = 0b000;
        pub const FSGNJN_D : u8 = 0b001;
        pub const FSGNJX_D : u8 = 0b010;
        pub const FMIN_D   : u8 = 0b000;
        pub const FMAX_D   : u8 = 0b001;
        pub const FEQ_D    : u8 = 0b010;
        pub const FLT_D    : u8 = 0b001;
        pub const FLE_D    : u8 = 0b000;
        pub const FCLASS_D : u8 = 0b001;
    }

    pub mod rs2 {
        pub const FSQRT_D   : u8 = 0b00000;
        pub const FCVT_S_D  : u8 = 0b00001;
        pub const FCVT_D_S  : u8 = 0b00000;
        pub const FCLASS_D  : u8 = 0b00000;
        pub const FCVT_W_D  : u8 = 0b00000;
        pub const FCVT_WU_D : u8 = 0b00001;
        pub const FCVT_D_W  : u8 = 0b00000;
        pub const FCVT_D_WU : u8 = 0b00001;
    }

    pub mod funct7 {
        pub const FADD_D    : u8 = 0b0000001;
        pub const FSUB_D    : u8 = 0b0000101;
        pub const FMUL_D    : u8 = 0b0001001;
        pub const FDIV_D    : u8 = 0b0001101;
        pub const FSQRT_D   : u8 = 0b0101101;
        pub const FSGNJ_D   : u8 = 0b0010001;
        pub const FSGNJN_D  : u8 = 0b0010001;
        pub const FSGNJX_D  : u8 = 0b0010001;
        pub const FMIN_D    : u8 = 0b0010101;
        pub const FMAX_D    : u8 = 0b0010101;
        pub const FCVT_S_D  : u8 = 0b0100000;
        pub const FCVT_D_S  : u8 = 0b0100001;
        pub const FEQ_D     : u8 = 0b1010001;
        pub const FLT_D     : u8 = 0b1010001;
        pub const FLE_D     : u8 = 0b1010001;
        pub const FCLASS_D  : u8 = 0b1110001;
        pub const FCVT_W_D  : u8 = 0b1100001;
        pub const FCVT_WU_D : u8 = 0b1100001;
        pub const FCVT_D_W  : u8 = 0b1101001;
        pub const FCVT_D_WU : u8 = 0b1101001;
    }
}

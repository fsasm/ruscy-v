/*
 * decoder.rs
 * Author: Fabjan Sukalia <fsukalia@gmail.com>
 * Date: 2016-10-03
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

#![allow(dead_code,non_camel_case_types)]

use super::constants::*;

#[derive(PartialEq, Debug)]
pub enum FpRoundingMode {
    RoundToNearest,
    RoundTowardsZero,
    RoundDown,
    RoundUp,
    RoundToNearestTieMaxMagnitude,
    DynamicRounding,
    Invalid
}

/* all immeadiates are already shifted */
#[derive(PartialEq, Debug)]
pub enum Instruction {
    /* RV32I */
    LUI {rd: u8, imm: i32},
    AUIPC {rd: u8, imm: i32},
    JAL {rd: u8, imm: i32},
    JALR {rd: u8, rs1: u8, imm: i16},
    BEQ {rs1: u8, rs2: u8, imm: i16},
    BNE {rs1: u8, rs2: u8, imm: i16},
    BLT {rs1: u8, rs2: u8, imm: i16},
    BGE {rs1: u8, rs2: u8, imm: i16},
    BLTU {rs1: u8, rs2: u8, imm: i16},
    BGEU {rs1: u8, rs2: u8, imm: i16},
    LB {rd: u8, rs1: u8, imm: i16},
    LH {rd: u8, rs1: u8, imm: i16},
    LW {rd: u8, rs1: u8, imm: i16},
    LBU {rd: u8, rs1: u8, imm: i16},
    LHU {rd: u8, rs1: u8, imm: i16},
    SB {rs1: u8, rs2: u8, imm: i16},
    SH {rs1: u8, rs2: u8, imm: i16},
    SW {rs1: u8, rs2: u8, imm: i16},
    ADDI {rd: u8, rs1: u8, imm: i16},
    SLTI {rd: u8, rs1: u8, imm: i16},
    SLTIU {rd: u8, rs1: u8, imm: i16},
    XORI {rd: u8, rs1: u8, imm: i16},
    ORI {rd: u8, rs1: u8, imm: i16},
    ANDI {rd: u8, rs1: u8, imm: i16},
    SLLI {rd: u8, rs1: u8, shamt: u8},
    SRLI {rd: u8, rs1: u8, shamt: u8},
    SRAI {rd: u8, rs1: u8, shamt: u8},
    ADD {rd: u8, rs1: u8, rs2: u8},
    SUB {rd: u8, rs1: u8, rs2: u8},
    SLL {rd: u8, rs1: u8, rs2: u8},
    SLT {rd: u8, rs1: u8, rs2: u8},
    SLTU {rd: u8, rs1: u8, rs2: u8},
    XOR {rd: u8, rs1: u8, rs2: u8},
    SRL {rd: u8, rs1: u8, rs2: u8},
    SRA {rd: u8, rs1: u8, rs2: u8},
    OR {rd: u8, rs1: u8, rs2: u8},
    AND {rd: u8, rs1: u8, rs2: u8},
    FENCE {succ: u8, pred: u8},
    FENCE_I,
    ECALL,
    EBREAK,
    CSRRW {rd: u8, rs1: u8, csr: u16},
    CSRRS {rd: u8, rs1: u8, csr: u16},
    CSRRC {rd: u8, rs1: u8, csr: u16},
    CSRRWI {rd: u8, zimm: u8, csr: u16},
    CSRRSI {rd: u8, zimm: u8, csr: u16},
    CSRRCI {rd: u8, zimm: u8, csr: u16},

    /* RV64I */
    LWU {rd: u8, rs1: u8, imm: i16},
    LD {rd: u8, rs1: u8, imm: i16},
    SD {rs1: u8, rs2: u8, imm: i16},
    ADDIW {rd: u8, rs1: u8, imm: i16},
    SLLIW {rd: u8, rs1: u8, shamt: u8},
    SRLIW {rd: u8, rs1: u8, shamt: u8},
    SRAIW {rd: u8, rs1: u8, shamt: u8},
    ADDW {rd: u8, rs1: u8, rs2: u8},
    SUBW {rd: u8, rs1: u8, rs2: u8},
    SLLW {rd: u8, rs1: u8, rs2: u8},
    SRLW {rd: u8, rs1: u8, rs2: u8},
    SRAW {rd: u8, rs1: u8, rs2: u8},

    /* RV32M */
    MUL {rd: u8, rs1: u8, rs2: u8},
    MULH {rd: u8, rs1: u8, rs2: u8},
    MULHSU {rd: u8, rs1: u8, rs2: u8},
    MULHU {rd: u8, rs1: u8, rs2: u8},
    DIV {rd: u8, rs1: u8, rs2: u8},
    DIVU {rd: u8, rs1: u8, rs2: u8},
    REM {rd: u8, rs1: u8, rs2: u8},
    REMU {rd: u8, rs1: u8, rs2: u8},

    /* RV64M */
    MULW {rd: u8, rs1: u8, rs2: u8},
    DIVW {rd: u8, rs1: u8, rs2: u8},
    DIVUW {rd: u8, rs1: u8, rs2: u8},
    REMW {rd: u8, rs1: u8, rs2: u8},
    REMUW {rd: u8, rs1: u8, rs2: u8},

    /* RV32A */
    LR_W {rd: u8, rs1: u8, aq: bool, rl: bool},
    SC_W {rd: u8, rs1: u8, aq: bool, rl: bool},
    AMOSWAP_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOADD_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOXOR_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOAND_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOOR_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOMIN_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOMAX_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOMINU_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOMAXU_W {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},

    /* RV64A */
    LR_D {rd: u8, rs1: u8, aq: bool, rl: bool},
    SC_D {rd: u8, rs1: u8, aq: bool, rl: bool},
    AMOSWAP_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOADD_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOXOR_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOAND_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOOR_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOMIN_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOMAX_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOMINU_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},
    AMOMAXU_D {rd: u8, rs1: u8, rs2: u8, aq: bool, rl: bool},

    /* RV32F */
    FLW {rd: u8, rs1: u8, imm: i16},
    FSW {rd: u8, rs1: u8, imm: i16},
    FMADD_S {rd: u8, rs1: u8, rs2: u8, rs3: u8, rm: FpRoundingMode},
    FMSUB_S {rd: u8, rs1: u8, rs2: u8, rs3: u8, rm: FpRoundingMode},
    FNMSUB_S {rd: u8, rs1: u8, rs2: u8, rs3: u8, rm: FpRoundingMode},
    FNMADD_S {rd: u8, rs1: u8, rs2: u8, rs3: u8, rm: FpRoundingMode},
    FADD_S {rd: u8, rs1: u8, rs2: u8, rm: FpRoundingMode},
    FSUB_S {rd: u8, rs1: u8, rs2: u8, rm: FpRoundingMode},
    FMUL_S {rd: u8, rs1: u8, rs2: u8, rm: FpRoundingMode},
    FDIV_S {rd: u8, rs1: u8, rs2: u8, rm: FpRoundingMode},
    FSQRT_S {rd: u8, rs1: u8, rm: FpRoundingMode},
    FSGNJ_S {rd: u8, rs1: u8, rs2: u8},
    FSGNJN_S {rd: u8, rs1: u8, rs2: u8},
    FSGNJX_S {rd: u8, rs1: u8, rs2: u8},
    FMIN_S {rd: u8, rs1: u8, rs2: u8},
    FMAX_S {rd: u8, rs1: u8, rs2: u8},
    FCVT_W_S {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_WU_S {rd: u8, rs1: u8, rm: FpRoundingMode},
    FMV_X_S {rd: u8, rs1: u8},
    FEQ_S {rd: u8, rs1: u8, rs2: u8},
    FLT_S {rd: u8, rs1: u8, rs2: u8},
    FLE_S {rd: u8, rs1: u8, rs2: u8},
    FCLASS_S {rd: u8, rs1: u8},
    FCVT_S_W {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_S_WU {rd: u8, rs1: u8, rm: FpRoundingMode},
    FMV_S_X {rd: u8, rs1: u8},

    /* RV64F */
    FCVT_L_S {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_LU_S {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_S_L {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_S_LU {rd: u8, rs1: u8, rm: FpRoundingMode},

    /* RV32D */
    FLD {rd: u8, rs1: u8, imm: i16},
    FSD {rd: u8, rs1: u8, imm: i16},
    FMADD_D {rd: u8, rs1: u8, rs2: u8, rs3: u8, rm: FpRoundingMode},
    FMSUB_D {rd: u8, rs1: u8, rs2: u8, rs3: u8, rm: FpRoundingMode},
    FNMSUB_D {rd: u8, rs1: u8, rs2: u8, rs3: u8, rm: FpRoundingMode},
    FNMADD_D {rd: u8, rs1: u8, rs2: u8, rs3: u8, rm: FpRoundingMode},
    FADD_D {rd: u8, rs1: u8, rs2: u8, rm: FpRoundingMode},
    FSUB_D {rd: u8, rs1: u8, rs2: u8, rm: FpRoundingMode},
    FMUL_D {rd: u8, rs1: u8, rs2: u8, rm: FpRoundingMode},
    FDIV_D {rd: u8, rs1: u8, rs2: u8, rm: FpRoundingMode},
    FSQRT_D {rd: u8, rs1: u8, rm: FpRoundingMode},
    FSGNJ_D {rd: u8, rs1: u8, rs2: u8},
    FSGNJN_D {rd: u8, rs1: u8, rs2: u8},
    FSGNJX_D {rd: u8, rs1: u8, rs2: u8},
    FMIN_D {rd: u8, rs1: u8, rs2: u8},
    FMAX_D {rd: u8, rs1: u8, rs2: u8},
    FCVT_S_D {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_D_S {rd: u8, rs1: u8, rm: FpRoundingMode},
    FEQ_D {rd: u8, rs1: u8, rs2: u8},
    FLT_D {rd: u8, rs1: u8, rs2: u8},
    FLE_D {rd: u8, rs1: u8, rs2: u8},
    FCLASS_D {rd: u8, rs1: u8},
    FCVT_W_D {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_WU_D {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_D_W {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_D_WU {rd: u8, rs1: u8, rm: FpRoundingMode},

    /* RV64F */
    FCVT_L_D {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_LU_D {rd: u8, rs1: u8, rm: FpRoundingMode},
    FMV_X_D {rd: u8, rs1: u8},
    FCVT_D_L {rd: u8, rs1: u8, rm: FpRoundingMode},
    FCVT_D_LU {rd: u8, rs1: u8, rm: FpRoundingMode},
    FMV_D_X {rd: u8, rs1: u8},
}

fn get_i_imm12(word : u32) -> i16 {
    let word = word >> 20;
    if word < 0x0800 {
        word as i16
    } else {
        let pos = ((!word) + 1) & 0x0FFF;
        -(pos as i16)
    }
}

fn get_s_imm12(word : u32) -> i16 {
    let imm5 = (word >> 7) & 0x1F;
    let imm12 = word >> 25;
    let imm = (imm12 << 5) | imm5;

    if imm < 0x0800 {
        imm as i16
    } else {
        let pos = ((!imm) + 1) & 0x0FFF;
        -(pos as i16)
    }
}

fn get_sb_imm12(word : u32) -> i16 {
    let imm4 = (word >> 8) & 0x0F;
    let imm10 = (word >> 25) & 0x3F;
    let imm11 = (word >> 7) & 0x01;
    let imm12 = word >> 31;
    //println!("{:x} {:x} {:x} {:x}", imm12, imm11, imm10, imm4);
    let imm = (imm12 << 12) | (imm11 << 11) | (imm10 << 5) | (imm4 << 1);

    if imm12 == 0 {
        imm as i16
    } else {
        let pos = ((!imm) + 1) & 0x1FFE;
        -(pos as i16)
    }
}

fn get_jal_imm20(word: u32) -> i32 {
    let imm10 = (word >> 21) & 0x03FF;
    let imm11 = (word >> 20) & 0x01;
    let imm19 = (word >> 12) & 0xFF;
    let imm20 = (word >> 31) & 0x01;
    let imm = (imm20 << 20) | (imm19 << 12) | (imm11 << 11) | (imm10 << 1);

    if imm20 == 0 {
        imm as i32
    } else {
        let pos = ((!imm) + 1) & 0x1FFFFF;
        -(pos as i32)
    }
}

impl Instruction {
    /* decodes an 32-bit sized instruction (RV32I and RV64I) */
    pub fn decode32(instr : u32, rv64: bool) -> Result<Instruction, ()> {
        if (instr & 0x03) != 0x03 {
            return Err(());
        }

        let op        : u8 = ((instr >> 2) & 0x1F) as u8;
        let rd        : u8 = ((instr >> 7) & 0x1F) as u8;
        let rs1       : u8 = ((instr >> 15) & 0x1F) as u8;
        let rs2       : u8 = ((instr >> 20) & 0x1F) as u8;
        let shamt     : u8 = ((instr >> 20) & 0x1F) as u8;
        let shamt64   : u8 = ((instr >> 20) & 0x3F) as u8;
        let funct3    : u8 = ((instr >> 12) & 0x07) as u8;
        let funct7    : u8 = ((instr >> 25) & 0x7F) as u8;
        let csr       : u16 = ((instr >> 20) & 0x0FFF) as u16;
        let i_imm12   : i16 = get_i_imm12(instr);
        let s_imm12   : i16 = get_s_imm12(instr);
        let sb_imm12  : i16 = get_sb_imm12(instr);
        let jal_imm20 : i32 = get_jal_imm20(instr);

        /* TODO separate decode32 to decode_rv32i, decode_rv64i, decode_rv32m, 
         * and so on. decode32 first tries to decode the extensions and then
         * the base ISA, first 64 bit and then 32 bit. */

        match (rv64, op, funct3, funct7) {
            (_, opcode::LUI, _, _) => Ok(Instruction::LUI {rd: rd, imm: (instr & 0xFFFFF000) as i32}), 
            (_, opcode::AUIPC, _, _) => Ok(Instruction::AUIPC {rd: rd, imm: (instr & 0xFFFFF000) as i32}), 
            (_, opcode::JAL, _, _) => Ok(Instruction::JAL {rd: rd, imm: jal_imm20}), 
            (_, opcode::JALR, funct3::JALR, _) => Ok(Instruction::JALR {rd: rd, rs1: rs1, imm: i_imm12}), 
            (_, opcode::BRANCH, funct3::BEQ, _) => Ok(Instruction::BEQ {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (_, opcode::BRANCH, funct3::BNE, _) => Ok(Instruction::BNE {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (_, opcode::BRANCH, funct3::BLT, _) => Ok(Instruction::BLT {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (_, opcode::BRANCH, funct3::BGE, _) => Ok(Instruction::BGE {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (_, opcode::BRANCH, funct3::BLTU, _) => Ok(Instruction::BLTU {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (_, opcode::BRANCH, funct3::BGEU, _) => Ok(Instruction::BGEU {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (_, opcode::LOAD, funct3::LB, _) => Ok(Instruction::LB {rd: rd, rs1: rs1, imm: i_imm12}), 
            (_, opcode::LOAD, funct3::LH, _) => Ok(Instruction::LH {rd: rd, rs1: rs1, imm: i_imm12}), 
            (_, opcode::LOAD, funct3::LW, _) => Ok(Instruction::LW {rd: rd, rs1: rs1, imm: i_imm12}), 
            (true, opcode::LOAD, funct3::LD, _) => Ok(Instruction::LD {rd: rd, rs1: rs1, imm: i_imm12}), 
            (_, opcode::LOAD, funct3::LBU, _) => Ok(Instruction::LBU {rd: rd, rs1: rs1, imm: i_imm12}), 
            (_, opcode::LOAD, funct3::LHU, _) => Ok(Instruction::LHU {rd: rd, rs1: rs1, imm: i_imm12}), 
            (true, opcode::LOAD, funct3::LWU, _) => Ok(Instruction::LWU {rd: rd, rs1: rs1, imm: i_imm12}), 
            (_, opcode::STORE, funct3::SB, _) => Ok(Instruction::SB {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (_, opcode::STORE, funct3::SH, _) => Ok(Instruction::SH {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (_, opcode::STORE, funct3::SW, _) => Ok(Instruction::SW {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (true, opcode::STORE, funct3::SD, _) => Ok(Instruction::SD {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (_, opcode::OP_IMM, funct3::ADDI, _) => Ok(Instruction::ADDI {rd: rd, rs1: rs1, imm: i_imm12}),
            (true, opcode::OP_IMM_32, funct3::ADDIW, _) => Ok(Instruction::ADDIW {rd: rd, rs1: rs1, imm: i_imm12}),
            (_, opcode::OP_IMM, funct3::SLTI, _) => Ok(Instruction::SLTI {rd: rd, rs1: rs1, imm: i_imm12}),
            (_, opcode::OP_IMM, funct3::SLTIU, _) => Ok(Instruction::SLTIU {rd: rd, rs1: rs1, imm: i_imm12}),
            (_, opcode::OP_IMM, funct3::XORI, _) => Ok(Instruction::XORI {rd: rd, rs1: rs1, imm: i_imm12}),
            (_, opcode::OP_IMM, funct3::ORI, _) => Ok(Instruction::ORI {rd: rd, rs1: rs1, imm: i_imm12}),
            (_, opcode::OP_IMM, funct3::ANDI, _) => Ok(Instruction::ANDI {rd: rd, rs1: rs1, imm: i_imm12}),
            (false, opcode::OP_IMM, funct3::SLLI, funct7::SLLI) => Ok(Instruction::SLLI {rd: rd, rs1: rs1, shamt: shamt}),
            (true, opcode::OP_IMM_32, funct3::SLLIW, funct7::SLLIW) => Ok(Instruction::SLLIW {rd: rd, rs1: rs1, shamt: shamt}),
            (true, opcode::OP_IMM, funct3::SLLI, funct7::SLLI) | (true, opcode::OP_IMM, funct3::SLLI, 0b0000001) => Ok(Instruction::SLLI {rd: rd, rs1: rs1, shamt: shamt64}),
            (false, opcode::OP_IMM, funct3::SRLI, funct7::SRLI) => Ok(Instruction::SRLI {rd: rd, rs1: rs1, shamt: shamt}),
            (true, opcode::OP_IMM_32, funct3::SRLIW, funct7::SRLIW) => Ok(Instruction::SRLIW {rd: rd, rs1: rs1, shamt: shamt}),
            (true, opcode::OP_IMM, funct3::SRLI, funct7::SRLI) | (true, opcode::OP_IMM, funct3::SRLI, 0b0000001) => Ok(Instruction::SRLI {rd: rd, rs1: rs1, shamt: shamt64}),
            (false, opcode::OP_IMM, funct3::SRAI, funct7::SRAI) => Ok(Instruction::SRAI {rd: rd, rs1: rs1, shamt: shamt}),
            (true, opcode::OP_IMM_32, funct3::SRAIW, funct7::SRAIW) => Ok(Instruction::SRAIW {rd: rd, rs1: rs1, shamt: shamt}),
            (true, opcode::OP_IMM, funct3::SRAI, funct7::SRAI) | (true, opcode::OP_IMM, funct3::SRAI, 0b010000) => Ok(Instruction::SRAI {rd: rd, rs1: rs1, shamt: shamt64}),
            (_, opcode::OP, funct3::ADD, funct7::ADD) => Ok(Instruction::ADD {rd: rd, rs1: rs1, rs2: rs2}),
            (true, opcode::OP_32, funct3::ADDW, funct7::ADDW) => Ok(Instruction::ADDW {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::SUB, funct7::SUB) => Ok(Instruction::SUB {rd: rd, rs1: rs1, rs2: rs2}),
            (true, opcode::OP_32, funct3::SUBW, funct7::SUBW) => Ok(Instruction::SUBW {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::SLL, funct7::SLL) => Ok(Instruction::SLL {rd: rd, rs1: rs1, rs2: rs2}),
            (true, opcode::OP_32, funct3::SLLW, funct7::SLLW) => Ok(Instruction::SLLW {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::SLT, funct7::SLT) => Ok(Instruction::SLT {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::SLTU, funct7::SLTU) => Ok(Instruction::SLTU {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::XOR, funct7::XOR) => Ok(Instruction::XOR {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::SRL, funct7::SRL) => Ok(Instruction::SRL {rd: rd, rs1: rs1, rs2: rs2}),
            (true, opcode::OP_32, funct3::SRLW, funct7::SRLW) => Ok(Instruction::SRLW {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::SRA, funct7::SRA) => Ok(Instruction::SRA {rd: rd, rs1: rs1, rs2: rs2}),
            (true, opcode::OP_32, funct3::SRAW, funct7::SRAW) => Ok(Instruction::SRAW {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::OR, funct7::OR) => Ok(Instruction::OR {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::OP, funct3::AND, funct7::AND) => Ok(Instruction::AND {rd: rd, rs1: rs1, rs2: rs2}),
            (_, opcode::MISC_MEM, _, _) => {
                if rd == 0 && funct3 == 0 && rs1 == 0 && (instr >> 28) == 0 {
                    Ok(Instruction::FENCE {succ: ((instr >> 20) & 0x0F) as u8,
                        pred: ((instr >> 24) & 0x0F) as u8})
                } else if rd == 0 && funct3 == 1 && rs1 == 0 && i_imm12 == 0 {
                    Ok(Instruction::FENCE_I)
                } else {
                    Err(())
                }
            },
            (_, opcode::SYSTEM, funct3::ECALL, _) => {
                if instr == 0x00000073 {
                    Ok(Instruction::ECALL)
                } else if instr == 0x00010073 {
                    Ok(Instruction::EBREAK)
                } else {
                    Err(())
                }
            },
            (_, opcode::SYSTEM, funct3::CSRRW, _) => Ok(Instruction::CSRRW {rd: rd, rs1: rs1, csr: csr}),
            (_, opcode::SYSTEM, funct3::CSRRS, _) => Ok(Instruction::CSRRS {rd: rd, rs1: rs1, csr: csr}),
            (_, opcode::SYSTEM, funct3::CSRRC, _) => Ok(Instruction::CSRRC {rd: rd, rs1: rs1, csr: csr}),
            (_, opcode::SYSTEM, funct3::CSRRWI, _) => Ok(Instruction::CSRRWI {rd: rd, zimm: rs1, csr: csr}),
            (_, opcode::SYSTEM, funct3::CSRRSI, _) => Ok(Instruction::CSRRSI {rd: rd, zimm: rs1, csr: csr}),
            (_, opcode::SYSTEM, funct3::CSRRCI, _) => Ok(Instruction::CSRRCI {rd: rd, zimm: rs1, csr: csr}),
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Instruction, get_i_imm12, get_s_imm12, get_sb_imm12, get_jal_imm20};

    #[test]
    fn test_decode32() {
        // LB x14, 267(x3)
        assert_eq!(Instruction::decode32(0x10B18703, false), Ok(Instruction::LB{rd: 14, rs1: 3, imm: 267}));
    
        // ADDI x30, x20, -1036
        assert_eq!(Instruction::decode32(0xBF4A0F13, false), Ok(Instruction::ADDI{rd: 30, rs1: 20, imm: -1036}));
    
        // ORI x10, x12, -33
        assert_eq!(Instruction::decode32(0xFDF66513, false), Ok(Instruction::ORI{rd: 10, rs1: 12, imm: -33}));
    
        // SB x18, -1654(x6)
        assert_eq!(Instruction::decode32(0x99230523, false), Ok(Instruction::SB{rs1: 6, rs2: 18, imm: -1654}));
    
        // SH x27, -1069(x21)
        assert_eq!(Instruction::decode32(0xBDBA99A3, false), Ok(Instruction::SH{rs1: 21, rs2: 27, imm: -1069}));
    
        // SW x5, 1951(x8)
        assert_eq!(Instruction::decode32(0x78542FA3, false), Ok(Instruction::SW{rs1: 8, rs2: 5, imm: 1951}));

        // BEQ x19, x14, 438
        assert_eq!(Instruction::decode32(0x36E98663, false), Ok(Instruction::BEQ{rs1: 19, rs2: 14, imm: 876}));

        // BGE x7, x11, 2015
        assert_eq!(Instruction::decode32(0x7AB3DFE3, false), Ok(Instruction::BGE{rs1: 7, rs2: 11, imm: 4030}));

        // BLTU x17, x6, -885
        assert_eq!(Instruction::decode32(0x9108EBE3, false), Ok(Instruction::BLTU{rs1: 17, rs2: 16, imm: -1770}));

        // JAL x9, -1760(-3520)
        assert_eq!(Instruction::decode32(0xA40FF4EF, false), Ok(Instruction::JAL{rd: 9, imm: -3520}));
    }

    #[test]
    fn test_get_i_imm12 () {
        assert_eq!(get_i_imm12(0x00000000), 0);
        assert_eq!(get_i_imm12(0x00100000), 1);
        assert_eq!(get_i_imm12(0x00200000), 2);
        assert_eq!(get_i_imm12(0xFFFFFFFF), -1);
        assert_eq!(get_i_imm12(0xFFF00000), -1);
        assert_eq!(get_i_imm12(0xFFE00000), -2);
        assert_eq!(get_i_imm12(0xFFD00000), -3);
        assert_eq!(get_i_imm12(0x80000000), -2048);
        assert_eq!(get_i_imm12(0x7FF00000), 2047);
        assert_eq!(get_i_imm12(0x80100000), -2047);
        assert_eq!(get_i_imm12(0x7FE00000), 2046);
        assert_eq!(get_i_imm12(0xAAA00000), -1366);
        assert_eq!(get_i_imm12(0x55500000), 1365);
    
        // LB x14, 267(x3)
        assert_eq!(get_i_imm12(0x10B18703), 267);
    
        // ADDI x30, x20, -1036
        assert_eq!(get_i_imm12(0xBF4A0F13), -1036);
    
        // ORI x10, x12, -33
        assert_eq!(get_i_imm12(0xFDF66513), -33);
    }
    
    #[test]
    fn test_get_s_imm12 () {
        assert_eq!(get_s_imm12(0x00000000), 0);
        assert_eq!(get_s_imm12(0x00000080), 1);
        assert_eq!(get_s_imm12(0x00000100), 2);
        assert_eq!(get_s_imm12(0xFFFFFFFF), -1);
        assert_eq!(get_s_imm12(0xFE000F80), -1);
        assert_eq!(get_s_imm12(0xFE000F00), -2);
        assert_eq!(get_s_imm12(0xFE000E80), -3);
        assert_eq!(get_s_imm12(0x80000000), -2048);
        assert_eq!(get_s_imm12(0x7E000F80), 2047);
        assert_eq!(get_s_imm12(0x80000080), -2047);
        assert_eq!(get_s_imm12(0x7E000F00), 2046);
        assert_eq!(get_s_imm12(0xAA000500), -1366);
        assert_eq!(get_s_imm12(0x54000A80), 1365);
    
        // SB x18, -1654(x6)
        assert_eq!(get_s_imm12(0x99230523), -1654);
    
        // SH x27, -1069(x21)
        assert_eq!(get_s_imm12(0xBDBA99A3), -1069);
    
        // SW x5, 1951(x8)
        assert_eq!(get_s_imm12(0x78542FA3), 1951);
    }

    #[test]
    fn test_get_sb_imm12 () {
        assert_eq!(get_sb_imm12(0x00000000), 0);
        assert_eq!(get_sb_imm12(0x00000100), 2);
        assert_eq!(get_sb_imm12(0x00000200), 4);
        assert_eq!(get_sb_imm12(0x00000300), 6);
        assert_eq!(get_sb_imm12(0x00000700), 14);
        assert_eq!(get_sb_imm12(0x00000F00), 30);
        assert_eq!(get_sb_imm12(0x02000F00), 62);
        assert_eq!(get_sb_imm12(0x06000F00), 126);
        assert_eq!(get_sb_imm12(0x0E000F00), 254);
        assert_eq!(get_sb_imm12(0x1E000F00), 510);
        assert_eq!(get_sb_imm12(0x3E000F00), 1022);
        assert_eq!(get_sb_imm12(0x7E000F00), 2046);
        assert_eq!(get_sb_imm12(0x7E000F80), 4094);
        assert_eq!(get_sb_imm12(0xFE000F80), -2);
        assert_eq!(get_sb_imm12(0xFE000E80), -4);
        assert_eq!(get_sb_imm12(0xFE000D80), -6);
        assert_eq!(get_sb_imm12(0xFE000C80), -8);
        assert_eq!(get_sb_imm12(0xD4000A00), -1366 * 2);
        assert_eq!(get_sb_imm12(0x2A000580), 1365 * 2);
        assert_eq!(get_sb_imm12(0x80000000), -4096);

        // BEQ x19, x14, 438(876)
        assert_eq!(get_sb_imm12(0x36E98663), 876);

        // BGE x7, x11, 2015(4030)
        assert_eq!(get_sb_imm12(0x7AB3DFE3), 4030);

        // BLTU x17, x16, -885(-1770)
        assert_eq!(get_sb_imm12(0x9108EBE3), -1770);
    }

    #[test]
    fn test_get_jal_imm20() {
        assert_eq!(get_jal_imm20(0x00000000), 0);
        assert_eq!(get_jal_imm20(0x00200000), 2);
        assert_eq!(get_jal_imm20(0x00400000), 4);
        assert_eq!(get_jal_imm20(0x00600000), 6);
        assert_eq!(get_jal_imm20(0x00E00000), 14);
        assert_eq!(get_jal_imm20(0x01E00000), 30);
        assert_eq!(get_jal_imm20(0x03E00000), 62);
        assert_eq!(get_jal_imm20(0x07E00000), 126);
        assert_eq!(get_jal_imm20(0x0FE00000), 254);
        assert_eq!(get_jal_imm20(0x1FE00000), 510);
        assert_eq!(get_jal_imm20(0x3FE00000), 1022);
        assert_eq!(get_jal_imm20(0x7FE00000), 2046);
        assert_eq!(get_jal_imm20(0x7FF00000), 4094);
        assert_eq!(get_jal_imm20(0x7FF01000), 8190);
        assert_eq!(get_jal_imm20(0x7FF03000), 16382);
        assert_eq!(get_jal_imm20(0x7FF07000), 32766);
        assert_eq!(get_jal_imm20(0x7FF0F000), 65534);
        assert_eq!(get_jal_imm20(0x7FF1F000), 131070);
        assert_eq!(get_jal_imm20(0x7FF3F000), 262142);
        assert_eq!(get_jal_imm20(0x7FF7F000), 524286);
        assert_eq!(get_jal_imm20(0x7FFFF000), 1048574);
        assert_eq!(get_jal_imm20(0xFFFFF000), -2);
        assert_eq!(get_jal_imm20(0xFFDFF000), -4);
        assert_eq!(get_jal_imm20(0xFFBFF000), -6);
        assert_eq!(get_jal_imm20(0xFF9FF000), -8);
        assert_eq!(get_jal_imm20(0x2ABAA000), 349525 * 2);
        assert_eq!(get_jal_imm20(0xD5455000), -349526 * 2);
        assert_eq!(get_jal_imm20(0x80000000), -1048576);
        assert_eq!(get_jal_imm20(0x80200000), -1048574);
        assert_eq!(get_jal_imm20(0x80400000), -1048572);
        assert_eq!(get_jal_imm20(0x80600000), -1048570);


        // JAL x9, -1760(-3520)
        assert_eq!(get_jal_imm20(0xA40FF4EF), -3520);
    }

    #[test]
    fn test_lui() {
        assert_eq!((0xFFFFFFFFu32 & 0xFFFFF000u32) as i32, -4096);
        assert_eq!((0x80000000u32 & 0xFFFFF000u32) as i32, -2147483648);
    }
}


/*
 * decoder.rs
 * Author: Fabjan Sukalia <fsukalia@gmail.com>
 * Date: 2016-10-03
 */

#![allow(dead_code,non_camel_case_types)]

use super::constants::*;

/* all immeadiates are already shifted */
pub enum Instruction {
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
    CSRRCI {rd: u8, zimm: u8, csr: u16}
}

fn get_i_imm12(word : u32) -> i16 {
    let word = word >> 20;
    if word < 0x0800 {
        word as i16
    } else {
        let pos = ((!word) + 1) & 0x07FF;
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
        let pos = ((!imm) + 1) & 0x07FF;
        -(pos as i16)
    }
}

fn get_sb_imm12(word : u32) -> i16 {
    let imm4 = (word >> 7) & 0x1E;
    let imm11 = (word >> 25) & 0x3F;
    let imm12 = (word >> 7) & 0x01;
    let imm13 = word >> 31;
    let imm = (imm13 << 13) | (imm12 << 12) | (imm11 << 6) | imm4;

    if imm < 0x1000 {
        imm as i16
    } else {
        let pos = ((!imm) + 1) & 0x0FFF;
        -(pos as i16)
    }
}

fn get_jal_imm20(word: u32) -> i32 {
    let imm10 = (word >> 21) & 0x07FE;
    let imm11 = (word >> 20) & 0x01;
    let imm19 = (word >> 12) & 0xFF;
    let imm20 = (word >> 31) & 0x01;
    let imm = (imm20 << 20) | (imm19 << 12) | (imm11 << 11) | imm10;

    if imm20 == 0 {
        imm as i32
    } else {
        let pos = ((!imm) + 1) & 0x0FFFFF;
        -(pos as i32)
    }
}

#[test]
fn working_get_i_imm12 () {
    assert_eq!(get_i_imm12(0), 0);
    assert_eq!(get_i_imm12(0xFFFFFFFF), -1);
}

impl Instruction {
    pub fn decode32(instr : u32) -> Result<Instruction, ()> {
        if (instr & 0x03) != 0x03 {
            return Err(());
        }

        let op : u8 = ((instr >> 2) & 0x1F) as u8;
        let rd : u8 = ((instr >> 7) & 0x1F) as u8;
        let rs1 : u8 = ((instr >> 15) & 0x1F) as u8;
        let rs2 : u8 = ((instr >> 20) & 0x1F) as u8;
        let funct3 : u8 = ((instr >> 12) & 0x07) as u8;
        let funct7 : u8 = ((instr >> 25) & 0x7F) as u8;
        let csr : u16 = ((instr >> 20) & 0x0FFF) as u16;
        let i_imm12 : i16 = get_i_imm12(instr);
        let s_imm12 : i16 = get_s_imm12(instr);
        let sb_imm12 : i16 = get_sb_imm12(instr);
        let jal_imm20 : i32 = get_jal_imm20(instr);

        match (op, funct3, funct7) {
            (opcode::LUI, _, _) => Ok(Instruction::LUI {rd: rd, imm: (instr & 0xFFFFF000) as i32}), 
            (opcode::AUIPC, _, _) => Ok(Instruction::AUIPC {rd: rd, imm: (instr & 0xFFFFF000) as i32}), 
            (opcode::JAL, _, _) => Ok(Instruction::JAL {rd: rd, imm: jal_imm20}), 
            (opcode::JALR, funct3::JALR, _) => Ok(Instruction::JALR {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::BRANCH, funct3::BEQ, _) => Ok(Instruction::BEQ {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (opcode::BRANCH, funct3::BNE, _) => Ok(Instruction::BNE {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (opcode::BRANCH, funct3::BLT, _) => Ok(Instruction::BLT {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (opcode::BRANCH, funct3::BGE, _) => Ok(Instruction::BGE {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (opcode::BRANCH, funct3::BLTU, _) => Ok(Instruction::BLTU {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (opcode::BRANCH, funct3::BGEU, _) => Ok(Instruction::BGEU {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (opcode::LOAD, funct3::LB, _) => Ok(Instruction::LB {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::LOAD, funct3::LH, _) => Ok(Instruction::LH {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::LOAD, funct3::LW, _) => Ok(Instruction::LW {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::LOAD, funct3::LBU, _) => Ok(Instruction::LBU {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::LOAD, funct3::LHU, _) => Ok(Instruction::LHU {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::STORE, funct3::SB, _) => Ok(Instruction::SB {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::STORE, funct3::SH, _) => Ok(Instruction::SH {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::STORE, funct3::SW, _) => Ok(Instruction::SW {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::OP_IMM, funct3::ADDI, _) => Ok(Instruction::ADDI {rd: rd, rs1: rs1, imm: i_imm12}),
            (opcode::OP_IMM, funct3::SLTI, _) => Ok(Instruction::SLTI {rd: rd, rs1: rs1, imm: i_imm12}),
            (opcode::OP_IMM, funct3::SLTIU, _) => Ok(Instruction::SLTIU {rd: rd, rs1: rs1, imm: i_imm12}),
            (opcode::OP_IMM, funct3::XORI, _) => Ok(Instruction::XORI {rd: rd, rs1: rs1, imm: i_imm12}),
            (opcode::OP_IMM, funct3::ORI, _) => Ok(Instruction::ORI {rd: rd, rs1: rs1, imm: i_imm12}),
            (opcode::OP_IMM, funct3::ANDI, _) => Ok(Instruction::ANDI {rd: rd, rs1: rs1, imm: i_imm12}),
            (opcode::OP_IMM, funct3::SLLI, funct7::SLLI) => Ok(Instruction::SLLI {rd: rd, rs1: rs1, shamt: rs2}),
            (opcode::OP_IMM, funct3::SRLI, funct7::SRLI) => Ok(Instruction::SRLI {rd: rd, rs1: rs1, shamt: rs2}),
            (opcode::OP_IMM, funct3::SRAI, funct7::SRAI) => Ok(Instruction::SRAI {rd: rd, rs1: rs1, shamt: rs2}),
            (opcode::OP, funct3::ADD, funct7::ADD) => Ok(Instruction::ADD {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::SUB, funct7::SUB) => Ok(Instruction::SUB {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::SLL, funct7::SLL) => Ok(Instruction::SLL {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::SLT, funct7::SLT) => Ok(Instruction::SLT {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::SLTU, funct7::SLTU) => Ok(Instruction::SLTU {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::XOR, funct7::XOR) => Ok(Instruction::XOR {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::SRL, funct7::SRL) => Ok(Instruction::SRL {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::SRA, funct7::SRA) => Ok(Instruction::SRA {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::OR, funct7::OR) => Ok(Instruction::OR {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::OP, funct3::AND, funct7::AND) => Ok(Instruction::AND {rd: rd, rs1: rs1, rs2: rs2}),
            (opcode::MISC_MEM, _, _) => {
                if rd == 0 && funct3 == 0 && rs1 == 0 && (instr >> 28) == 0 {
                    Ok(Instruction::FENCE {succ: ((instr >> 20) & 0x0F) as u8,
                        pred: ((instr >> 24) & 0x0F) as u8})
                } else if rd == 0 && funct3 == 1 && rs1 == 0 && i_imm12 == 0 {
                    Ok(Instruction::FENCE_I)
                } else {
                    Err(())
                }
            },
            (opcode::SYSTEM, funct3::ECALL, _) => {
                if instr == 0x00000073 {
                    Ok(Instruction::ECALL)
                } else if instr == 0x00010073 {
                    Ok(Instruction::EBREAK)
                } else {
                    Err(())
                }
            },
            (opcode::SYSTEM, funct3::CSRRW, _) => Ok(Instruction::CSRRW {rd: rd, rs1: rs1, csr: csr}),
            (opcode::SYSTEM, funct3::CSRRS, _) => Ok(Instruction::CSRRS {rd: rd, rs1: rs1, csr: csr}),
            (opcode::SYSTEM, funct3::CSRRC, _) => Ok(Instruction::CSRRC {rd: rd, rs1: rs1, csr: csr}),
            (opcode::SYSTEM, funct3::CSRRWI, _) => Ok(Instruction::CSRRWI {rd: rd, zimm: rs1, csr: csr}),
            (opcode::SYSTEM, funct3::CSRRSI, _) => Ok(Instruction::CSRRSI {rd: rd, zimm: rs1, csr: csr}),
            (opcode::SYSTEM, funct3::CSRRCI, _) => Ok(Instruction::CSRRCI {rd: rd, zimm: rs1, csr: csr}),
            _ => Err(())
        }
    }
}


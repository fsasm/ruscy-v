/*
 * decoder.rs
 * Author: Fabjan Sukalia <fsukalia@gmail.com>
 * Date: 2016-10-03
 */

#![allow(dead_code,non_camel_case_types)]

use super::constants::*;

/* all immeadiates are already shifted */
#[derive(PartialEq, Debug)]
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
            (opcode::BRANCH, funct3::BEQ, _) => Ok(Instruction::BEQ {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::BRANCH, funct3::BNE, _) => Ok(Instruction::BNE {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::BRANCH, funct3::BLT, _) => Ok(Instruction::BLT {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::BRANCH, funct3::BGE, _) => Ok(Instruction::BGE {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::BRANCH, funct3::BLTU, _) => Ok(Instruction::BLTU {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::BRANCH, funct3::BGEU, _) => Ok(Instruction::BGEU {rs1: rs1, rs2: rs2, imm: sb_imm12}),
            (opcode::LOAD, funct3::LB, _) => Ok(Instruction::LB {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::LOAD, funct3::LH, _) => Ok(Instruction::LH {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::LOAD, funct3::LW, _) => Ok(Instruction::LW {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::LOAD, funct3::LBU, _) => Ok(Instruction::LBU {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::LOAD, funct3::LHU, _) => Ok(Instruction::LHU {rd: rd, rs1: rs1, imm: i_imm12}), 
            (opcode::STORE, funct3::SB, _) => Ok(Instruction::SB {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (opcode::STORE, funct3::SH, _) => Ok(Instruction::SH {rs1: rs1, rs2: rs2, imm: s_imm12}),
            (opcode::STORE, funct3::SW, _) => Ok(Instruction::SW {rs1: rs1, rs2: rs2, imm: s_imm12}),
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

#[cfg(test)]
mod test {
    use super::{Instruction, get_i_imm12, get_s_imm12, get_sb_imm12, get_jal_imm20};

    #[test]
    fn test_decode32() {
        // LB x14, 267(x3)
        assert_eq!(Instruction::decode32(0x10B18703), Ok(Instruction::LB{rd: 14, rs1: 3, imm: 267}));
    
        // ADDI x30, x20, -1036
        assert_eq!(Instruction::decode32(0xBF4A0F13), Ok(Instruction::ADDI{rd: 30, rs1: 20, imm: -1036}));
    
        // ORI x10, x12, -33
        assert_eq!(Instruction::decode32(0xFDF66513), Ok(Instruction::ORI{rd: 10, rs1: 12, imm: -33}));
    
        // SB x18, -1654(x6)
        assert_eq!(Instruction::decode32(0x99230523), Ok(Instruction::SB{rs1: 6, rs2: 18, imm: -1654}));
    
        // SH x27, -1069(x21)
        assert_eq!(Instruction::decode32(0xBDBA99A3), Ok(Instruction::SH{rs1: 21, rs2: 27, imm: -1069}));
    
        // SW x5, 1951(x8)
        assert_eq!(Instruction::decode32(0x78542FA3), Ok(Instruction::SW{rs1: 8, rs2: 5, imm: 1951}));

        // BEQ x19, x14, 438
        assert_eq!(Instruction::decode32(0x36E98663), Ok(Instruction::BEQ{rs1: 19, rs2: 14, imm: 876}));

        // BGE x7, x11, 2015
        assert_eq!(Instruction::decode32(0x7AB3DFE3), Ok(Instruction::BGE{rs1: 7, rs2: 11, imm: 4030}));

        // BLTU x17, x6, -885
        assert_eq!(Instruction::decode32(0x9108EBE3), Ok(Instruction::BLTU{rs1: 17, rs2: 16, imm: -1770}));

        // JAL x9, -1760(-3520)
        assert_eq!(Instruction::decode32(0xA40FF4EF), Ok(Instruction::JAL{rd: 9, imm: -3520}));
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

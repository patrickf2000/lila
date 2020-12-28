// This file is part of the Lila compiler
// Copyright (C) 2020 Patrick Flynn
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::utils::*;

// Builds unconditional jump
pub fn riscv64_build_jump(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  j ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");

    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_jump] Write failed.");
}

// Small utility function
fn riscv64_signed_cmp(instr : &LtacType) -> bool {
    match instr {
        LtacType::I8Cmp | LtacType::I16Cmp
        | LtacType::I32Cmp | LtacType::I64Cmp => return true,

        _ => return false,
    }
}

// Builds conditional jumps
// On RISC-V, these are interesting; there's no comparison instruction, instead both things
// happen in the branch instruction
pub fn riscv64_build_cond_jump(writer : &mut BufWriter<File>, cmp : &LtacInstr, jmp : &LtacInstr) {
    let mut line = String::new();

    // First, check the second operand of the comparison instruction. If its an immediate, we have
    // to load to a register (s2)
    // TODO: Find a way to clean up
    match &cmp.arg2 {
        LtacArg::Byte(val) => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::UByte(val) => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
        
        LtacArg::I16(val) => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::U16(val) => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
        
        LtacArg::I32(val) => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::U32(val) => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::I64(val) => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::U64(val) => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        _ => {},
    }

    // If we have small registers, we have to sign-extend
    // TODO: Is there a way to clean up?
    match &cmp.arg1 {
        LtacArg::Reg16(pos) => {
            let reg = riscv64_op_reg(*pos);

            line.push_str("  sext.w ");
            line.push_str(&reg);
            line.push_str(", ");
            line.push_str(&reg);
            line.push_str("\n");
        },

        _ => {},
    }

    match &cmp.arg2 {
        LtacArg::Reg16(pos) => {
            let reg = riscv64_op_reg(*pos);

            line.push_str("  sext.w ");
            line.push_str(&reg);
            line.push_str(", ");
            line.push_str(&reg);
            line.push_str("\n");
        },

        _ => {},
    }

    // Second, write the proper branch instruction
    match &jmp.instr_type {
        LtacType::Be => line.push_str("  beq "),
        LtacType::Bne => line.push_str("  bne "),

        LtacType::Bl if riscv64_signed_cmp(&cmp.instr_type) => line.push_str("  blt "),
        LtacType::Bl if !riscv64_signed_cmp(&cmp.instr_type) => line.push_str("  bltu "),

        LtacType::Ble if riscv64_signed_cmp(&cmp.instr_type) => line.push_str("  ble "),
        LtacType::Ble if !riscv64_signed_cmp(&cmp.instr_type) => line.push_str("  bleu "),

        LtacType::Bg if riscv64_signed_cmp(&cmp.instr_type) => line.push_str("  bgt "),
        LtacType::Bg if !riscv64_signed_cmp(&cmp.instr_type) => line.push_str("  bgtu "),

        LtacType::Bge if riscv64_signed_cmp(&cmp.instr_type) => line.push_str("  bge "),
        LtacType::Bge if !riscv64_signed_cmp(&cmp.instr_type) => line.push_str("  bgeu "),

        _ => {},
    }

    // Now, write the first operand
    match &cmp.arg1 {
        LtacArg::Reg8(pos) | LtacArg::Reg16(pos)
        | LtacArg::Reg32(pos) | LtacArg::Reg64(pos) => {
            let reg = riscv64_op_reg(*pos);
            line.push_str(&reg);
        },

        _ => {},
    }

    line.push_str(", ");

    // Now, write the second operand
    match &cmp.arg2 {
        LtacArg::Reg8(pos) | LtacArg::Reg16(pos)
        | LtacArg::Reg32(pos) | LtacArg::Reg64(pos) => {
            let reg = riscv64_op_reg(*pos);
            line.push_str(&reg);
        },

        LtacArg::Byte(_v) => line.push_str("s2"),
        LtacArg::UByte(_v) => line.push_str("s2"),

        LtacArg::I16(_v) => line.push_str("s2"),
        LtacArg::U16(_v) => line.push_str("s2"),
    
        LtacArg::I32(_v) => line.push_str("s2"),
        LtacArg::U32(_v) => line.push_str("s2"),

        LtacArg::I64(_v) => line.push_str("s2"),
        LtacArg::U64(_v) => line.push_str("s2"),

        _ => {},
    }

    line.push_str(", ");

    // Finally, add the label, and write the rest out
    line.push_str(&jmp.name);
    line.push_str("\n\n");

    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_cond_jump] Write failed.");
}

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

// Builds hardware conversion instructions
pub fn riscv64_build_cvt(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();

    match &code.instr_type {
        LtacType::CvtF32F64 => line = "  fcvt.d.s ".to_string(),
        _ => {},
    }

    match &code.arg1 {
        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = riscv64_op_freg(*pos);
            line.push_str(&reg);
        },

        _ => {},
    }

    line.push_str(", ");

    match &code.arg2 {
        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = riscv64_op_freg(*pos);
            line.push_str(&reg);
        },

        _ => {},
    }

    line.push_str("\n");

    // Write it all out
    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_cvt] Write failed.");
}

// Builds the load-store instructions
pub fn riscv64_build_ld_str(writer : &mut BufWriter<File>, code : &LtacInstr, stack_top : i32, is_load : bool) {
    let mut line = String::new();
    let mut full_line = String::new();

    match &code.instr_type {
        LtacType::LdB => line.push_str("  lb "),
        LtacType::LdUB => line.push_str("  lbu "),
        LtacType::LdW => line.push_str("  lh "),
        LtacType::LdUW => line.push_str("  lhu "),
        LtacType::Ld | LtacType::LdU => line.push_str("  lw "),
        LtacType::LdQ | LtacType::LdUQ => line.push_str("  ld "),
        LtacType::LdF32 => line.push_str("  flw "),

        LtacType::StrB | LtacType::StrUB => line.push_str("  sb "),
        LtacType::StrW | LtacType::StrUW => line.push_str("  sh "),
        LtacType::Str | LtacType::StrU => line.push_str("  sw "),
        LtacType::StrQ | LtacType::StrUQ => line.push_str("  sd "),
        LtacType::StrF32 => line.push_str("  fsw "),
        
        _ => {},
    }

    // Write the registers
    match &code.arg2 {
        LtacArg::Reg8(pos) | LtacArg::Reg16(pos)
        | LtacArg::Reg32(pos) | LtacArg::Reg64(pos) => {
            let reg = riscv64_op_reg(*pos);
            line.push_str(&reg);
        },

        LtacArg::FltReg(pos) => {
            let reg = riscv64_op_freg(*pos);
            line.push_str(&reg);
        },

        _ => {},
    }

    line.push_str(", ");

    // Write out the memory
    match &code.arg1 {
        LtacArg::Mem(val) => {
            let mut pos = stack_top - (*val);

            // If you comment the if condition out, all the integer tests pass. If you don't, all but one array test passes
            // We need to look into this
            if code.instr_type == LtacType::LdQ || code.instr_type == LtacType::LdUQ 
                || code.instr_type == LtacType::StrQ || code.instr_type == LtacType::StrUQ {
                if pos + 8 == stack_top {
                    pos += 8;
                }
            }
            
            line.push_str("-");
            line.push_str(&pos.to_string());
            line.push_str("(s0)");
        },

        LtacArg::MemOffsetImm(pos, offset) => {
            // Load the array
            let array_pos = stack_top - (*pos) + 8;
            full_line.push_str("  ld s2, -");
            full_line.push_str(&array_pos.to_string());
            full_line.push_str("(s0)\n");

            // The format changes slightly for load-store
            if is_load {
                line.push_str(&offset.to_string());
                line.push_str("(s2)");
            } else {
                // Add the offset
                full_line.push_str("  addi s2, s2, ");
                full_line.push_str(&offset.to_string());
                full_line.push_str("\n");

                // Store the result
                line.push_str("0(s2)");
            }
        },

        LtacArg::MemOffsetMem(pos, offset, size) => {
            // Load the array
            let array_pos = stack_top - (*pos) + 8;
            full_line.push_str("  ld s2, -");
            full_line.push_str(&array_pos.to_string());
            full_line.push_str("(s0)\n");

            // Load the offset and the size
            let offset_pos = stack_top - (*offset);
            full_line.push_str("  lw s3, -");
            full_line.push_str(&offset_pos.to_string());
            full_line.push_str("(s0)\n");

            if (*size) == 2 {
                full_line.push_str("  slli s3, s3, 1\n");
            } else if (*size) == 4 {
                full_line.push_str("  slli s3, s3, 2\n");                
            } else if (*size) == 8 {
                full_line.push_str("  slli s3, s3, 3\n");
            }

            // Add the offset
            full_line.push_str("  add s2, s2, s3\n");

            // Store the result
            line.push_str("0(s2)");
        },

        LtacArg::MemOffsetReg(pos, reg_pos, size) => {
            // Load the array
            let array_pos = stack_top - (*pos) + 8;
            full_line.push_str("  ld s2, -");
            full_line.push_str(&array_pos.to_string());
            full_line.push_str("(s0)\n");

            // Now for the offset
            let reg = riscv64_op_reg(*reg_pos);

            if (*size) > 1 {
                full_line.push_str("  slli ");
                full_line.push_str(&reg);
                full_line.push_str(", ");
                full_line.push_str(&reg);

                if (*size) == 2 {
                    full_line.push_str(", 1\n");
                } else if (*size) == 4 {
                    full_line.push_str(", 2\n");
                } else if (*size) == 8 {
                    full_line.push_str(", 3\n");
                }
            }

            full_line.push_str("  add s2, s2, ");
            full_line.push_str(&reg);
            full_line.push_str("\n");

            // Store the result
            line.push_str("0(s2)");
        },

        _ => {},
    }

    // Write the rest out
    line.push_str("\n");
    full_line.push_str(&line);

    writer.write(&full_line.into_bytes())
        .expect("[RISCV64_build_ld_str] Write failed.");
}

// Builds a RISC-V MOV instruction
// On RISC-V, there are separate instructions for register and immediate moves
pub fn riscv64_build_mov(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();

    // Determine the instruction
    match &code.instr_type {
        LtacType::MovB | LtacType::MovUB => {
            match &code.arg2 {
                LtacArg::Byte(_v) => line.push_str("  li "),
                LtacArg::UByte(_v) => line.push_str("  li "),
                _ => line.push_str("  mv "),
            }
        },
        
        LtacType::MovW | LtacType::MovUW => {
            match &code.arg2 {
                LtacArg::I16(_v) => line.push_str("  li "),
                LtacArg::U16(_v) => line.push_str("  li "),
                _ => line.push_str("  mv "),
            }
        },
    
        LtacType::Mov | LtacType::MovU => {
            match &code.arg2 {
                LtacArg::I32(_v) => line.push_str("  li "),
                LtacArg::U32(_v) => line.push_str("  li "),
                _ => line.push_str("  mv "),
            }
        },

        LtacType::MovQ | LtacType::MovUQ => {
            match &code.arg2 {
                LtacArg::I64(_v) => line.push_str("  li "),
                LtacArg::U64(_v) => line.push_str("  li "),
                _ => line.push_str("  mv "),
            }
        },
        
        LtacType::MovF32 => {
            match &code.arg2 {
                LtacArg::F32(ref val) => {
                    line.push_str("  lui a2, %hi(");
                    line.push_str(&val);
                    line.push_str(")\n");

                    line.push_str("  flw ");
                },

                _ => line.push_str("  fmv.s.s "),
            }
        },

        LtacType::MovF64Int => line.push_str("  fmv.x.d "),

        _ => {},
    }

    // Operands
    // Write the first operand
    match &code.arg1 {
        LtacArg::RetRegI8 | LtacArg::RetRegU8 |
        LtacArg::RetRegI16 | LtacArg::RetRegU16 |
        LtacArg::RetRegI32 | LtacArg::RetRegU32 |
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("a0, "),

        LtacArg::Reg8(pos) | LtacArg::Reg16(pos) |
        LtacArg::Reg32(pos) | LtacArg::Reg64(pos) => {
            let reg = riscv64_op_reg(*pos);

            line.push_str(&reg);
            line.push_str(", ");
        },

        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = riscv64_op_freg(*pos);

            line.push_str(&reg);
            line.push_str(", ");
        },
        
        _ => {},
    }

    // Write the second operand
    match &code.arg2 {
        LtacArg::Reg8(pos) | LtacArg::Reg16(pos) |
        LtacArg::Reg32(pos) | LtacArg::Reg64(pos) => {
            let reg = riscv64_op_reg(*pos);
            line.push_str(&reg);
        },

        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = riscv64_op_freg(*pos);
            line.push_str(&reg);
        },

        LtacArg::RetRegI8 | LtacArg::RetRegU8 |
        LtacArg::RetRegI16 | LtacArg::RetRegU16 |
        LtacArg::RetRegI32 | LtacArg::RetRegU32 |
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("a0"),

        LtacArg::Byte(val) => line.push_str(&val.to_string()),
        LtacArg::UByte(val) => line.push_str(&val.to_string()),

        LtacArg::I16(val) => line.push_str(&val.to_string()),
        LtacArg::U16(val) => line.push_str(&val.to_string()),
    
        LtacArg::I32(val) => line.push_str(&val.to_string()),
        LtacArg::U32(val) => line.push_str(&val.to_string()),

        LtacArg::I64(val) => line.push_str(&val.to_string()),
        LtacArg::U64(val) => line.push_str(&val.to_string()),

        LtacArg::F32(ref val) => {
            line.push_str("%lo(");
            line.push_str(&val);
            line.push_str(")(a2)");
        },

        _ => {},
    }

    // Write the rest out
    line.push_str("\n");

    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_mov] Write failed.");
}

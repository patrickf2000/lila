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

use parser::ltac::{LtacInstr, LtacArg};
use crate::utils::*;

// Builds function/sytem call arguments
pub fn riscv64_build_pusharg(writer : &mut BufWriter<File>, code : &LtacInstr, is_karg : bool, stack_top : i32) {
    let mut reg = riscv64_arg_reg(code.arg2_val);

    if is_karg {
        reg = riscv64_karg_reg(code.arg2_val);
    }

    let mut line = String::new();

    match &code.arg1 {

        LtacArg::Mem(val) => {
            let pos = stack_top - (*val);

            match &code.arg2 {
                LtacArg::I16(_v) => line.push_str("  lh "),
                LtacArg::U16(_v) => line.push_str("  lhu "),

                _ => line.push_str("  lw "),
            }
            
            line.push_str(&reg);
            line.push_str(", -");
            line.push_str(&pos.to_string());
            line.push_str("(s0)\n");
        },

        LtacArg::Ptr(val) => {
            let pos = stack_top - (*val) + 8;

            line.push_str("  ld ");
            line.push_str(&reg);
            line.push_str(", -");
            line.push_str(&pos.to_string());
            line.push_str("(s0)\n");
        },

        // TODO: Clean this up
        LtacArg::I16(val) => {
            line.push_str("  li ");
            line.push_str(&reg);
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::U16(val) => {
            line.push_str("  li ");
            line.push_str(&reg);
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
        
        LtacArg::I32(val) => {
            line.push_str("  li ");
            line.push_str(&reg);
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::U32(val) => {
            line.push_str("  li ");
            line.push_str(&reg);
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
    
        LtacArg::PtrLcl(ref val) => {
            line.push_str("  lui a5, %hi(");
            line.push_str(val);
            line.push_str(")\n");

            line.push_str("  addi ");
            line.push_str(&reg);
            line.push_str(", a5, %lo(");
            line.push_str(val);
            line.push_str(")\n");
        },

        _ => {},
    }

    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_pusharg] Write failed.");
}

// Builds a function call
// Param: name
pub fn riscv64_build_call(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  call ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_call] Write failed.");
}


// This file is part of the Lila compiler
// Copyright (C) 2020-2021 Patrick Flynn
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

//Builds the move-to-vector instruction for integers
pub fn amd64_build_vector_instr(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let instr : String;
    
    match code.arg2 {
        LtacArg::Mem(pos) if code.arg2_val != -1 => {
            line.push_str("  mov r15, QWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::MemOffsetMem(pos, offset_pos, _s) => {
            if code.arg2_val != -1 {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            }
            
            line.push_str("  mov r14d, DWORD PTR [rbp-");
            line.push_str(&offset_pos.to_string());
            line.push_str("]\n");
        },
        
        _ => {},
    }
    
    match &code.instr_type {
        LtacType::MovI32Vec =>  instr = "  vmovups ".to_string(),
        LtacType::I32VAdd => instr = "  vaddps ".to_string(),
        _ => return,
    }
    
    match &code.arg1 {
        LtacArg::Mem(pos) => {
            line.push_str("  mov r15, QWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            line.push_str(&instr);
            line.push_str("[r15], ");
        },
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_vector_i32(*pos);
            
            line.push_str(&instr);
            line.push_str(&reg);
            line.push_str(", ");
            
            if code.instr_type != LtacType::MovI32Vec {
                line.push_str(&reg);
                line.push_str(", ");
            }
        },
        
        LtacArg::I32(_v) => {},
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Mem(_p) => {
            line.push_str("[r15]\n");
        },
        
        LtacArg::MemOffsetImm(_p, offset) => {
            line.push_str("[r15+");
            line.push_str(&offset.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::MemOffsetMem(_p, _op, size) => {
            line.push_str("[r15+r14*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I32(_v) => {},
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_vector_i32(*pos);
            
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    // Write everything to the file
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_mov_vector] Write failed.");
}


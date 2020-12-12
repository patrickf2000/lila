
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


// This module translates the RISC load/store instructions
// Although x86 doesn't have these, we can replicate it with the general move instructions
//
// You ideally wouldn't be using these in real life, this is only if you need to test RISC
// transformation on an x86 machine, which quite frankly is more realistic since Raspberry PI's and emulators
// are slow.

use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::utils::*;

// Build the store instructions
pub fn amd64_build_load_store(writer : &mut BufWriter<File>, code : &LtacInstr, is_load : bool) {
    let mut line = String::new();
    
    match code.instr_type {
        LtacType::LdB | LtacType::LdUB |
        LtacType::LdW | LtacType::LdUW |
        LtacType::Ld | LtacType::LdU |
        LtacType::LdQ | LtacType::LdUQ |
        
        LtacType::StrB | LtacType::StrUB |
        LtacType::StrW | LtacType::StrUW |
        LtacType::Str | LtacType::StrU |
        LtacType::StrQ | LtacType::StrUQ => line = "  mov ".to_string(),
        
        LtacType::LdF32 | LtacType::StrF32 => line = "  movss ".to_string(),
        LtacType::LdF64 | LtacType::StrF64 => line = "  movsd ".to_string(),
        
        _ => {},
    }
    
    let pos = match &code.arg1 {
        LtacArg::Mem(pos) => *pos,
        
        LtacArg::MemOffsetImm(pos, offset) if !is_load => {
            if code.instr_type == LtacType::StrF32 || code.instr_type == LtacType::StrF64 {
                line = "  mov ".to_string();
            }
            
            line.push_str("r15, QWORD PTR ");
            
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            line.push_str("  add r15, ");
            line.push_str(&offset.to_string());
            line.push_str("\n");
            
            match &code.arg2 {
                LtacArg::Reg8(_v) => line.push_str("  mov BYTE PTR "),
                LtacArg::Reg16(_v) => line.push_str("  mov WORD PTR "),
                LtacArg::Reg32(_v) => line.push_str("  mov DWORD PTR "),
                LtacArg::Reg64(_v) => line.push_str("  mov QWORD PTR "),
                LtacArg::FltReg(_v) => line.push_str("  movss "),
                LtacArg::FltReg64(_v) => line.push_str("  movsd "),
                _ => line.push_str("  mov "),
            };
            line.push_str("[r15], ");
            
            0
        },
        
        LtacArg::MemOffsetMem(pos, offset, size) if !is_load => {
            if code.instr_type == LtacType::StrF32 || code.instr_type == LtacType::StrF64 {
                line = "  mov ".to_string();
            }
            
            // Load the variable
            line.push_str("r15d, DWORD PTR ");
            
            line.push_str("[rbp-");
            line.push_str(&offset.to_string());
            line.push_str("]\n");
            
            // Load the effective address
            line.push_str("  lea r14, ");
            line.push_str("[0+r15*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
            
            // Load the array
            line.push_str("  mov r15, QWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            // Add to get the proper offset
            line.push_str("  add r15, r14\n");
            
            // Now set up for the final move
            match &code.arg2 {
                LtacArg::Reg8(_v) => line.push_str("  mov BYTE PTR "),
                LtacArg::Reg16(_v) => line.push_str("  mov WORD PTR "),
                LtacArg::Reg64(_v) => line.push_str("  mov QWORD PTR "),
                LtacArg::FltReg(_v) => line.push_str("  movss "),
                LtacArg::FltReg64(_v) => line.push_str("  movsd "),
                _ => line.push_str("  mov "),
            }
            line.push_str("[r15], ");
            
            0
        },
        
        LtacArg::MemOffsetReg(pos, reg, size) if !is_load => {
            if code.instr_type == LtacType::StrF32 || code.instr_type == LtacType::StrF64 {
                line = "  mov ".to_string();
            }
            
            // Determine the right register
            let src_reg = amd64_op_reg32(*reg);
            let size_mod : String;
            let mov_instr : String;
            
            match &code.arg2 {
                LtacArg::Reg8(_v) => { size_mod = "BYTE PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::Reg16(_v) => { size_mod = "WORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::Reg64(_v) => { size_mod = "QWORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::FltReg(_v) => { size_mod = "DWORD PTR".to_string(); mov_instr = "  movss ".to_string(); },
                LtacArg::FltReg64(_v) => { size_mod = "QWORD PTR".to_string(); mov_instr = "  movsd ".to_string(); },
                _ => { size_mod = "DWORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
            }
        
            // Load the array
            line.push_str("r15, QWORD PTR ");
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            // Load the effective address
            line.push_str("  lea r14, ");
            
            line.push_str("[0+");
            line.push_str(&src_reg);
            line.push_str("*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
            
            // Add to get the proper offset
            line.push_str("  add r15, r14\n");
            
            // Now set up for the final move
            line.push_str(&mov_instr);
            line.push_str(&size_mod);
            line.push_str("[r15], ");
            
            0
        },
        
        // ========================================================================================
        // Load
        LtacArg::MemOffsetImm(pos, offset) if is_load => {
            if code.instr_type == LtacType::LdF32 || code.instr_type == LtacType::LdF64 {
                line = "  mov ".to_string();
            }
            
            line.push_str("r15, QWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            match &code.arg2 {
                LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15+"),
                LtacArg::Reg16(_p) => line.push_str("  mov r15w, WORD PTR [r15+"),
                LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15+"),
                LtacArg::FltReg(_p) => line.push_str("  movss xmm1, DWORD PTR [r15+"),
                LtacArg::FltReg64(_p) => line.push_str("  movsd xmm1, QWORD PTR [r15+"),
                _ => line.push_str("  mov r15d, DWORD PTR [r15+"),
            }
            
            line.push_str(&offset.to_string());
            line.push_str("]\n");
            
            0
        },
        
        LtacArg::MemOffsetMem(pos, offset, size) if is_load => {
            if code.instr_type == LtacType::LdF32 || code.instr_type == LtacType::LdF64 {
                line = "  mov ".to_string();
            }
            
            // Load the variable
            line.push_str("r15d, DWORD PTR ");
            
            line.push_str("[rbp-");
            line.push_str(&offset.to_string());
            line.push_str("]\n");
            
            // Load the effective address
            line.push_str("  lea r14, ");
            line.push_str("[0+r15*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
            
            // Load the array
            line.push_str("  mov r15, QWORD PTR ");
            
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            // Add to get the proper offset
            line.push_str("  add r15, r14\n");
            
            // Store
            match &code.arg2 {
                LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15]\n"),
                LtacArg::Reg16(_p) => line.push_str("  mov r15w, WORD PTR [r15]\n"),
                LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15]\n"),
                LtacArg::FltReg(_p) => line.push_str("  movss xmm1, DWORD PTR [r15]\n"),
                LtacArg::FltReg64(_p) => line.push_str("  movsd xmm1, QWORD PTR [r15]\n"),
                _ => line.push_str("  mov r15d, DWORD PTR [r15]\n"),
            }
            
            0
        },
        
        LtacArg::MemOffsetReg(pos, reg, size) if is_load => {
            if code.instr_type == LtacType::LdF32 || code.instr_type == LtacType::LdF64 {
                line = "  mov ".to_string();
            }
            
            // Determine the right register
            let src_reg = amd64_op_reg32(*reg);
            
            // Load the array
            line.push_str("r15, QWORD PTR ");
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            // Load the effective address
            line.push_str("  lea r14, ");
            line.push_str("[0+");
            line.push_str(&src_reg);
            line.push_str("*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
            
            // Add to get the proper offset
            line.push_str("  add r15, r14\n");
            
            // Store
            match &code.arg2 {
                LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15]\n"),
                LtacArg::Reg16(_p) => line.push_str("  mov r15w, WORD PTR [r15]\n"),
                LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15]\n"),
                LtacArg::FltReg(_p) => line.push_str("  movss xmm1, DWORD PTR [r15]\n"),
                LtacArg::FltReg64(_p) => line.push_str("  movsd xmm1, QWORD PTR [r15]\n"),
                _ => line.push_str("  mov r15d, DWORD PTR [r15]\n"),
            }
            
            0
        },
        
        _ => 0,
    };
    
    let (reg, src_reg) : (String, String) = match &code.arg2 {
        LtacArg::Reg8(pos) => (amd64_op_reg8(*pos), "r15b".to_string()),
        LtacArg::Reg16(pos) => (amd64_op_reg16(*pos), "r15w".to_string()),
        LtacArg::Reg32(pos) => (amd64_op_reg32(*pos), "r15d".to_string()),
        LtacArg::Reg64(pos) => (amd64_op_reg64(*pos), "r15".to_string()),
        LtacArg::FltReg(pos) => (amd64_op_flt(*pos), "xmm1".to_string()),
        LtacArg::FltReg64(pos) => (amd64_op_flt(*pos), "xmm1".to_string()),
        
        _ => (String::new(), "r15d".to_string()),
    };
    
    let mov : String = match &code.arg2 {
        LtacArg::FltReg(_p) => "  movss ".to_string(),
        LtacArg::FltReg64(_p) => "  movsd ".to_string(),
        
        _ => "  mov ".to_string(),
    };
    
    if is_load {
        if pos == 0 {
            line.push_str(&mov);
            line.push_str(&reg);
            line.push_str(", ");
            line.push_str(&src_reg);
            line.push_str("\n");
        } else {
            line.push_str(&reg);
            line.push_str(", [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        }
    } else {
        if pos != 0 {
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("], ");
        }
        
        line.push_str(&reg);
        line.push_str("\n");
    }
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_store] Store failed.");
}


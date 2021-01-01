
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

// Builds an extern declaration
pub fn amd64_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(".extern ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_extern] Write failed.");
}

// Builds a label
pub fn amd64_build_label(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(&code.name);
    line.push_str(":\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_label] Write failed.");
}

// Builds a function
// Params: name -> function name
//         arg1_val -> stack size
pub fn amd64_build_func(writer : &mut BufWriter<File>, code : &LtacInstr, is_pic : bool) {
    let mut line = String::new();

    line.push_str("\n.global ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    if is_pic {
        line.push_str(".type ");
        line.push_str(&code.name);
        line.push_str(", @function\n");
    }
    
    line.push_str(&code.name);
    line.push_str(":\n");
    
    line.push_str("  push rbp\n");
    line.push_str("  mov rbp, rsp\n");
    
    if code.arg1_val > 0 {
        line.push_str("  sub rsp, ");
        line.push_str(&code.arg1_val.to_string());
        line.push_str("\n");
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_func] Write failed.");
}

// Load a function argument to a variable
// In the LtacInstr:
//      -> arg1_val = memory location
//      -> arg2_val = register position
pub fn amd64_build_ldarg(writer : &mut BufWriter<File>, code : &LtacInstr, is_pic : bool) {
    let mut line = String::new();
    
    match &code.arg1 {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
        },
        
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
        },
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
        },
        
        LtacArg::Mem(pos) => {
            if is_pic {
                line.push_str("  mov -");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]");
            } else {
                line.push_str("  mov [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]");
            }
        },
        _ => {},
    }
    
    line.push_str(", ");
    
    if code.instr_type == LtacType::LdArgI8 || code.instr_type == LtacType::LdArgU8 {
        let reg = amd64_arg_reg8(code.arg2_val);
        line.push_str(&reg);
    } else if code.instr_type == LtacType::LdArgI16 || code.instr_type == LtacType::LdArgU16 {
        let reg = amd64_arg_reg16(code.arg2_val);
        line.push_str(&reg);
    } else if code.instr_type == LtacType::LdArgI32 || code.instr_type == LtacType::LdArgU32 {
        let reg = amd64_arg_reg32(code.arg2_val);
        line.push_str(&reg);
    } else if code.instr_type == LtacType::LdArgI64 || code.instr_type == LtacType::LdArgU64 ||
            code.instr_type == LtacType::LdArgPtr {
        let reg = amd64_arg_reg64(code.arg2_val);
        line.push_str(&reg);
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_ldarg] Write failed.");
}

// Loads a 32-bit floating point argument to a variable
// In the LtacInstr:
//      -> arg1_val = memory location
//      -> arg2_val = register position
pub fn amd64_build_ldarg_float(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let reg = amd64_arg_flt(code.arg2_val);
    
    if code.instr_type == LtacType::LdArgF32 {
        line.push_str("  cvtsd2ss ");
        line.push_str(&reg);
        line.push_str(", ");
        line.push_str(&reg);
        line.push_str("\n");
        
        line.push_str("  movss ");
    } else {
        line.push_str("  movsd ");
    }
    
    match &code.arg1 {
        LtacArg::Mem(pos) => {
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]");
        },
        
        LtacArg::FltReg(pos) => {
            let reg = amd64_op_flt(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            line.push_str(&reg);
        },
        
        _ => {},
    }
    
    line.push_str(", ");
    line.push_str(&reg);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_ldarg_f32] Write failed.");
}

// Builds a return statement
// Yes, we could do this more cleanly, but I want to make it obvious what I'm doing.
pub fn amd64_build_ret(writer : &mut BufWriter<File>) {
    let mut line = String::new();
    line.push_str("\n");
    line.push_str("  leave\n");
    line.push_str("  ret\n");
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_ret] Write failed.");
}


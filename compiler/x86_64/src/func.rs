
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

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::asm::*;

// Builds an extern declaration
pub fn amd64_build_extern(x86_code : &mut Vec<X86Instr>, code : &LtacInstr) {
    let mut instr = create_x86instr(X86Type::Extern);
    instr.name = code.name.clone();
    
    x86_code.push(instr);
}

// Builds a label
pub fn amd64_build_label(x86_code : &mut Vec<X86Instr>, code : &LtacInstr) {
    let mut instr = create_x86instr(X86Type::Label);
    instr.name = code.name.clone();
    
    x86_code.push(instr);
}

// Builds a function
// Params: name -> function name
//         arg1_val -> stack size
pub fn amd64_build_func(x86_code : &mut Vec<X86Instr>, code : &LtacInstr, is_pic : bool) {
    let mut instr = create_x86instr(X86Type::Global);
    instr.name = code.name.clone();
    x86_code.push(instr.clone());
    
    if is_pic {
        instr = create_x86instr(X86Type::Type);
        instr.name = code.name.clone();
        x86_code.push(instr.clone());
    }
    
    instr = create_x86instr(X86Type::Label);
    instr.name = code.name.clone();
    x86_code.push(instr.clone());
    
    // Setup the stack
    // push rbp
    // mov rbp, rsp
    // sub rsp, stack_size
    //
    
    instr = create_x86instr(X86Type::Push);
    instr.arg1 = X86Arg::Reg64(X86Reg::RBP);
    x86_code.push(instr.clone());
    
    instr = create_x86instr(X86Type::Mov);
    instr.arg1 = X86Arg::Reg64(X86Reg::RBP);
    instr.arg2 = X86Arg::Reg64(X86Reg::RSP);
    x86_code.push(instr.clone());
    
    instr = create_x86instr(X86Type::Sub);
    instr.arg1 = X86Arg::Reg64(X86Reg::RSP);
    instr.arg2 = X86Arg::Imm32(code.arg1_val);
    x86_code.push(instr);
}

// Builds a return statement
pub fn amd64_build_ret(x86_code : &mut Vec<X86Instr>) {
    let mut instr = create_x86instr(X86Type::Leave);
    x86_code.push(instr.clone());
    
    instr = create_x86instr(X86Type::Ret);
    x86_code.push(instr);
}

// Load a function argument to a variable
// In the LtacInstr:
//      -> arg1_val = memory location
//      -> arg2_val = register position
pub fn amd64_build_ldarg(x86_code : &mut Vec<X86Instr>, code : &LtacInstr, _is_pic : bool) {
    let mut instr = create_x86instr(X86Type::Mov);
    
    match &code.arg1 {
        LtacArg::Reg8(pos) => instr.arg1 = amd64_op_reg8(*pos),
        LtacArg::Reg16(pos) => instr.arg1 = amd64_op_reg16(*pos),
        LtacArg::Reg32(pos) => instr.arg1 = amd64_op_reg32(*pos),
        LtacArg::Reg64(pos) => instr.arg1 = amd64_op_reg64(*pos),
        
        LtacArg::Mem(pos) => {
            /*if is_pic {
                line.push_str("  mov -");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]");
            } else {*/
                instr.arg1 = X86Arg::Mem(X86Reg::RBP, *pos);
            //}
        },
        _ => {},
    }
    
    match code.instr_type {
        LtacType::LdArgI8 | LtacType::LdArgU8 => instr.arg2 = amd64_arg_reg8(code.arg2_val),
        LtacType::LdArgI16 | LtacType::LdArgU16 => instr.arg2 = amd64_arg_reg16(code.arg2_val),
        LtacType::LdArgI32 | LtacType::LdArgU32 => instr.arg2 = amd64_arg_reg32(code.arg2_val),
        LtacType::LdArgI64 | LtacType::LdArgU64 
        | LtacType::LdArgPtr => instr.arg2 = amd64_arg_reg64(code.arg2_val),
        
        _ => {},
    }
    
    x86_code.push(instr);
}


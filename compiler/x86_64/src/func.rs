
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

use parser::ltac::{LtacInstr};
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


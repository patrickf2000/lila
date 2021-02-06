
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

extern crate llvm_sys as llvm;

use std::ffi::CString;
use llvm::*;
use llvm::core::*;

use parser::llir::{LLirInstr, LLirArg, LLirDataType};
use crate::*;

pub unsafe fn llvm_build_arith(builder : &mut Builder, line : &LLirInstr) {
    // TODO: Tipdeketo
    let op_type = LLVMInt32TypeInContext(builder.context);
    
    let lval = match &line.arg2 {
        LLirArg::Int(val) => LLVMConstInt(op_type, *val as u64, 1),
        _ => return,
    };
    
    let rval = match &line.arg3 {
        LLirArg::Int(val) => LLVMConstInt(op_type, *val as u64, 1),
        _ => return,
    };
    
    let dest_pos = match &line.arg1 {
        LLirArg::Reg(pos) => *pos,
        _ => 0,
    };
    
    let c_dest_name = CString::new(dest_pos.to_string()).unwrap();
    let dest = LLVMBuildAdd(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _);
    
    builder.regs.insert(dest_pos, dest);
}



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
use llvm::core::*;

use parser::llir::{LLirInstr, LLirArg};
use crate::*;

unsafe fn llvm_build_load(builder : &mut Builder, var_name : String) -> LLVMValueRef {
    let var = match builder.vars.get(&var_name) {
        Some(v) => v.clone(),
        _ => MaybeUninit::uninit().assume_init(),
    };
    
    let mut reg_str = "reg".to_string();
    reg_str.push_str(&builder.reg_pos.to_string());
    builder.reg_pos += 1;
    
    let reg_name = CString::new(reg_str).unwrap();
    let reg = LLVMBuildLoad(builder.builder, var, reg_name.as_ptr() as *const _);
    return reg;
}

pub unsafe fn llvm_build_arith(builder : &mut Builder, line : &LLirInstr) {
    // TODO: Tipdeketo
    let op_type = LLVMInt32TypeInContext(builder.context);
    
    let lval = match &line.arg2 {
        LLirArg::Int(val) => LLVMConstInt(op_type, *val as u64, 1),
        LLirArg::Mem(val) => llvm_build_load(builder, val.to_string()),
        _ => return,
    };
    
    let rval = match &line.arg3 {
        LLirArg::Int(val) => LLVMConstInt(op_type, *val as u64, 1),
        LLirArg::Mem(val) => llvm_build_load(builder, val.to_string()),
        _ => return,
    };
    
    let dest_pos = match &line.arg1 {
        LLirArg::Reg(pos) => *pos,
        _ => 0,
    };
    
    let c_dest_name = CString::new(dest_pos.to_string()).unwrap();
    
    let dest : LLVMValueRef = match &line.instr_type {
        LLirType::Add => LLVMBuildAdd(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        LLirType::Sub => LLVMBuildSub(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        
        LLirType::Mul | LLirType::UMul
            => LLVMBuildMul(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
            
        LLirType::Div => LLVMBuildSDiv(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        LLirType::UDiv => LLVMBuildUDiv(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        
        LLirType::Rem => LLVMBuildSRem(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        LLirType::URem => LLVMBuildURem(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        
        LLirType::And => LLVMBuildAnd(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        LLirType::Or => LLVMBuildOr(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        LLirType::Xor => LLVMBuildXor(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        
        LLirType::Lsh => LLVMBuildShl(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        LLirType::Rsh => LLVMBuildLShr(builder.builder, lval, rval, c_dest_name.as_ptr() as *const _),
        
        _ => MaybeUninit::uninit().assume_init(),
    };
    
    builder.regs.insert(dest_pos, dest);
}


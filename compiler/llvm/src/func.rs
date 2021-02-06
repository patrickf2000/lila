
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

// Konstrui LLVM funkiojn kaj eksterjan funkiojn
pub fn llvm_build_func(builder : &mut Builder, line : &LLirInstr, is_extern : bool) {
    unsafe {
        // Krei la funkcion
        let func_name = match &line.arg1 {
            LLirArg::Label(name) => name.clone(),
            _ => String::new(),
        };
        
        let func_type : LLVMTypeRef;
        match &line.data_type {
            LLirDataType::Int => func_type = LLVMInt32TypeInContext(builder.context),
            _ => func_type = LLVMVoidTypeInContext(builder.context),
        }
        
        let mut args = [];
        let function_type = LLVMFunctionType(func_type, args.as_mut_ptr(), args.len() as u32, 0);
        
        let c_str = CString::new(func_name.clone()).unwrap();
        let func = LLVMAddFunction(builder.module, c_str.as_ptr() as *const _, function_type);
        LLVMSetLinkage(func, LLVMLinkage::LLVMExternalLinkage);
        
        builder.funcs.insert(func_name.clone(), func);
        
        if !is_extern {
            // Agordi la funkcion blokon
            let mut bb_name = "bb_".to_string();
            bb_name.push_str(&func_name);
            let block_name = CString::new(bb_name).unwrap();
            
            let func_block = LLVMAppendBasicBlockInContext(builder.context, func, block_name.as_ptr() as *const _);
            LLVMPositionBuilderAtEnd(builder.builder, func_block);
        }
    }
}

// Konstrui LLVM funkion alvokon
pub unsafe fn llvm_build_call(builder : &mut Builder, line : &LLirInstr) {
    let call_args = match &line.arg2 {
        LLirArg::ArgList(list) => list,
        _ => return,
    };
    
    let mut args : Vec<LLVMValueRef> = Vec::new();
    
    for arg in call_args {
        match &arg {
            LLirArg::StrLiteral(val) => {
                /*let c_str = CString::new(val.clone()).unwrap();
                let str_ref = LLVMConstString(c_str.as_ptr() as *const _, val.len() as u32, 0);
                args.push(str_ref);*/
                
                let str_name : String = "str1".to_string();
                let c_str_name = CString::new(str_name).unwrap();
                
                let c_str = CString::new(val.clone()).unwrap();
                let str_ref = LLVMBuildGlobalString(builder.builder, c_str.as_ptr() as *const _, c_str_name.as_ptr() as *const _);
                args.push(str_ref);
            },
            
            _ => {},
        }
    }
    
    // La funkio
    let func_name = match &line.arg1 {
        LLirArg::Label(val) => val.clone(),
        _ => String::new(),
    };
    
    let func = match &builder.funcs.get(&func_name) {
        Some(v) => *v.clone(),
        _ => return,
    };
    
    // La alvoko
    LLVMBuildCall(builder.builder, func, args.as_mut_ptr(), args.len() as u32, func_name.as_ptr() as *const _);
}

// Konstrui LLVM funkion revenon
pub fn llvm_build_return(builder : &mut Builder, line : &LLirInstr) {
    unsafe {
        match &line.arg1 {
            LLirArg::None => {LLVMBuildRetVoid(builder.builder);},
            
            // TODO: Pli tipdetekton?
            LLirArg::Int(val) => {
                let i32_type = LLVMInt32TypeInContext(builder.context);
                let i32_val = LLVMConstInt(i32_type, *val as u64, 1);
                
                LLVMBuildRet(builder.builder, i32_val);
            },
            
            LLirArg::Reg(val) => {
                let reg = match &builder.regs.get(val) {
                    Some(v) => *v.clone(),
                    _ => return,
                };
                
                LLVMBuildRet(builder.builder, reg);
            },
            
            _ => {},
        }
    }
}


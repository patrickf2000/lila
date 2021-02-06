
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

use std::io;
use std::mem::MaybeUninit;
use std::ffi::CString;
use std::collections::HashMap;

use llvm::prelude::*;
use llvm::core::*;
use llvm::target::*;
use llvm::target_machine::*;

use parser::llir::{LLirFile, LLirInstr, LLirType, LLirArg};

mod func;

use crate::func::*;

pub struct Builder {
    context : LLVMContextRef,
    module : LLVMModuleRef,
    builder : LLVMBuilderRef,
    
    funcs : HashMap<String, LLVMValueRef>,
    vars : HashMap<String, LLVMValueRef>,
    regs : HashMap<i32, LLVMValueRef>,
}

pub fn compile(llir_file : &LLirFile) -> io::Result<()> {
    unsafe {
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithNameInContext(b"first\0".as_ptr() as *const _, context);
        let builder = LLVMCreateBuilderInContext(context);
        
        // Start generating
        let mut builder_struct = Builder {
            context : context,
            module : module,
            builder : builder,
            funcs : HashMap::new(),
            vars : HashMap::new(),
            regs : HashMap::new(),
        };
        write_code(&mut builder_struct, &llir_file.code);
        
        // Create a function
        /*let i32t = LLVMInt32TypeInContext(context);
        
        let mut args = [i32t, i32t, i32t];
        let function_type = LLVMFunctionType(i32t, args.as_mut_ptr(), args.len() as u32, 0);
        let function = LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);
        
        // Create the block
        let block = LLVMAppendBasicBlockInContext(context, function, b"entry\0".as_ptr() as *const _);
        LLVMPositionBuilderAtEnd(builder, block);
        
        // Load arguments
        let x = LLVMGetParam(function, 0);
        let y = LLVMGetParam(function, 1);
        let z = LLVMGetParam(function, 2);
        
        let sum = LLVMBuildAdd(builder, x, y, b"sum.1\0".as_ptr() as *const _);
        let sum = LLVMBuildAdd(builder, sum, z, b"sum.2\0".as_ptr() as *const _);
        
        LLVMBuildRet(builder, sum);*/
        
        // Dump module
        LLVMDumpModule(module);
        
        // Setup the machine
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmParsers();
        LLVM_InitializeAllAsmPrinters();
        
        let triple = LLVMGetDefaultTargetTriple();

        let mut target : LLVMTargetRef = MaybeUninit::uninit().assume_init();
        let mut err = MaybeUninit::uninit().assume_init();
        LLVMGetTargetFromTriple(triple, &mut target, &mut err);
        
        let cpu = LLVMGetHostCPUName();
        let features = LLVMGetHostCPUFeatures();
        let opt = LLVMCodeGenOptLevel::LLVMCodeGenLevelNone;
        let reloc = LLVMRelocMode::LLVMRelocDefault;
        let code = LLVMCodeModel::LLVMCodeModelDefault;
        
        let machine = LLVMCreateTargetMachine(target, triple, cpu, features, opt, reloc, code);
        
        // Generate the assembly
        LLVMTargetMachineEmitToFile(machine, module, b"/tmp/first.asm\0".as_ptr() as *mut _, LLVMCodeGenFileType::LLVMAssemblyFile, &mut err);
        
        /*let err_str = CStr::from_ptr(err).to_string_lossy().into_owned();
        println!("{:?}", err_str);*/
        
        LLVMDisposeMessage(cpu);
        LLVMDisposeMessage(features);
        LLVMDisposeMessage(triple);
        LLVMDisposeTargetMachine(machine);
        
        // Clean up
        //LLVMDumpModule(module);
        LLVMDisposeBuilder(builder);
        LLVMContextDispose(context);
    }
    
    Ok(())
}

pub unsafe fn write_code(builder : &mut Builder, code : &Vec<LLirInstr>) {
    for ln in code {
        match ln.instr_type {
            LLirType::Extern => llvm_build_func(builder, ln, true),
            LLirType::Func => llvm_build_func(builder, ln, false),
            LLirType::Call => llvm_build_call(builder, ln),
            LLirType::Ret => llvm_build_return(builder, ln),
            
            LLirType::AllocArr
            | LLirType::AllocB | LLirType::AllocW
            | LLirType::AllocDW | LLirType::AllocQW
            | LLirType::AllocF32 | LLirType::AllocF64 => llvm_build_alloc(builder, ln),
            
            LLirType::LdB | LLirType::UldB
            | LLirType::LdW | LLirType::UldW
            | LLirType::LdDW | LLirType::UldDW
            | LLirType::LdQW | LLirType::UldQW
            | LLirType::LdF32
            | LLirType::LdF64 => llvm_build_load(builder, ln),
            
            LLirType::StrB | LLirType::UstrB
            | LLirType::StrW | LLirType::UstrW
            | LLirType::StrDW | LLirType::UstrDW
            | LLirType::StrQW | LLirType::UstrQW
            | LLirType::StrF32
            | LLirType::StrF64 => llvm_build_store(builder, ln),
            
            _ => {},
        }
    }
}

// Konstruas alloc instrukcion
pub unsafe fn llvm_build_alloc(builder : &mut Builder, line : &LLirInstr) {
    let var_type : LLVMTypeRef;
    
    match &line.instr_type {
        LLirType::AllocB => var_type = LLVMInt8TypeInContext(builder.context),
        LLirType::AllocW => var_type = LLVMInt16TypeInContext(builder.context),
        LLirType::AllocDW => var_type = LLVMInt32TypeInContext(builder.context),
        LLirType::AllocQW => var_type = LLVMInt64TypeInContext(builder.context),
        
        _ => return,
    }
    
    let name = match &line.arg1 {
        LLirArg::Label(name) => name.clone(),
        _ => String::new(),
    };
    
    let c_str = CString::new(name.clone()).unwrap();
    let var = LLVMBuildAlloca(builder.builder, var_type, c_str.as_ptr() as *const _);
    
    builder.vars.insert(name, var);
}

// Konstruas ŝarĝo instrukcion
pub unsafe fn llvm_build_load(builder : &mut Builder, line : &LLirInstr) {
    let name = match &line.arg2 {
        LLirArg::Mem(name) => name.clone(),
        _ => String::new(),
    };
    
    let var = match &builder.vars.get(&name) {
        Some(v) => *v.clone(),
        _ => return,
    };
    
    // The register to load to
    let reg_no = match &line.arg1 {
        LLirArg::Reg(val) => *val,
        _ => -1,
    };
    
    let c_str = CString::new(reg_no.to_string()).unwrap();
    let reg = LLVMBuildLoad(builder.builder, var, c_str.as_ptr() as *const _);
    
    builder.regs.insert(reg_no, reg);
}

// Konstruas vendejo instrukcion
pub unsafe fn llvm_build_store(builder : &mut Builder, line : &LLirInstr) {
    let val = match &line.arg2 {
        LLirArg::Int(val) if line.instr_type == LLirType::StrDW => {
            let i32_t = LLVMInt32TypeInContext(builder.context);
            LLVMConstInt(i32_t, *val as u64, 1)
        },
        
        _ => return,
    };
    
    let name = match &line.arg1 {
        LLirArg::Mem(name) => name.clone(),
        _ => String::new(),
    };
    
    let var = match &builder.vars.get(&name) {
        Some(v) => *v.clone(),
        _ => return,
    };
    
    LLVMBuildStore(builder.builder, val, var as LLVMValueRef);
}


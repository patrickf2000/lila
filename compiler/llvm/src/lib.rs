
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
//use std::ffi::CStr;

use llvm::prelude::*;
use llvm::core::*;
use llvm::target::*;
use llvm::target_machine::*;

use parser::llir::{LLirFile, LLirInstr, LLirType};

mod func;

use crate::func::*;

pub struct Builder {
    context : LLVMContextRef,
    module : LLVMModuleRef,
    builder : LLVMBuilderRef,
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

pub fn write_code(builder : &mut Builder, code : &Vec<LLirInstr>) {
    for ln in code {
        match ln.instr_type {
            LLirType::Func => llvm_build_func(builder, ln),
            LLirType::Ret => llvm_build_return(builder, ln),
            _ => {},
        }
    }
}



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


use std::collections::HashMap;

use parser::ltac;
use parser::ltac::{LtacFile, LtacType, LtacArg};
use parser::Arch;

// Import any local modules
mod risc;
mod riscv;

use risc::*;
use riscv::*;

// Architectures
// 1-> x86-64
// 2-> AArch64
// 3-> Riscv64

// The main transformation function
pub fn run(file : &LtacFile, arch : Arch, use_c : bool, risc_mode : bool) -> Result<LtacFile, ()> {
    let mut file2 = match check_builtins(file, use_c) {
        Ok(ltac) => ltac,
        Err(_e) => return Err(()),
    };
    
    if risc_mode || arch == Arch::AArch64 || arch == Arch::Riscv64 {
        file2 = match risc_optimize(&file2) {
            Ok(ltac) => ltac,
            Err(_e) => return Err(()),
        }
    }
    
    if arch == Arch::Riscv64 {
        file2 = match riscv_optimize(&file2) {
            Ok(ltac) => ltac,
            Err(_e) => return Err(()),
        }
    }
    
    Ok(file2)
}

// Scans the code for malloc, free, and exit instructions
// If we are using the C libraries, these are simply transforms to a function call
// Otherwise, we must transform them to a system call
fn check_builtins(file : &LtacFile, use_c : bool) -> Result<LtacFile, ()> {
    let mut file2 = LtacFile {
        name : file.name.clone(),
        data : file.data.clone(),
        code : Vec::new(),
    };
    
    let code = file.code.clone();
    let mut index = 0;
    
    let mut mm_map : HashMap<i32, i32> = HashMap::new();
    
    for line in code.iter() {
        match &line.instr_type {
            
            // We have a separate exit type for two reasons
            // First, when we exit, we want to make sure to de-allocate everything
            // Second, because "exit" is a keyword, the corelib function has a different name
            LtacType::Exit => {
                let mut instr = ltac::create_instr(LtacType::PushArg);
                instr.arg1 = line.arg1.clone();
                instr.arg2_val = 1;
                file2.code.push(instr);
                    
                if use_c {
                    instr = ltac::create_instr(LtacType::Call);
                    instr.name = "exit".to_string();
                    file2.code.push(instr);
                } else {
                    instr = ltac::create_instr(LtacType::Call);
                    instr.name = "sys_exit".to_string();
                    file2.code.push(instr);
                }
            },
        
            LtacType::Malloc => {
                if use_c {
                    let mut instr = ltac::create_instr(LtacType::Call);
                    instr.name = "malloc".to_string();
                    file2.code.push(instr);
                } else {
                    let size_instr = code.iter().nth(index-1).unwrap();
                    let pos_instr = code.iter().nth(index+1).unwrap();
                    file2.code.pop();
                    
                    // Push the memory location and size to the hash map
                    let pos = match &pos_instr.arg1 {
                        LtacArg::Mem(pos) => *pos,
                        LtacArg::Ptr(pos) => *pos,
                        _ => 0,
                    };
                    
                    let size = match &size_instr.arg1 {
                        LtacArg::I32(val) => *val,
                        _ => 0,
                    };
                    
                    mm_map.insert(pos, size);
                    
                    // Make the call
                    let mut instr = ltac::create_instr(LtacType::PushArg);
                    instr.arg1 = size_instr.arg1.clone();
                    instr.arg2_val = 1;
                    file2.code.push(instr.clone());
                    
                    instr = ltac::create_instr(LtacType::Call);
                    instr.name = "malloc".to_string();
                    file2.code.push(instr);
                }
            },
            
            LtacType::Free => {
                if use_c {
                    let mut instr = ltac::create_instr(LtacType::Call);
                    instr.name = "free".to_string();
                    file2.code.push(instr);
                } else {
                    let addr_instr = code.iter().nth(index-1).unwrap();
                    file2.code.pop();
                    
                    // Address
                    let mut instr = ltac::create_instr(LtacType::PushArg);
                    instr.arg1 = addr_instr.arg1.clone();
                    instr.arg2_val = 1;
                    file2.code.push(instr.clone());
                    
                    // Memory segment size
                    let pos = match &addr_instr.arg1 {
                        LtacArg::Ptr(pos) => *pos,
                        _ => 0,
                    };
                    
                    match &mm_map.get(&pos) {
                        Some(size) => instr.arg1 = LtacArg::I32(**size),
                        None => instr.arg1 = LtacArg::I32(0),
                    }
                    
                    instr.arg2_val = 2;
                    file2.code.push(instr.clone());
                    
                    // The system call
                    instr = ltac::create_instr(LtacType::Call);
                    instr.name = "free".to_string();
                    file2.code.push(instr.clone());
                }
            },
            
            _ => file2.code.push(line.clone()),
        }
        
        index += 1;
    }
    
    Ok(file2)
}



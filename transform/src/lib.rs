
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
    let mut file2 = match check_builtins(file, arch, use_c) {
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
fn check_builtins(file : &LtacFile, arch : Arch, use_c : bool) -> Result<LtacFile, ()> {
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
            LtacType::Exit => {
                if use_c {
                    let mut instr = ltac::create_instr(LtacType::PushArg);
                    instr.arg1 = line.arg1.clone();
                    instr.arg2_val = 1;
                    file2.code.push(instr);
                    
                    instr = ltac::create_instr(LtacType::Call);
                    instr.name = "exit".to_string();
                    file2.code.push(instr);
                } else {
                    // System call number (for exit)
                    let mut instr = ltac::create_instr(LtacType::KPushArg);
                    instr.arg2_val = 1;
                    
                    match arch {
                        Arch::X86_64 => instr.arg1 = LtacArg::I32(60),       // Linux x86-64
                        Arch::AArch64 => instr.arg1 = LtacArg::I32(93),       // Linux AArch64
                        Arch::Riscv64 => {},
                    };
                    
                    file2.code.push(instr.clone());
                    
                    // Exit code
                    instr.arg1 = line.arg1.clone();
                    instr.arg2_val = 2;
                    file2.code.push(instr.clone());
                    
                    // The system call
                    instr = ltac::create_instr(LtacType::Syscall);
                    file2.code.push(instr.clone());
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
                    mm_map.insert(pos_instr.arg1_val, size_instr.arg1_val);
                    
                    // System call number (for mmap)
                    let mut instr = ltac::create_instr(LtacType::KPushArg);
                    instr.arg2_val = 1;
                    
                    match arch {
                        Arch::X86_64 => instr.arg1 = LtacArg::I32(9),
                        Arch::AArch64 => instr.arg1 = LtacArg::I32(222),
                        Arch::Riscv64 => {},
                    };
                    
                    file2.code.push(instr.clone());
                    
                    // Address (0 by default)
                    instr.arg1 = LtacArg::I32(0);
                    instr.arg2_val = 2;
                    file2.code.push(instr.clone());
                    
                    // Memory segment size
                    instr.arg1 = size_instr.arg1.clone();
                    instr.arg2_val = 3;
                    file2.code.push(instr.clone());
                    
                    // All other are various flags and stuff
                    instr.arg1 = LtacArg::I32(3);
                    instr.arg2_val = 4;
                    file2.code.push(instr.clone());
                    
                    instr.arg1 = LtacArg::I32(34);
                    instr.arg2_val = 5;
                    file2.code.push(instr.clone());
                    
                    instr.arg1 = LtacArg::I32(-1);
                    instr.arg2_val = 6;
                    file2.code.push(instr.clone());
                    
                    instr.arg1 = LtacArg::I32(0);
                    instr.arg2_val = 7;
                    file2.code.push(instr.clone());
                    
                    // The system call
                    instr = ltac::create_instr(LtacType::Syscall);
                    file2.code.push(instr.clone());
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
                    
                    // System call number (for munmap)
                    let mut instr = ltac::create_instr(LtacType::KPushArg);
                    instr.arg2_val = 1;
                    
                    match arch {
                        Arch::X86_64 => instr.arg1 = LtacArg::I32(11),
                        Arch::AArch64 => instr.arg1 = LtacArg::I32(215),
                        Arch::Riscv64 => {},
                    };
                    
                    file2.code.push(instr.clone());
                    
                    // Address
                    instr.arg1 = addr_instr.arg1.clone();
                    instr.arg2_val = 2;
                    file2.code.push(instr.clone());
                    
                    // Memory segment size
                    match &mm_map.get(&addr_instr.arg1_val) {
                        Some(size) => instr.arg1_val = **size,
                        None => {},
                    }
                    
                    instr.arg2_val = 3;
                    file2.code.push(instr.clone());
                    
                    // The system call
                    instr = ltac::create_instr(LtacType::Syscall);
                    file2.code.push(instr.clone());
                }
            },
            
            _ => file2.code.push(line.clone()),
        }
        
        index += 1;
    }
    
    Ok(file2)
}



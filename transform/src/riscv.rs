
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

// This is a specialized module for performing RISC-V specific optimizations (basically, to deal
// with RISC-V quirks...)

use parser::ltac;
use parser::ltac::{LtacFile, LtacType, LtacArg};

// The main RISC-V optimizer loop
pub fn riscv_optimize(file : &LtacFile) -> Result<LtacFile, ()> {
    let mut file2 = LtacFile {
        name : file.name.clone(),
        data : file.data.clone(),
        code : Vec::new(),
    };
    
    let code = file.code.clone();
    
    for line in code.iter() {
        let mut instr2 = line.clone();
        
        file2.code.push(instr2);
    }
    
    Ok(file2)
}
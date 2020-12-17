
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
    let mut index = 0;
    
    for line in code.iter() {
        let mut instr2 = line.clone();
        let mut arg_count = 1;
        
        // If we are making a printf call with a float argument, we have to do some
        // special conversions.
        if instr2.instr_type == LtacType::PushArg {
            let flt_arg : bool;
            
            match &instr2.arg2 {
                LtacArg::FltReg(_) => flt_arg = true,
                LtacArg::FltReg64(_) => flt_arg = false,
                _ => {
                    arg_count += 1;
                    flt_arg = false;
                },
            }
            
            if !flt_arg {
                file2.code.push(instr2);
                index += 1;
                continue;
            }
            
            let mut function_name = String::new();
            
            for i in index .. code.len() {
                let current = code.iter().nth(i).unwrap();
                
                if current.instr_type == LtacType::Call {
                    function_name = current.name.clone();
                    break;
                } else if current.instr_type == LtacType::PushArg {
                    match &current.arg2 {
                        LtacArg::FltReg(_) | LtacArg::FltReg64(_) => {},
                        _ => arg_count += 1,
                    }
                }
            }
            
            if function_name != "printf" {
                file2.code.push(instr2);
                index += 1;
                continue;
            }
            
            // If we make it this far, we have found a printf call
            // First, convert the pusharg to a regular f32.ld
            instr2.instr_type = LtacType::LdF32;
            let pos = match &instr2.arg2 {
                LtacArg::FltReg(pos) => *pos,
                _ => 0,
            };
            
            file2.code.push(instr2);
            
            // Now, convert to a double
            let mut cvt_instr = ltac::create_instr(LtacType::CvtF32F64);
            cvt_instr.arg1 = LtacArg::FltReg64(pos);
            cvt_instr.arg2 = LtacArg::FltReg(pos);
            file2.code.push(cvt_instr);
            
            // Now, move it to an integer register
            let mut mv_instr = ltac::create_instr(LtacType::MovF32Int);
            mv_instr.arg1 = LtacArg::Reg32(0);
            mv_instr.arg2 = LtacArg::FltReg64(pos);
            file2.code.push(mv_instr);
            
            // Finally, create the pusharg
            let mut pusharg = ltac::create_instr(LtacType::PushArg);
            pusharg.arg1 = LtacArg::Reg32(0);
            pusharg.arg2_val = arg_count + 1;
            file2.code.push(pusharg);
            
            index += 1;
        } else {
            file2.code.push(instr2);
        }
        
        index += 1;
    }
    
    Ok(file2)
}

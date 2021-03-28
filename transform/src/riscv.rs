
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

// This is a specialized module for performing RISC-V specific optimizations (basically, to deal
// with RISC-V quirks...)

use crate::ltac;
use crate::ltac::{LtacFile, LtacType, LtacArg};

// The main RISC-V optimizer loop
pub fn riscv_optimize(file : &LtacFile) -> Result<LtacFile, ()> {
    let mut file2 = LtacFile {
        name : file.name.clone(),
        data : file.data.clone(),
        code : Vec::new(),
    };
    
    let code = file.code.clone();
    let mut skip_next = false;
    
    for index in 0 .. code.len() {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        let line = code.iter().nth(index).unwrap();
        
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
            let mut mv_instr = ltac::create_instr(LtacType::MovF64Int);
            mv_instr.arg1 = LtacArg::Reg32(0);
            mv_instr.arg2 = LtacArg::FltReg64(pos);
            file2.code.push(mv_instr);
            
            // Finally, create the pusharg
            let mut pusharg = ltac::create_instr(LtacType::PushArg);
            pusharg.arg1 = LtacArg::Reg32(0);
            pusharg.arg2_val = arg_count + 1;
            file2.code.push(pusharg);
            
        // For some odd reason, you cannot move between float registers on RISC-V
        } else if instr2.instr_type == LtacType::MovF32 {
            match &instr2.arg2 {
                LtacArg::FltReg(_p) | LtacArg::FltReg64(_p) => {},
                
                _ => {
                    file2.code.push(instr2);
                    continue;
                },
            }
            
            let mut next_instr = code.iter().nth(index + 1).unwrap().clone();
            
            if next_instr.instr_type == LtacType::StrF32 || next_instr.instr_type == LtacType::StrF64 {
                next_instr.arg2 = instr2.arg1.clone();
            } else {
                next_instr.arg1 = instr2.arg2.clone();
            }
            
            file2.code.push(next_instr);
            
            skip_next = true;
        
        // Otherwise, just add the current line
        } else {
            file2.code.push(instr2);
        }
    }
    
    Ok(file2)
}

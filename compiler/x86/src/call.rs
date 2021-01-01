
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

use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacArg};
use crate::utils::*;

// Builds a function argument
pub fn amd64_build_pusharg(writer : &mut BufWriter<File>, code : &LtacInstr, is_karg : bool, is_pic : bool) {
    let mut line = "  mov ".to_string();
    
    // Get the argument registers
    let mut reg32 = amd64_arg_reg32(code.arg2_val);
    let mut reg64 = amd64_arg_reg64(code.arg2_val);
    let mut reg_flt = amd64_arg_flt(code.arg2_val);
    
    if is_karg {
        reg32 = amd64_karg_reg32(code.arg2_val);
        reg64 = amd64_karg_reg64(code.arg2_val);
    }
    
    // Determine move type
    match &code.arg1 {
        LtacArg::Reg8(_p) => line = "  movzx ".to_string(),
        LtacArg::F32(_p) => line = "  movss ".to_string(),
        LtacArg::F64(_p) => line = "  movsd ".to_string(),
        _ => {},
    }
    
    match code.arg2 {
        LtacArg::Byte(_v) => line = "  movsx ".to_string(),
        LtacArg::UByte(_v) => line = "  movzx ".to_string(),
        LtacArg::I16(_v) => line = "  movsx ".to_string(),
        LtacArg::U16(_v) => line = "  movzx ".to_string(),
        
        LtacArg::FltReg(pos) => {
            line = "  movss ".to_string();
            reg_flt = amd64_arg_flt(pos);
        },
        
        LtacArg::FltReg64(pos) => {
            line = "  movsd ".to_string();
            reg_flt = amd64_arg_flt(pos);
        },
        
        _ => {},
    }
    
    // Assemble
    match &code.arg1 {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&reg);
        },
        
        LtacArg::Reg16(_p) => {},
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&reg);
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            
            line.push_str(&reg64);
            line.push_str(", ");
            line.push_str(&reg);
        },
        
        LtacArg::FltReg(_p) => {},
        LtacArg::FltReg64(_p) => {},
        
        LtacArg::RetRegI32 => {},
        LtacArg::RetRegI64 => {},
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => {},
        
        LtacArg::Mem(pos) => {
            match code.arg2 {
                LtacArg::FltReg(_p) | LtacArg::FltReg64(_p) => {
                    line.push_str(&reg_flt);
                    line.push_str(", ");
                },
                
                LtacArg::Byte(_v) => {
                    line.push_str(&reg32);
                    line.push_str(", BYTE PTR ");
                },
                
                LtacArg::UByte(_v) => {
                    line.push_str(&reg32);
                    line.push_str(", BYTE PTR ");
                },
                
                LtacArg::I16(_v) => {
                    line.push_str(&reg32);
                    line.push_str(", WORD PTR ");
                },
                
                LtacArg::U16(_v) => {
                    line.push_str(&reg32);
                    line.push_str(", WORD PTR ");
                },
                
                LtacArg::I64(_v) => {
                    line.push_str(&reg64);
                    line.push_str(", QWORD PTR ");
                },
                
                LtacArg::U64(_v) => {
                    line.push_str(&reg64);
                    line.push_str(", QWORD PTR ");
                },
            
                _ => {
                    line.push_str(&reg32);
                    line.push_str(", ");
                },
            }
            
            if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]");
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]");
            }
        },
        
        // Literals are always passed as unsigned
        LtacArg::UByte(val) => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::U16(val) => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::I32(val) => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::U32(val) => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::F32(ref val) => {
            line.push_str(&reg_flt);
            line.push_str(", DWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]");
        },
        
        LtacArg::F64(ref val) => {
            line.push_str(&reg_flt);
            line.push_str(", QWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]");
        },
        
        LtacArg::Ptr(pos) => {
            line.push_str(&reg64);
            if is_pic {
                line.push_str(", -");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]");
            } else {
                line.push_str(", [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]");
            }
        },
        
        LtacArg::PtrLcl(ref val) => {
            if is_pic {
                line = "  lea ".to_string();
                line.push_str(&reg64);
                line.push_str(", ");
                line.push_str(&val);
                line.push_str("[rip]");
            } else {
                line.push_str(&reg64);
                line.push_str(", OFFSET FLAT:");
                line.push_str(&val);
            }
        },
        
        _ => {},
    }
    
    line.push_str("\n");
    
    // If we have a 32-bit float variable, we have to convert it to a double
    //TODO: combine
    match code.arg2 {
        LtacArg::FltReg(_p) => {
            line.push_str("  cvtss2sd ");
            line.push_str(&reg_flt);
            line.push_str(", ");
            line.push_str(&reg_flt);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg1 {
        LtacArg::F32(_v) => {
            line.push_str("  cvtss2sd ");
            line.push_str(&reg_flt);
            line.push_str(", ");
            line.push_str(&reg_flt);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_pusharg Write failed.");
}

// Builds a function call
// Param: name
pub fn amd64_build_call(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  call ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_call] Write failed.");
}

// Builds a system call
pub fn amd64_build_syscall(writer : &mut BufWriter<File>) {
    writer.write(b"  syscall\n\n")
        .expect("[AMD64_build_syscall] Write failed.");
}


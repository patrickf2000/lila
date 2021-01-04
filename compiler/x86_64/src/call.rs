
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

use parser::ltac::{LtacInstr, LtacArg};
use crate::asm::*;

// Builds a function argument
pub fn amd64_build_pusharg(x86_code : &mut Vec<X86Instr>, code : &LtacInstr, is_karg : bool, _is_pic : bool) {
    // Get the argument registers
    let mut reg32 = amd64_arg_reg32(code.arg2_val);
    let mut reg64 = amd64_arg_reg64(code.arg2_val);
    //let mut reg_flt = amd64_arg_flt(code.arg2_val);
    
    if is_karg {
        reg32 = amd64_karg_reg32(code.arg2_val);
        reg64 = amd64_karg_reg64(code.arg2_val);
    }
    
    // Determine move type
    let mut mov_type = X86Type::Mov;
    
    match &code.arg1 {
        LtacArg::Reg8(_p) => mov_type = X86Type::MovZX,
        LtacArg::F32(_p) => mov_type = X86Type::MovSS,
        LtacArg::F64(_p) => mov_type = X86Type::MovSD,
        _ => {},
    }
    
    match code.arg2 {
        LtacArg::Byte(_v) => mov_type = X86Type::MovSX,
        LtacArg::UByte(_v) => mov_type = X86Type::MovZX,
        LtacArg::I16(_v) => mov_type = X86Type::MovSX,
        LtacArg::U16(_v) => mov_type = X86Type::MovZX,
        
        // TODO: What do those second lines for the float registers do?
        LtacArg::FltReg(_pos) => {
            mov_type = X86Type::MovSS;
            //reg_flt = amd64_arg_flt(pos);
        },
        
        LtacArg::FltReg64(_pos) => {
            mov_type = X86Type::MovSD;
            //reg_flt = amd64_arg_flt(pos);
        },
        
        _ => {},
    }
    
    let mut instr = create_x86instr(mov_type);
    
    // Assemble
    match &code.arg1 {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            instr.arg1 = reg32;
            instr.arg2 = amd64_op_reg8(*pos);
        },
        
        LtacArg::Reg16(_p) => {},
        
        LtacArg::Reg32(pos) => {
            instr.arg1 = reg32;
            instr.arg2 = amd64_op_reg32(*pos);
        },
        
        LtacArg::Reg64(pos) => {
            instr.arg1 = reg64;
            instr.arg2 = amd64_op_reg64(*pos);
        },
        
        LtacArg::FltReg(_p) => {},
        LtacArg::FltReg64(_p) => {},
        
        LtacArg::RetRegI32 => {},
        LtacArg::RetRegI64 => {},
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => {},
        
        LtacArg::Mem(pos) => {
            match code.arg2 {
                /*LtacArg::FltReg(_p) | LtacArg::FltReg64(_p) => {
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
                },*/
            
                _ => instr.arg1 = reg32,
            }
            
            /*if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]");
            } else {*/
                instr.arg2 = X86Arg::DwordMem(X86Reg::RBP, *pos);
            //}
        },
        
        // Literals are always passed as unsigned
        /*LtacArg::UByte(val) => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::U16(val) => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
        },*/
        
        LtacArg::I32(val) => {
            instr.arg1 = reg32;
            instr.arg2 = X86Arg::Imm32(*val);
        },
        
        LtacArg::U32(val) => {
            instr.arg1 = reg32;
            instr.arg2 = X86Arg::Imm32(*val as i32);
        },
        
        /*LtacArg::F32(ref val) => {
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
        },*/
        
        LtacArg::PtrLcl(ref val) => {
            /*if is_pic {
                line = "  lea ".to_string();
                line.push_str(&reg64);
                line.push_str(", ");
                line.push_str(&val);
                line.push_str("[rip]");
            } else {*/
                instr.arg1 = reg64;
                instr.arg2 = X86Arg::LclMem(val.to_string());
            //}
        },
        
        _ => {},
    }
    
    /*line.push_str("\n");
    
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
        .expect("[AMD64_build_pusharg Write failed.");*/
        
    x86_code.push(instr);
}

// Builds a function call
// Param: name
pub fn amd64_build_call(x86_code : &mut Vec<X86Instr>, code : &LtacInstr) {
    let mut instr = create_x86instr(X86Type::Call);
    instr.name = code.name.clone();
    
    x86_code.push(instr);
}

// Builds a system call
pub fn amd64_build_syscall(x86_code : &mut Vec<X86Instr>) {
    let instr = create_x86instr(X86Type::Syscall);
    x86_code.push(instr);
}


//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use parser::ltac::{LtacInstr, LtacArg};
use crate::asm::*;

// Builds a function argument
pub fn amd64_build_pusharg(x86_code : &mut Vec<X86Instr>, code : &LtacInstr, is_karg : bool, is_pic : bool) {
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
        LtacArg::PtrLcl(_p) if is_pic => mov_type = X86Type::Lea,
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
        
        LtacArg::Reg16(pos) => {
            instr.arg1 = reg32;
            instr.arg2 = amd64_op_reg16(*pos);
        },
        
        LtacArg::Reg32(pos) => {
            instr.arg1 = reg32;
            instr.arg2 = amd64_op_reg32(*pos);
        },
        
        LtacArg::Reg64(pos) => {
            instr.arg1 = reg64;
            instr.arg2 = amd64_op_reg64(*pos);
        },
        
        LtacArg::Mem(pos) => {
            match code.arg2 {
                LtacArg::Byte(_v) => {
                    instr.arg1 = reg32;
                    instr.arg2 = X86Arg::BwordMem(X86Reg::RBP, *pos, is_pic);
                },
                
                LtacArg::UByte(_v) => {
                    instr.arg1 = reg32;
                    instr.arg2 = X86Arg::BwordMem(X86Reg::RBP, *pos, is_pic);
                },
                
                LtacArg::I16(_v) => {
                    instr.arg1 = reg32;
                    instr.arg2 = X86Arg::WordMem(X86Reg::RBP, *pos, is_pic);
                },
                
                LtacArg::U16(_v) => {
                    instr.arg1 = reg32;
                    instr.arg2 = X86Arg::WordMem(X86Reg::RBP, *pos, is_pic);
                },
                
                LtacArg::I64(_v) => {
                    instr.arg1 = reg64;
                    instr.arg2 = X86Arg::QwordMem(X86Reg::RBP, *pos, is_pic);
                },
                
                LtacArg::U64(_v) => {
                    instr.arg1 = reg64;
                    instr.arg2 = X86Arg::QwordMem(X86Reg::RBP, *pos, is_pic);
                },
            
                _ => {
                    instr.arg1 = reg32;
                    instr.arg2 = X86Arg::DwordMem(X86Reg::RBP, *pos, is_pic);
                },
            }
        },
        
        // Literals are always passed as unsigned
        LtacArg::UByte(val) => {
            instr.arg1 = reg32;
            instr.arg2 = X86Arg::Imm32(*val as i32);
        },
        
        LtacArg::U16(val) => {
            instr.arg1 = reg32;
            instr.arg2 = X86Arg::Imm32(*val as i32);
        },
        
        LtacArg::I32(val) => {
            instr.arg1 = reg32;
            instr.arg2 = X86Arg::Imm32(*val);
        },
        
        LtacArg::U32(val) => {
            instr.arg1 = reg32;
            instr.arg2 = X86Arg::Imm32(*val as i32);
        },
        
        LtacArg::Ptr(pos) => {
            instr.arg1 = reg64;
            instr.arg2 = X86Arg::QwordMem(X86Reg::RBP, *pos, is_pic);
        },
        
        LtacArg::PtrLcl(ref val) => {
            instr.arg1 = reg64;
            instr.arg2 = X86Arg::LclMem(val.to_string(), is_pic);
        },
        
        _ => {},
    }
        
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


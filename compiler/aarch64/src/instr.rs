//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::asm::*; 

// Builds a load or store instruction
pub fn arm64_build_ld_str(code : &mut Vec<Arm64Instr>, instr : &LtacInstr, stack_size : i32) {
    let mut line : Arm64Instr;
    let is_store : bool;
    
    match instr.instr_type {
        LtacType::Str => {
            line = create_arm64_instr(Arm64Type::Str);
            is_store = true;
        },
        
        LtacType::Ld => {
            line = create_arm64_instr(Arm64Type::Ldr);
            is_store = false;
        },
        
        _ => return,
    }
    
    // Build store arguments
    if is_store {
        match instr.arg1 {
            LtacArg::Mem(val) => line.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - val),
            
            _ => return,
        }
        
        match instr.arg2 {
            LtacArg::Reg8(val) | LtacArg::Reg16(val)
            | LtacArg::Reg32(val) => line.arg1 = Arm64Arg::Reg(arm64_arg_reg32(val)),
            
            _ => return,
        }
    // Build load arguments
    } else {
        match instr.arg1 {
            LtacArg::Reg8(val) | LtacArg::Reg16(val)
            | LtacArg::Reg32(val) => line.arg1 = Arm64Arg::Reg(arm64_arg_reg32(val)),
            
            LtacArg::RetRegI8 | LtacArg::RetRegU8
            | LtacArg::RetRegI16 | LtacArg::RetRegU16
            | LtacArg::RetRegI32 | LtacArg::RetRegU32 => {
                line.arg1 = Arm64Arg::Reg(Arm64Reg::W0);
            },
            
            _ => return,
        }
        
        match instr.arg2 {
            LtacArg::Mem(val) => line.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - val),
            _ => return,
        }
    }
    
    code.push(line);
}

// Builds a move instruction
pub fn arm64_build_mov(code : &mut Vec<Arm64Instr>, instr : &LtacInstr) {
    let mut mov = create_arm64_instr(Arm64Type::Mov);
    
    match instr.arg1 {
        LtacArg::Reg8(val) | LtacArg::Reg16(val)
        | LtacArg::Reg32(val) => mov.arg1 = Arm64Arg::Reg(arm64_arg_reg32(val)),
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8
        | LtacArg::RetRegI16 | LtacArg::RetRegU16
        | LtacArg::RetRegI32 | LtacArg::RetRegU32 => {
            mov.arg1 = Arm64Arg::Reg(Arm64Reg::W0);
        },
        
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => mov.arg1 = Arm64Arg::Reg(Arm64Reg::X0),
        
        _ => {},
    }
    
    match instr.arg2 {
        LtacArg::Reg8(val) | LtacArg::Reg16(val)
        | LtacArg::Reg32(val) => mov.arg2 = Arm64Arg::Reg(arm64_arg_reg32(val)),
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8
        | LtacArg::RetRegI16 | LtacArg::RetRegU16
        | LtacArg::RetRegI32 | LtacArg::RetRegU32 => {
            mov.arg2 = Arm64Arg::Reg(Arm64Reg::W0);
        },
        
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => mov.arg2 = Arm64Arg::Reg(Arm64Reg::X0),
        
        LtacArg::I32(val) => mov.arg2 = Arm64Arg::Imm32(val),
        LtacArg::U32(val) => mov.arg2 = Arm64Arg::Imm32(val as i32),
        
        _ => {},
    }
    
    code.push(mov);
}

// Returns a register given a numerical value
fn arm64_arg_reg32(val : i32) -> Arm64Reg {
    match val {
        0 => Arm64Reg::W9,
        1 => Arm64Reg::W10,
        2 => Arm64Reg::W11,
        3 => Arm64Reg::W12,
        4 => Arm64Reg::W13,
        5 => Arm64Reg::W14,
        6 => Arm64Reg::W15,
        7 => Arm64Reg::W16,
        _ => Arm64Reg::W17,
    }
}

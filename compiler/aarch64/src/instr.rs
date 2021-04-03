//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use parser::ltac::{LtacInstr, LtacArg};
use crate::asm::*; 

// Builds a move instruction
pub fn arm64_build_mov(code : &mut Vec<Arm64Instr>, instr : &LtacInstr) {
    let mut mov = create_arm64_instr(Arm64Type::Mov);
    
    match instr.arg1 {
        /*LtacArg::Reg8(val) | LtacArg::Reg16(val)
        | LtacArg::Reg32(val) => mov.arg1 = arm64_arg_reg(val),*/
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8
        | LtacArg::RetRegI16 | LtacArg::RetRegU16
        | LtacArg::RetRegI32 | LtacArg::RetRegU32 => {
            mov.arg1 = Arm64Arg::Reg(Arm64Reg::W0);
        },
        
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => mov.arg1 = Arm64Arg::Reg(Arm64Reg::X0),
        
        _ => {},
    }
    
    match instr.arg2 {
        /*LtacArg::Reg8(val) | LtacArg::Reg16(val)
        | LtacArg::Reg32(val) => mov.arg1 = arm64_arg_reg(val),*/
        
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

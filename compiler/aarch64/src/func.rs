//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use parser::ltac::{LtacInstr, LtacArg};
use crate::asm::*;

// Builds a function declaration
pub fn arm64_build_func(code : &mut Vec<Arm64Instr>, instr : &LtacInstr) {
    let mut glbl = create_arm64_instr(Arm64Type::Global);
    glbl.name = instr.name.clone();
    code.push(glbl);

    let mut instr2 = create_arm64_instr(Arm64Type::Label);
    instr2.name = instr.name.clone();
    code.push(instr2);
    
    // stp x29, x30, [sp, -size]
    let size = (instr.arg1_val + 16) * -1;
    
    let mut stp = create_arm64_instr(Arm64Type::Stp);
    stp.arg1 = Arm64Arg::Reg(Arm64Reg::X29);
    stp.arg2 = Arm64Arg::Reg(Arm64Reg::X30);
    stp.arg3 = Arm64Arg::Mem(Arm64Reg::SP, size);
    code.push(stp);
    
    // mov x29, sp
    let mut mov = create_arm64_instr(Arm64Type::Mov);
    mov.arg1 = Arm64Arg::Reg(Arm64Reg::X29);
    mov.arg2 = Arm64Arg::Reg(Arm64Reg::SP);
    code.push(mov);
}

// Builds a function return
pub fn arm64_build_ret(code : &mut Vec<Arm64Instr>, stack_size : i32) {
    // ldp x29, x30, [sp], stack_size
    let mut ldp = create_arm64_instr(Arm64Type::Ldp);
    ldp.arg1 = Arm64Arg::Reg(Arm64Reg::X29);
    ldp.arg2 = Arm64Arg::Reg(Arm64Reg::X30);
    ldp.arg3 = Arm64Arg::RegRef(Arm64Reg::SP);
    ldp.arg4 = Arm64Arg::Imm32(stack_size);
    code.push(ldp);

    // ret
    let ret = create_arm64_instr(Arm64Type::Ret);
    code.push(ret);
}

// Builds a function pusharg statement
pub fn arm64_build_pusharg(code : &mut Vec<Arm64Instr>, instr : &LtacInstr, stack_size : i32, karg : bool) {
    let mut dest = arm64_arg_reg(instr.arg2_val);
    if karg {
        dest = arm64_karg_reg(instr.arg2_val);
    }
    
    match instr.arg1 {
        LtacArg::Mem(pos) => {
            let mut ld = create_arm64_instr(Arm64Type::Ldr);
            ld.arg1 = Arm64Arg::Reg(dest);
            ld.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - pos);
            code.push(ld);
        },
        
        LtacArg::I32(val) => {
            let mut mov = create_arm64_instr(Arm64Type::Mov);
            mov.arg1 = Arm64Arg::Reg(dest);
            mov.arg2 = Arm64Arg::Imm32(val);
            code.push(mov);
        },
        
        LtacArg::U32(val) => {
            let mut mov = create_arm64_instr(Arm64Type::Mov);
            mov.arg1 = Arm64Arg::Reg(dest);
            mov.arg2 = Arm64Arg::Imm32(val as i32);
            code.push(mov);
        },
        
        LtacArg::PtrLcl(ref val) => {
            let mut instr1 = create_arm64_instr(Arm64Type::Adrp);
            instr1.arg1 = Arm64Arg::Reg(dest.clone());
            instr1.arg2 = Arm64Arg::PtrLcl(val.clone());
            code.push(instr1);
            
            let mut instr2 = create_arm64_instr(Arm64Type::Add);
            instr2.arg1 = Arm64Arg::Reg(dest.clone());
            instr2.arg2 = Arm64Arg::Reg(dest);
            instr2.arg3 = Arm64Arg::PtrLclLow(val.clone());
            code.push(instr2);
        },
        
        _ => {},
    }
}

fn arm64_arg_reg(pos : i32) -> Arm64Reg {
    match pos {
        1 => Arm64Reg::X0,
        2 => Arm64Reg::X1,
        3 => Arm64Reg::X2,
        4 => Arm64Reg::X3,
        5 => Arm64Reg::X4,
        6 => Arm64Reg::X5,
        7 => Arm64Reg::X6,
        _ => Arm64Reg::X7,
    }
}

fn arm64_karg_reg(pos : i32) -> Arm64Reg {
    match pos {
        1 => Arm64Reg::X8,
        2 => Arm64Reg::X0,
        3 => Arm64Reg::X1,
        4 => Arm64Reg::X2,
        5 => Arm64Reg::X3,
        6 => Arm64Reg::X4,
        _ => Arm64Reg::X5,
    }
}

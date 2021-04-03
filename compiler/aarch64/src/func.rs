//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use parser::ltac::{LtacInstr};
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

//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

// This represents AArch64 instructions

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Arm64Type {
    Extern,
    Global,
    Label,
    
    Ldp,
    Stp,
    Adrp,
    Mov,
    
    Str,
    Ldr,
    
    Add,
    Sub,
    Mul,
    SDiv,
    MSub,
    
    And,
    Orr,
    Eor,
    Lsl,
    Lsr,
    
    Call,
    Ret,
    Svc
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Arm64Arg {
    Empty,
    
    Mem(Arm64Reg, i32),
    RegRef(Arm64Reg),
    
    Imm32(i32),
    Imm64(i64),
    
    PtrLcl(String),
    PtrLclLow(String),
    
    Reg(Arm64Reg)
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub enum Arm64Reg {
    SP,
    XZR,
    
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8,
    X9, X10, X11, X12, X13, X14, X15, X16, X17,
    X18,
    X19, X20, X21, X22, X23, X24, X25, X26, X27, X28,
    X29,
    X30,
    
    W0, W1, W2, W3, W4, W5, W6, W7,
    W8,
    W9, W10, W11, W12, W13, W14, W15, W16, W17,
    W18,
    W19, W20, W21, W22, W23, W24, W25, W26, W27, W28,
    W29,
    W30
}

#[derive(Clone)]
pub struct Arm64Instr {
    pub instr_type : Arm64Type,
    pub name : String,
    pub arg1 : Arm64Arg,
    pub arg2 : Arm64Arg,
    pub arg3 : Arm64Arg,
    pub arg4 : Arm64Arg,
}

pub fn create_arm64_instr(instr_type : Arm64Type) -> Arm64Instr{
    Arm64Instr {
        instr_type : instr_type,
        name : String::new(),
        arg1 : Arm64Arg::Empty,
        arg2 : Arm64Arg::Empty,
        arg3 : Arm64Arg::Empty,
        arg4 : Arm64Arg::Empty,
    }
}


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

// This represents x86-64 instructions

#[derive(Clone, PartialEq)]
pub enum X86Type {
    Extern,
    Global,
    Type,
    Label,

    Nop,
    
    Push,
    
    Mov,
    MovZX,
    MovSX,
    MovSS,
    MovSD,
    
    Add,
    Sub,
    IMul,
    Mul,
    IDiv,
    Div,
    
    And,
    Or,
    Xor,
    Shl,
    Shr,
    
    AddSS,
    SubSS,
    MulSS,
    DivSS,
    
    AddSD,
    SubSD,
    MulSD,
    DivSD,
    
    Cmp,
    Ucomiss,
    Ucomisd,
    
    Call,
    Syscall,
    Leave,
    Ret
}

#[derive(Clone, PartialEq)]
pub enum X86Arg {
    Empty,
    
    Mem(X86Reg, i32),
    DwordMem(X86Reg, i32),
    QwordMem(X86Reg, i32),
    LclMem(String),
    
    Imm32(i32),
    
    Reg64(X86Reg),
    Reg32(X86Reg),
    Reg16(X86Reg),
    Reg8(X86Reg),
    
    Xmm(i32),
    Ymm(i32),
}

// These are the 64-bit register names; in reality, we can use them for all types
#[derive(Clone, PartialEq)]
pub enum X86Reg {
    RAX,
    RBX,
    RCX,
    RDX,
    
    RSP,
    RBP,
    
    RDI,
    RSI,
    
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15
}

// Most x86-64 instructions have 1 or 2 arguments, but a few extended ones have 3
#[derive(Clone)]
pub struct X86Instr {
    pub instr_type : X86Type,
    pub name : String,
    pub arg1 : X86Arg,
    pub arg2 : X86Arg,
    pub arg3 : X86Arg,
}

// Utility functions
pub fn create_x86instr(instr_type : X86Type) -> X86Instr {
    X86Instr {
        instr_type : instr_type,
        name : String::new(),
        arg1 : X86Arg::Empty,
        arg2 : X86Arg::Empty,
        arg3 : X86Arg::Empty,
    }
}

pub fn reg2str(reg : &X86Reg, size : i32) -> String {
    if size == 32 {
        match reg {
            X86Reg::RAX => "eax".to_string(),
            X86Reg::RBX => "ebx".to_string(),
            X86Reg::RCX => "ecx".to_string(),
            X86Reg::RDX => "edx".to_string(),
            
            X86Reg::RSP => "esp".to_string(),
            X86Reg::RBP => "ebp".to_string(),
            
            X86Reg::RDI => "edi".to_string(),
            X86Reg::RSI => "esi".to_string(),
            
            X86Reg::R8 => "r8d".to_string(),
            X86Reg::R9 => "r9d".to_string(),
            X86Reg::R10 => "r10d".to_string(),
            X86Reg::R11 => "r11d".to_string(),
            X86Reg::R12 => "r12d".to_string(),
            X86Reg::R13 => "r13d".to_string(),
            X86Reg::R14 => "r14d".to_string(),
            X86Reg::R15 => "r15d".to_string()
        }
    } else {
        match reg {
            X86Reg::RAX => "rax".to_string(),
            X86Reg::RBX => "rbx".to_string(),
            X86Reg::RCX => "rcx".to_string(),
            X86Reg::RDX => "rdx".to_string(),
            
            X86Reg::RSP => "rsp".to_string(),
            X86Reg::RBP => "rbp".to_string(),
            
            X86Reg::RDI => "rdi".to_string(),
            X86Reg::RSI => "rsi".to_string(),
            
            X86Reg::R8 => "r8".to_string(),
            X86Reg::R9 => "r9".to_string(),
            X86Reg::R10 => "r10".to_string(),
            X86Reg::R11 => "r11".to_string(),
            X86Reg::R12 => "r12".to_string(),
            X86Reg::R13 => "r13".to_string(),
            X86Reg::R14 => "r14".to_string(),
            X86Reg::R15 => "r15".to_string()
        }
    }
}

// Gets a register based on position
// Kernel argument registers
pub fn amd64_karg_reg32(pos : i32) -> X86Arg {
    match pos {
        1 => return X86Arg::Reg32(X86Reg::RAX),
        2 => return X86Arg::Reg32(X86Reg::RDI),
        3 => return X86Arg::Reg32(X86Reg::RSI),
        4 => return X86Arg::Reg32(X86Reg::RDX),
        5 => return X86Arg::Reg32(X86Reg::R10),
        6 => return X86Arg::Reg32(X86Reg::R8),
        7 => return X86Arg::Reg32(X86Reg::R9),
        _ => return X86Arg::Empty,
    };
}

pub fn amd64_karg_reg64(pos : i32) -> X86Arg {
    match pos {
        1 => return X86Arg::Reg64(X86Reg::RAX),
        2 => return X86Arg::Reg64(X86Reg::RDI),
        3 => return X86Arg::Reg64(X86Reg::RSI),
        4 => return X86Arg::Reg64(X86Reg::RDX),
        5 => return X86Arg::Reg64(X86Reg::R10),
        6 => return X86Arg::Reg64(X86Reg::R8),
        7 => return X86Arg::Reg64(X86Reg::R9),
        _ => return X86Arg::Empty,
    };
}

// Function argument registers
pub fn amd64_arg_reg8(pos : i32) -> X86Arg {
    match pos {
        1 => return X86Arg::Reg8(X86Reg::RDI),
        2 => return X86Arg::Reg8(X86Reg::RSI),
        3 => return X86Arg::Reg8(X86Reg::RDX),
        4 => return X86Arg::Reg8(X86Reg::RCX),
        5 => return X86Arg::Reg8(X86Reg::R8),
        6 => return X86Arg::Reg8(X86Reg::R9),
        _ => return X86Arg::Empty,
    };
}

pub fn amd64_arg_reg16(pos : i32) -> X86Arg {
    match pos {
        1 => return X86Arg::Reg16(X86Reg::RDI),
        2 => return X86Arg::Reg16(X86Reg::RSI),
        3 => return X86Arg::Reg16(X86Reg::RDX),
        4 => return X86Arg::Reg16(X86Reg::RCX),
        5 => return X86Arg::Reg16(X86Reg::R8),
        6 => return X86Arg::Reg16(X86Reg::R9),
        _ => return X86Arg::Empty,
    };
}

pub fn amd64_arg_reg32(pos : i32) -> X86Arg {
    match pos {
        1 => return X86Arg::Reg32(X86Reg::RDI),
        2 => return X86Arg::Reg32(X86Reg::RSI),
        3 => return X86Arg::Reg32(X86Reg::RDX),
        4 => return X86Arg::Reg32(X86Reg::RCX),
        5 => return X86Arg::Reg32(X86Reg::R8),
        6 => return X86Arg::Reg32(X86Reg::R9),
        _ => return X86Arg::Empty,
    };
}

pub fn amd64_arg_reg64(pos : i32) -> X86Arg {
    match pos {
        1 => return X86Arg::Reg64(X86Reg::RDI),
        2 => return X86Arg::Reg64(X86Reg::RSI),
        3 => return X86Arg::Reg64(X86Reg::RDX),
        4 => return X86Arg::Reg64(X86Reg::RCX),
        5 => return X86Arg::Reg64(X86Reg::R8),
        6 => return X86Arg::Reg64(X86Reg::R9),
        _ => return X86Arg::Empty,
    };
}

pub fn amd64_arg_flt(pos : i32) -> X86Arg {
    match pos {
        1 => return X86Arg::Xmm(0),
        2 => return X86Arg::Xmm(1),
        3 => return X86Arg::Xmm(2),
        4 => return X86Arg::Xmm(3),
        5 => return X86Arg::Xmm(4),
        6 => return X86Arg::Xmm(5),
        7 => return X86Arg::Xmm(6),
        8 => return X86Arg::Xmm(7),
        _ => return X86Arg::Empty,
    };
}

// Operation registers
// EAX -> Return register
// R15, R14 -> Operations register
pub fn amd64_op_reg8(pos : i32) -> X86Arg {
    match pos {
        0 => return X86Arg::Reg8(X86Reg::RBX),
        1 => return X86Arg::Reg8(X86Reg::RCX),
        2 => return X86Arg::Reg8(X86Reg::R10),
        3 => return X86Arg::Reg8(X86Reg::R11),
        4 => return X86Arg::Reg8(X86Reg::R12),
        _ => return X86Arg::Empty,
    };
}

pub fn amd64_op_reg16(pos : i32) -> X86Arg {
    match pos {
        0 => return X86Arg::Reg16(X86Reg::RBX),
        1 => return X86Arg::Reg16(X86Reg::RCX),
        2 => return X86Arg::Reg16(X86Reg::R10),
        3 => return X86Arg::Reg16(X86Reg::R11),
        4 => return X86Arg::Reg16(X86Reg::R12),
        _ => return X86Arg::Empty,
    };
}

pub fn amd64_op_reg32(pos : i32) -> X86Arg {
    match pos {
        0 => return X86Arg::Reg32(X86Reg::RBX),
        1 => return X86Arg::Reg32(X86Reg::RCX),
        2 => return X86Arg::Reg32(X86Reg::R10),
        3 => return X86Arg::Reg32(X86Reg::R11),
        4 => return X86Arg::Reg32(X86Reg::R12),
        _ => return X86Arg::Empty,
    };
}

pub fn amd64_op_reg64(pos : i32) -> X86Arg {
    match pos {
        0 => return X86Arg::Reg64(X86Reg::RBX),
        1 => return X86Arg::Reg64(X86Reg::RCX),
        2 => return X86Arg::Reg64(X86Reg::R10),
        3 => return X86Arg::Reg64(X86Reg::R11),
        4 => return X86Arg::Reg64(X86Reg::R12),
        _ => return X86Arg::Empty,
    };
}

// xmm0 and xmm1 are reserved for internal operations
/*pub fn amd64_op_flt(pos : i32) -> String {
    match pos {
        0 => return "xmm10".to_string(),
        1 => return "xmm11".to_string(),
        2 => return "xmm12".to_string(),
        3 => return "xmm13".to_string(),
        4 => return "xmm14".to_string(),
        5 => return "xmm15".to_string(),
        _ => return String::new(),
    }
}

// Vector registers
// ymm0 and ymm1 are reserved for internal operations
pub fn amd64_vector_i32(pos : i32) -> String {
    match pos {
        0 => return "ymm3".to_string(),
        1 => return "ymm4".to_string(),
        2 => return "ymm5".to_string(),
        3 => return "ymm6".to_string(),
        4 => return "ymm7".to_string(),
        5 => return "ymm8".to_string(),
        _ => return String::new(),
    }
}*/


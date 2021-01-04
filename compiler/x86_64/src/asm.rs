
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
    
    Add,
    Sub,
    
    Call,
    Syscall,
    Leave,
    Ret
}

#[derive(Clone, PartialEq)]
pub enum X86Arg {
    Empty,
    
    DwordMem(i32),
    LclMem(String),
    
    Imm32(i32),
    
    Reg64(X86Reg),
    Reg32(X86Reg),
    Reg16(X86Reg),
    Reg8(X86Reg),
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


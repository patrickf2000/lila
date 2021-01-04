
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

pub enum X86Type {
    Extern,
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

pub enum X86Arg {
    DwordMem(i32),
    LclMem(String),
    
    Imm32(i32),
    
    Reg64(i32),
    Reg32(i32),
    Reg16(i32),
    Reg8(i32),
}

// Most x86-64 instructions have 1 or 2 arguments, but a few extended ones have 3
pub struct X86Instr {
    instr_type : X86Type,
    arg1 : X86Arg,
    arg2 : X86Arg,
    arg3 : X86Arg,
}


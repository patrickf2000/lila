
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

use std::collections::HashMap;

// Represents an instruction type
#[derive(Debug, Clone, PartialEq)]
pub enum LLirType {
    None,
    
    // Base and system instructions
    Label,
    Func,
    Extern,
    Ret,
    
    PushArg,
    KPushArg,
    Call,
    Syscall,
    
    // Integer arithmetic instructions
    Add,
    Sub,
    Mul,    UMul,
    Div,    UDiv,
    Rem,    URem,
    
    // Logic instructions
    And,
    Or,
    Xor,
    Lsh,
    Rsh,
    
    // Floating point instructions
    AddF32,     AddF64,
    SubF32,     SubF64,
    MulF32,     MulF64,
    DivF32,     DivF64,
    
    // Move instructions
    Mov,
    Li,
    MovSX,      MovZX,
    
    // Stack allocation instructions
    AllocB,
    AllocW,
    AllocDW,
    AllocQW,
    AllocF32,
    AllocF64,
    
    // Load and store instructions
    LdB,        UldB,
    LdW,        UldW,
    LdDW,       UldDW,
    LdQW,       UldQW,
    LdF32,
    LdF64,
    
    StrB,       UstrB,
    StrW,       UstrW,
    StrDW,      UstrDW,
    StrQW,      UstrQW,
    StrF32,
    StrF64,
    
    LdAddr,
    
    // Function argument loading
    LdArgB,         ULdArgB,
    LdArgW,         ULdArgW,
    LdArgDW,        ULdArgDW,
    LdArgQW,        ULdArgQW,
    LdArgF32,
    LdArgF64,
    
    // Flow control
    Jmp,
    CeqB,       CneqB,      // Equal / not equal
    CeqW,       CneqW,
    CeqDW,      CneqDW,
    CeqQW,      CneqQW,
    
    CsleB,      CuleB,      // Signed less than or equal / unsigned less than or equal
    CsleW,      CuleW,
    CsleDW,     CuleDW,
    CsleQW,     CuleQW,
    
    CsltB,      CultB,      // Signed less than / unsigned less than
    CsltW,      CultW,
    CsltDW,     CultDW,
    CsltQW,     CultQW,
    
    CsgeB,      CugeB,      // Signed greater than or equal / unsigned greater than or equal
    CsgeW,      CugeW,
    CsgeDW,     CugeDW,
    CsgeQW,     CugeQW,
    
    CsgtB,      CugtB,      // Signed greater than / unsigned greater than
    CsgtW,      CugtW,
    CsgtDW,     CugtDW,
    CsgtQW,     CugtQW
}

// Represents an LLIR instruction operand
#[derive(Debug, Clone, PartialEq)]
pub enum LLirArg {
    None,
    
    Int(i64), UInt(i64),
    
    Label(String),
    StrLiteral(String),
    
    Reg(i32),
    ArgReg(i32),
}

// Represents an LLIR instruction
#[derive(Debug, Clone, PartialEq)]
pub struct LLirInstr {
    instr_type : LLirType,
    arg1 : LLirArg,
    arg2 : LLirArg,
    arg3 : LLirArg,
}

// Represents an LLIR file
#[derive(Debug, Clone, PartialEq)]
pub struct LLirFile {
    pub name        : String,
    pub strings     : HashMap<String, String>,
    pub code        : Vec<LLirInstr>,
}

// Creates an LLIR instruction
pub fn create_instr(instr_type : LLirType) -> LLirInstr {
    LLirInstr {
        instr_type : instr_type,
        arg1 : LLirArg::None,
        arg2 : LLirArg::None,
        arg3 : LLirArg::None,
    }
}



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

use crate::ast::{AstStmt, AstModType, AstArgType};
use crate::llir;
use crate::llir::*;
use crate::llir_builder::*;

// Builds an LLIR variable declaration
// Note for array sizes:
//    Array sizes are 12 bytes long.
//    The first 8 bytes hold the pointer, and the second 4 hold the size
//
pub fn build_var_dec(builder : &mut LLirBuilder, line : &AstStmt) -> bool {
    let name = line.name.clone();
    let ast_data_type = &line.modifiers[0];
    let mut data_type = LLirDataType::Void;
    let sub_type = LLirDataType::Void;
    
    match &ast_data_type.mod_type {
        AstModType::Byte | AstModType::Char => data_type = LLirDataType::Byte,
        AstModType::UByte => data_type = LLirDataType::UByte,
        
        AstModType::Short => data_type = LLirDataType::Word,
        AstModType::UShort => data_type = LLirDataType::UWord,
        
        AstModType::Int => data_type = LLirDataType::Int,
        AstModType::UInt => data_type = LLirDataType::UInt,
        
        AstModType::Int64 => data_type = LLirDataType::Int64,
        AstModType::UInt64 => data_type = LLirDataType::UInt64,
        
        AstModType::Str => data_type = LLirDataType::Str,
        
        AstModType::ByteDynArray | AstModType::UByteDynArray
        | AstModType::ShortDynArray | AstModType::UShortDynArray
        | AstModType::IntDynArray | AstModType::UIntDynArray
        | AstModType::I64DynArray | AstModType::U64DynArray
        | AstModType::FloatDynArray | AstModType::DoubleDynArray
        | AstModType::StrDynArray => {
            // TODO
        },
        
        _ => return false,
    }
    
    // Krei la alloc instrukion
    let instr_type : LLirType;
    
    match &data_type {
        LLirDataType::Byte | LLirDataType::UByte => instr_type = LLirType::AllocB,
        LLirDataType::Word | LLirDataType::UWord => instr_type = LLirType::AllocW,
        LLirDataType::Int | LLirDataType::UInt => instr_type = LLirType::AllocDW,
        LLirDataType::Int64 | LLirDataType::UInt64 
        | LLirDataType::Str => instr_type = LLirType::AllocQW,
        LLirDataType::Ptr => instr_type = LLirType::AllocArr,
        
        _ => instr_type = LLirType::Nop,
    }
    
    let mut instr = llir::create_instr(instr_type);
    instr.data_type = data_type.clone();
    instr.arg1 = LLirArg::Label(name.clone());
    builder.add_code(instr);
    
    // Puŝi la variablon
    let var = Var {
        name : name,
        data_type : data_type,
        sub_type : sub_type,
    };
    
    if !build_expr(builder, line, &var) {
        return false;
    }
    
    builder.vars.push(var);
    
    true
}

// Konstrui variablon esprimon
pub fn build_expr(builder : &mut LLirBuilder, line : &AstStmt, var : &Var) -> bool {
    let args = &line.args;

    // Se ni havas unu argumento kaj ĝis laŭvorto, ni nur povas stoki.
    if args.len() == 1 {
        let first = args.first().unwrap();
        
        let mut instr = store_for_type(&var.data_type);
        instr.data_type = var.data_type.clone();
        instr.arg1 = LLirArg::Mem(var.name.clone());
        
        match &first.arg_type {
            AstArgType::IntL => {
                if is_unsigned(&var.data_type) {
                    instr.arg2 = LLirArg::UInt(first.u64_val);
                } else {
                    instr.arg2 = LLirArg::Int(first.u64_val as i64);
                }
            },
            
            _ => {},
        }
        
        builder.add_code(instr);
    }

    true
}


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

use crate::ast::{DataType, AstStmt, AstArgType};
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
    let data_type : LLirDataType;
    let sub_type = LLirDataType::Void;
    
    match &line.data_type {
        DataType::Byte | DataType::Char => data_type = LLirDataType::Byte,
        DataType::UByte => data_type = LLirDataType::UByte,
        
        DataType::Short => data_type = LLirDataType::Word,
        DataType::UShort => data_type = LLirDataType::UWord,
        
        DataType::Int => data_type = LLirDataType::Int,
        DataType::UInt => data_type = LLirDataType::UInt,
        
        DataType::Int64 => data_type = LLirDataType::Int64,
        DataType::UInt64 => data_type = LLirDataType::UInt64,
        
        DataType::Str => data_type = LLirDataType::Str,
        
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
        name : name.clone(),
        data_type : data_type,
        sub_type : sub_type,
    };
    
    if !build_expr(builder, line, &var) {
        return false;
    }
    
    builder.vars.push(var);
    
    true
}

pub fn build_var_assign(builder : &mut LLirBuilder, line : &AstStmt) -> bool {
    let vars = builder.vars.clone();
    for v in &vars {
        if v.name == line.name {
            return build_expr(builder, line, &v);
        }
    }
    
    false
}

// Konstrui variablon esprimon
pub fn build_expr(builder : &mut LLirBuilder, line : &AstStmt, var : &Var) -> bool {
    let args = &line.args;
    let mut stack : Vec<LLirArg> = Vec::new();
    
    for arg in args.iter() {
        match &arg.arg_type {
            AstArgType::IntL => {
                let intl : LLirArg;
                
                if is_unsigned(&var.data_type) {
                    intl = LLirArg::UInt(arg.u64_val);
                } else {
                    intl = LLirArg::Int(arg.u64_val as i64);
                }
                
                stack.push(intl);
            },
            
            AstArgType::Id => {
                let id = LLirArg::Mem(arg.str_val.clone());
                stack.push(id);
            },
            
            AstArgType::OpAdd | AstArgType::OpSub
            | AstArgType::OpMul | AstArgType::OpDiv | AstArgType::OpMod
            | AstArgType::OpAnd | AstArgType::OpOr | AstArgType::OpXor
            | AstArgType::OpLeftShift | AstArgType::OpRightShift
            if stack.len() >= 2 => {
                let arg2 = stack.pop().unwrap();
                let arg1 = stack.pop().unwrap();
                
                let dest = LLirArg::Reg(builder.reg_pos);
                builder.reg_pos += 1;
                
                //let mut instr = llir::create_instr(LLirType::Add);
                let mut instr : LLirInstr;
                
                match &arg.arg_type {
                    AstArgType::OpAdd => instr = llir::create_instr(LLirType::Add),
                    AstArgType::OpSub => instr = llir::create_instr(LLirType::Sub),
                    
                    AstArgType::OpMul if is_unsigned(&var.data_type)
                        => instr = llir::create_instr(LLirType::UMul),
                    AstArgType::OpMul => instr = llir::create_instr(LLirType::Mul),
                    
                    AstArgType::OpDiv if is_unsigned(&var.data_type)
                        => instr = llir::create_instr(LLirType::UDiv),
                    AstArgType::OpDiv => instr = llir::create_instr(LLirType::Div),
                    
                    AstArgType::OpMod if is_unsigned(&var.data_type)
                        => instr = llir::create_instr(LLirType::URem),
                    AstArgType::OpMod => instr = llir::create_instr(LLirType::Rem),
                    
                    AstArgType::OpAnd => instr = llir::create_instr(LLirType::And),
                    AstArgType::OpOr => instr = llir::create_instr(LLirType::Or),
                    AstArgType::OpXor => instr = llir::create_instr(LLirType::Xor),
                    AstArgType::OpLeftShift => instr = llir::create_instr(LLirType::Lsh),
                    AstArgType::OpRightShift => instr = llir::create_instr(LLirType::Rsh),
                    
                    // We should never get to this point
                    _ => return false,
                }
                
                instr.arg1 = dest.clone();
                instr.arg2 = arg1;
                instr.arg3 = arg2;
                builder.add_code(instr);
                
                stack.push(dest);
            },
            
            _ => {
                println!("Syntax error- build_expr");
                return false;
            },
        }
    }
    
    if stack.len() >= 1 {
        let dest = stack.pop().unwrap();
        
        let mut instr = store_for_type(&var.data_type);
        instr.data_type = var.data_type.clone();
        instr.arg1 = LLirArg::Mem(var.name.clone());
        instr.arg2 = dest;
        builder.add_code(instr);
    }
    
    true
}

// Konstrui variablon esprimon
/*pub fn build_expr(builder : &mut LLirBuilder, line : &AstStmt, var : &Var) -> bool {
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
}*/


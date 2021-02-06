
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

use crate::ast::{AstStmt, AstArgType};
use crate::llir;
use crate::llir::*;
use crate::llir_builder::*;

// Konstruas reveno aserto.
pub fn build_return(builder : &mut LLirBuilder, line : &AstStmt) -> bool {
    let mut instr = llir::create_instr(LLirType::Ret);
    
    if line.args.len() == 1 {
        let arg = line.args.first().unwrap();
        match arg.arg_type {
            // TODO: Ni bezonas tipdetekton.
            AstArgType::IntL => instr.arg1 = LLirArg::Int(arg.u64_val as i64),
            
            // TODO: Tipdetekton
            AstArgType::Id => {
                let mut instr2 = llir::create_instr(LLirType::LdDW);
                instr2.arg1 = LLirArg::Reg(builder.reg_pos);
                instr2.arg2 = LLirArg::Mem(arg.str_val.clone());
                builder.add_code(instr2);
                
                instr.arg1 = LLirArg::Reg(builder.reg_pos);
                
                builder.reg_pos += 1;
            },
            
            _ => {},
        }
    } else {
        // TODO: Konstrui esprimon
    }
    
    builder.add_code(instr);
    true
}

pub fn build_end(builder : &mut LLirBuilder, _line : &AstStmt) -> bool {
    let instr = llir::create_instr(LLirType::Ret);
    builder.add_code(instr);
    
    true
}

// Konstruas funkcion alvokon
pub fn build_func_call(builder : &mut LLirBuilder, line : &AstStmt) -> bool {
    let args = &line.args;
    let mut arg_list : Vec<LLirArg> = Vec::new();
    
    for arg in args {
        match &arg.arg_type {
            AstArgType::StringL => {
                arg_list.push(LLirArg::StrLiteral(arg.str_val.clone()));
            },
            
            AstArgType::Id => {
                arg_list.push(LLirArg::Mem(arg.str_val.clone()));
            },
            
            _ => {},
        }
    }

    let mut instr = llir::create_instr(LLirType::Call);
    instr.arg1 = LLirArg::Label(line.name.clone());
    instr.arg2 = LLirArg::ArgList(arg_list);
    
    builder.add_code(instr);
    true
}


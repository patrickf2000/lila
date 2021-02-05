
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
            
            _ => {},
        }
    } else {
        // TODO: Konstrui esprimon
    }
    
    builder.add_code(instr);
    true
}


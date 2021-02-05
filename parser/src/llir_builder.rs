
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

use crate::ast::*;
use crate::llir;
use crate::llir::*;
use crate::syntax::*;

use crate::llir_func::*;
use crate::llir_var::*;

pub struct Var {
    pub name : String,
    pub data_type : LLirDataType,
    pub sub_type : LLirDataType,
}

pub struct LLirBuilder {
    pub file : LLirFile,
    pub syntax : ErrorManager,
    
    pub reg_pos : i32,
    pub str_pos : i32,
    
    pub vars : Vec<Var>,
}

pub fn new_llir_builder(name : String, syntax : &mut ErrorManager) -> LLirBuilder {
    LLirBuilder {
        file : LLirFile {
            name : name,
            strings : HashMap::new(),
            code : Vec::new(),
        },
        syntax : syntax.clone(),
        
        reg_pos : 0,
        str_pos : 0,
        
        vars : Vec::new(),
    }
}

impl LLirBuilder {

    // Konstruas la ĉefan LLIR dosieron.
    pub fn build_llir(&mut self, tree : &AstTree) -> Result<LLirFile, ()> {
        // Cache the constants
        /*if !self.build_global_constants(tree) {
            self.syntax.print_errors();
            return Err(());
        }*/
        
        // Konstrui la funkciojn.
        if !self.build_functions(tree) {
            self.syntax.print_errors();
            return Err(());
        }
        
        Ok(self.file.clone())
    }
    
    // Konstrui la funkciojn.
    fn build_functions(&mut self, tree : &AstTree) -> bool {
        for func in tree.functions.iter() {
            if func.is_extern {
                let mut def = llir::create_instr(LLirType::Extern);
                def.arg1 = LLirArg::Label(func.name.clone());
                
                def.data_type = LLirDataType::Void;
                
                self.add_code(def);
            } else {
                let mut def = llir::create_instr(LLirType::Func);
                def.data_type = LLirDataType::Int;
                def.arg1 = LLirArg::Label(func.name.clone());
                self.add_code(def);
                
                // Konstrui la blokon.
                if !self.build_block(&func.statements) {
                    return false;
                }        
            }
        }
        
        true
    }
    
    // Konstrui la funkcion korpon.
    fn build_block(&mut self, statements : &Vec<AstStmt>) -> bool {
        let mut code = true;
    
        for line in statements {
            match &line.stmt_type {
                AstStmtType::VarDec => code = build_var_dec(self, &line),
                //AstStmtType::VarAssign => code = build_var_assign(self, &line),
                //AstStmtType::ArrayAssign => code = build_array_assign(self, &line),
                //AstStmtType::If => build_cond(self, &line),
                //AstStmtType::Elif => build_cond(self, &line),
                //AstStmtType::Else => build_cond(self, &line),
                //AstStmtType::While => build_while(self, &line),
                //AstStmtType::For => build_for_loop(self, &line),
                //AstStmtType::Break => build_break(self),
                //AstStmtType::Continue => build_continue(self),
                //AstStmtType::FuncCall => code = build_func_call(self, &line),
                AstStmtType::Return => code = build_return(self, &line),
                //AstStmtType::Exit => code = build_exit(self, &line),
                
                // TODO: Forigi post la super faritas.
                _ => {},
            }
            
            if !code {
                break;
            }
        }
        
        code
    }
    
    // Aldonas linio de kodo al la vektoro.
    pub fn add_code(&mut self, code : LLirInstr) {
        self.file.code.push(code);
    }
}

// Utilaj funkcioj
pub fn store_for_type(data_type : &LLirDataType) -> LLirInstr {
    match &data_type {
        LLirDataType::Byte => llir::create_instr(LLirType::StrB),
        LLirDataType::UByte => llir::create_instr(LLirType::UstrB),
        LLirDataType::Word => llir::create_instr(LLirType::StrW),
        LLirDataType::UWord => llir::create_instr(LLirType::UstrW),
        LLirDataType::Int => llir::create_instr(LLirType::StrDW),
        LLirDataType::UInt => llir::create_instr(LLirType::UstrDW),
        LLirDataType::Int64 => llir::create_instr(LLirType::StrQW),
        LLirDataType::UInt64 => llir::create_instr(LLirType::UstrQW),
        LLirDataType::Str => llir::create_instr(LLirType::StrQW),
        LLirDataType::Ptr => llir::create_instr(LLirType::StrQW),
        _ => llir::create_instr(LLirType::Nop),
    }
}

pub fn is_unsigned(data_type : &LLirDataType) -> bool {
    match &data_type {
        LLirDataType::UByte | LLirDataType::UWord
        | LLirDataType::UInt | LLirDataType::UInt64 => return true,
        _ => return false,
    }
}


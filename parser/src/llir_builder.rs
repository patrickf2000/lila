
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
use crate::llir::*;
use crate::syntax::*;

pub struct LLirBuilder {
    pub file : LLirFile,
    pub syntax : ErrorManager,
    
    pub str_pos : i32,
}

pub fn new_llir_builder(name : String, syntax : &mut ErrorManager) -> LLirBuilder {
    LLirBuilder {
        file : LLirFile {
            name : name,
            strings : HashMap::new(),
            code : Vec::new(),
        },
        syntax : syntax.clone(),
        
        str_pos : 0,
    }
}

impl LLirBuilder {

    // Builds the main LTAC file
    pub fn build_llir(&mut self, _tree : &AstTree) -> Result<LLirFile, ()> {
        // Cache the constants
        /*if !self.build_global_constants(tree) {
            self.syntax.print_errors();
            return Err(());
        }*/
        
        // Build functions
        /*if !self.build_functions(tree) {
            self.syntax.print_errors();
            return Err(());
        }*/
        
        Ok(self.file.clone())
    }
    
}


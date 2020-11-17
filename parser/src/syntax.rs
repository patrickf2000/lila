
// This file is part of the Dash compiler
// Copyright (C) 2020 Patrick Flynn
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


use crate::lex::*;
use crate::ast::AstStmt;

#[derive(Clone)]
pub struct SyntaxError {
    pub line_no : i32,
    pub line : String,
    pub message : String,
}

#[derive(Clone)]
pub struct ErrorManager {
    pub errors : Vec<SyntaxError>,
}

pub fn create_error_manager() -> ErrorManager {
    ErrorManager {
        errors : Vec::new(),
    }
}

impl ErrorManager {

    // Called when the AST is being built
    pub fn syntax_error(&mut self, scanner : &mut Lex, msg : String) {
        let error = SyntaxError {
            line_no : scanner.get_line_no(),
            line : scanner.get_current_line(),
            message : msg,
        };
        
        self.errors.push(error);
    }
    
    // Called when the AST is being translated to the LTAC
    pub fn ltac_error(&mut self, stmt : &AstStmt, msg : String) {
        let error = SyntaxError {
            line_no : stmt.line_no,
            line : stmt.line.clone(),
            message : msg,
        };
        
        self.errors.push(error);
    }
    
    // Called to print any syntax errors
    pub fn print_errors(&mut self) {
        for error in self.errors.iter() {
            println!("Syntax Error: {}", error.message);
            println!(" -> [{}] {}", error.line_no, error.line);
            println!("");
        }
    }
}


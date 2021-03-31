//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

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
    pub current_ln : String,
    pub current_ln_no : i32,
}

pub fn create_error_manager() -> ErrorManager {
    ErrorManager {
        errors : Vec::new(),
        current_ln : String::new(),
        current_ln_no : 0,
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
    
    pub fn ltac_error2(&mut self, msg : String) {
        let error = SyntaxError {
            line_no : self.current_ln_no,
            line : self.current_ln.clone(),
            message : msg,
        };
        
        self.errors.push(error);
    }
    
    // Set the current line to make it easier to call LTAC errors
    pub fn set_data(&mut self, stmt : &AstStmt) {
        self.current_ln = stmt.line.clone();
        self.current_ln_no = stmt.line_no;
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


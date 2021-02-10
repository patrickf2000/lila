
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

// Import what we need
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex, create_lex};
use crate::syntax;
use crate::Arch;

use crate::ast_func::*;
use crate::ast_utils::*;
use crate::ast_var::*;
use crate::module;
use crate::module::*;
use crate::syntax::ErrorManager;

pub struct AstBuilder {
    pub scanner : Lex,
    pub tree : AstTree,
    pub current_block : Vec<AstStmt>,
    pub keep_postfix : bool,
    pub syntax : ErrorManager,
}

impl AstBuilder {

    pub fn get_token(&mut self) -> Token {
        return self.scanner.get_token();
    }
    
    pub fn syntax_error(&mut self, msg : String) {
        self.syntax.syntax_error(&mut self.scanner, msg);
    }
    
    pub fn add_stmt(&mut self, stmt : AstStmt) {
        self.current_block.push(stmt);
    }
}

// The AST building function
// This function opens the file and reads a line; 
// the line is then passed to another function which lexically analyzes
// it and builds an AST node.
//
// In Quik, each line is a self-contained expression; as a result, we read a line
// and then lexically analyze and build an AST node from it
//
pub fn build_ast(path : String, arch : Arch, name : String, include_core : bool, keep_postfix : bool) -> Result<AstTree, ()> {   
    let tree = AstTree {
        file_name : name,
        arch : arch,
        module : String::new(),
        functions : Vec::new(),
        constants : Vec::new(),
    };
    
    let mut builder = AstBuilder {
        scanner : create_lex(),
        tree : tree,
        current_block : Vec::new(),
        keep_postfix : keep_postfix,
        syntax : syntax::create_error_manager(),
    };
    
    // Open the file
    let file = File::open(&path)
        .expect("Error: Unable to open input file.");
    let reader = BufReader::new(file);
    
    // Include the core modules
    if include_core {
        include_module("core.mem".to_string(), &mut builder);
        include_module("core.string".to_string(), &mut builder);
        include_module("core.io".to_string(), &mut builder);
    }
    
    // Read the thing line by line
    let mut line_no = 0;
    let mut layer = 0;
    let mut in_begin = false;
    
    for line in reader.lines() {
        let mut current = line.unwrap();
        current = current.trim().to_string();
        line_no += 1;
        
        if current.len() == 0 {
            continue;
        }
        
        builder.scanner.tokenize(current, line_no);
    }
    
    loop {
        let (ret, new_layer, begin, done) = build_line(layer, in_begin, &mut builder);
        layer = new_layer;
        in_begin = begin;
        
        if done {
            break;
        }
        
        if !ret {
            builder.syntax.print_errors();
            return Err(());
        }
    }
    
    Ok(builder.tree)
}

// Loads a module into the current tree
pub fn include_module(name : String, builder : &mut AstBuilder) -> bool {
    let path = module::get_module_path(&name);
    
    // Open the file
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_e) => {
            println!("Invalid module: {}", name);
            return false;
        },
    };
    
    let reader = BufReader::new(file);
    
    // Read the thing line by line
    let mut line_no = 0;
    let mut layer = 0;
    let mut in_begin = false;
    
    let old_scanner = builder.scanner.clone();
    builder.scanner = create_lex();
    
    for line in reader.lines() {
        let mut current = line.unwrap();
        current = current.trim().to_string();
        line_no += 1;
        
        if current.len() == 0 {
            continue;
        }
        
        builder.scanner.tokenize(current, line_no);
    }
    
    loop {
        let (ret, new_layer, begin, done) = build_line(layer, in_begin, builder);
        layer = new_layer;
        in_begin = begin;
        
        if done {
            break;
        }
        
        if !ret {
            return false;
        }
    }
    
    builder.scanner = old_scanner;
    
    true
}

// Converts a line to an AST node
// TODO: This is the most ridiculous function signature. We need to change this
fn build_line(layer : i32, in_begin : bool, builder : &mut AstBuilder) -> (bool, i32, bool, bool) {    
    let mut code = true;
    let mut new_layer = layer;
    let mut in_code = in_begin;
    
    // Get the first token
    let mut token = builder.get_token();
    
    match token {
        
        Token::Module => code = build_module(builder),
        Token::Use => code = build_use(builder),
    
        Token::Extern => {
            token = builder.scanner.get_token();
            match token {
                Token::Func => {},
                _ => {
                    builder.syntax_error("Expected \"func\" keyword.".to_string());
                    return (false, 0, false, false);
                }
            }
                
            code = build_func(builder, true)
        },
        
        Token::Func => {
            in_code = false;
            code = build_func(builder, false);
            new_layer += 1;
        },
        
        // Indicates the end of the variable section and start of the code section
        Token::Begin => {
            if in_code {
                builder.syntax_error("Unexpected \"begin\"-> Already in code.".to_string());
                return (false, 0, false, false);
            } else {
                in_code = true;
            }
        },
        
        Token::Return if in_code => code = build_return(builder),
        Token::Exit if in_code => code = build_exit(builder),
        
        Token::End => {
            build_end(builder);
            new_layer -= 1;
            
            if new_layer == 0 {
                for stmt in builder.current_block.iter() {
                    ast::add_stmt(&mut builder.tree, stmt.clone());
                }
                
                builder.current_block.clear();
            }
        },
        
        Token::Const => code = build_const(builder, layer),
        
        Token::Enum => {
            if in_code {
                builder.syntax_error("You cannot define an enum in the code body.".to_string());
                return (false, 0, false, false);
            } else {
                code = build_enum(builder, layer);
            }
        },
        
        Token::Id(ref val) if in_code => code = build_id(builder, val.to_string()),
        Token::Id(ref val) => code = build_var_dec(builder, val.to_string()),
        
        Token::If if in_code => {
            code = build_cond(builder, Token::If);
            new_layer += 1;
        },
        
        Token::Elif if in_code => code = build_cond(builder, Token::Elif),
        Token::Else if in_code => code = build_cond(builder, Token::Else),
        
        Token::While if in_code => {
            code = build_cond(builder, Token::While);
            new_layer += 1;
        },
        
        Token::For if in_code => {
            code = build_for_loop(builder);
            new_layer += 1;
        },
        
        // TODO: For break and continue
        // Create a common function for the lack of semicolons
        Token::Break if in_code => {
            let br = ast::create_stmt(AstStmtType::Break, &mut builder.scanner);
            builder.add_stmt(br);
            
            if builder.get_token() != Token::Semicolon {
                builder.syntax_error("Expected terminator".to_string());
                return (false, 0, false, false);
            }
        },
        
        Token::Continue if in_code => {
            let cont = ast::create_stmt(AstStmtType::Continue, &mut builder.scanner);
            builder.add_stmt(cont);
            
            if builder.get_token() != Token::Semicolon {
                builder.syntax_error("Expected terminator".to_string());
                return (false, 0, false, false);
            }
        },
        
        Token::Eof => {},
        Token::EoI => return (true, 0, false, true),
        
        _ => {
            if in_code {
                builder.syntax_error("Invalid token in context.".to_string());
            } else {
                builder.syntax_error("Invalid context- Expecting \"begin\" before code.".to_string());
            }
            
            code = false;
        }
    }
    
    (code, new_layer, in_code, false)
}

// Builds a constant
fn build_const(builder : &mut AstBuilder, layer : i32) -> bool {
    let mut token = builder.get_token();
    let data_type : DataType;
    let arg : AstArg;
    let name : String;
    
    match &token {
        Token::Byte => data_type = DataType::Byte,
        Token::UByte => data_type = DataType::UByte,
        Token::Short => data_type = DataType::Short,
        Token::UShort => data_type = DataType::UShort,
        Token::Int => data_type = DataType::Int,
        Token::UInt => data_type = DataType::UInt,
        Token::Int64 => data_type = DataType::Int64,
        Token::UInt64 => data_type = DataType::UInt64,
        Token::Float => data_type = DataType::Float,
        Token::Double => data_type = DataType::Double,
        Token::Char => data_type = DataType::Char,
        Token::TStr => data_type = DataType::Str,
        
        _ => {
            builder.syntax_error("Expected data type.".to_string());
            return false;
        },
    }
    
    token = builder.get_token();
    
    match &token {
        Token::Id(ref val) => name = val.to_string(),
        
        _ => {
            builder.syntax_error("Missing constant name.".to_string());
            return false;
        },
    }
    
    token = builder.get_token();
    
    if token != Token::Assign {
        builder.syntax_error("Expected assignment operator.".to_string());
        return false;
    }
    
    token = builder.get_token();
    
    match &token {
        Token::ByteL(val) => arg = ast::create_byte(*val),
        Token::ShortL(val) => arg = ast::create_short(*val),
        Token::IntL(val) => arg = ast::create_int(*val),
        Token::FloatL(val) => arg = ast::create_float(*val),
        Token::CharL(val) => arg = ast::create_char(*val),
        Token::StringL(ref val) => arg = ast::create_string(val.to_string()),
        
        _ => {
            builder.syntax_error("Constants can only be literal values.".to_string());
            return false;
        },
    }
    
    let constant = AstConst {
        name : name,
        data_type : data_type,
        value : arg,
        
        line_no : builder.scanner.get_line_no(),
        line : builder.scanner.get_current_line(),
    };
    
    if layer == 0 {
        builder.tree.constants.push(constant);
    } else {
        builder.syntax_error("Constants are not yet supported on the local level.".to_string());
        return false;
    }
    
    token = builder.get_token();
    
    if token != Token::Semicolon {
        builder.syntax_error("Expected terminator.".to_string());
        return false;
    }
    
    true
}

// Builds an enumeration
fn build_enum(builder : &mut AstBuilder, layer : i32) -> bool {
    let mut token = builder.get_token();
    let name : String;
    
    // Get the name
    match token {
        Token::Id(ref val) => name = val.to_string(),
        
        _ => {
            builder.syntax_error("Expected enum name".to_string());
            return false;
        },
    }
    
    // Next token should be assign
    if builder.get_token() != Token::Assign {
        builder.syntax_error("Expected assignment operator.".to_string());
        return false;
    }
    
    // Now create the AST enumeration and read the definition
    let mut new_enum = AstEnum {
        name : name,
        data_type : DataType::Int,      // TODO: We need type detection
        values : HashMap::new(),
    };
    
    let mut value = 0;
    token = builder.get_token();
    
    loop {
        match token {
            Token::Id(ref val) => {
                new_enum.values.insert(val.to_string(), value);
                value += 1;
            },
            
            _ => {
                println!("{:?}", token);
                builder.syntax_error("Invalid enumeration -> Expected name".to_string());
                return false;
            },
        }
        
        token = builder.get_token();
        
        if token == Token::Comma {
            token = builder.get_token();
            continue;
        } else if token == Token::Semicolon {
            break;
        } else {
            builder.syntax_error("Expected \',\' or \';\'".to_string());
            return false;
        }
    }
    
    // Finally, add to the tree
    if layer == 0 {
        // TODO: Global enums
    } else {
        ast::add_func_enum(&mut builder.tree, new_enum);
    }
    
    true
}

// Handles cases when an identifier is the first token
fn build_id(builder : &mut AstBuilder, id_val : String) -> bool {
    // If the next token is an assignment, we have a variable assignment
    // If the next token is a parantheses, we have a function call
    let token = builder.get_token();
    let code : bool;
    
    match token {
        Token::AddAssign | Token::SubAssign
        | Token::MulAssign | Token::DivAssign
        | Token::ModAssign
        | Token::OpInc | Token::OpDec
        | Token::Assign => code = build_var_assign(&mut builder.scanner, &mut builder.current_block, id_val, token, builder.keep_postfix, &mut builder.syntax),
        
        Token::LParen => code = build_func_call(builder, id_val),
        Token::LBracket => code = build_array_assign(&mut builder.scanner, &mut builder.current_block, id_val, builder.keep_postfix, &mut builder.syntax),
        _ => {
            builder.syntax_error("Invalid assignment or call.".to_string());
            return false;
        },
    }
    
    code
}

// Builds conditional statements
fn build_cond(builder : &mut AstBuilder, cond_type : Token) -> bool {
    let mut ast_cond_type : AstStmtType = AstStmtType::If;
    match cond_type {
        Token::If => ast_cond_type = AstStmtType::If,
        Token::Elif => ast_cond_type = AstStmtType::Elif,
        Token::Else => ast_cond_type = AstStmtType::Else,
        Token::While => ast_cond_type = AstStmtType::While,
        _ => {},
    }
    
    let mut cond = ast::create_stmt(ast_cond_type, &mut builder.scanner);
    
    // Build the rest arguments
    if cond_type != Token::Else {
        if !build_args(&mut builder.scanner, &mut cond, Token::Eof, &mut builder.syntax) {
            return false;
        }
    }
    
    builder.add_stmt(cond);
    
    true
}

// Builds a for loop
// Syntax: for <index> in <var> | <start> .. <end>
fn build_for_loop(builder : &mut AstBuilder) -> bool {
    let mut for_loop = ast::create_stmt(AstStmtType::For, &mut builder.scanner);
    let token = builder.get_token();
    
    match token {
        Token::Id(ref val) => {
            let mut id = ast::create_arg(AstArgType::Id);
            id.str_val = val.to_string();
            for_loop.args.push(id);
        },
        
        _ => {
            builder.syntax_error("Expected variable name.".to_string());
            return false;
        },
    }
    
    if builder.get_token() != Token::In {
        builder.syntax_error("Expected \"in\".".to_string());
        return false;
    }
    
    // Build the rest of the arguments
    if !build_args(&mut builder.scanner, &mut for_loop, Token::Eof, &mut builder.syntax) {
        return false;
    }
    
    builder.add_stmt(for_loop);
    
    true
}


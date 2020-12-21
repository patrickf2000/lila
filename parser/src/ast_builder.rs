
// This file is part of the Lila compiler
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

// Import what we need
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex, create_lex};
use crate::module;
use crate::syntax::ErrorManager;

use crate::ast_func::*;
use crate::ast_utils::*;

// The AST building function
// This function opens the file and reads a line; 
// the line is then passed to another function which lexically analyzes
// it and builds an AST node.
//
// In Quik, each line is a self-contained expression; as a result, we read a line
// and then lexically analyze and build an AST node from it
//
pub fn build_ast(path : String, name : String, syntax : &mut ErrorManager) -> Result<AstTree, ()> {   
    let mut tree = AstTree {
        file_name : name,
        module : String::new(),
        functions : Vec::new(),
        constants : Vec::new(),
    };
    
    // Open the file
    let file = File::open(&path)
        .expect("Error: Unable to open input file.");
    let reader = BufReader::new(file);
    
    // Read the thing line by line
    let mut line_no = 0;
    let mut layer = 0;
    let mut in_begin = false;
    
    let mut scanner = create_lex();
    
    for line in reader.lines() {
        let mut current = line.unwrap();
        current = current.trim().to_string();
        line_no += 1;
        
        if current.len() == 0 {
            continue;
        }
        
        scanner.tokenize(current, line_no);
    }
    
    loop {
        let (ret, new_layer, begin, done) = build_line(&mut scanner, layer, in_begin, &mut tree, syntax);
        layer = new_layer;
        in_begin = begin;
        
        if done {
            break;
        }
        
        if !ret {
            syntax.print_errors();
            return Err(());
        }
    }
    
    Ok(tree)
}

// 12/21/2020- This is untested following the lexical analyzer redesign
//
// Loads a module into the current tree
// TODO:
//      1) This should go into the module builder
//      2) There should be something for the system-wide modules
fn include_module(name : String, tree : &mut AstTree, syntax : &mut ErrorManager) -> bool {
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
    
    let mut scanner = create_lex();
    
    for line in reader.lines() {
        let mut current = line.unwrap();
        current = current.trim().to_string();
        line_no += 1;
        
        if current.len() == 0 {
            continue;
        }
        
        scanner.tokenize(current, line_no);
    }
    
    loop {
        let (ret, new_layer, begin, done) = build_line(&mut scanner, layer, in_begin, tree, syntax);
        layer = new_layer;
        in_begin = begin;
        
        if done {
            break;
        }
        
        if !ret {
            return false;
        }
    }
    
    true
}

// Converts a line to an AST node
fn build_line(scanner : &mut Lex, layer : i32, in_begin : bool, tree : &mut AstTree, syntax : &mut ErrorManager) -> (bool, i32, bool, bool) {    
    let mut code = true;
    let mut new_layer = layer;
    let mut in_code = in_begin;
    
    // Get the first token
    let mut token = scanner.get_token();
    
    match token {
        Token::Module => {
            if tree.module.len() > 0 {
                syntax.syntax_error(scanner, "Duplicate module declarations.".to_string());
                return (false, 0, false, false);
            }
            
            token = scanner.get_token();
            
            match token {
                Token::Id(ref val) => tree.module = val.clone(),
                _ => {
                    syntax.syntax_error(scanner, "Module names must be an identifier.".to_string());
                    return (false, 0, false, false);
                },
            }
        },
        
        Token::Use => {
            let module : String;
            token = scanner.get_token();
            
            match token {
                Token::Id(ref val) => module = val.clone(),
                _ => {
                    syntax.syntax_error(scanner, "Module names must be an identifier.".to_string());
                    return (false, 0, false, false);
                },
            }
            
            code = include_module(module, tree, syntax);
        },
    
        Token::Extern => {
            token = scanner.get_token();
            match token {
                Token::Func => {},
                _ => {
                    syntax.syntax_error(scanner, "Expected \"func\" keyword.".to_string());
                    return (false, 0, false, false);
                }
            }
                
            code = build_func(scanner, tree, syntax, true)
        },
        
        Token::Func => {
            in_code = false;
            code = build_func(scanner, tree, syntax, false);
            new_layer += 1;
        },
        
        // Indicates the end of the variable section and start of the code section
        Token::Begin => {
            if in_code {
                syntax.syntax_error(scanner, "Unexpected \"begin\"-> Already in code.".to_string());
                return (false, 0, false, false);
            } else {
                in_code = true;
            }
        },
        
        Token::Return if in_code => code = build_return(scanner, tree, syntax),
        Token::Exit if in_code => code = build_exit(scanner, tree, syntax),
        
        Token::End => {
            build_end(scanner, tree);
            new_layer -= 1;
        },
        
        Token::Const => code = build_const(scanner, tree, syntax, layer),
        
        Token::Id(ref val) if in_code => code = build_id(scanner, tree, val.to_string(), syntax),
        Token::Id(ref val) => code = build_var_dec(scanner, tree, val.to_string(), syntax),
        
        Token::If if in_code => {
            code = build_cond(scanner, tree, Token::If, syntax);
            new_layer += 1;
        },
        
        Token::Elif if in_code => code = build_cond(scanner, tree, Token::Elif, syntax),
        Token::Else if in_code => code = build_cond(scanner, tree, Token::Else, syntax),
        
        Token::While if in_code => {
            code = build_cond(scanner, tree, Token::While, syntax);
            new_layer += 1;
        },
        
        Token::Break if in_code => {
            let br = ast::create_stmt(AstStmtType::Break, scanner);
            ast::add_stmt(tree, br);
        },
        
        Token::Continue if in_code => {
            let cont = ast::create_stmt(AstStmtType::Continue, scanner);
            ast::add_stmt(tree, cont);
        },
        
        Token::Eof => {},
        Token::EoI => return (true, 0, false, true),
        
        _ => {
            if in_code {
                syntax.syntax_error(scanner, "Invalid token or context.".to_string());
            } else {
                syntax.syntax_error(scanner, "Invalid context- Expecting \"begin\" before code.".to_string());
            }
            
            code = false;
        }
    }
    
    (code, new_layer, in_code, false)
}

// Builds an integer variable declaration
fn build_var_dec(scanner : &mut Lex, tree : &mut AstTree, name : String, syntax : &mut ErrorManager) -> bool {
    let mut var_dec = ast::create_stmt(AstStmtType::VarDec, scanner);
    var_dec.name = name;
    
    // This will hold any additional names (for multi-variable declarations)
    // If the list is empty, we only have one declaration
    let mut extra_names : Vec<String> = Vec::new();
    
    // Gather information
    // The first token should be the colon, followed by the type and optionally array arguments
    let mut token = scanner.get_token();
    
    while token == Token::Comma {
        token = scanner.get_token();
        
        match token {
            Token::Id(ref val) => extra_names.push(val.to_string()),
            
            _ => {
                syntax.syntax_error(scanner, "Expected variable name.".to_string());
                return false;
            },
        }
        
        token = scanner.get_token();
    }
    
    if token != Token::Colon {
        syntax.syntax_error(scanner, "Expected \':\' after variable name.".to_string());
        return false;
    }
    
    // Now for the type
    let mut is_array = false;
    let dtype : AstModType;
    
    token = scanner.get_token();
    
    match token {
        Token::Byte => dtype = AstModType::Byte,
        Token::UByte => dtype = AstModType::UByte,
        Token::Short => dtype = AstModType::Short,
        Token::UShort => dtype = AstModType::UShort,
        Token::Int => dtype = AstModType::Int,
        Token::UInt => dtype = AstModType::UInt,
        Token::Int64 => dtype = AstModType::Int64,
        Token::UInt64 => dtype = AstModType::UInt64,
        Token::Float => dtype = AstModType::Float,
        Token::Double => dtype = AstModType::Double,
        Token::Char => dtype = AstModType::Char,
        Token::TStr => dtype = AstModType::Str,
        
        _ => {
            syntax.syntax_error(scanner, "Invalid type.".to_string());
            return false;
        },
    }
        
    let mut data_type = AstMod {
        mod_type : dtype.clone(),
    };
    
    // Check for arrays
    token = scanner.get_token();
    
    match token {
        Token::Assign => {},
        
        Token::LBracket => {
            is_array = true;
            if !build_args(scanner, &mut var_dec, Token::RBracket, syntax) {
                return false;
            }
            
            token = scanner.get_token();
            
            if token != Token::Assign {
                syntax.syntax_error(scanner, "Expected assignment operator.".to_string());
                return false;
            }
        },
        
        _ => {
            syntax.syntax_error(scanner, "Expected assignment operator.".to_string());
            return false;
        },
    }
    
    // Build the remaining arguments
    if !build_args(scanner, &mut var_dec, Token::Eof, syntax) {
        return false;
    }
    
    var_dec.args = check_operations(&var_dec.args);
    
    // If we have the array, check the array type
    if is_array {
        if var_dec.args.len() == 1 && var_dec.args.last().unwrap().arg_type == AstArgType::Array {
            match &dtype {
                AstModType::Byte | AstModType::Char => data_type.mod_type = AstModType::ByteDynArray,
                AstModType::UByte => data_type.mod_type = AstModType::UByteDynArray,
                AstModType::Short => data_type.mod_type = AstModType::ShortDynArray,
                AstModType::UShort => data_type.mod_type = AstModType::UShortDynArray,
                AstModType::Int => data_type.mod_type = AstModType::IntDynArray,
                AstModType::UInt => data_type.mod_type = AstModType::UIntDynArray,
                AstModType::Int64 => data_type.mod_type = AstModType::I64DynArray,
                AstModType::UInt64 => data_type.mod_type = AstModType::U64DynArray,
                AstModType::Float => data_type.mod_type = AstModType::FloatDynArray,
                AstModType::Double => data_type.mod_type = AstModType::DoubleDynArray,
                
                _ => {},
            }
        } else {
            //TODO
        }
    }
    
    var_dec.modifiers.push(data_type);
    ast::add_stmt(tree, var_dec.clone());
    
    for n in extra_names.iter() {
        var_dec.name = n.to_string();
        ast::add_stmt(tree, var_dec.clone());
    }
    
    true
}

// Builds a constant
fn build_const(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager, layer : i32) -> bool {
    let mut token = scanner.get_token();
    let data_type : AstModType;
    let arg : AstArg;
    let name : String;
    
    match &token {
        Token::Byte => data_type = AstModType::Byte,
        Token::UByte => data_type = AstModType::UByte,
        Token::Short => data_type = AstModType::Short,
        Token::UShort => data_type = AstModType::UShort,
        Token::Int => data_type = AstModType::Int,
        Token::UInt => data_type = AstModType::UInt,
        Token::Int64 => data_type = AstModType::Int64,
        Token::UInt64 => data_type = AstModType::UInt64,
        Token::Float => data_type = AstModType::Float,
        Token::Double => data_type = AstModType::Double,
        Token::Char => data_type = AstModType::Char,
        Token::TStr => data_type = AstModType::Str,
        
        _ => {
            syntax.syntax_error(scanner, "Expected data type.".to_string());
            return false;
        },
    }
    
    token = scanner.get_token();
    
    match &token {
        Token::Id(ref val) => name = val.to_string(),
        
        _ => {
            syntax.syntax_error(scanner, "Missing constant name.".to_string());
            return false;
        },
    }
    
    token = scanner.get_token();
    
    if token != Token::Assign {
        syntax.syntax_error(scanner, "Expected assignment operator.".to_string());
        return false;
    }
    
    token = scanner.get_token();
    
    match &token {
        Token::ByteL(val) => arg = ast::create_byte(*val),
        Token::ShortL(val) => arg = ast::create_short(*val),
        Token::IntL(val) => arg = ast::create_int(*val),
        Token::FloatL(val) => arg = ast::create_float(*val),
        Token::CharL(val) => arg = ast::create_char(*val),
        Token::StringL(ref val) => arg = ast::create_string(val.to_string()),
        
        _ => {
            syntax.syntax_error(scanner, "Constants can only be literal values.".to_string());
            return false;
        },
    }
    
    let modifier = AstMod { mod_type : data_type, };
    let constant = AstConst {
        name : name,
        data_type : modifier,
        value : arg,
        
        line_no : scanner.get_line_no(),
        line : scanner.get_current_line(),
    };
    
    if layer == 0 {
        tree.constants.push(constant);
    } else {
        syntax.syntax_error(scanner, "Constants are not yet supported on the local level.".to_string());
        return false;
    }
    
    true
}

// Builds a variable assignment
fn build_var_assign(scanner : &mut Lex, tree : &mut AstTree, name : String, syntax : &mut ErrorManager) -> bool {
    let mut var_assign = ast::create_stmt(AstStmtType::VarAssign, scanner);
    var_assign.name = name;
    
    if !build_args(scanner, &mut var_assign, Token::Eof, syntax) {
        return false;
    }
    
    ast::add_stmt(tree, var_assign);
    
    true
}

// Builds an array assignment
fn build_array_assign(scanner : &mut Lex, tree : &mut AstTree, id_val : String, syntax : &mut ErrorManager) -> bool {
    let mut array_assign = ast::create_stmt(AstStmtType::ArrayAssign, scanner);
    array_assign.name = id_val;
    
    // For the array index
    if !build_args(scanner, &mut array_assign, Token::RBracket, syntax) {
        return false;
    }
    
    if scanner.get_token() != Token::Assign {
        syntax.syntax_error(scanner, "Expected \'=\' in array assignment.".to_string());
        return false;
    }
    
    // Tokens being assigned to the array
    if !build_args(scanner, &mut array_assign, Token::Eof, syntax) {
        return false;
    }
    
    ast::add_stmt(tree, array_assign);
    
    true
}

// Handles cases when an identifier is the first token
fn build_id(scanner : &mut Lex, tree : &mut AstTree, id_val : String, syntax : &mut ErrorManager) -> bool {
    // If the next token is an assignment, we have a variable assignment
    // If the next token is a parantheses, we have a function call
    let token = scanner.get_token();
    let code : bool;
    
    match token {
        Token::Assign => code = build_var_assign(scanner, tree, id_val, syntax),
        Token::LParen => code = build_func_call(scanner, tree, id_val, syntax),
        Token::LBracket => code = build_array_assign(scanner, tree, id_val, syntax),
        _ => {
            syntax.syntax_error(scanner, "Invalid assignment or call.".to_string());
            return false;
        },
    }
    
    code
}

// Builds conditional statements
fn build_cond(scanner : &mut Lex, tree : &mut AstTree, cond_type : Token, syntax : &mut ErrorManager) -> bool {
    let mut ast_cond_type : AstStmtType = AstStmtType::If;
    match cond_type {
        Token::If => ast_cond_type = AstStmtType::If,
        Token::Elif => ast_cond_type = AstStmtType::Elif,
        Token::Else => ast_cond_type = AstStmtType::Else,
        Token::While => ast_cond_type = AstStmtType::While,
        _ => {},
    }
    
    let mut cond = ast::create_stmt(ast_cond_type, scanner);
    
    // Build the rest arguments
    if cond_type != Token::Else {
        if !build_args(scanner, &mut cond, Token::Eof, syntax) {
            return false;
        }
    }
    
    ast::add_stmt(tree, cond);
    
    true
}



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

use crate::ast;
use crate::ast::*;
use crate::lex::Token;

use crate::ast_builder::AstBuilder;
use crate::ast_utils::*;

// A utility function for returning a type modifier from a token
// NOTE: I don't know if we need a subtype, but if so, we'll have to go back and make adjustments
fn token_to_mod(token : &Token, is_array : bool) -> (DataType, DataType) {   
    match token {
        Token::Byte if is_array => return (DataType::Ptr, DataType::Byte),
        Token::Byte => return (DataType::Byte, DataType::None),
        
        Token::UByte if is_array => return (DataType::Ptr, DataType::UByte),
        Token::UByte | Token::Char => return (DataType::UByte, DataType::None),
        
        Token::Short if is_array => return (DataType::Ptr, DataType::Short),
        Token::Short => return (DataType::Short, DataType::None),
        
        Token::UShort if is_array => return (DataType::Ptr, DataType::UShort),
        Token::UShort => return (DataType::UShort, DataType::None),
    
        Token::Int if is_array => return (DataType::Ptr, DataType::Int),
        Token::Int => return (DataType::Int, DataType::None),
        
        Token::UInt => return (DataType::UInt, DataType::None),
        
        Token::Int64 => return (DataType::Int64, DataType::None),
        Token::UInt64 => return (DataType::UInt64, DataType::None),
        
        Token::Float if is_array => return (DataType::Ptr, DataType::Float),
        Token::Float => return (DataType::Float, DataType::None),
        
        Token::Double if is_array => return (DataType::Ptr, DataType::Double),
        Token::Double => return (DataType::Double, DataType::None),
        
        Token::TStr if is_array => return (DataType::Ptr, DataType::Str),
        Token::TStr => return (DataType::Str, DataType::None),
        
        _ => return (DataType::None, DataType::None),
    }
}

// A helper function for the function declaration builder
fn build_func_return(builder : &mut AstBuilder, func : &mut AstFunc) -> bool {
    let token = builder.get_token();
    let (ret, _) = token_to_mod(&token, false);
    
    if ret == DataType::None {
        builder.syntax_error("Invalid function return type.".to_string());
        return false;
    }
    
    func.data_type = ret;
    true
}

// Builds a regular function declaration
pub fn build_func(builder : &mut AstBuilder, is_extern : bool) -> bool {
    // The first token should be the function name
    let mut token = builder.get_token();
    let name : String;
    
    match token {
        Token::Id(ref val) => name = val.to_string(),
        _ => {
            builder.syntax_error("Expected function name.".to_string());
            return false;
        },
    }
    
    let mut func : AstFunc;
    
    if is_extern {
        func = ast::create_extern_func(name);
    } else {
        func = ast::create_func(name);
        func.line = builder.scanner.get_current_line();
    }
    
    // Check for arguments, and get them if so
    token = builder.get_token();
    
    if token != Token::LParen {
        if token == Token::Arrow {
            let ret = build_func_return(builder, &mut func);
            
            if !ret {
                return false;
            }
        }
        
        builder.tree.functions.push(func);
        return true;
    }
    
    let mut last_token = Token::LParen;
    
    while token != Token::RParen && token != Token::Eof {
        let name_token = builder.get_token();
        
        let mut arg = ast::create_stmt(AstStmtType::VarDec, &mut builder.scanner);
        
        match name_token {
            Token::Id(ref val) => arg.name = val.to_string(),
            
            Token::Any => {
                token = builder.get_token();
                
                if token == Token::Comma || token == Token::RParen || token == Token::Eof {
                    continue;
                } else {
                    builder.syntax_error("The \"..\" token has no type or name.".to_string());
                    return false;
                }
            },
            
            Token::RParen => {
                if last_token != Token::LParen {
                    builder.syntax_error("Invalid function arguments list.".to_string());
                    return false;
                } else {
                    break;
                }
            },
            
            _ => {
                builder.syntax_error("Expected function argument name.".to_string());
                return false;
            },
        }
        
        let sym_token = builder.get_token();
        let type_token = builder.get_token();
        let mut is_array = false;
        
        last_token = name_token.clone();
        
        if sym_token != Token::Colon {
            builder.syntax_error("Arguments should have a colon between name and type.".to_string());
            return false;
        }
        
        token = builder.get_token();
        
        if token == Token::LBracket {
            token = builder.get_token();
            is_array = true;
            
            if token != Token::RBracket {
                builder.syntax_error("Expected closing \']\'.".to_string());
                return false;
            }
            
            token = builder.get_token();
        }
        
        let (val, sub_val) = token_to_mod(&type_token, is_array);
    
        if val == DataType::None {
            builder.syntax_error("Invalid or missing function argument type.".to_string());
            return false;
        }
        
        arg.data_type = val;
        arg.sub_type = sub_val;
        func.args.push(arg);
        
        if token != Token::Comma && token != Token::RParen {
            builder.syntax_error("Invalid function arguments list.".to_string());
            return false;
        }
    }
    
    token = builder.get_token();
    
    if token == Token::Arrow {
        let ret = build_func_return(builder, &mut func);
        
        if !ret {
            return false;
        }
    }
    
    builder.tree.functions.push(func);
    
    true
}

// Builds a return statement
pub fn build_return(builder : &mut AstBuilder) -> bool {
    let mut ret = ast::create_stmt(AstStmtType::Return, &mut builder.scanner);
    
    if !build_args(builder, &mut ret, Token::Semicolon) {
        return false;
    }
    
    builder.add_stmt(ret);
    
    true
}

// Builds the exit statement
pub fn build_exit(builder : &mut AstBuilder) -> bool {
    let mut exit = ast::create_stmt(AstStmtType::Exit, &mut builder.scanner);
    
    // Build arguments
    if !build_args(builder, &mut exit, Token::Semicolon) {
        return false;
    }
    
    builder.add_stmt(exit);
    
    true
}

// Builds the end statement
pub fn build_end(builder : &mut AstBuilder) {
    let stmt = ast::create_stmt(AstStmtType::End, &mut builder.scanner);
    builder.add_stmt(stmt);
    
    for stmt in builder.current_block.iter() {
        ast::add_stmt(&mut builder.tree, stmt.clone());
    }
    
    builder.current_block.clear();
}

// Builds function calls
pub fn build_func_call(builder : &mut AstBuilder, id_val : String) -> bool {
    let mut fc = ast::create_stmt(AstStmtType::FuncCall, &mut builder.scanner);
    fc.name = id_val;
    
    // Build arguments
    if !build_args(builder, &mut fc, Token::RParen) {
        return false;
    }
    
    let token = builder.get_token();
        
    if token != Token::Semicolon {
        builder.syntax_error("Expected terminator".to_string());
        return false;
    }
    
    // Add the call
    builder.add_stmt(fc);
    
    true
}


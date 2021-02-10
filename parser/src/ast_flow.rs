
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

// Builds conditional statements
pub fn build_cond(builder : &mut AstBuilder, cond_type : Token) -> bool {
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
pub fn build_for_loop(builder : &mut AstBuilder) -> bool {
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


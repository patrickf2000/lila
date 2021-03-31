//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use crate::ast;
use crate::ast::*;
use crate::lex::Token;

use crate::ast_builder::*;
use crate::ast_func::*;
use crate::ast_utils::*;

// Responsible for building a block in a conditional statement or loop
fn build_block(builder : &mut AstBuilder, mut cond_stmt : AstStmt) -> bool {
    let old_block = builder.current_block.clone();
    builder.current_block.clear();
    
    let mut token = builder.get_token();
    let mut code = true;
    
    loop {
        match token {
            Token::Return => code = build_return(builder),
            Token::Exit => code = build_exit(builder),
            Token::Id(ref val) => code = build_id(builder, val.to_string()),
            Token::If => code = build_cond(builder, Token::If),
            Token::While => code = build_cond(builder, Token::While),
            Token::For => code = build_for_loop(builder),
            Token::Eof => {},
            
            Token::End => {
                let stmt = ast::create_stmt(AstStmtType::End, &mut builder.scanner);
                builder.add_stmt(stmt);
                break;
            },
            
            Token::Elif => {
                code = build_cond(builder, Token::Elif);
                break;   
            },
            
            Token::Else => {
                code = build_cond(builder, Token::Else);
                break;
            },
            
            Token::Break => {
                let br = ast::create_stmt(AstStmtType::Break, &mut builder.scanner);
                builder.add_stmt(br);
                
                if builder.get_token() != Token::Semicolon {
                    builder.syntax_error("Expected terminator".to_string());
                    return false;
                }
            },
            
            Token::Continue => {
                let cont = ast::create_stmt(AstStmtType::Continue, &mut builder.scanner);
                builder.add_stmt(cont);
                
                if builder.get_token() != Token::Semicolon {
                    builder.syntax_error("Expected terminator".to_string());
                    return false;
                }
            },
            
            _ => {
                builder.syntax_error("Invalid token in context.".to_string());
                return false;
            }
        }
        
        token = builder.get_token();
    }
    
    cond_stmt.sub_block = builder.current_block.clone();
    builder.current_block.clear();
    builder.current_block = old_block;
    builder.add_stmt(cond_stmt);
    
    code
}

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
        if !build_args(builder, &mut cond, Token::Eof) {
            return false;
        }
    }
    
    build_block(builder, cond);
    
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
    if !build_args(builder, &mut for_loop, Token::Eof) {
        return false;
    }
    
    build_block(builder, for_loop);
    
    true
}


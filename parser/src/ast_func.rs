
use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex};
use crate::syntax::ErrorManager;

use crate::ast_utils::*;

// Builds an external function
pub fn build_extern(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager) -> i32 {
    // Syntax check
    // The first token should be the "func" keyword, and the second token should be an ID
    let token1 = scanner.get_token();
    let token2 = scanner.get_token();
    let name : String;
    
    match token1 {
        Token::Func => {},
        _ => { 
            syntax.syntax_error(scanner, "Expected \"func\" keyword.".to_string());
            return 1;
        }
    }
    
    match token2 {
        Token::Id(ref val) => name = val.to_string(),
        _ => {
            syntax.syntax_error(scanner, "Invalid extern-> Expected function name.".to_string());
            return 1;
        }
    }
    
    let func = ast::create_extern_func(name);
    tree.functions.push(func);
    
    0
}

// A helper function for the function declaration builder
fn build_func_return(scanner : &mut Lex, func : &mut AstFunc, syntax : &mut ErrorManager) -> i32 {
    let token = scanner.get_token();
        
    match token {
        Token::Int => {
            let func_type = AstMod { mod_type : AstModType::Int, };
            func.modifiers.push(func_type);
        },
        
        _ => {
            syntax.syntax_error(scanner, "Invalid function return type.".to_string());
            return 1;
        },
    }
    
    0
}

// Builds a regular function declaration
pub fn build_func(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager) -> i32 {
    // The first token should be the function name
    let mut token = scanner.get_token();
    let name : String;
    
    match token {
        Token::Id(ref val) => name = val.to_string(),
        _ => {
            syntax.syntax_error(scanner, "Expected function name.".to_string());
            return 1;
        },
    }
    
    let mut func = ast::create_func(name);
    
    // Check for arguments, and get them if so
    token = scanner.get_token();
    
    if token != Token::LParen {
        if token == Token::Arrow {
            let ret = build_func_return(scanner, &mut func, syntax);
            
            if ret == 1 {
                return 1;
            }
        }
        
        tree.functions.push(func);
        return 0;
    }
    
    while token != Token::RParen {
        let name_token = scanner.get_token();
        let sym_token = scanner.get_token();
        let type_token = scanner.get_token();
        
        let mut arg = ast::create_stmt(AstStmtType::VarDec);
        
        match name_token {
            Token::Id(ref val) => arg.name = val.to_string(),
            Token::RParen => break,
            
            _ => {
                syntax.syntax_error(scanner, "Expected function argument name.".to_string());
                return 1;
            },
        }
        
        if sym_token != Token::Colon {
            syntax.syntax_error(scanner, "Arguments should have a colon between name and type.".to_string());
            return 1;
        }
        
        match type_token {
            Token::Int => {
                let val_type = AstMod { mod_type : AstModType::Int, };
                arg.modifiers.push(val_type);
            },
            
            Token::TStr => {},
            _ => println!("Error: Invalid function argument type."),
        }
        
        func.args.push(arg);
        
        token = scanner.get_token();
        if token != Token::Comma && token != Token::RParen {
            println!("Error: Invalid function arguments list.");
            return 1;
        }
    }
    
    token = scanner.get_token();
    
    if token == Token::Arrow {
        let ret = build_func_return(scanner, &mut func, syntax);
        
        if ret == 1 {
            return 1;
        }
    }
    
    tree.functions.push(func);
    
    0
}

// Builds a return statement
pub fn build_return(scanner : &mut Lex, tree : &mut AstTree) {
    let mut ret = ast::create_stmt(AstStmtType::Return);
    build_args(scanner, &mut ret, Token::Eof);
    
    ast::add_stmt(tree, ret);
}

// Builds the end statement
pub fn build_end(tree : &mut AstTree) {
    let stmt = ast::create_stmt(AstStmtType::End);
    ast::add_stmt(tree, stmt);
}

// Builds function calls
pub fn build_func_call(scanner : &mut Lex, tree : &mut AstTree, id_val : String) {
    let mut fc = ast::create_stmt(AstStmtType::FuncCall);
    fc.name = id_val;
    
    // Build arguments
    build_args(scanner, &mut fc, Token::RParen);
    
    // Add the call
    ast::add_stmt(tree, fc);
}


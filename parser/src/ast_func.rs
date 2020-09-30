
use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex};

use crate::ast_utils::*;

// Builds an external function
pub fn build_extern(scanner : &mut Lex, tree : &mut AstTree) {
    // Syntax check
    // The first token should be the "func" keyword, and the second token should be an ID
    let token1 = scanner.get_token();
    let token2 = scanner.get_token();
    let mut name = String::new();
    
    // TODO: Better syntax error
    match token1 {
        Token::Func => {},
        _ => println!("Error: Invalid token-> {:?}", token1),
    }
    
    // TODO: Better syntax error
    match token2 {
        Token::Id(ref val) => name = val.to_string(),
        _ => println!("Error: Invalid extern name-> {:?}", token2),
    }
    
    let func = ast::create_extern_func(name);
    tree.functions.push(func);
}

// Builds a regular function declaration
pub fn build_func(scanner : &mut Lex, tree : &mut AstTree) {
    // The first token should be the function name
    let mut token = scanner.get_token();
    let mut name = String::new();
    
    // TODO: Better syntax error
    match token {
        Token::Id(ref val) => name = val.to_string(),
        _ => println!("Error: Invalid function name-> {:?}", token),
    }
    
    let mut func = ast::create_func(name);
    
    // Check for arguments, and get them if so
    token = scanner.get_token();
    
    if token != Token::LParen {
        tree.functions.push(func);
        return;
    }
    
    while token != Token::RParen {
        let name_token = scanner.get_token();
        let sym_token = scanner.get_token();
        let type_token = scanner.get_token();
        
        let mut arg = ast::create_stmt(AstStmtType::VarDec);
        
        match name_token {
            Token::Id(ref val) => arg.name = val.to_string(),
            Token::RParen => break,
            _ => println!("Error: Invalid function argument name-> {:?}", name_token),
        }
        
        if sym_token != Token::Colon {
            println!("Error: Function arguments should have a colon between name and type.");
            return;
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
            return;
        }
    }
    
    tree.functions.push(func);
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


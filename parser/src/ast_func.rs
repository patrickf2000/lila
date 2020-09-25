
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
    let token = scanner.get_token();
    let mut name = String::new();
    
    // TODO: Better syntax error
    match token {
        Token::Id(ref val) => name = val.to_string(),
        _ => println!("Error: Invalid function name-> {:?}", token),
    }
    
    let func = ast::create_func(name);
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



// Import what we need
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex, create_lex};

// The AST building function
// This function opens the file and reads a line; 
// the line is then passed to another function which lexically analyzes
// it and builds an AST node.
//
// In Quik, each line is a self-contained expression; as a result, we read a line
// and then lexically analyze and build an AST node from it
//
pub fn build_ast(path : String, name : String) -> AstTree {   
    let mut tree = AstTree {
        file_name : name,
        functions : Vec::new(),
    };
    
    // Open the file
    let file = File::open(&path)
        .expect("Error: Unable to open input file.");
    let reader = BufReader::new(file);
    
    // Read the thing line by line
    for line in reader.lines() {
        let mut current = line.unwrap();
        current = current.trim().to_string();
        
        if current.len() == 0 {
            continue;
        }
        
        build_line(current, &mut tree);
    }
    
    tree
}

// Converts a line to an AST node
fn build_line(line : String, tree : &mut AstTree) {
    let mut analyzer = create_lex(line);
    analyzer.tokenize();
    
    // Get the first token
    let token = analyzer.get_token();
    
    match token {
        Token::Extern => build_extern(&mut analyzer, tree),
        Token::Func => build_func(&mut analyzer, tree),
        Token::End => build_end(tree),
        Token::Int => build_i32var_dec(&mut analyzer, tree),
        Token::TStr => println!("TStr: {:?}", token),
        Token::Id(ref val) => build_id(&mut analyzer, tree, val.to_string()),
        _ => println!("Error: {:?}", token),
    }
}

// Builds an external function
fn build_extern(scanner : &mut Lex, tree : &mut AstTree) {
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
fn build_func(scanner : &mut Lex, tree : &mut AstTree) {
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

// Builds the end statement
fn build_end(tree : &mut AstTree) {
    let stmt = ast::create_stmt(AstStmtType::End);
    ast::add_stmt(tree, stmt);
}

// Builds an integer variable declaration
fn build_i32var_dec(scanner : &mut Lex, tree : &mut AstTree) {
    let mut var_dec = ast::create_stmt(AstStmtType::VarDec);
        
    let data_type = AstMod {
        mod_type : AstModType::Int,
    };
    var_dec.modifiers.push(data_type);
    
    // Gather information
    // The first token should be the name
    let mut token = scanner.get_token();
    
    // TODO: Better syntax error
    match token {
        Token::Id(ref val) => var_dec.name = val.to_string(),
        _ => println!("Error: Invalid variable name-> {:?}", token),
    }
    
    // The next token should be the assign operator
    token = scanner.get_token();
    
    // TODO: Better syntax error
    match token {
        Token::Assign => {},
        _ => println!("Error: Missing assignment"),
    }
    
    // Build the remaining arguments
    build_args(scanner, &mut var_dec, Token::Eof);

    // Add the declaration
    ast::add_stmt(tree, var_dec);
}

// Handles cases when an identifier is the first token
fn build_id(scanner : &mut Lex, tree : &mut AstTree, id_val : String) {
    // If the next token is an assignment, we have a variable assignment
    // If the next token is a parantheses, we have a function call
    let token = scanner.get_token();
    
    // TODO: Better assignment
    match token {
        Token::Assign => {},
        Token::LParen => build_func_call(scanner, tree, id_val),
        _ => println!("Invalid declaration or assignment"),
    }
}

// Builds function calls
fn build_func_call(scanner : &mut Lex, tree : &mut AstTree, id_val : String) {
    let mut fc = ast::create_stmt(AstStmtType::FuncCall);
    fc.name = id_val;
    
    // Build arguments
    build_args(scanner, &mut fc, Token::RParen);
    
    // Add the call
    ast::add_stmt(tree, fc);
}

// A common function for building statement arguments
fn build_args(scanner : &mut Lex, stmt : &mut AstStmt, end : Token) {
    let mut token = scanner.get_token();
    
    while token != end {
        match token {
            Token::IntL(val) => {
                let arg = ast::create_int(val);
                stmt.args.push(arg);
            },
            
            Token::StringL(ref val) => {
                let arg = ast::create_string(val.to_string());
                stmt.args.push(arg);
            },
            
            Token::Id(ref val) => {
                let mut arg = ast::create_arg(AstArgType::Id);
                arg.str_val = val.to_string();
                stmt.args.push(arg);
            },
            
            Token::OpAdd => {
                let arg = ast::create_arg(AstArgType::OpAdd);
                stmt.args.push(arg);
            },
            
            Token::OpMul => {
                let arg = ast::create_arg(AstArgType::OpMul);
                stmt.args.push(arg);
            },
            
            // TODO: Better syntax error
            _ => println!("Invalid expression argument: {:?}", token),
        }
    
        token = scanner.get_token();
    }
}

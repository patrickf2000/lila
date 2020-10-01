
// Import what we need
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex, create_lex};

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
        Token::Return => build_return(&mut analyzer, tree),
        Token::End => build_end(tree),
        Token::Int => build_i32var_dec(&mut analyzer, tree),
        Token::TStr => println!("TStr: {:?}", token),
        Token::Id(ref val) => build_id(&mut analyzer, tree, val.to_string()),
        Token::If => build_cond(&mut analyzer, tree, Token::If),
        Token::Elif => build_cond(&mut analyzer, tree, Token::Elif),
        Token::Else => build_cond(&mut analyzer, tree, Token::Else),
        Token::While => build_cond(&mut analyzer, tree, Token::While),
        
        Token::Break => {
            let br = ast::create_stmt(AstStmtType::Break);
            ast::add_stmt(tree, br);
        },
        
        Token::Continue => {
            let cont = ast::create_stmt(AstStmtType::Continue);
            ast::add_stmt(tree, cont);
        },
        
        Token::Eof => {},
        _ => println!("Error: {:?}", token),
    }
}

// Builds an integer variable declaration
fn build_i32var_dec(scanner : &mut Lex, tree : &mut AstTree) {
    let mut var_dec = ast::create_stmt(AstStmtType::VarDec);
    let mut is_array = false;
        
    let mut data_type = AstMod {
        mod_type : AstModType::Int,
    };
    
    // Gather information
    // The first token should be the name
    let mut token = scanner.get_token();
    
    // TODO: Better syntax error
    match token {
        Token::Id(ref val) => var_dec.name = val.to_string(),
        
        Token::LBracket => {
            is_array = true;
            build_args(scanner, &mut var_dec, Token::RBracket);
            
            token = scanner.get_token();
            match token {
                Token::Id(ref val) => var_dec.name = val.to_string(),
                _ => println!("Error: Invalid array name-> {:?}", token),
            }
        },
        
        _ => println!("Error: Invalid variable-> {:?}", token),
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
    
    // If we have the array, check the array type
    if is_array {
        if var_dec.args.len() == 1 && var_dec.args.last().unwrap().arg_type == AstArgType::Array {
            data_type.mod_type = AstModType::IntDynArray;
        } else {
            //TODO
        }
    }
    
    var_dec.modifiers.push(data_type);
    ast::add_stmt(tree, var_dec);
}

// Builds a variable assignment
fn build_var_assign(scanner : &mut Lex, tree : &mut AstTree, name : String) {
    let mut var_assign = ast::create_stmt(AstStmtType::VarAssign);
    var_assign.name = name;
    
    build_args(scanner, &mut var_assign, Token::Eof);
    ast::add_stmt(tree, var_assign);
}

// Builds an array assignment
fn build_array_assign(scanner : &mut Lex, tree : &mut AstTree, id_val : String) {
    let mut array_assign = ast::create_stmt(AstStmtType::ArrayAssign);
    array_assign.name = id_val;
    build_args(scanner, &mut array_assign, Token::RBracket);
    
    // TODO: Better error
    if scanner.get_token() != Token::Assign {
        println!("Expected \'=\' in array assignment.");
    }
    
    build_args(scanner, &mut array_assign, Token::Eof);
    ast::add_stmt(tree, array_assign);
}

// Handles cases when an identifier is the first token
fn build_id(scanner : &mut Lex, tree : &mut AstTree, id_val : String) {
    // If the next token is an assignment, we have a variable assignment
    // If the next token is a parantheses, we have a function call
    let token = scanner.get_token();
    
    // TODO: Better assignment error
    match token {
        Token::Assign => build_var_assign(scanner, tree, id_val),
        Token::LParen => build_func_call(scanner, tree, id_val),
        Token::LBracket => build_array_assign(scanner, tree, id_val),
        _ => println!("Invalid declaration or assignment"),
    }
}

// Builds conditional statements
fn build_cond(scanner : &mut Lex, tree : &mut AstTree, cond_type : Token) {
    let mut ast_cond_type : AstStmtType = AstStmtType::If;
    match cond_type {
        Token::If => ast_cond_type = AstStmtType::If,
        Token::Elif => ast_cond_type = AstStmtType::Elif,
        Token::Else => ast_cond_type = AstStmtType::Else,
        Token::While => ast_cond_type = AstStmtType::While,
        _ => {},
    }
    
    let mut cond = ast::create_stmt(ast_cond_type);
    
    // Build arguments
    if cond_type != Token::Else {
        build_args(scanner, &mut cond, Token::Eof);
    }
    
    // Add the conditional
    ast::add_stmt(tree, cond);
}


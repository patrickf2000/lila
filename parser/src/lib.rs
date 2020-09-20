
// Expose the AST and LTAC libraries
pub mod ast;
pub mod ltac;

mod lex;

// Import what we need
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

use ast::AstTree;
use lex::{Token, Lex, create_lex};

// The main parse function
// This function opens the file and reads a line; 
// the line is then passed to another function which lexically analyzes
// it and builds an AST node.
//
// In Quik, each line is a self-contained expression; as a result, we read a line
// and then lexically analyze and build an AST node from it
//
pub fn parse(path : String) {
    let file_path = Path::new(&path);
    let name = file_path.file_stem()
        .unwrap().to_os_string()
        .into_string().unwrap();
    
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
    
    // TODO: Remove this
    tree.print();
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
        Token::End => println!("End: {:?}", token),
        Token::Int => println!("Int: {:?}", token),
        Token::TStr => println!("TStr: {:?}", token),
        Token::Id(ref _val) => println!("Id: {:?}", token),
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
    
    // TODO: Syntax error
    match token1 {
        Token::Func => {},
        _ => println!("Error: Invalid token-> {:?}", token1),
    }
    
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
    
    match token {
        Token::Id(ref val) => name = val.to_string(),
        _ => println!("Error: Invalid function name-> {:?}", token),
    }
    
    let func = ast::create_func(name);
    tree.functions.push(func);
}

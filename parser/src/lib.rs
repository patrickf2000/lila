
// Expose the AST and LTAC libraries
pub mod ast;
pub mod ltac;

mod lex;

// Import what we need
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

use ast::AstTree;
use lex::{Token, create_lex};

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
    
    let tree = AstTree {
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
        
        build_line(current);
    }
    
    // TODO: Remove this
    tree.print();
}

// Converts a line to an AST node
fn build_line(line : String) {
    let mut analyzer = create_lex(line);
    analyzer.tokenize();
    
    let mut token = analyzer.get_token();
    
    while token != Token::Eof {
        print!("{:?} ", token);
        token = analyzer.get_token();
    }
    
    println!("");
}

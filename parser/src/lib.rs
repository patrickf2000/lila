
// Expose the AST and LTAC libraries
pub mod ast;
pub mod ltac;

mod ast_builder;
mod ast_func;
mod ast_utils;
mod ltac_builder;
mod ltac_array;
mod ltac_flow;
mod ltac_func;
mod ltac_var;
mod lex;

// Import what we need
use std::path::Path;

use ast::AstTree;
use ltac::LtacFile;

// Returns the ast
pub fn get_ast(path : &String) -> AstTree {
    let name = get_name(path);
    let tree = ast_builder::build_ast(path.to_string(), name.clone());
    tree
}

// The main parse function
pub fn parse(path : String) -> LtacFile {
    let tree = get_ast(&path.to_string());
    let name = get_name(&path);
    
    let mut ltac_builder = ltac_builder::new_ltac_builder(name.clone());
    let ltac = ltac_builder.build_ltac(&tree);
    
    ltac
}

// Returns the file name for a given string
fn get_name(path : &String) -> String {
    let file_path = Path::new(path);
    let name = file_path.file_stem()
        .unwrap().to_os_string()
        .into_string().unwrap();
    name
}

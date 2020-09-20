
// Expose the AST and LTAC libraries
pub mod ast;
pub mod ltac;

mod ast_builder;
mod lex;

// The main parse function
pub fn parse(path : String) {
    let tree = ast_builder::build_ast(path);
    
    println!("AST:");
    tree.print();
}

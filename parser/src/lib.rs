
// Expose the AST and LTAC libraries
pub mod ast;
pub mod ltac;

mod ast_builder;
mod ltac_builder;
mod lex;

use std::path::Path;

// The main parse function
pub fn parse(path : String) {
    let file_path = Path::new(&path);
    let name = file_path.file_stem()
        .unwrap().to_os_string()
        .into_string().unwrap();
    
    let tree = ast_builder::build_ast(path, name.clone());
    
    let mut ltac_builder = ltac_builder::new_ltac_builder(name.clone());
    let ltac = ltac_builder.build_ltac(&tree);
    
    println!("AST:");
    tree.print();
    
    println!("");
    println!("LTAC:");
    ltac.print();
}

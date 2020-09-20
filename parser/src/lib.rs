
// Expose the AST and LTAC libraries
pub mod ast;
pub mod ltac;

mod ast_builder;
mod ltac_builder;
mod lex;

// Import what we need
use std::path::Path;

use ltac::LtacFile;

// The main parse function
pub fn parse(path : String) -> LtacFile {
    let file_path = Path::new(&path);
    let name = file_path.file_stem()
        .unwrap().to_os_string()
        .into_string().unwrap();
    
    let tree = ast_builder::build_ast(path, name.clone());
    
    let mut ltac_builder = ltac_builder::new_ltac_builder(name.clone());
    let ltac = ltac_builder.build_ltac(&tree);
    
    ltac
}

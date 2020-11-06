
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs;
use std::fs::File;

use crate::ast::*;

// Generates a header definition
pub fn generate_module(tree : &AstTree) -> io::Result<()> {
    let mut path = "./".to_string();
    
    if tree.module != "default" {
        path = tree.module.replace(".", "/");
        path.push('/');
        fs::create_dir_all(&path)?;
    }
    
    path.push_str(&tree.file_name);
    path.push_str(".di");
    
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    
    let mut line = String::new();
    
    line.push_str("# MODULE ");
    line.push_str(&tree.module);
    line.push_str(".");
    line.push_str(&tree.file_name);
    line.push_str("\n");
    
    line.push_str("# DO NOT MODIFY. This will be rewritten each time you compile.");
    line.push_str("\n\n");
    
    // Now iterate through each function
    for func in tree.functions.iter() {
        line.push_str("extern ");
        line.push_str(&func.line);
        line.push_str("\n");
    }
    
    // Write it all out
    writer.write(&line.into_bytes())
        .expect("Module write failed.");
    
    Ok(())
}

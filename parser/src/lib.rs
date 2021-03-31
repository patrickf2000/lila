//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

// Expose the AST and LTAC libraries
pub mod ast;
pub mod ltac;
pub mod syntax;
pub mod module;

mod ast_builder;
mod ast_func;
mod ast_flow;
mod ast_utils;
mod ast_var;
mod lex;

mod ltac_builder;
mod ltac_expr;
mod ltac_array;
mod ltac_flow;
mod ltac_for;
mod ltac_func;
mod ltac_utils;
mod ltac_var;

#[derive(PartialEq, Clone, Copy)]
pub enum Arch {
    X86_64,
    AArch64,
    Riscv64,
}

// Import what we need
use std::path::Path;

use ast::AstTree;
use ltac::LtacFile;

// Returns the ast
pub fn get_ast(path : &String, arch : Arch, include_core : bool, keep_postfix : bool) -> Result<AstTree, ()> {
    let name = get_name(path);
    let tree = match ast_builder::build_ast(path.to_string(), arch, name.clone(), include_core, keep_postfix) {
        Ok(tree) => tree,
        Err(_e) => return Err(()),
    };
    
    Ok(tree)
}

// The main parse function
pub fn parse(path : String, arch : Arch, include_core : bool) -> Result<LtacFile, ()> {
    let tree = match get_ast(&path.to_string(), arch, include_core, false) {
        Ok(tree) => tree,
        Err(_e) => return Err(()),
    };
    
    if tree.module.len() > 0 {
        match module::generate_module(&tree) {
            Ok(()) => {},
            Err(_e) => {
                println!("Error generating module header");
                return Err(());
            },
        }
    }
    
    let mut syntax = syntax::create_error_manager();
    let name = get_name(&path);
    
    let mut ltac_builder = ltac_builder::new_ltac_builder(name.clone(), &mut syntax);
    let ltac = match ltac_builder.build_ltac(&tree) {
        Ok(ltac) => ltac,
        Err(_e) => return Err(()),
    };
    
    Ok(ltac)
}

// Returns the file name for a given string
pub fn get_name(path : &String) -> String {
    let file_path = Path::new(path);
    let name = file_path.file_stem()
        .unwrap().to_os_string()
        .into_string().unwrap();
    name
}



// This file is part of the Dash compiler
// Copyright (C) 2020 Patrick Flynn
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.


// Expose the AST and LTAC libraries
pub mod ast;
pub mod ltac;

mod ast_builder;
mod ast_func;
mod ast_utils;
mod ltac_builder;
mod ltac_expr;
mod ltac_array;
mod ltac_flow;
mod ltac_func;
mod ltac_var;
mod lex;
mod module;
mod syntax;

// Import what we need
use std::path::Path;

use ast::AstTree;
use ltac::LtacFile;

// Returns the ast
pub fn get_ast(path : &String) -> Result<AstTree, ()> {
    let mut syntax = syntax::create_error_manager();

    let name = get_name(path);
    let tree = match ast_builder::build_ast(path.to_string(), name.clone(), &mut syntax) {
        Ok(tree) => tree,
        Err(_e) => return Err(()),
    };
    
    Ok(tree)
}

// The main parse function
pub fn parse(path : String) -> Result<LtacFile, ()> {
    let tree = match get_ast(&path.to_string()) {
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
fn get_name(path : &String) -> String {
    let file_path = Path::new(path);
    let name = file_path.file_stem()
        .unwrap().to_os_string()
        .into_string().unwrap();
    name
}


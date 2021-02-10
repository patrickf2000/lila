
// This file is part of the Lila compiler
// Copyright (C) 2020-2021 Patrick Flynn
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
pub mod llir;
pub mod syntax;
pub mod module;

mod ast_builder;
mod ast_func;
mod ast_flow;
mod ast_utils;
mod ast_var;
mod lex;

mod llir_builder;
mod llir_func;
mod llir_var;

#[derive(PartialEq, Clone, Copy)]
pub enum Arch {
    X86_64,
    AArch64,
    Riscv64,
}

// Import what we need
use std::path::Path;

use ast::AstTree;
use llir::LLirFile;

// Returns the ast
pub fn get_ast(path : &String, arch : Arch, include_core : bool, keep_postfix : bool) -> Result<AstTree, ()> {
    let name = get_name(path);
    let tree = match ast_builder::build_ast(path.to_string(), arch, name.clone(), include_core, keep_postfix) {
        Ok(tree) => tree,
        Err(_e) => return Err(()),
    };
    
    Ok(tree)
}

// The parse function for the LLIR layer
// This will eventually replace the function above
pub fn parse2(path : String, arch : Arch, include_core : bool) -> Result<LLirFile, ()> {
    let tree = match get_ast(&path.to_string(), arch, include_core, true) {
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
    
    let mut llir_builder = llir_builder::new_llir_builder(name.clone(), &mut syntax);
    let llir = match llir_builder.build_llir(&tree) {
        Ok(llir) => llir,
        Err(_e) => return Err(()),
    };
    
    Ok(llir)
}

// Returns the file name for a given string
pub fn get_name(path : &String) -> String {
    let file_path = Path::new(path);
    let name = file_path.file_stem()
        .unwrap().to_os_string()
        .into_string().unwrap();
    name
}


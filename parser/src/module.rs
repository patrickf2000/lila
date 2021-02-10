
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


use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::Arch;
use crate::ast_builder::{AstBuilder, include_module};
use crate::ast::*;
use crate::lex::{Token, Lex};
use crate::syntax::ErrorManager;

// Builds a "module" declaration
pub fn build_module(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager) -> bool {
    if tree.module.len() > 0 {
        syntax.syntax_error(scanner, "Duplicate module declarations.".to_string());
        return false;
    }
    
    let token = scanner.get_token();
    
    match token {
        Token::Id(ref val) => tree.module = val.clone(),
        _ => {
            syntax.syntax_error(scanner, "Module names must be an identifier.".to_string());
            return false;
        },
    }
    
    if scanner.get_token() != Token::Semicolon {
        syntax.syntax_error(scanner, "Expecting terminator".to_string());
        return false;
    }
    
    true
}

// Builds a "use" declaration
pub fn build_use(b : &mut AstBuilder) -> bool {
    let module : String;
    let mut do_include = true;
    let mut token = b.scanner.get_token();
    
    match token {
        Token::Id(ref val) => module = val.clone(),
        _ => {
            b.syntax.syntax_error(&mut b.scanner, "Module names must be an identifier.".to_string());
            return false;
        },
    }
    
    token = b.scanner.get_token();
    
    if token == Token::If {
        token = b.scanner.get_token();
        
        let arch_str = match token {
            Token::StringL(ref val) => val.clone(),
            _ => {
                b.syntax.syntax_error(&mut b.scanner, "Expected string with architecture type.".to_string());
                return false;
            },
        };
        
        token = b.scanner.get_token();
        if token != Token::Semicolon {
            b.syntax.syntax_error(&mut b.scanner, "Expecting terminator".to_string());
            return false;
        }
        
        let arch2 = match arch_str.as_str() {
            "x86_64" => Arch::X86_64,
            "aarch64" => Arch::AArch64,
            "riscv64" => Arch::Riscv64,
            
            _ => {
                b.syntax.syntax_error(&mut b.scanner, "Invalid architecture".to_string());
                return false;
            },
        };
        
        if arch2 != b.tree.arch {
            do_include = false;
        }
    } else if token != Token::Semicolon {
        b.syntax.syntax_error(&mut b.scanner, "Expecting terminator".to_string());
        return false;
    }
    
    if do_include {
        return include_module(module, b);
    }
    
    true
}

// Builds a module path and performs various checks
pub fn get_module_path(name : &String) -> String {
    let mut path = name.replace("default", "");
    path = path.replace(".", "/");
    path.push_str(".lh");
    
    if Path::new(&path).exists() {
        return path;
    }
    
    // The three paths to check, in order of importance
    let mut path1 = "/usr/lib/lila/".to_string();
    path1.push_str(&path);
    
    let mut path2 = "/usr/local/lib/lila/".to_string();
    path2.push_str(&path);
    
    let mut path3 = "/opt/lila/".to_string();
    path3.push_str(&path);
    
    if Path::new(&path1).exists() {
        return path1;
    }
    
    if Path::new(&path2).exists() {
        return path2;
    }
    
    if Path::new(&path3).exists() {
        return path3;
    }
    
    path
}

// Generates a header definition
pub fn generate_module(tree : &AstTree) -> io::Result<()> {
    let mut path = "./".to_string();
    
    if tree.module != "default" {
        path = tree.module.replace(".", "/");
        path.push('/');
        fs::create_dir_all(&path)?;
    }
    
    path.push_str(&tree.file_name);
    path.push_str(".lh");
    
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
    
    // Iterate through all the constants
    for c in tree.constants.iter() {
        if c.line.len() == 0 {
            continue;
        }
        
        line.push_str(&c.line);
        line.push_str("\n");
    }
    
    // Now iterate through each function
    for func in tree.functions.iter() {
        if func.line.len() == 0 {
            continue;
        }
        
        line.push_str("extern ");
        line.push_str(&func.line);
        line.push_str("\n");
    }
    
    // Write it all out
    writer.write(&line.into_bytes())
        .expect("Module write failed.");
    
    Ok(())
}

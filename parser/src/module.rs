
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


use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::ast::*;

// Builds a module path and performs various checks
pub fn get_module_path(name : &String) -> String {
    let mut path = name.replace("default", "");
    path = path.replace(".", "/");
    path.push_str(".di");
    
    if Path::new(&path).exists() {
        return path;
    }
    
    // The three paths to check, in order of importance
    let mut path1 = "/usr/lib/dash/".to_string();
    path1.push_str(&path);
    
    let mut path2 = "/usr/local/lib/dash/".to_string();
    path2.push_str(&path);
    
    let mut path3 = "/opt/dash/".to_string();
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

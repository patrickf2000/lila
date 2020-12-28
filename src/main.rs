
// This file is part of the Lila compiler
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


use std::env;
use std::process;

use parser;
use parser::Arch;
use transform;

#[cfg(target_arch = "x86_64")]
fn get_arch() -> Arch {
    Arch::X86_64
}

#[cfg(target_arch = "aarch64")]
fn get_arch() -> Arch {
    Arch::AArch64
}

#[cfg(target_arch = "riscv64")]
fn get_arch() -> Arch {
    Arch::Riscv64
}

// TODO: Is there a better way to do this?
fn main() {
    let code = run();
    process::exit(code);
}

fn run() -> i32 {
    let mut args : Vec<String> = env::args().collect();
    args.remove(0);
    
    if args.is_empty() {
        println!("Fatal: No input file specified.");
        return 2;
    }
    
    let mut print_ast = false;
    let mut print_ltac = false;
    let mut use_c = false;
    let mut link_lib = false;
    let mut no_link = false;
    let mut pic = false;
    let mut risc_mode = false;      // This is a dev feature to allow us to work on the RISC optimizer on x86
    let mut arch = get_arch();
    let mut inputs : Vec<String> = Vec::new();
    let mut output : String = "a.out".to_string();
    
    let mut next_output = false;
    
    for arg in args {
        if next_output {
            output = arg.clone();
            next_output = false;
            continue;
        }
    
        match arg.as_ref() {
            "--ast" => print_ast = true,
            "--ltac" => print_ltac = true,
            "--use-c" => use_c = true,
            "--lib" => {
                link_lib = true;
                pic = true;
            },
            "--pic" => pic = true,
            "--risc" => risc_mode = true,
            "--no-link" => no_link = true,
            "-o" => next_output = true,
            
            "-march=riscv64" => arch = Arch::Riscv64,
            
            "-h" | "--help" => {
                help();
                return 0;
            },
            
            _ => inputs.push(arg.clone()),
        }
    }
    
    if print_ast {
        let input = inputs.last().unwrap();
        let ast = match parser::get_ast(&input, arch) {
            Ok(ast) => ast,
            Err(_e) => return 1,
        };
        
        ast.print();
        return 0;
    }
    
    let mut all_names : Vec<String> = Vec::new();
    
    for input in inputs {
        if input.starts_with("-l") || input.ends_with(".o") {
            all_names.push(input);
            continue;
        }
    
        // Build the LTAC portion
        let mut ltac = match parser::parse(input, arch) {
            Ok(ltac) => ltac,
            Err(_e) => return 1,
        };
        
        // Do any needed transformations or optimizations
        ltac = match transform::run(&ltac, arch, use_c, risc_mode) {
            Ok(ltac) => ltac,
            Err(_e) => return 1,
        };
        
        all_names.push(ltac.name.clone());
        
        // Now compile
        if print_ltac {
            ltac_printer::compile(&ltac).expect("LTAC Codegen failed with unknown error."); 
        } else if arch == Arch::X86_64 {
            x86::compile(&ltac, pic, risc_mode).expect("Codegen failed with unknown error.");
            x86::build_asm(&ltac.name, no_link);
        } else if arch == Arch::AArch64 {
            aarch64::compile(&ltac).expect("Codegen failed with unknown error.");
            aarch64::build_asm(&ltac.name, no_link);
        } else if arch == Arch::Riscv64 {
            riscv64::compile(&ltac).expect("Codegen failed with unknown error.");
            riscv64::build_asm(&ltac.name, no_link);
        } else {
            // TODO
        }
    }
    
    // Link
    if !no_link && !print_ltac {
        if arch == Arch::X86_64 {
            x86::link(&all_names, &output, use_c, link_lib);
        } else if arch == Arch::AArch64 {
            aarch64::link(&all_names, &output, use_c, link_lib);
        } else if arch == Arch::Riscv64 {
            riscv64::link(&all_names, &output, use_c, link_lib);
        }
    }
    
    0
}

// Displays compiler help
fn help() {
    println!("lilac version 0.1");
    println!("");
    println!("--ast \t\t Print a textual representation of the AST");
    println!("--ltac \t\t Save the LTAC IR to a file.");
    println!("--use-c \t Link to C start-up files and the C standard library.");
    println!("--lib \t\t Generate a dynamic library.");
    println!("--pic \t\t Generate position independent code (x86 only- you need this if you are building a library)");
    println!("--no-link \t Only generate an object file.");
    println!("-l<lib> \t Link to a certain library.");
    println!("-o <name> \t Specify the output name.");
    println!("--risc \t\t Run the RISC optimizer regardless of platform (the x86 code generator can convert RISC instructions).");
    println!("-h, --help \t Display this message and exit.");
    println!("");
}

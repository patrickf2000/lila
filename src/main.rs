
use std::env;
use std::process;

use parser;
use transform;
use transform::Arch;

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
    let arch = Arch::X86_64;
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
            "--no-link" => no_link = true,
            "-o" => next_output = true,
            _ => inputs.push(arg.clone()),
        }
    }
    
    if print_ast {
        let input = inputs.last().unwrap();
        let ast = match parser::get_ast(&input) {
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
        let mut ltac = match parser::parse(input) {
            Ok(ltac) => ltac,
            Err(_e) => return 1,
        };
        
        // Do any needed transformations or optimizations
        ltac = match transform::run(&ltac, arch, use_c) {
            Ok(ltac) => ltac,
            Err(_e) => return 1,
        };
        
        all_names.push(ltac.name.clone());
        
        // Now compile
        if print_ltac {
            ltac_printer::compile(&ltac).expect("LTAC Codegen failed with unknown error."); 
        } else if arch == Arch::X86_64 {
            x86::compile(&ltac, pic).expect("Codegen failed with unknown error.");
            x86::build_asm(&ltac.name, no_link);
        } else {
            // TODO
        }
    }
    
    // Link
    if !no_link && !print_ltac {
        if arch == Arch::X86_64 {
            x86::link(&all_names, &output, use_c, link_lib);
        }
    }
    
    0
}


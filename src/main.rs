
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
    
    let mut print_ast = false;
    let mut print_ltac = false;
    let mut use_c = false;
    let mut arch = Arch::X86_64;
    let mut input = String::new();
    
    if args.is_empty() {
        println!("Fatal: No input file specified.");
        return 2;
    }
    
    for arg in args {
        match arg.as_ref() {
            "--ast" => print_ast = true,
            "--ltac" => print_ltac = true,
            "--use-c" => use_c = true,
            "--amd64" => arch = Arch::X86_64,
            "--aarch64" => arch = Arch::AArch64,
            _ => input = arg.clone(),
        }
    }
    
    if print_ast {
        let ast = match parser::get_ast(&input) {
            Ok(ast) => ast,
            Err(_e) => return 1,
        };
        
        ast.print();
    } else {
        let mut ltac = match parser::parse(input) {
            Ok(ltac) => ltac,
            Err(_e) => return 1,
        };
        
        // Do any needed transformations or optimizations
        ltac = match transform::run(&ltac, arch, use_c) {
            Ok(ltac) => ltac,
            Err(_e) => return 1,
        };
            
        // Print if the user wants
        if print_ltac {
            ltac.print();
            return 0;
        }
        
        if arch == Arch::X86_64 {
            x86::compile(&ltac).expect("Codegen failed with unknown error.");
            x86::build_asm(&ltac.name, use_c);
        } else if arch == Arch::AArch64 {
            aarch64::compile(&ltac).expect("Codegen failed with unknown error.");
            aarch64::build_asm(&ltac.name, use_c);
        }
    }
    
    0
}

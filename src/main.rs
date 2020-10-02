use std::env;

use parser;
//use x86;
//use aarch64;

fn main() {
    let mut args : Vec<String> = env::args().collect();
    args.remove(0);
    
    let mut print_ast = false;
    let mut print_ltac = false;
    let mut use_c = false;
    let mut amd64 = true;
    let mut input = String::new();
    
    if args.is_empty() {
        println!("Fatal: No input file specified.");
        return;
    }
    
    for arg in args {
        match arg.as_ref() {
            "--ast" => print_ast = true,
            "--ltac" => print_ltac = true,
            "--use-c" => use_c = true,
            "--amd64" => amd64 = true,
            "--arm64" => amd64 = false,
            _ => input = arg.clone(),
        }
    }
    
    if print_ast {
        let ast = match parser::get_ast(&input) {
            Ok(ast) => ast,
            Err(_e) => return,
        };
        
        ast.print();
    } else {
        let ltac = match parser::parse(input) {
            Ok(ltac) => ltac,
            Err(_e) => return,
        };
        
        if print_ltac {
            ltac.print();
            return;
        }
        
        if amd64 {
            x86::compile(&ltac).expect("Codegen failed with unknown error.");
            x86::build_asm(&ltac.name, use_c);
        } else {
            aarch64::compile(&ltac).expect("Codegen failed with unknown error.");
            aarch64::build_asm(&ltac.name, use_c);
        }
    }
}

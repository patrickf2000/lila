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
            _ => input = arg.clone(),
        }
    }
    
    if print_ast {
        let ast = parser::get_ast(&input);
        ast.print();
    } else if print_ltac {
        let ltac = parser::parse(input);
        ltac.print();
    } else {
        let ltac = parser::parse(input);
        x86::compile(&ltac).expect("Codegen failed with unknown error.");
        x86::build_asm(&ltac.name, use_c);
        
        //aarch64::compile(&ltac).expect("Codegen failed with unknown error.");
        //aarch64::build_asm(&ltac.name, use_c);
    }
}

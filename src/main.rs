use std::env;

use parser;
use x86;

mod test1;

fn main() {
    let mut args : Vec<String> = env::args().collect();
    args.remove(0);
    
    let mut test1 = false;
    let mut input = String::new();
    
    if args.is_empty() {
        println!("Fatal: No input file specified.");
        return;
    }
    
    for arg in args {
        match arg.as_ref() {
            "--test1" => test1 = true,
            _ => input = arg.clone(),
        }
    }
    
    if test1 {
        println!("AST:");
        test1::build_ast();
        
        println!("");
        
        println!("LTAC:");
        test1::build_ltac();
    } else {
        let ltac = parser::parse(input);
        x86::compile(&ltac).expect("Codegen failed with unknown error.");
        x86::build_asm(&ltac.name);
    }
}

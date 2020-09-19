use std::env;

mod test1;

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut test1 = false;
    
    for arg in args {
        if arg == "--test1" {
            test1 = true;
        }
    }
    
    if test1 {
        println!("AST:");
        test1::build_ast();
        
        println!("");
        
        println!("LTAC:");
        test1::build_ltac();
    } else {
        println!("Welcome to rquik");
    }
}

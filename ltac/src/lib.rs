
// Import any local modules
pub mod transform;
pub mod ltac;

mod risc;
mod riscv;
mod ltac_builder;
mod ltac_expr;
mod ltac_array;
mod ltac_flow;
mod ltac_for;
mod ltac_func;
mod ltac_utils;
mod ltac_var;

use ltac::LtacFile;
use parser::*;

// The main parse function
pub fn parse(path : String, arch : Arch, include_core : bool) -> Result<LtacFile, ()> {
    let tree = match get_ast(&path.to_string(), arch, include_core, false) {
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
    
    let mut ltac_builder = ltac_builder::new_ltac_builder(name.clone(), &mut syntax);
    let ltac = match ltac_builder.build_ltac(&tree) {
        Ok(ltac) => ltac,
        Err(_e) => return Err(()),
    };
    
    Ok(ltac)
}


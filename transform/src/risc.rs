
use parser::ltac::{LtacFile/*, LtacType, LtacArg*/};

pub fn risc_optimize(file : &LtacFile) -> Result<LtacFile, ()> {
    let file2 = LtacFile {
        name : file.name.clone(),
        data : file.data.clone(),
        code : Vec::new(),
    };
    
    //let code = file.code.clone();
    
    Ok(file2)
}


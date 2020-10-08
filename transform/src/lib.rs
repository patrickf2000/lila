
use parser::ltac::LtacFile;

// The main transformation function
pub fn run(file : &LtacFile, _use_c : bool) -> Result<LtacFile, ()> {
    let file2 = file.clone();
    Ok(file2)
}


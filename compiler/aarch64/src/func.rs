use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType, LtacArg};

// Builds an extern declaration
pub fn aarch64_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(".extern ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_extern] Write failed.");
}

// Builds a label
pub fn aarch64_build_label(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(&code.name);
    line.push_str(":\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_label] Write failed.");
}

// Builds a function

// Builds a return statement

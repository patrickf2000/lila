
// This module translates the RISC load/store instructions
// Although x86 doesn't have these, we can replicate it with the general move instructions
//
// You ideally wouldn't be using these in real life, this is only if you need to test RISC
// transformation on an x86 machine, which quite frankly is more realistic since Raspberry PI's and emulators
// are slow.

use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::utils::*;

// Build the store instructions
pub fn amd64_build_load_store(writer : &mut BufWriter<File>, code : &LtacInstr, is_load : bool) {
    let mut line = String::new();
    
    match code.instr_type {
        LtacType::Ld | LtacType::Str => line = "  mov ".to_string(),
        
        _ => {},
    }
    
    let pos = match &code.arg1 {
        LtacArg::Mem(pos) => *pos,
        _ => 0,
    };
    
    let reg : String = match &code.arg2 {
        LtacArg::Reg32(pos) => amd64_op_reg32(*pos),
        
        _ => String::new(),
    };
    
    if is_load {
        line.push_str(&reg);
        line.push_str(", [rbp-");
        line.push_str(&pos.to_string());
        line.push_str("]\n");
    } else {
        line.push_str("[rbp-");
        line.push_str(&pos.to_string());
        line.push_str("], ");
        line.push_str(&reg);
        line.push_str("\n");
    }
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_store] Store failed.");
}


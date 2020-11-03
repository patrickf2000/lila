use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::utils::*;

//Builds the move-to-vector instruction for integers
pub fn amd64_build_vector_instr(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let instr : String;
    
    match code.arg2 {
        LtacArg::Mem(pos) => {
            line.push_str("  mov r15, QWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        _ => {},
    }
    
    match &code.instr_type {
        LtacType::MovI32Vec =>  instr = "  vmovups ".to_string(),
        LtacType::I32VAdd => instr = "  vaddps ".to_string(),
        _ => return,
    }
    
    match &code.arg1 {
        LtacArg::Mem(pos) => {
            line.push_str("  mov r15, QWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            line.push_str(&instr);
            line.push_str("[r15+");
            line.push_str(&code.arg1_offset.to_string());
            line.push_str("*");
            line.push_str(&code.arg2_offset_size.to_string());
            line.push_str("], ");
        },
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_vector_i32(*pos);
            
            line.push_str(&instr);
            line.push_str(&reg);
            line.push_str(", ");
            
            if code.instr_type != LtacType::MovI32Vec {
                line.push_str(&reg);
                line.push_str(", ");
            }
        },
        
        LtacArg::I32(_v) => {},
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Mem(_p) => {
            line.push_str("[r15+");
            line.push_str(&code.arg2_offset.to_string());
            line.push_str("*");
            line.push_str(&code.arg2_offset_size.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I32(_v) => {},
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_vector_i32(*pos);
            
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    // Write everything to the file
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_mov_vector] Write failed.");
}


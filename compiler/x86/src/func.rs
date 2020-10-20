use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType};
use crate::utils::*;

// Builds an extern declaration
pub fn amd64_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(".extern ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_extern] Write failed.");
}

// Builds a label
pub fn amd64_build_label(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(&code.name);
    line.push_str(":\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_label] Write failed.");
}

// Builds a function
// Params: name -> function name
//         arg1_val -> stack size
pub fn amd64_build_func(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();

    line.push_str("\n.global ");
    line.push_str(&code.name);
    line.push_str("\n");
    line.push_str(&code.name);
    line.push_str(":\n");
    
    line.push_str("  push rbp\n");
    line.push_str("  mov rbp, rsp\n");
    
    if code.arg1_val > 0 {
        line.push_str("  sub rsp, ");
        line.push_str(&code.arg1_val.to_string());
        line.push_str("\n");
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_func] Write failed.");
}

// Load a function argument to a variable
// In the LtacInstr:
//      -> arg1_val = memory location
//      -> arg2_val = register position
pub fn amd64_build_ldarg(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    line.push_str("  mov [rbp-");
    line.push_str(&code.arg1_val.to_string());
    line.push_str("], ");
    
    if code.instr_type == LtacType::LdArgI8 {
        let reg = amd64_arg_reg8(code.arg2_val);
        line.push_str(&reg);
    } else if code.instr_type == LtacType::LdArgI32 {
        let reg = amd64_arg_reg32(code.arg2_val);
        line.push_str(&reg);
    } else if code.instr_type == LtacType::LdArgPtr {
        let reg = amd64_arg_reg64(code.arg2_val);
        line.push_str(&reg);
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_ldarg] Write failed.");
}

// Loads a 32-bit floating point argument to a variable
// In the LtacInstr:
//      -> arg1_val = memory location
//      -> arg2_val = register position
pub fn amd64_build_ldarg_float(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let reg = amd64_arg_flt(code.arg2_val);
    
    if code.instr_type == LtacType::LdArgF32 {
        line.push_str("  cvtsd2ss ");
        line.push_str(&reg);
        line.push_str(", ");
        line.push_str(&reg);
        line.push_str("\n");
        
        line.push_str("  movss ");
    } else {
        line.push_str("  movsd ");
    }
       
    line.push_str("[rbp-");
    line.push_str(&code.arg1_val.to_string());
    line.push_str("], ");
    line.push_str(&reg);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_ldarg_f32] Write failed.");
}

// Builds a return statement
// Yes, we could do this more cleanly, but I want to make it obvious what I'm doing.
pub fn amd64_build_ret(writer : &mut BufWriter<File>) {
    let mut line = String::new();
    line.push_str("\n");
    line.push_str("  leave\n");
    line.push_str("  ret\n");
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_ret] Write failed.");
}


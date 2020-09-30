use std::fs::File;
use std::io::{Write, BufWriter};

use parser::ltac::{LtacInstr, LtacArg};

// Function argument registers
fn aarch64_arg_reg32(pos : i32) -> String {
    match pos {
        1 => "w0".to_string(),
        2 => "w1".to_string(),
        3 => "w2".to_string(),
        4 => "w3".to_string(),
        5 => "w4".to_string(),
        6 => "w5".to_string(),
        7 => "w6".to_string(),
        8 => "w7".to_string(),
        _ => String::new(),
    }
}

fn aarch64_arg_reg64(pos : i32) -> String {
    match pos {
        1 => "x0".to_string(),
        2 => "x1".to_string(),
        3 => "x2".to_string(),
        4 => "x3".to_string(),
        5 => "x4".to_string(),
        6 => "x5".to_string(),
        7 => "x6".to_string(),
        8 => "x7".to_string(),
        _ => String::new(),
    }
}

// Kernel argument registers
fn aarch64_karg_reg32(pos : i32) -> String {
    match pos {
        1 => "w8".to_string(),
        2 => "w0".to_string(),
        3 => "w1".to_string(),
        4 => "w2".to_string(),
        5 => "w3".to_string(),
        6 => "w4".to_string(),
        7 => "w5".to_string(),
        _ => String::new(),
    }
}

fn aarch64_karg_reg64(pos : i32) -> String {
    match pos {
        1 => "x8".to_string(),
        2 => "x0".to_string(),
        3 => "x1".to_string(),
        4 => "x2".to_string(),
        5 => "x3".to_string(),
        6 => "x4".to_string(),
        7 => "x5".to_string(),
        _ => String::new(),
    }
}

// Loads an argument for a function call
pub fn aarch64_build_pusharg(writer : &mut BufWriter<File>, code : &LtacInstr, karg : bool, stack_size : i32) {
    let mut line = String::new();
    
    let mut reg32 = aarch64_arg_reg32(code.arg2_val);
    let mut reg64 = aarch64_arg_reg64(code.arg2_val);
    
    if karg {
        reg32 = aarch64_karg_reg32(code.arg2_val);
        reg64 = aarch64_karg_reg64(code.arg2_val);
    }
    
    match &code.arg1_type {
        LtacArg::Reg => {},
        
        LtacArg::Mem => {
            let pos = stack_size - code.arg1_val;
            line.push_str("  ldr ");
            line.push_str(&reg32);
            line.push_str(", [sp, ");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I32 => {
            line.push_str("  mov ");
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&code.arg1_val.to_string());
            line.push_str("\n");
        },
        
        LtacArg::Ptr => {
            line.push_str("  adrp ");
            line.push_str(&reg64);
            line.push_str(", ");
            line.push_str(&code.arg1_sval);
            
            line.push_str("\n  add ");
            line.push_str(&reg64);
            line.push_str(", ");
            line.push_str(&reg64);
            line.push_str(", :lo12:");
            line.push_str(&code.arg1_sval);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_pusharg] Write failed.");
}

// Call a function
pub fn aarch64_build_call(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  bl ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_func_call] Write failed.");
}

// Build a system call
pub fn aarch64_build_syscall(writer : &mut BufWriter<File>) {
    let line = "  svc 0\n\n".to_string();
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_syscall] Write failed.");
}

use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacArg};

// Gets a register based on position
// Kernel argument registers
fn amd64_karg_reg32(pos : i32) -> String {
    match pos {
        1 => return "eax".to_string(),
        2 => return "edi".to_string(),
        3 => return "esi".to_string(),
        4 => return "edx".to_string(),
        _ => return String::new(),
    };
}

fn amd64_karg_reg64(pos : i32) -> String {
    match pos {
        1 => return "rax".to_string(),
        2 => return "rdi".to_string(),
        3 => return "rsi".to_string(),
        4 => return "rdx".to_string(),
        _ => return String::new(),
    };
}

// Function argument registers
fn amd64_arg_reg32(pos : i32) -> String {
    match pos {
        1 => return "edx".to_string(),
        2 => return "esi".to_string(),
        _ => return String::new(),
    };
}

fn amd64_arg_reg64(pos : i32) -> String {
    match pos {
        1 => return "rdi".to_string(),
        2 => return "rsi".to_string(),
        _ => return String::new(),
    };
}

// Builds a function argument
pub fn amd64_build_pusharg(writer : &mut BufWriter<File>, code : &LtacInstr, is_karg : bool) {
    let mut line = "  mov ".to_string();
    
    // Get the argument registers
    let mut reg32 = amd64_arg_reg32(code.arg2_val);
    let mut reg64 = amd64_arg_reg64(code.arg2_val);
    
    if is_karg {
        reg32 = amd64_karg_reg32(code.arg2_val);
        reg64 = amd64_karg_reg64(code.arg2_val);
    }
    
    // Assemble
    match &code.arg1_type {
        LtacArg::Empty => {},
        LtacArg::Reg => {},
        LtacArg::RetRegI32 => {},
        
        LtacArg::Mem => {
            line.push_str(&reg32);
            line.push_str(", [rbp-");
            line.push_str(&code.arg1_val.to_string());
            line.push_str("]");
        },
        
        LtacArg::I32 => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&code.arg1_val.to_string());
        },
        
        LtacArg::Ptr => {
            line.push_str(&reg64);
            line.push_str(", ");
            line.push_str(&code.arg1_sval);
        },
    }
    
    line.push_str("\n");
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_pusharg Write failed.");
}

// Builds a function call
// Param: name
pub fn amd64_build_call(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  call ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_call] Write failed.");
}

// Builds a system call
pub fn amd64_build_syscall(writer : &mut BufWriter<File>) {
    let mut line = "  syscall".to_string();
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_syscall] Write failed.");
}


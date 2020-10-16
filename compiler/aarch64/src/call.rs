use std::fs::File;
use std::io::{Write, BufWriter};

use parser::ltac::{LtacInstr, LtacArg};
use crate::utils::*;

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
        LtacArg::Reg8(pos) | LtacArg::Reg32(pos) => {
            let reg = aarch64_op_reg32(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&reg);
            line.push_str("\n");
        },
    
        LtacArg::Mem(p) => {
            let pos = stack_size - *p;
            line.push_str("  ldr ");
            line.push_str(&reg32);
            line.push_str(", [sp, ");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::Byte(val) => {
            line.push_str("  mov ");
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
        
        LtacArg::I32(val) => {
            line.push_str("  mov ");
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
        
        LtacArg::Ptr => {
            if code.arg1_sval.len() > 0 {
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
            } else {
                let pos = stack_size - code.arg1_val;
                
                line.push_str("  ldr ");
                line.push_str(&reg64);
                line.push_str(", [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            }
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

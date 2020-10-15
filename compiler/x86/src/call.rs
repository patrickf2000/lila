use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacArg};
use crate::utils::*;

// Builds a function argument
pub fn amd64_build_pusharg(writer : &mut BufWriter<File>, code : &LtacInstr, is_karg : bool) {
    let mut line = "  mov ".to_string();
    
    // Get the argument registers
    let mut reg32 = amd64_arg_reg32(code.arg2_val);
    let mut reg64 = amd64_arg_reg64(code.arg2_val);
    let reg_flt = amd64_arg_flt(code.arg2_val);
    
    if is_karg {
        reg32 = amd64_karg_reg32(code.arg2_val);
        reg64 = amd64_karg_reg64(code.arg2_val);
    }
    
    if code.arg1_type == LtacArg::Reg8 {
        line = "  movzx ".to_string();
    } else if code.arg2_type == LtacArg::I16 {
        line = "  movsx ".to_string();
    } else if code.arg2_type == LtacArg::FltReg || code.arg1_type == LtacArg::F32 {
        line = "  movss ".to_string();
    } else if code.arg2_type == LtacArg::FltReg64 || code.arg2_type == LtacArg::F64 {
        line = "  movsd ".to_string();
    }
    
    // Assemble
    match &code.arg1_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg => {},
        
        LtacArg::Reg8 => {
            let reg = amd64_op_reg8(code.arg1_val);
            
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&reg);
        },
        
        LtacArg::Reg16 => {},
        
        LtacArg::Reg64 => {},
        LtacArg::FltReg => {},
        LtacArg::FltReg64 => {},
        
        LtacArg::RetRegI32 => {},
        LtacArg::RetRegI64 => {},
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => {},
        
        LtacArg::Mem => {
            if code.arg2_type == LtacArg::FltReg || code.arg2_type == LtacArg::FltReg64 {
                line.push_str(&reg_flt);
                line.push_str(", ");
            } else if code.arg2_type == LtacArg::I16 {
                line.push_str(&reg32);
                line.push_str(", WORD PTR ");
            } else {
                line.push_str(&reg32);
                line.push_str(", ");
            }
            
            line.push_str("[rbp-");
            line.push_str(&code.arg1_val.to_string());
            line.push_str("]");
        },
        
        LtacArg::Byte => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&code.arg1_bval.to_string());
        },
        
        LtacArg::I16 => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&code.arg1_wval.to_string());
        },
        
        LtacArg::I32 => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&code.arg1_val.to_string());
        },
        
        LtacArg::F32 => {
            line.push_str(&reg_flt);
            line.push_str(", DWORD PTR ");
            line.push_str(&code.arg1_sval);
            line.push_str("[rip]");
        },
        
        LtacArg::F64 => {
            line.push_str(&reg_flt);
            line.push_str(", QWORD PTR ");
            line.push_str(&code.arg1_sval);
            line.push_str("[rip]");
        },
        
        LtacArg::Ptr => {
            line.push_str(&reg64);
            
            if code.arg1_sval.len() > 0 {
                line.push_str(", OFFSET FLAT:");
                line.push_str(&code.arg1_sval);
            } else {
                line.push_str(", [rbp-");
                line.push_str(&code.arg1_val.to_string());
                line.push_str("]");
            }
        },
    }
    
    line.push_str("\n");
    
    // If we have a 32-bit float variable, we have to convert it to a double
    if code.arg2_type == LtacArg::FltReg || code.arg1_type == LtacArg::F32 {
        line.push_str("  cvtss2sd ");
        line.push_str(&reg_flt);
        line.push_str(", ");
        line.push_str(&reg_flt);
        line.push_str("\n");
    }
    
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
    writer.write(b"  syscall\n\n")
        .expect("[AMD64_build_syscall] Write failed.");
}


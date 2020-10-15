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
    let mut reg_flt = amd64_arg_flt(code.arg2_val);
    
    if is_karg {
        reg32 = amd64_karg_reg32(code.arg2_val);
        reg64 = amd64_karg_reg64(code.arg2_val);
    }
    
    // Determine move type
    match code.arg1_type {
        LtacArg::Reg8(_p) => line = "  movzx ".to_string(),
        LtacArg::F32 => line = "  movss ".to_string(),
        LtacArg::F64 => line = "  movsd ".to_string(),
        _ => {},
    }
    
    match code.arg2_type {
        LtacArg::I16 => line = "  movsx ".to_string(),
        LtacArg::FltReg(pos) => {
            line = "  movss ".to_string();
            reg_flt = amd64_arg_flt(pos);
        },
        
        LtacArg::FltReg64(pos) => {
            line = "  movsd ".to_string();
            reg_flt = amd64_arg_flt(pos);
        },
        
        _ => {},
    }
    
    // Assemble
    match &code.arg1_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&reg);
        },
        
        LtacArg::Reg16(_p) => {},
        LtacArg::Reg32(_p) => {},
        LtacArg::Reg64(_p) => {},
        LtacArg::FltReg(_p) => {},
        LtacArg::FltReg64(_p) => {},
        
        LtacArg::RetRegI32 => {},
        LtacArg::RetRegI64 => {},
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => {},
        
        LtacArg::Mem => {
            match code.arg2_type {
                LtacArg::FltReg(_p) | LtacArg::FltReg64(_p) => {
                    line.push_str(&reg_flt);
                    line.push_str(", ");
                },
                
                LtacArg::I16 => {
                    line.push_str(&reg32);
                    line.push_str(", WORD PTR ");
                },
            
                _ => {
                    line.push_str(&reg32);
                    line.push_str(", ");
                },
            }
            
            line.push_str("[rbp-");
            line.push_str(&code.arg1_val.to_string());
            line.push_str("]");
        },
        
        // TODO: We need to revist this
        LtacArg::Byte(val) => {
            let v = *val as u8;
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&v.to_string());
        },
        
        LtacArg::I16 => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&code.arg1_wval.to_string());
        },
        
        LtacArg::I32(val) => {
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
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
    //TODO: combine
    match code.arg2_type {
        LtacArg::FltReg(_p) => {
            line.push_str("  cvtss2sd ");
            line.push_str(&reg_flt);
            line.push_str(", ");
            line.push_str(&reg_flt);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    if code.arg1_type == LtacArg::F32 {
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


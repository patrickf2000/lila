
use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::utils::*;

// Builds multiplication for byte values
// On x86 this is also a little strange...
pub fn amd64_build_byte_mul(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    line.push_str("  xor eax, eax\n");
    
    match &code.arg1_type {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  mov al, ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  imul ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  imul [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::Byte(val) => {
            line.push_str("  mov r15b, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  mul r15b\n");
        },
        
        _ => {},
    }
    
    // Move the result back to the proper register
    let reg = amd64_op_reg16(code.arg1_val);
    
    line.push_str("  mov ");
    line.push_str(&reg);
    line.push_str(", ax\n");
    
    // Write
    writer.write(&line.into_bytes())
        .expect("[AMD64_byte_mul] Write failed.");
}

// Builds division for byte values
pub fn amd64_build_byte_div(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut dest = String::new();
    
    line.push_str("  xor eax, eax\n");
    line.push_str("  xor edx, edx\n");
    
    match &code.arg1_type {
        LtacArg::Reg8(pos) => {
            dest = amd64_op_reg8(*pos);
            
            line.push_str("  mov al, ");
            line.push_str(&dest);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  idiv ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  idiv BYTE PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::Byte(val) => {
            line.push_str("  mov r15b, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  idiv r15b\n");
        },
        
        _ => {},
    }
    
    // Move the result back to the proper register
    line.push_str("  mov ");
    line.push_str(&dest);
    line.push_str(", ");
    
    if code.instr_type == LtacType::BMod {
        line.push_str("ah\n");
    } else {
        line.push_str("al\n");
    }
    
    // Write
    writer.write(&line.into_bytes())
        .expect("[AMD64_byte_div] Write failed.");
}

// Builds division for short values
pub fn amd64_build_short_div(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut dest = String::new();
    
    line.push_str("  xor eax, eax\n");
    line.push_str("  xor edx, edx\n");
    
    match &code.arg1_type {
        LtacArg::Reg16(pos) => {
            dest = amd64_op_reg16(*pos);
            
            line.push_str("  mov ax, ");
            line.push_str(&dest);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            
            line.push_str("  idiv ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  idiv WORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I16(val) => {
            line.push_str("  mov r15w, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  idiv r15w\n");
        },
        
        _ => {},
    }
    
    // Move the result back to the proper register
    line.push_str("  mov ");
    line.push_str(&dest);
    line.push_str(", ");
    
    if code.instr_type == LtacType::I16Mod {
        line.push_str("dx\n");
    } else {
        line.push_str("ax\n");
    }
    
    // Write
    writer.write(&line.into_bytes())
        .expect("[AMD64_byte_div] Write failed.");
}

// Builds the integer and modulus instructions
// On x86 these are a little weird...
pub fn amd64_build_div(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut dest_line = String::new();
    
    line.push_str("  xor edx, edx\n");
    
    match &code.arg1_type {
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str("  mov eax, ");
            line.push_str(&reg);
            line.push_str("\n");
            
            dest_line.push_str("  mov ");
            dest_line.push_str(&reg);
            dest_line.push_str(", ");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  mov eax, DWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            dest_line.push_str("  mov DWORD PTR [rbp-");
            dest_line.push_str(&pos.to_string());
            dest_line.push_str("], ");
        },
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str("  idiv ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  idiv DWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I32(val) => {
            line.push_str("  mov r15d, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  idiv r15d\n");
        },
        
        _ => {},
    }
    
    line.push_str(&dest_line);
    
    if code.instr_type == LtacType::I32Mod {
        line.push_str("edx\n");
    } else {
        line.push_str("eax\n");
    }
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_div] Write failed.");
}



use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::utils::*;

// Builds multiplication for byte values
// On x86 this is also a little strange...
pub fn amd64_build_byte_mul(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut instr = "  imul ".to_string();
    
    if code.instr_type == LtacType::U8Mul {
        instr = "  mul ".to_string();
    }
    
    line.push_str("  xor eax, eax\n");
    
    match &code.arg1 {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  mov al, ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str(&instr);
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str(&instr);
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::Byte(val) => {
            line.push_str("  mov r15b, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  imul r15b\n");
        },
        
        LtacArg::UByte(val) => {
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
    let mut instr = "  idiv ".to_string();
    
    if code.instr_type == LtacType::U8Div || code.instr_type == LtacType::U8Mod {
        instr = "  div ".to_string();
    }
    
    line.push_str("  xor eax, eax\n");
    line.push_str("  xor edx, edx\n");
    
    match &code.arg1 {
        LtacArg::Reg8(pos) => {
            dest = amd64_op_reg8(*pos);
            
            line.push_str("  mov al, ");
            line.push_str(&dest);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str(&instr);
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str(&instr);
            line.push_str("BYTE PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::Byte(val) => {
            line.push_str("  mov r15b, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  idiv r15b\n");
        },
        
        LtacArg::UByte(val) => {
            line.push_str("  mov r15b, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  div r15b\n");
        },
        
        _ => {},
    }
    
    // Move the result back to the proper register
    line.push_str("  mov ");
    line.push_str(&dest);
    line.push_str(", ");
    
    if code.instr_type == LtacType::I8Mod || code.instr_type == LtacType::U8Mod {
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
    let mut instr = "  idiv ".to_string();
    
    if code.instr_type == LtacType::U16Div || code.instr_type == LtacType::U16Mod {
        instr = "  div ".to_string();
    }
    
    line.push_str("  xor eax, eax\n");
    line.push_str("  xor edx, edx\n");
    
    match &code.arg1 {
        LtacArg::Reg16(pos) => {
            dest = amd64_op_reg16(*pos);
            
            line.push_str("  mov ax, ");
            line.push_str(&dest);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            
            line.push_str(&instr);
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str(&instr);
            line.push_str("WORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I16(val) => {
            line.push_str("  mov r15w, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  idiv r15w\n");
        },
        
        LtacArg::U16(val) => {
            line.push_str("  mov r15w, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  div r15w\n");
        },
        
        _ => {},
    }
    
    // Move the result back to the proper register
    line.push_str("  mov ");
    line.push_str(&dest);
    line.push_str(", ");
    
    if code.instr_type == LtacType::I16Mod || code.instr_type == LtacType::U16Mod {
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
    let mut instr = "  idiv ".to_string();
    
    if code.instr_type == LtacType::U32Div || code.instr_type == LtacType::U32Mod
        || code.instr_type == LtacType::U64Div || code.instr_type == LtacType::U64Mod {
        instr = "  div ".to_string();
    }
    
    line.push_str("  xor rdx, rdx\n");
    
    match &code.arg1 {
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str("  mov eax, ");
            line.push_str(&reg);
            line.push_str("\n");
            
            dest_line.push_str("  mov ");
            dest_line.push_str(&reg);
            dest_line.push_str(", ");
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            
            line.push_str("  mov rax, ");
            line.push_str(&reg);
            line.push_str("\n");
            
            dest_line.push_str("  mov ");
            dest_line.push_str(&reg);
            dest_line.push_str(", ");
        },
        
        LtacArg::Mem(pos) => {
            let mov_line1 : String;
            let mov_line2 : String;
        
            if code.instr_type == LtacType::I64Div || code.instr_type == LtacType::I64Mod {
                mov_line1 = "  mov rax, QWORD PTR [rbp-".to_string();
                mov_line2 = "  mov QWORD PTR [rbp-".to_string();
            } else {
                mov_line1 = "  mov eax, DWORD PTR [rbp-".to_string();
                mov_line2 = "  mov DWORD PTR [rbp-".to_string();
            }
            
            line.push_str(&mov_line1);
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            dest_line.push_str(&mov_line2);
            dest_line.push_str(&pos.to_string());
            dest_line.push_str("], ");
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str(&instr);
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            
            line.push_str(&instr);
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str(&instr);
            
            if code.instr_type == LtacType::I64Div || code.instr_type == LtacType::I64Mod {
                line.push_str("QWORD PTR [rbp-");
            } else {
                line.push_str("DWORD PTR [rbp-");
            }
            
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I32(val) => {
            line.push_str("  mov r15d, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  idiv r15d\n");
        },
        
        LtacArg::U32(val) => {
            line.push_str("  mov r15d, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  div r15d\n");
        },
        
        LtacArg::I64(val) => {
            line.push_str("  mov r15, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  idiv r15\n");
        },
        
        LtacArg::U64(val) => {
            line.push_str("  mov r15, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  div r15\n");
        },
        
        _ => {},
    }
    
    line.push_str(&dest_line);
    
    match &code.instr_type {
        LtacType::I32Div | LtacType::U32Div => line.push_str("eax\n"),
        LtacType::I64Div | LtacType::U64Div => line.push_str("rax\n"),
        
        LtacType::I32Mod | LtacType::U32Mod => line.push_str("edx\n"),
        LtacType::I64Mod | LtacType::U64Mod => line.push_str("rdx\n"),
        
        _ => {},
    }
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_div] Write failed.");
}


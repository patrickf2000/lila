use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::{Command, Output};

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

// Import local modules
mod call;
mod func;
mod utils;
mod vector;

use call::*;
use func::*;
use utils::*;
use vector::*;

pub fn compile(ltac_file : &LtacFile) -> io::Result<()> {
    let mut name = "/tmp/".to_string();
    name.push_str(&ltac_file.name);
    name.push_str(".asm");
    
    // Write it out
    let file = File::create(&name)?;
    let mut writer = BufWriter::new(file);
    
    //GNU AS specific
    writer.write(b".intel_syntax noprefix\n")
        .expect("[AMD64_setup] Write failed.");
    
    write_data(&mut writer, &ltac_file.data);
    write_code(&mut writer, &ltac_file.code);
    
    Ok(())
}
 
pub fn build_asm(name : &String, use_c : bool) {
    // Create all the names
    let mut asm_name = "/tmp/".to_string();
    asm_name.push_str(name);
    asm_name.push_str(".asm");
    
    let mut obj_name = "/tmp/".to_string();
    obj_name.push_str(name);
    obj_name.push_str(".o");
    
    let output = &mut name.clone();

    // Assemble
    let asm = Command::new("as")
        .args(&[&asm_name, "-o", &obj_name])
        .output()
        .expect("Fatal: Assembly failed.");
        
    if !asm.status.success() {
        io::stdout().write_all(&asm.stdout).unwrap();
        io::stderr().write_all(&asm.stderr).unwrap();
    }
    
    // Link
    let ld : Output;
    
    if use_c {
        let args = [
            "/usr/lib/x86_64-linux-gnu/crti.o",
            "/usr/lib/x86_64-linux-gnu/crtn.o",
            "/usr/lib/x86_64-linux-gnu/crt1.o",
            &obj_name,
            "-dynamic-linker",
            "/lib64/ld-linux-x86-64.so.2",
            "-lc",
            "-o",
            output
        ];
        
        ld = Command::new("ld")
            .args(&args)
            .output()
            .expect("Fatal: Linking failed.");
    } else {
        let args = [
            &obj_name,
            "-o",
            output
        ];
        
        ld = Command::new("ld")
            .args(&args)
            .output()
            .expect("Fatal: Linking failed.");
    }
    
    if !ld.status.success() {
        io::stdout().write_all(&ld.stdout).unwrap();
        io::stderr().write_all(&ld.stderr).unwrap();
    }
}

// Writes the .data section
fn write_data(writer : &mut BufWriter<File>, data : &Vec<LtacData>) {
    let mut line = String::new();
    line.push_str(".data\n");

    for data in data.iter() {
        match &data.data_type {
            LtacDataType::StringL => {
                line.push_str(&data.name);
                line.push_str(": .string \"");
                line.push_str(&data.val);
                line.push_str("\"\n");
            },
            
            LtacDataType::FloatL => {
                line.push_str(&data.name);
                line.push_str(": .long ");
                line.push_str(&data.val);
                line.push_str("\n");
            },
            
            LtacDataType::DoubleL => {
                line.push_str(&data.name);
                line.push_str(": .quad ");
                line.push_str(&data.val);
                line.push_str("\n");
            },
        }
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_data] Write failed in .data");
}

// Writes the .text section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[AMD64_code] Write failed");

    for code in code.iter() {
        match &code.instr_type {
            LtacType::Extern => amd64_build_extern(writer, &code),
            LtacType::Label => amd64_build_label(writer, &code),
            LtacType::Func => amd64_build_func(writer, &code),
            LtacType::LdArgI32 => amd64_build_ldarg(writer, &code),
            LtacType::LdArgF32 => amd64_build_ldarg_float(writer, &code),
            LtacType::LdArgF64 => amd64_build_ldarg_float(writer, &code),
            LtacType::LdArgPtr => amd64_build_ldarg(writer, &code),
            LtacType::Ret => amd64_build_ret(writer),
            
            LtacType::Mov => amd64_build_instr(writer, &code),
            LtacType::MovB => {},
            LtacType::MovF32 => amd64_build_instr(writer, &code),
            LtacType::MovF64 => amd64_build_instr(writer, &code),
            LtacType::MovOffImm => amd64_build_mov_offset(writer, &code),
            LtacType::MovOffMem => amd64_build_mov_offset(writer, &code),
            LtacType::MovI32Vec => amd64_build_vector_instr(writer, &code),
            
            LtacType::PushArg => amd64_build_pusharg(writer, &code, false),
            LtacType::KPushArg => amd64_build_pusharg(writer, &code, true),
            LtacType::Call => amd64_build_call(writer, &code),
            LtacType::Syscall => amd64_build_syscall(writer),
            
            LtacType::I32Cmp => amd64_build_instr(writer, &code),
            LtacType::F32Cmp => amd64_build_instr(writer, &code),
            LtacType::F64Cmp => amd64_build_instr(writer, &code),
            LtacType::StrCmp => amd64_build_strcmp(writer, &code),
            
            LtacType::Br => amd64_build_jump(writer, &code),
            LtacType::Be => amd64_build_jump(writer, &code),
            LtacType::Bne => amd64_build_jump(writer, &code),
            LtacType::Bl => amd64_build_jump(writer, &code),
            LtacType::Ble => amd64_build_jump(writer, &code),
            LtacType::Bfl => amd64_build_jump(writer, &code),
            LtacType::Bfle => amd64_build_jump(writer, &code),
            LtacType::Bg => amd64_build_jump(writer, &code),
            LtacType::Bge => amd64_build_jump(writer, &code),
            LtacType::Bfg => amd64_build_jump(writer, &code),
            LtacType::Bfge => amd64_build_jump(writer, &code),
            
            LtacType::I32Add => amd64_build_instr(writer, &code),
            LtacType::I32Sub => amd64_build_instr(writer, &code),
            LtacType::I32Mul => amd64_build_instr(writer, &code),
            LtacType::I32Div => amd64_build_div(writer, &code),
            LtacType::I32Mod => amd64_build_div(writer, &code),
            
            LtacType::F32Add => amd64_build_instr(writer, &code),
            LtacType::F32Sub => amd64_build_instr(writer, &code),
            LtacType::F32Mul => amd64_build_instr(writer, &code),
            LtacType::F32Div => amd64_build_instr(writer, &code),
            
            LtacType::F64Add => amd64_build_instr(writer, &code),
            LtacType::F64Sub => amd64_build_instr(writer, &code),
            LtacType::F64Mul => amd64_build_instr(writer, &code),
            LtacType::F64Div => amd64_build_instr(writer, &code),
            
            LtacType::I32And => amd64_build_instr(writer, &code),
            LtacType::I32Or => amd64_build_instr(writer, &code),
            LtacType::I32Xor => amd64_build_instr(writer, &code),
            LtacType::I32Lsh => amd64_build_instr(writer, &code),
            LtacType::I32Rsh => amd64_build_instr(writer, &code),
            LtacType::I32VAdd => amd64_build_vector_instr(writer, &code),
            
            // We shouldn't generate any assembly for these
            LtacType::Exit => {},
            LtacType::Malloc => {},
            LtacType::Free => {},
            
            // These are RISC-specific. At some point, we should generate instructions for them
            LtacType::Ld => {},
            LtacType::LdB => {},
            LtacType::Str => {},
            LtacType::StrB => {},
            LtacType::StrPtr => {},
        }
    }
}

// Many instructions have common syntax
fn amd64_build_instr(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    // Specific for float literals
    if code.arg1_type == LtacArg::F32 {
        line.push_str("  movss xmm1, DWORD PTR ");
        line.push_str(&code.arg1_sval);
        line.push_str("[rip]\n");
    }
    if code.arg2_type == LtacArg::F32 {
        line.push_str("  movss xmm0, DWORD PTR ");
        line.push_str(&code.arg2_sval);
        line.push_str("[rip]\n");
    }
    if code.arg1_type == LtacArg::F64 {
        line.push_str("  movsd xmm1, QWORD PTR ");
        line.push_str(&code.arg1_sval);
        line.push_str("[rip]\n");
    }
    if code.arg2_type == LtacArg::F64 {
        line.push_str("  movsd xmm0, QWORD PTR ");
        line.push_str(&code.arg2_sval);
        line.push_str("[rip]\n");
    }
    
    // The instruction
    match &code.instr_type {
        LtacType::Mov => line.push_str("  mov "),
        LtacType::MovF32 => line.push_str("  movss "),
        LtacType::MovF64 => line.push_str("  movsd "),
        
        LtacType::I32Add => line.push_str("  add "),
        LtacType::I32Sub => line.push_str("  sub "),
        LtacType::I32Mul => line.push_str("  imul "),
        
        LtacType::F32Add => line.push_str("  addss "),
        LtacType::F32Sub => line.push_str("  subss "),
        LtacType::F32Mul => line.push_str("  mulss "),
        LtacType::F32Div => line.push_str("  divss "),
        
        LtacType::F64Add => line.push_str("  addsd "),
        LtacType::F64Sub => line.push_str("  subsd "),
        LtacType::F64Mul => line.push_str("  mulsd "),
        LtacType::F64Div => line.push_str("  divsd "),
        
        LtacType::I32And => line.push_str("  and "),
        LtacType::I32Or => line.push_str("  or "),
        LtacType::I32Xor => line.push_str("  xor "),
        LtacType::I32Lsh => line.push_str("  shl "),
        LtacType::I32Rsh => line.push_str("  shr "),
        
        LtacType::I32Cmp => line.push_str("  cmp "),
        LtacType::F32Cmp => line.push_str("  ucomiss "),
        LtacType::F64Cmp => line.push_str("  ucomisd "),
        
        _ => {},
    }

    // The arguments
    match &code.arg1_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg1_val);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg8 => {},
        
        LtacArg::Reg64 => {
            let reg = amd64_op_reg64(code.arg1_val);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::FltReg | LtacArg::FltReg64 => {
            let reg = amd64_op_flt(code.arg1_val);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::RetRegI32 => line.push_str("eax, "),
        LtacArg::RetRegI64 => line.push_str("rax, "),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0, "),
        
        LtacArg::Mem => {
            if code.arg2_type == LtacArg::I32 || code.arg2_type == LtacArg::F32 {
                line.push_str("DWORD PTR ");
            } else if code.arg2_type == LtacArg::F64 || code.arg2_type == LtacArg::Ptr {
                line.push_str("QWORD PTR ");
            }
            
            line.push_str("[rbp-");
            line.push_str(&code.arg1_val.to_string());
            line.push_str("], ");
        },
        
        LtacArg::Byte => {},
        LtacArg::I32 => {},
        
        LtacArg::F32 | LtacArg::F64 => {
            line.push_str("xmm1, ");
        },
        
        LtacArg::Ptr => {},
    }
    
    match &code.arg2_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::Reg8 => {},
        
        LtacArg::Reg64 => {
            let reg = amd64_op_reg64(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::FltReg | LtacArg::FltReg64 => {
            let reg = amd64_op_flt(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI32 => line.push_str("eax"),
        LtacArg::RetRegI64 => line.push_str("rax"),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0"),
        
        LtacArg::Mem => {
            line.push_str("[rbp-");
            line.push_str(&code.arg2_val.to_string());
            line.push_str("]");
        },
        
        LtacArg::Byte => {},
        
        LtacArg::I32 => {
            line.push_str(&code.arg2_val.to_string());
        },
        
        LtacArg::F32 | LtacArg::F64 => {
            line.push_str("xmm0\n");
        },
        
        LtacArg::Ptr => {
            line.push_str("OFFSET FLAT:");
            line.push_str(&code.arg2_sval);
        },
    }
    
    // Write to the file
    line.push_str("\n");
    writer.write(&line.into_bytes())
        .expect("[AMD64_write_instr] Write failed.");
}

// Builds a move-offset instruction
fn amd64_build_mov_offset(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    // Needed if the source is an array index
    if code.arg2_offset > 0 && code.instr_type == LtacType::MovOffImm {
        line.push_str("  mov r15, QWORD PTR [rbp-");
        line.push_str(&code.arg2_val.to_string());
        line.push_str("]\n");
        
        line.push_str("  mov r15d, DWORD PTR [r15+");
        line.push_str(&code.arg2_offset.to_string());
        line.push_str("]\n");
    } else if code.arg2_offset > 0 && code.instr_type == LtacType::MovOffMem {
        // Load the variable
        line.push_str("  mov r15d, DWORD PTR [rbp-");
        line.push_str(&code.arg2_offset.to_string());
        line.push_str("]\n");
        
        // Clear flags
        line.push_str("  cdqe\n");
        
        // Load the effective address
        line.push_str("  lea r14, [0+r15*");
        line.push_str(&code.arg2_offset_size.to_string());
        line.push_str("]\n");
        
        // Load the array
        line.push_str("  mov r15, QWORD PTR [rbp-");
        line.push_str(&code.arg2_val.to_string());
        line.push_str("]\n");
        
        // Add to get the proper offset
        line.push_str("  add r15, r14\n");
        
        // Store
        line.push_str("  mov r15d, DWORD PTR [r15]\n");
    }
    
    // The arguments
    match &code.arg1_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg1_val);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg8 => {},
        
        LtacArg::Reg64 => {
            let reg = amd64_op_reg64(code.arg1_val);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::FltReg => {},
        LtacArg::FltReg64 => {},
        
        LtacArg::RetRegI32 => line.push_str("  mov eax, "),
        LtacArg::RetRegI64 => line.push_str("  mov rax, "),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0, "),
        
        LtacArg::Mem => {
            if code.arg1_offset > 0 && code.instr_type == LtacType::MovOffImm {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&code.arg1_val.to_string());
                line.push_str("]\n");
                
                line.push_str("  add r15, ");
                line.push_str(&code.arg1_offset.to_string());
                line.push_str("\n");
                
                line.push_str("  mov ");
                if code.arg2_type == LtacArg::I32 {
                    line.push_str("DWORD PTR ");
                }
                line.push_str("[r15], ");
            } else if code.arg1_offset > 0 && code.instr_type == LtacType::MovOffMem {
                // Load the variable
                line.push_str("  mov r15d, DWORD PTR [rbp-");
                line.push_str(&code.arg1_offset.to_string());
                line.push_str("]\n");
                
                // Clear flags
                line.push_str("  cdqe\n");
                
                // Load the effective address
                line.push_str("  lea r14, [0+r15*");
                line.push_str(&code.arg1_offset_size.to_string());
                line.push_str("]\n");
                
                // Load the array
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&code.arg1_val.to_string());
                line.push_str("]\n");
                
                // Add to get the proper offset
                line.push_str("  add r15, r14\n");
                
                // Now set up for the final move
                line.push_str("  mov ");
                if code.arg2_type == LtacArg::I32 {
                    line.push_str("DWORD PTR ");
                }
                line.push_str("[r15], ");
            } else {
                if code.arg2_type == LtacArg::I32 {
                    line.push_str("  mov DWORD PTR ");
                } else {
                    line.push_str("  mov ");
                }
                
                line.push_str("[rbp-");
                line.push_str(&code.arg1_val.to_string());
                line.push_str("], ");
            }
        },
        
        LtacArg::Byte => {},
        LtacArg::I32 => {},
        LtacArg::F32 => {},
        LtacArg::F64 => {},
        LtacArg::Ptr => {},
    }
    
    match &code.arg2_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::Reg64 => {
            let reg = amd64_op_reg64(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::Reg8 => {},
        
        LtacArg::FltReg => {},
        LtacArg::FltReg64 => {},
        
        LtacArg::RetRegI32 => line.push_str("eax"),
        LtacArg::RetRegI64 => line.push_str("rax"),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0"),
        
        LtacArg::Mem => {
            if code.instr_type == LtacType::MovOffImm || code.instr_type == LtacType::MovOffMem {
                line.push_str("r15d");
            } else {
                line.push_str("[rbp-");
                line.push_str(&code.arg2_val.to_string());
                line.push_str("]");
            }
        },
        
        LtacArg::Byte => {},
        
        LtacArg::I32 => {
            line.push_str(&code.arg2_val.to_string());
        },
        
        LtacArg::F32 => {},
        
        LtacArg::F64 => {},
        
        LtacArg::Ptr => {},
    }
    
    line.push_str("\n");

    writer.write(&line.into_bytes())
        .expect("[AMD64_writer_instr] Write failed.");
}

// Builds the integer and modulus instructions
// On x86 these are a little weird...
fn amd64_build_div(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut dest_line = String::new();
    
    line.push_str("  xor edx, edx\n");
    
    match &code.arg1_type {
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg1_val);
            
            line.push_str("  mov eax, ");
            line.push_str(&reg);
            line.push_str("\n");
            
            dest_line.push_str("  mov ");
            dest_line.push_str(&reg);
            dest_line.push_str(", ");
        },
        
        LtacArg::Mem => {
            line.push_str("  mov eax, DWORD PTR [rbp-");
            line.push_str(&code.arg1_val.to_string());
            line.push_str("]\n");
            
            dest_line.push_str("  mov DWORD PTR [rbp-");
            dest_line.push_str(&code.arg1_val.to_string());
            dest_line.push_str("], ");
        },
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg2_val);
            
            line.push_str("  idiv ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem => {
            line.push_str("  idiv DWORD PTR [rbp-");
            line.push_str(&code.arg2_val.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I32 => {
            line.push_str("  mov r15d, ");
            line.push_str(&code.arg2_val.to_string());
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

// Builds a branch (actually kinda called "jumps" in x86...)
fn amd64_build_jump(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  ".to_string();
    
    match &code.instr_type {
        LtacType::Br => line.push_str("jmp "),
        LtacType::Be => line.push_str("je "),
        LtacType::Bne => line.push_str("jne "),
        LtacType::Bl => line.push_str("jl "),
        LtacType::Ble => line.push_str("jle "),
        LtacType::Bfl => line.push_str("jb "),
        LtacType::Bfle => line.push_str("jbe "),
        LtacType::Bg => line.push_str("jg "),
        LtacType::Bge => line.push_str("jge "),
        LtacType::Bfg => line.push_str("ja "),
        LtacType::Bfge => line.push_str("jae "),
        _ => {},
    }
    
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_jump] Write failed.");
}

// Builds a string comparison
fn amd64_build_strcmp(writer : &mut BufWriter<File>, _code : &LtacInstr) {
    let mut line = String::new();
    line.push_str("  call strcmp\n");
    line.push_str("  cmp eax, 0\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_strcmp] Write failed.");
}


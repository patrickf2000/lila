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

use call::*;
use func::*;
use utils::*;

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
            LtacType::Ret => amd64_build_ret(writer),
            LtacType::Mov => amd64_build_instr(writer, &code),
            LtacType::MovOffImm => amd64_build_mov_offset(writer, &code),
            LtacType::PushArg => amd64_build_pusharg(writer, &code, false),
            LtacType::KPushArg => amd64_build_pusharg(writer, &code, true),
            LtacType::Call => amd64_build_call(writer, &code),
            LtacType::Syscall => amd64_build_syscall(writer),
            LtacType::I32Cmp => amd64_build_instr(writer, &code),
            LtacType::Br => amd64_build_jump(writer, &code),
            LtacType::Be => amd64_build_jump(writer, &code),
            LtacType::Bne => amd64_build_jump(writer, &code),
            LtacType::Bl => amd64_build_jump(writer, &code),
            LtacType::Ble => amd64_build_jump(writer, &code),
            LtacType::Bg => amd64_build_jump(writer, &code),
            LtacType::Bge => amd64_build_jump(writer, &code),
            LtacType::I32Add => amd64_build_instr(writer, &code),
            LtacType::I32Mul => amd64_build_instr(writer, &code),
        }
    }
}

// Many instructions have common syntax
fn amd64_build_instr(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    // The instruction
    if code.instr_type == LtacType::Mov {
        line = "  mov".to_string();
    } else if code.instr_type == LtacType::I32Add {
        line = "  add".to_string();
    } else if code.instr_type == LtacType::I32Mul {
        line = "  imul".to_string();
    } else if code.instr_type == LtacType::I32Cmp {
        line = "  cmp".to_string();
    }
    
    line.push_str(" ");

    // The arguments
    match &code.arg1_type {
        LtacArg::Empty => {},
        
        // TODO: We need register indexing
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg1_val);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::RetRegI32 => line.push_str("eax, "),
        LtacArg::RetRegI64 => line.push_str("rax, "),
        
        LtacArg::Mem => {
            if code.arg2_type == LtacArg::I32 {
                line.push_str("DWORD PTR ");
            }
            
            line.push_str("[rbp-");
            line.push_str(&code.arg1_val.to_string());
            line.push_str("], ");
        },
        
        LtacArg::I32 => {},
        LtacArg::Ptr => {},
    }
    
    match &code.arg2_type {
        LtacArg::Empty => {},
        
        // TODO: We need register indexing
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI32 => line.push_str("eax"),
        LtacArg::RetRegI64 => line.push_str("rax"),
        
        LtacArg::Mem => {
            line.push_str("[rbp-");
            line.push_str(&code.arg2_val.to_string());
            line.push_str("]");
        },
        
        LtacArg::I32 => {
            line.push_str(&code.arg2_val.to_string());
        },
        
        LtacArg::Ptr => {},
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
        
        LtacArg::RetRegI32 => line.push_str("  mov eax, "),
        LtacArg::RetRegI64 => line.push_str("  mov rax, "),
        
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
        
        LtacArg::I32 => {},
        LtacArg::Ptr => {},
    }
    
    match &code.arg2_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg => {
            let reg = amd64_op_reg32(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI32 => line.push_str("eax"),
        LtacArg::RetRegI64 => line.push_str("rax"),
        
        LtacArg::Mem => {
            if code.arg2_offset > 0 && code.instr_type == LtacType::MovOffImm {
                line.push_str("r15d");
            } else {
                line.push_str("[rbp-");
                line.push_str(&code.arg2_val.to_string());
                line.push_str("]");
            }
        },
        
        LtacArg::I32 => {
            line.push_str(&code.arg2_val.to_string());
        },
        
        LtacArg::Ptr => {},
    }
    
    line.push_str("\n");

    writer.write(&line.into_bytes())
        .expect("[AMD64_writer_instr] Write failed.");
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
        LtacType::Bg => line.push_str("jg "),
        LtacType::Bge => line.push_str("jge "),
        _ => {},
    }
    
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_jump] Write failed.");
}


use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::Command;

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

pub fn compile(ltac_file : &LtacFile) -> io::Result<()> {
    let mut name = "/tmp/".to_string();
    name.push_str(&ltac_file.name);
    name.push_str(".asm");
    
    // Write it out
    let file = File::create(&name)?;
    let mut writer = BufWriter::new(file);
    
    write_data(&mut writer, &ltac_file.data);
    write_code(&mut writer, &ltac_file.code);
    
    Ok(())
}
 
pub fn build_asm(name : &String) {
    // Create all the names
    let mut asm_name = "/tmp/".to_string();
    asm_name.push_str(name);
    asm_name.push_str(".asm");
    
    let mut obj_name = "/tmp/".to_string();
    obj_name.push_str(name);
    obj_name.push_str(".o");
    
    let output = &mut name.clone();

    // Assemble
    let asm = Command::new("asmx86")
        .args(&[&asm_name, "-o", &obj_name])
        .output()
        .expect("Fatal: Assembly failed.");
        
    if !asm.status.success() {
        io::stdout().write_all(&asm.stdout).unwrap();
        io::stderr().write_all(&asm.stderr).unwrap();
    }
    
    // Link
    let args = [
        &obj_name,
        "-o",
        output
    ];
    
    let ld = Command::new("ld")
        .args(&args)
        .output()
        .expect("Fatal: Linking failed.");
    
    if !ld.status.success() {
        io::stdout().write_all(&ld.stdout).unwrap();
        io::stderr().write_all(&ld.stderr).unwrap();
    }
}

// Writes the .data section
fn write_data(writer : &mut BufWriter<File>, data : &Vec<LtacData>) {
    for data in data.iter() {
        let mut line = String::new();
        
        match &data.data_type {
            LtacDataType::StringL => {
                line.push_str(&data.name);
                line.push_str(" .string \"");
                line.push_str(&data.val);
                line.push_str("\"\n");
            },
        }
        
        writer.write(&line.into_bytes())
            .expect("[AMD64_data] Write failed in .data");
    }
}

// Writes the .text section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>) {
    for code in code.iter() {
        match &code.instr_type {
            LtacType::Extern => amd64_build_extern(writer, &code),
            LtacType::Func => amd64_build_func(writer, &code),
            LtacType::Ret => amd64_build_ret(writer),
            LtacType::Mov => amd64_build_instr(writer, &code),
            LtacType::PushArg => amd64_build_pusharg(writer, &code, false),
            LtacType::KPushArg => amd64_build_pusharg(writer, &code, true),
            LtacType::Call => amd64_build_call(writer, &code),
            LtacType::Syscall => amd64_build_syscall(writer),
            LtacType::I32Add => amd64_build_instr(writer, &code),
            LtacType::I32Mul => amd64_build_instr(writer, &code),
        }
    }
}

// Builds an extern declaration
fn amd64_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str("extern ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_extern] Write failed.");
}

// Builds a function
// Params: name -> function name
//         arg1_val -> stack size
fn amd64_build_func(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();

    line.push_str("\nglobal ");
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

// Builds a return statement
// Yes, we could do this more cleanly, but I want to make it obvious what I'm doing.
fn amd64_build_ret(writer : &mut BufWriter<File>) {
    let mut line = String::new();
    line.push_str("\n");
    line.push_str("  leave\n");
    line.push_str("  ret\n");
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_ret] Write failed.");
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
    }
    
    line.push_str(" ");

    // The arguments
    match &code.arg1_type {
        LtacArg::Empty => {},
        
        // TODO: We need register indexing
        LtacArg::Reg => {
            line.push_str("eax, ");
        },
        
        LtacArg::Mem => {
            if code.arg2_type == LtacArg::I32 {
                line.push_str("dword ");
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
            line.push_str("eax");
        },
        
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
fn amd64_build_pusharg(writer : &mut BufWriter<File>, code : &LtacInstr, is_karg : bool) {
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
fn amd64_build_call(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  call ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_call] Write failed.");
}

// Builds a system call
fn amd64_build_syscall(writer : &mut BufWriter<File>) {
    let mut line = "  syscall".to_string();
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_syscall] Write failed.");
}


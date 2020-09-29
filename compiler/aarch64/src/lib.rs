use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::{Command, Output};

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
            "/usr/lib/aarch64-linux-gnu/crti.o",
            "/usr/lib/aarch64-linux-gnu/crtn.o",
            "/usr/lib/aarch64-linux-gnu/crt1.o",
            &obj_name,
            "-dynamic-linker",
            "/lib/ld-linux-aarch64.so.1",
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
    let mut line = ".data\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[ARCH64_data] Write failed in .data");

    for data in data.iter() {
        line = String::new();
        
        match &data.data_type {
            LtacDataType::StringL => {
                line.push_str(&data.name);
                line.push_str(": .string \"");
                line.push_str(&data.val);
                line.push_str("\"\n");
            },
        }
        
        writer.write(&line.into_bytes())
            .expect("[ARCH64_data] Write failed in .data");
    }
}

// Writes the .text section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>) {
    let line = "\n.text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[ARCH64_text] Write failed in .text");
        
    let mut stack_size = 0;
        
    // TODO: Store function stack size around here, then pass to return
    for code in code.iter() {
        match &code.instr_type {
            LtacType::Extern => aarch64_build_extern(writer, &code),
            LtacType::Label => {},
            LtacType::Func => {
                stack_size = aarch64_build_func(writer, &code);
            },
            LtacType::Ret => aarch64_build_ret(writer, stack_size),
            LtacType::Mov => aarch64_build_mov(writer, &code, stack_size),
            LtacType::PushArg => aarch64_build_pusharg(writer, &code, stack_size),
            LtacType::KPushArg => {},
            LtacType::Call => aarch64_build_call(writer, &code),
            LtacType::Syscall => {},
            LtacType::I32Cmp => {},
            LtacType::Br => {},
            LtacType::Be => {},
            LtacType::Bne => {},
            LtacType::I32Add => {},
            LtacType::I32Mul => {},
        }
    }
}

// Builds an extern declaration
fn aarch64_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(".extern ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[ARCH64_build_extern] Write failed.");
}

// Builds a function declaration
fn aarch64_build_func(writer : &mut BufWriter<File>, code : &LtacInstr) -> i32 {
    let name = &code.name;
    
    let mut stack_size = code.arg1_val;
    while (stack_size - code.arg2_val) < 24 {
        stack_size += 16;
    }
    
    let mut line = "\n.global ".to_string();
    line.push_str(name);
    line.push_str("\n");
    line.push_str(name);
    line.push_str(":\n");
    
    // Set up the stack
    line.push_str("  stp x29, x30, [sp, -");
    line.push_str(&stack_size.to_string());
    line.push_str("]!\n");
    line.push_str("  mov x29, sp\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[ARCH64_build_func] Write failed.");
        
    stack_size
}

// Builds a function return
fn aarch64_build_ret(writer : &mut BufWriter<File>, stack_size : i32) {
    let mut line = "\n  ".to_string();
    line.push_str("ldp x29, x30, [sp], ");
    line.push_str(&stack_size.to_string());
    line.push_str("\n");
    line.push_str("  ret\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_ret] Write failed.");
}

// A common function for data moves
fn aarch64_build_mov(writer : &mut BufWriter<File>, code : &LtacInstr, stack_size : i32) {
    let mut line = "".to_string();
    
    // Check if we're storing to a variable
    if code.arg1_type == LtacArg::Mem {
        match &code.arg2_type {
            LtacArg::Reg => {},
            LtacArg::RetRegI32 => {},
            LtacArg::Mem => {},
            
            LtacArg::I32 => {
                line.push_str("  mov w0, ");
                line.push_str(&code.arg2_val.to_string());
                line.push_str("\n");
            },
            
            LtacArg::Ptr => {},
            _ => {},
        }
        
        let pos = stack_size - code.arg1_val;
        
        line.push_str("  str w0, [sp, ");
        line.push_str(&pos.to_string());
        line.push_str("]\n");
        
    // Check if we are loading a variable
    } else if code.arg2_type == LtacArg::Mem {
        let pos = stack_size - code.arg2_val;
        
        match &code.arg1_type {
            LtacArg::Reg => {
                line.push_str("  ldr w0, [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            },
            
            LtacArg::RetRegI32 => {
                line.push_str("  ldr w0, [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            },
            
            _ => {},
        }
        
    // Otherwise, we're probably moving something to a register
    } else {
        match &code.arg1_type {
            LtacArg::Reg => {
                line.push_str("  mov w0, ");
            },
            
            LtacArg::RetRegI32 => {
                line.push_str("  mov w0, ");
            },
            
            _ => {},
        }
        
        match &code.arg2_type {
            LtacArg::I32 => {
                line.push_str(&code.arg2_val.to_string());
                line.push_str("\n");
            },
            
            LtacArg::Ptr => {},
            
            _ => {},
        }
    }
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_mov] Write failed.");
}

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

// Loads an argument for a function call
fn aarch64_build_pusharg(writer : &mut BufWriter<File>, code : &LtacInstr, stack_size : i32) {
    let mut line = String::new();
    
    let reg32 = aarch64_arg_reg32(code.arg2_val);
    let reg64 = aarch64_arg_reg64(code.arg2_val);
    
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
fn aarch64_build_call(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  bl ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_func_call] Write failed.");
}

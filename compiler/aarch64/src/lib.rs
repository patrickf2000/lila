use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::{Command, Output};

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

// Import and use the local modules
mod func;
mod call;
mod utils;

use func::*;
use call::*;
use utils::*;

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
            LtacType::Label => aarch64_build_label(writer, &code),
            LtacType::Func => {
                stack_size = aarch64_build_func(writer, &code);
            },
            LtacType::LdArgI32 => aarch64_build_ldarg(writer, &code, stack_size),
            LtacType::LdArgPtr => {},
            LtacType::Ret => aarch64_build_ret(writer, stack_size),
            LtacType::Exit => {},
            LtacType::Mov => aarch64_build_mov(writer, &code, stack_size),
            LtacType::MovOffImm => {},
            LtacType::MovOffMem => {},
            LtacType::MovI32Vec => {},
            LtacType::PushArg => aarch64_build_pusharg(writer, &code, false, stack_size),
            LtacType::KPushArg => aarch64_build_pusharg(writer, &code, true, stack_size),
            LtacType::Call => aarch64_build_call(writer, &code),
            LtacType::Syscall => aarch64_build_syscall(writer),
            LtacType::Malloc => {},
            LtacType::Free => {},
            LtacType::I32Cmp => aarch64_build_instr(writer, &code, stack_size),
            LtacType::StrCmp => {},
            LtacType::Br => aarch64_build_branch(writer, &code),
            LtacType::Be => aarch64_build_branch(writer, &code),
            LtacType::Bne => aarch64_build_branch(writer, &code),
            LtacType::Bl => aarch64_build_branch(writer, &code),
            LtacType::Ble => aarch64_build_branch(writer, &code),
            LtacType::Bg => aarch64_build_branch(writer, &code),
            LtacType::Bge => aarch64_build_branch(writer, &code),
            LtacType::I32Add => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32Sub => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32Mul => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32Div => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32Mod => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32And => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32Or => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32Xor => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32Lsh => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32Rsh => aarch64_build_instr(writer, &code, stack_size),
            LtacType::I32VAdd => {},
        }
    }
}

// A common function for data moves
fn aarch64_build_mov(writer : &mut BufWriter<File>, code : &LtacInstr, stack_size : i32) {
    let mut line = "".to_string();
    
    // Check if we're storing to a variable
    if code.arg1_type == LtacArg::Mem {
        let mut reg = "w4".to_string();
    
        match &code.arg2_type {
            LtacArg::Reg => {
                reg = aarch64_op_reg32(code.arg2_val);
            },
            
            LtacArg::RetRegI32 => {},
            LtacArg::Mem => {},
            
            LtacArg::I32 => {
                line.push_str("  mov w4, ");
                line.push_str(&code.arg2_val.to_string());
                line.push_str("\n");
            },
            
            LtacArg::Ptr => {},
            _ => {},
        }
        
        let pos = stack_size - code.arg1_val;
        
        line.push_str("  str ");
        line.push_str(&reg);
        line.push_str(", [sp, ");
        line.push_str(&pos.to_string());
        line.push_str("]\n");
        
    // Check if we are loading a variable
    } else if code.arg2_type == LtacArg::Mem {
        let pos = stack_size - code.arg2_val;
        
        match &code.arg1_type {
            LtacArg::Reg => {
                let reg = aarch64_op_reg32(code.arg1_val);
            
                line.push_str("  ldr ");
                line.push_str(&reg);
                line.push_str(", [sp, ");
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
                let reg = aarch64_op_reg32(code.arg1_val);
                
                line.push_str("  mov ");
                line.push_str(&reg);
                line.push_str(", ");
            },
            
            LtacArg::RetRegI32 => {
                line.push_str("  mov w0, ");
            },
            
            _ => {},
        }
        
        match &code.arg2_type {
            LtacArg::RetRegI32 => {
                line.push_str("w0\n");
            },
        
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

// A common function for several instructions
fn aarch64_build_instr(writer : &mut BufWriter<File>, code : &LtacInstr, stack_size : i32) {
    let mut line = String::new();
    
    match &code.instr_type {
        LtacType::I32Add => line.push_str("  add "),
        LtacType::I32Sub => line.push_str("  sub "),
        LtacType::I32Mul => line.push_str("  mul "),
        LtacType::I32Div => line.push_str("  sdiv "),
        LtacType::I32Mod => {},
        LtacType::I32And => line.push_str("  and "),
        LtacType::I32Or => line.push_str("  orr "),
        LtacType::I32Xor => line.push_str("  eor "),
        LtacType::I32Lsh => line.push_str("  lsl "),
        LtacType::I32Rsh => line.push_str("  lsr "),
        LtacType::I32Cmp => line.push_str("  cmp "),
        
        _ => {},
    }
    
    // Only needed for mod
    let mut dest = "w1".to_string();
    
    match &code.arg1_type {
        LtacArg::Reg => {
            let reg = aarch64_op_reg32(code.arg1_val);
            dest = reg.clone();
        
            if code.instr_type == LtacType::I32Cmp {
                line.push_str(&reg);
                line.push_str(", ");
            } else if code.instr_type == LtacType::I32Mod {
                line.push_str("  mov w3, ");
                line.push_str(&reg);
                line.push_str("\n");
            } else {
                line.push_str(&reg);
                line.push_str(", ");
                line.push_str(&reg);
                line.push_str(", ");
            }
        },
        
        LtacArg::RetRegI32 => {},
        LtacArg::Mem => {},
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg => {
            let reg = aarch64_op_reg32(code.arg2_val);
            
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Reg64 => {},
        
        LtacArg::RetRegI32 => {
            if code.instr_type == LtacType::I32Mod {
                line.push_str("  mov w4, w0\n");
            } else {
                line.push_str("w0\n");
            }
        },
        
        LtacArg::RetRegI64 => {},
        
        LtacArg::Mem => {
            let pos = stack_size - &code.arg2_val;
            
            let mut mov = String::new();
            mov.push_str("  ldr w5, [sp, ");
            mov.push_str(&pos.to_string());
            mov.push_str("]\n");
            line.insert_str(0, &mov);
            
            if code.instr_type == LtacType::I32Mod {
                line.push_str("  mov w4, ");
            }
            
            line.push_str("w5\n");
        },
        
        LtacArg::I32 => {
            let mut mov = String::new();
            mov.push_str("  mov w5, ");
            mov.push_str(&code.arg2_val.to_string());
            mov.push_str("\n");
            line.insert_str(0, &mov);
            
            if code.instr_type == LtacType::I32Mod {
                line.push_str("  mov w4, ");
            }
            
            line.push_str("w5\n");
        },
        
        _ => {},
    }
    
    if code.instr_type == LtacType::I32Mod {
        line.push_str("  sdiv w2, w3, w4\n");
        
        line.push_str("  msub ");
        line.push_str(&dest);
        line.push_str(", w2, w4, w3\n");
    }
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_math] Write failed.");
}

// Generates the flow control instructions
fn aarch64_build_branch(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    match &code.instr_type {
        LtacType::Br => line.push_str("  b "),
        LtacType::Be => line.push_str("  beq "),
        LtacType::Bne => line.push_str("  bne "),
        LtacType::Bl => line.push_str("  blt "),
        LtacType::Ble => line.push_str("  ble "),
        LtacType::Bg => line.push_str("  bgt "),
        LtacType::Bge => line.push_str("  bge "),
        _ => {},
    }
    
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_branch] Write failed.");
}

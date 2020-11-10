use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::{Command, Output};

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

// Import and use the local modules
mod func;
mod call;
mod mov;
mod utils;

use func::*;
use call::*;
use mov::*;
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
            
            LtacDataType::FloatL => {},
            LtacDataType::DoubleL => {},
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
            LtacType::None => {},
            
            LtacType::Extern => aarch64_build_extern(writer, &code),
            LtacType::Label => aarch64_build_label(writer, &code),
            LtacType::Func => {
                stack_size = aarch64_build_func(writer, &code);
            },
            
            LtacType::LdArgI8 | LtacType::LdArgU8 => {},
            LtacType::LdArgI16 | LtacType::LdArgU16 => {},
            LtacType::LdArgI32 | LtacType::LdArgU32 |
            LtacType::LdArgI64 | LtacType::LdArgU64 => aarch64_build_ldarg(writer, &code, stack_size),
            LtacType::LdArgF32 => {},
            LtacType::LdArgF64 => {},
            LtacType::LdArgPtr => aarch64_build_ldarg(writer, &code, stack_size),
            LtacType::Ret => aarch64_build_ret(writer, stack_size),
            LtacType::Exit => {},
            
            LtacType::Ld => aarch64_build_ld_str(writer, &code, stack_size),
            LtacType::LdB => aarch64_build_ld_str(writer, &code, stack_size),
            LtacType::LdUB => {},
            LtacType::LdW => {},
            LtacType::Str => aarch64_build_ld_str(writer, &code, stack_size),
            LtacType::StrB => aarch64_build_ld_str(writer, &code, stack_size),
            LtacType::StrUB => {},
            LtacType::StrW => {},
            LtacType::StrPtr => aarch64_build_strptr(writer, &code, stack_size),
            LtacType::Mov | LtacType::MovU => aarch64_build_mov(writer, &code),
            LtacType::MovB => aarch64_build_mov(writer, &code),
            LtacType::MovUB => {},
            LtacType::MovW => {},
            LtacType::MovUW => {},
            LtacType::MovQ | LtacType::MovUQ => {},
            LtacType::MovF32 => {},
            LtacType::MovF64 => {},
            LtacType::MovI32Vec => {},
            
            LtacType::Push | LtacType::Pop => {},
            
            LtacType::PushArg => aarch64_build_pusharg(writer, &code, false, stack_size),
            LtacType::KPushArg => aarch64_build_pusharg(writer, &code, true, stack_size),
            LtacType::Call => aarch64_build_call(writer, &code),
            LtacType::Syscall => aarch64_build_syscall(writer),
            LtacType::Malloc => {},
            LtacType::Free => {},
            
            LtacType::I8Cmp | LtacType::U8Cmp => {},
            LtacType::I16Cmp | LtacType::U16Cmp => {},
            LtacType::I32Cmp | LtacType::U32Cmp => aarch64_build_instr(writer, &code),
            LtacType::I64Cmp | LtacType::U64Cmp => {},
            LtacType::F32Cmp => {},
            LtacType::F64Cmp => {},
            LtacType::StrCmp => aarch64_build_strcmp(writer, &code),
            
            LtacType::Br => aarch64_build_branch(writer, &code),
            LtacType::Be => aarch64_build_branch(writer, &code),
            LtacType::Bne => aarch64_build_branch(writer, &code),
            LtacType::Bl => aarch64_build_branch(writer, &code),
            LtacType::Ble => aarch64_build_branch(writer, &code),
            LtacType::Bfl => {},
            LtacType::Bfle => {},
            LtacType::Bg => aarch64_build_branch(writer, &code),
            LtacType::Bge => aarch64_build_branch(writer, &code),
            LtacType::Bfg => {},
            LtacType::Bfge => {},
            
            LtacType::I8Add | LtacType::U8Add |
            LtacType::I16Add | LtacType::U16Add |
            LtacType::I32Add | LtacType::U32Add => aarch64_build_instr(writer, &code),
            
            LtacType::I8Sub | LtacType::I16Sub |
            LtacType::I32Sub => aarch64_build_instr(writer, &code),
            
            LtacType::I8Mul | LtacType::U8Mul |
            LtacType::I16Mul | LtacType::U16Mul |
            LtacType::I32Mul | LtacType::U32Mul => aarch64_build_instr(writer, &code),
            
            LtacType::I8Div | LtacType::U8Div |
            LtacType::I16Div | LtacType::U16Div |
            LtacType::I32Div | LtacType::U32Div => aarch64_build_instr(writer, &code),
            
            LtacType::I8Mod | LtacType::U8Mod |
            LtacType::I16Mod | LtacType::U16Mod |
            LtacType::I32Mod | LtacType::U32Mod => aarch64_build_instr(writer, &code),
            
            LtacType::I64Add | LtacType::U64Add => {},
            LtacType::I64Sub => {},
            LtacType::I64Mul | LtacType::U64Mul => {},
            LtacType::I64Div | LtacType::U64Div => {},
            LtacType::I64Mod | LtacType::U64Mod => {},
            
            LtacType::F32Add => {},
            LtacType::F32Sub => {},
            LtacType::F32Mul => {},
            LtacType::F32Div => {},
            
            LtacType::F64Add => {},
            LtacType::F64Sub => {},
            LtacType::F64Mul => {},
            LtacType::F64Div => {},
            
            LtacType::BAnd | LtacType::WAnd |
            LtacType::I32And => aarch64_build_instr(writer, &code),
            LtacType::BOr | LtacType::WOr |
            LtacType::I32Or => aarch64_build_instr(writer, &code),
            LtacType::BXor | LtacType::WXor |
            LtacType::I32Xor => aarch64_build_instr(writer, &code),
            LtacType::BLsh | LtacType::WLsh |
            LtacType::I32Lsh => aarch64_build_instr(writer, &code),
            LtacType::BRsh | LtacType::WRsh |
            LtacType::I32Rsh => aarch64_build_instr(writer, &code),
            
            LtacType::I64And => {},
            LtacType::I64Or => {},
            LtacType::I64Xor => {},
            LtacType::I64Lsh => {},
            LtacType::I64Rsh => {},
            
            LtacType::I32VAdd => {},
        }
    }
}

// A common function for several instructions
fn aarch64_build_instr(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut dest_line = String::new();
    
    let mut dest = "w0".to_string();
    let mut src = "w0".to_string();
    
    match &code.instr_type {
        LtacType::I8Add | LtacType::I32Add => line.push_str("  add "),
        LtacType::I8Sub | LtacType::I32Sub => line.push_str("  sub "),
        LtacType::I8Mul | LtacType::I32Mul => line.push_str("  mul "),
        LtacType::I8Div | LtacType::I32Div => line.push_str("  sdiv "),
        LtacType::I8Mod | LtacType::I32Mod => line.push_str("  udiv "),
        
        LtacType::BAnd | LtacType::I32And => line.push_str("  and "),
        LtacType::BOr | LtacType::I32Or => line.push_str("  orr "),
        LtacType::BXor | LtacType::I32Xor => line.push_str("  eor "),
        LtacType::BLsh | LtacType::I32Lsh => line.push_str("  lsl "),
        LtacType::BRsh | LtacType::I32Rsh => line.push_str("  lsr "),
        
        LtacType::I32Cmp => line.push_str("  cmp "),
        
        _ => {},
    }
    
    match &code.arg1 {
        LtacArg::Reg8(pos) | LtacArg::Reg32(pos) => {
            let reg = aarch64_op_reg32(*pos);
            dest = reg.clone();
        
            dest_line.push_str(&reg);
            dest_line.push_str(", ");
        },
        
        LtacArg::RetRegI32 => dest_line.push_str("w0, "),
        LtacArg::RetRegI64 => dest_line.push_str("x0, "),
        
        _ => {},
    }
    
    if code.instr_type == LtacType::I8Mod || code.instr_type == LtacType::I32Mod {
        line.push_str("w4, ");
    } else if code.instr_type != LtacType::I32Cmp {
        line.push_str(&dest_line);
    }
    
    line.push_str(&dest_line);
    
    match &code.arg2 {
        LtacArg::Reg8(pos) | LtacArg::Reg32(pos) => {
            let reg = aarch64_op_reg32(*pos);
            src = reg.clone();
            
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI32 => line.push_str("w0"),
        LtacArg::RetRegI64 => line.push_str("x0"),
        
        _ => {},
    }
    
    line.push_str("\n");
    
    // For modulo
    if code.instr_type == LtacType::I8Mod || code.instr_type == LtacType::I32Mod {
        line.push_str("  msub ");
        line.push_str(&dest);
        line.push_str(", w4, ");
        line.push_str(&src);
        line.push_str(", ");
        line.push_str(&dest);
        line.push_str("\n");
    }
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_instr] Write failed.");
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

// Generates the string comparison instructions
fn aarch64_build_strcmp(writer : &mut BufWriter<File>, _code : &LtacInstr) {
    let mut line = String::new();
    
    line.push_str("  bl strcmp\n");
    line.push_str("  cmp w0, 0\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_strcmp] Write failed.");
}


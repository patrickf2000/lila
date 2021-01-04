
// This file is part of the Lila compiler
// Copyright (C) 2020-2021 Patrick Flynn
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// Note: Some of this is copied directly from the original code gen layer

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::Command;

use parser::ltac::{LtacFile, LtacData, /*LtacDataType, LtacType,*/ LtacInstr, /*LtacArg*/};

pub fn compile(ltac_file : &LtacFile, pic : bool) -> io::Result<()> {
    let mut name = "/tmp/".to_string();
    name.push_str(&ltac_file.name);
    name.push_str(".asm");
    
    // Write it out
    let file = File::create(&name)?;
    let mut writer = BufWriter::new(file);
    
    //GNU AS specific
    writer.write(b".intel_syntax noprefix\n")
        .expect("[AMD64_setup] Write failed.");
    
    write_data(&mut writer, &ltac_file.data, pic);
    write_code(&mut writer, &ltac_file.code, pic);
    
    Ok(())
}

// Assemble a file
pub fn build_asm(name : &String, no_link : bool) {
    // Create all the names
    let mut asm_name = "/tmp/".to_string();
    asm_name.push_str(name);
    asm_name.push_str(".asm");
    
    let mut obj_name = "/tmp/".to_string();
    if no_link {
        obj_name = "./".to_string();
    }
    
    obj_name.push_str(name);
    obj_name.push_str(".o");

    // Assemble
    let asm = Command::new("as")
        .args(&[&asm_name, "-o", &obj_name])
        .output()
        .expect("Fatal: Assembly failed.");
        
    if !asm.status.success() {
        io::stdout().write_all(&asm.stdout).unwrap();
        io::stderr().write_all(&asm.stderr).unwrap();
    }
}
 
// Link everything
pub fn link(all_names : &Vec<String>, output : &String, use_corelib : bool, is_lib : bool, inc_start : bool) {
    let mut names : Vec<String> = Vec::new();
    let mut libs : Vec<String> = Vec::new();
    
    for name in all_names.iter() {
        if name.ends_with(".o") {
            names.push(name.clone());
        } else if name.starts_with("-l") {
            libs.push(name.clone());
        } else {
            let mut obj_name = "/tmp/".to_string();
            obj_name.push_str(&name);
            obj_name.push_str(".o");
            names.push(obj_name);
        }
    }
    
    // Link
    //let ld : Output;
    let mut args : Vec<&str> = Vec::new();
    args.push("-L./");
    
    if !is_lib && inc_start {
        args.push("/usr/lib/lila/lrt.o");
    }
    
    args.push("-dynamic-linker");
    args.push("/lib64/ld-linux-x86-64.so.2");
        
    for name in names.iter() {
        args.push(&name);
    }
        
    if is_lib {
        args.push("-shared");
    }
    
    for lib in libs.iter() {
        args.push(lib);
    }
        
    args.push("-o");
    args.push(output);
    
    if use_corelib {
        args.push("-llila");
        
        if use_corelib {
            args.push("-llila_core");
        }
    }
    
    let ld = Command::new("ld")
        .args(args.as_slice())
        .output()
        .expect("Fatal: Linking failed.");
    
    if !ld.status.success() {
        io::stdout().write_all(&ld.stdout).unwrap();
        io::stderr().write_all(&ld.stderr).unwrap();
    }
}

// Writes the .data section
fn write_data(_writer : &mut BufWriter<File>, _data : &Vec<LtacData>, _pic : bool) {
    /*let mut line = String::new();
    
    if !pic {
        line.push_str(".data\n");
    }

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
        .expect("[AMD64_data] Write failed in .data");*/
}

// Writes the .text section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>, _is_pic : bool) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[AMD64_code] Write failed");

    for _code in code.iter() {
        /*match &code.instr_type {
            LtacType::Extern => amd64_build_extern(writer, &code),
            LtacType::Label => amd64_build_label(writer, &code),
            LtacType::Func => amd64_build_func(writer, &code, is_pic),
            LtacType::Ret => amd64_build_ret(writer),
            
            LtacType::LdArgI8 | LtacType::LdArgU8 => amd64_build_ldarg(writer, &code, is_pic),
            LtacType::LdArgI16 | LtacType::LdArgU16 => amd64_build_ldarg(writer, &code, is_pic),
            LtacType::LdArgI32 | LtacType::LdArgU32 => amd64_build_ldarg(writer, &code, is_pic),
            LtacType::LdArgI64 | LtacType::LdArgU64 => amd64_build_ldarg(writer, &code, is_pic),
            LtacType::LdArgF32 => amd64_build_ldarg_float(writer, &code),
            LtacType::LdArgF64 => amd64_build_ldarg_float(writer, &code),
            LtacType::LdArgPtr => amd64_build_ldarg(writer, &code, is_pic),
            
            LtacType::MovI32Vec => amd64_build_vector_instr(writer, &code),
            
            LtacType::Push | LtacType::Pop => amd64_build_stackop(writer, &code),
            
            LtacType::PushArg => amd64_build_pusharg(writer, &code, false, is_pic),
            LtacType::KPushArg => amd64_build_pusharg(writer, &code, true, is_pic),
            LtacType::Call => amd64_build_call(writer, &code),
            LtacType::Syscall => amd64_build_syscall(writer),
            
            LtacType::StrCmp => amd64_build_strcmp(writer, use_c),
            
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
            
            LtacType::I8Mul | LtacType::U8Mul => amd64_build_byte_mul(writer, &code),
            LtacType::I8Div | LtacType::I8Mod |
            LtacType::U8Div | LtacType::U8Mod => amd64_build_byte_div(writer, &code),
            
            LtacType::I16Div | LtacType::I16Mod |
            LtacType::U16Div | LtacType::U16Mod => amd64_build_short_div(writer, &code),
            
            LtacType::I32Div | LtacType::U32Div => amd64_build_div(writer, &code),
            LtacType::I32Mod | LtacType::U32Mod => amd64_build_div(writer, &code),
            
            LtacType::I64Div | LtacType::U64Div => amd64_build_div(writer, &code),
            LtacType::I64Mod | LtacType::U64Mod => amd64_build_div(writer, &code),
            
            LtacType::I32VAdd => amd64_build_vector_instr(writer, &code),
            
            // We shouldn't generate any assembly for these
            LtacType::Exit => {},
            LtacType::Malloc => {},
            LtacType::Free => {},
            
            // These are RISC-specific. At some point, we should generate instructions for them
            LtacType::LdB | LtacType::LdUB |
            LtacType::LdW | LtacType::LdUW |
            LtacType::Ld | LtacType::LdU |
            LtacType::LdQ | LtacType::LdUQ |
            LtacType::LdF32 | LtacType::LdF64 => amd64_build_load_store(writer, &code, true),
            
            LtacType::StrB | LtacType::StrUB |
            LtacType::StrW | LtacType::StrUW |
            LtacType::Str | LtacType::StrU |
            LtacType::StrQ | LtacType::StrUQ |
            LtacType::StrF32 | LtacType::StrF64 => amd64_build_load_store(writer, &code, false),
            LtacType::StrPtr => {},
            
            // Everything else uses the common build instruction function
            _ => amd64_build_instr(writer, &code, is_pic, is_risc),
        }*/
    }
}


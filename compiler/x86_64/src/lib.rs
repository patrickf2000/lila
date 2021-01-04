
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

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr};

// Import and use local modules
mod asm;
mod func;

use asm::*;
use func::*;

// The entry point
pub fn compile(ltac_file : &LtacFile, pic : bool) -> io::Result<()> {
    // First, translate
    let mut x86_code : Vec<X86Instr> = Vec::new();
    translate_code(&mut x86_code, &ltac_file.code, pic);
    
    // Write it out
    let mut name = "/tmp/".to_string();
    name.push_str(&ltac_file.name);
    name.push_str(".asm");
    
    let file = File::create(&name)?;
    let mut writer = BufWriter::new(file);
    
    //GNU AS specific
    writer.write(b".intel_syntax noprefix\n")
        .expect("[AMD64_setup] Write failed.");
    
    write_data(&mut writer, &ltac_file.data, pic);
    write_code(&mut writer, &x86_code);
    
    Ok(())
}

// Writes the .data section
fn write_data(writer : &mut BufWriter<File>, data : &Vec<LtacData>, pic : bool) {
    let mut line = String::new();
    
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
        .expect("[AMD64_data] Write failed in .data");
}

// Translates the LTAC code section to x86 code
fn translate_code(x86_code : &mut Vec<X86Instr>, code : &Vec<LtacInstr>, is_pic : bool) {
    for code in code.iter() {
        match &code.instr_type {
            LtacType::Extern => amd64_build_extern(x86_code, &code),
            LtacType::Label => amd64_build_label(x86_code, &code),
            LtacType::Func => amd64_build_func(x86_code, &code, is_pic),
            
            _ => {},
        }
    }
}

// Writes the .text section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<X86Instr>) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[AMD64_code] Write failed");

    for code in code.iter() {
        match &code.instr_type {
            X86Type::Extern | X86Type::Global
            | X86Type::Type | X86Type::Label
            | X86Type::Call => amd64_write_named(writer, &code),
            
            X86Type::Push => amd64_write_instr(writer, &code, 1),
            
            X86Type::Mov
            | X86Type::Sub => amd64_write_instr(writer, &code, 2),
            
            _ => {},
        }
    
        /*match &code.instr_type {
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

// Writes a named directive
// These would be externs, globals, labels, and calls
fn amd64_write_named(writer : &mut BufWriter<File>, code : &X86Instr) {
    let mut line = String::new();
    
    match code.instr_type {
        X86Type::Extern => line.push_str(".extern "),
        X86Type::Global => line.push_str("\n.global "),
        X86Type::Type => line.push_str(".type "),
        X86Type::Call => line.push_str("  call "),
        _ => {},
    }
    
    line.push_str(&code.name);
    
    if code.instr_type == X86Type::Label {
        line.push_str(":");
    } else if code.instr_type == X86Type::Type {
        line.push_str(", @function");
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_extern] Write failed.");
}

// Writes an x86-instruction
fn amd64_write_instr(writer : &mut BufWriter<File>, code : &X86Instr, op_count : i32) {
    let mut line = "  ".to_string();
    
    match code.instr_type {
        X86Type::Push => line.push_str("push"),
        X86Type::Mov => line.push_str("mov"),
        
        X86Type::Add => line.push_str("add"),
        X86Type::Sub => line.push_str("sub"),
        _ => {},
    }
    
    if op_count == 1 {
        line.push_str(" ");
        let op = amd64_write_operand(&code.arg1);
        line.push_str(&op);
    } else if op_count == 2 {
        let op1 = amd64_write_operand(&code.arg1);
        let op2 = amd64_write_operand(&code.arg2);
        
        line.push_str(" ");
        line.push_str(&op1);
        line.push_str(", ");
        line.push_str(&op2);
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_write_instr] Write failed.");
}

// Writes an x86 operand
fn amd64_write_operand(arg : &X86Arg) -> String {
    let mut line = String::new();
    
    match &arg {
        X86Arg::Reg64(reg) => {
             let reg_str = reg2str(&reg, 64);
             line.push_str(&reg_str);
        },
        
        X86Arg::Imm32(val) => line.push_str(&val.to_string()),
        
        _ => {},
    }
    
    line
}


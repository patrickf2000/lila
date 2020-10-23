
/* Backend Example

This is an example fo what a Dash backend could look like. For consistency, I would like all backends to
have as similar a structure to this as possible.

By default, this is built with the rest of the program. The reason is so I'm sure it will always be up to
date (at the time of writing, the language itself is still being developed, so its quite possible we will
be adding new LTAC instructions).

*/

// TODO: ALWAYS remove this in production code.
#![allow(dead_code)]

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr};

pub fn compile(ltac_file : &LtacFile) -> io::Result<()> {
    let mut name = "./".to_string();
    name.push_str(&ltac_file.name);
    name.push_str(".asm");
    
    // Write it out
    let file = File::create(&name)?;
    let mut writer = BufWriter::new(file);
    
    write_data(&mut writer, &ltac_file.data);
    write_code(&mut writer, &ltac_file.code);
    
    Ok(())
}

// Write the data section
fn write_data(writer : &mut BufWriter<File>, data : &Vec<LtacData>) {
    let mut line = String::new();
    line.push_str(".data\n");

    for data in data.iter() {
        match &data.data_type {
            LtacDataType::StringL => {},
            LtacDataType::FloatL => {},
            LtacDataType::DoubleL => {},
        }
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[_data] Write failed in .data");
}

// Write the code section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[_code] Write failed");

    for code in code.iter() {
        match &code.instr_type {
        
            // Basic function instructions
            LtacType::Extern => {},
            LtacType::Label => {},
            LtacType::Func => {},
            LtacType::Ret => {},
            
            // Used to load function arguments
            LtacType::LdArgI8 => {},
            LtacType::LdArgI32 => {},
            LtacType::LdArgF32 => {},
            LtacType::LdArgF64 => {},
            LtacType::LdArgPtr => {},
            
            // All the move instructions
            LtacType::Mov => {},
            LtacType::MovB => {},
            LtacType::MovUB => {},
            LtacType::MovW => {},
            LtacType::MovUW => {},
            LtacType::MovF32 => {},
            LtacType::MovF64 => {},
            LtacType::MovOffImm => {},
            LtacType::MovOffMem => {},
            LtacType::MovI32Vec => {},
            
            // Argument and function call instructions
            LtacType::PushArg => {},
            LtacType::KPushArg => {},
            LtacType::Call => {},
            LtacType::Syscall => {},
            
            // Comparison instructons
            LtacType::I8Cmp => {},
            LtacType::I32Cmp => {},
            LtacType::F32Cmp => {},
            LtacType::F64Cmp => {},
            LtacType::StrCmp => {},
            
            // Branching instructions
            LtacType::Br => {},
            LtacType::Be => {},
            LtacType::Bne => {},
            LtacType::Bl => {},
            LtacType::Ble => {},
            LtacType::Bfl => {},
            LtacType::Bfle => {},
            LtacType::Bg => {},
            LtacType::Bge => {},
            LtacType::Bfg => {},
            LtacType::Bfge => {},
            
            // Signed byte math operations
            LtacType::BAdd => {},
            LtacType::BSub => {},
            LtacType::BMul => {},
            LtacType::BDiv => {},
            LtacType::BMod => {},
            
            // Signed word (2-byte) math operations
            LtacType::I16Add => {},
            LtacType::I16Sub => {},
            LtacType::I16Mul => {},
            LtacType::I16Div => {},
            LtacType::I16Mod => {},
            
            // Unsigned word (2-byte) math operations
            LtacType::U16Add => {},
            LtacType::U16Mul => {},
            LtacType::U16Div => {},
            LtacType::U16Mod => {},
            
            // Signed byte bitwise operations
            LtacType::BAnd => {},
            LtacType::BOr => {},
            LtacType::BXor => {},
            LtacType::BLsh => {},
            LtacType::BRsh => {},
            
            // Signed 32-bit integer math opreations
            LtacType::I32Add => {},
            LtacType::I32Sub => {},
            LtacType::I32Mul => {},
            LtacType::I32Div => {},
            LtacType::I32Mod => {},
            
            // Signed 32-bit integer bitwise operations
            LtacType::I32And => {},
            LtacType::I32Or => {},
            LtacType::I32Xor => {},
            LtacType::I32Lsh => {},
            LtacType::I32Rsh => {},
            
            // Signed 32-bit vector math operations
            LtacType::I32VAdd => {},
            
            // Single-precision float operations
            LtacType::F32Add => {},
            LtacType::F32Sub => {},
            LtacType::F32Mul => {},
            LtacType::F32Div => {},
            
            // Double-precision float operations
            LtacType::F64Add => {},
            LtacType::F64Sub => {},
            LtacType::F64Mul => {},
            LtacType::F64Div => {},
            
            // These are intrinsics if you will; they should never get down to a code generation layer
            LtacType::Exit => {},
            LtacType::Malloc => {},
            LtacType::Free => {},
            
            // These are specific to RISC machines
            // RISC Load instructions
            LtacType::Ld => {},
            LtacType::LdB => {},
            LtacType::LdUB => {},
            LtacType::LdW => {},
            
            // RISC store instructions
            LtacType::Str => {},
            LtacType::StrB => {},
            LtacType::StrUB => {},
            LtacType::StrW => {},
            LtacType::StrPtr => {},
        }
    }
}

// This is an example of what a code generation function could look like
fn example_build_label(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(&code.name);
    line.push_str(":\n");
    
    writer.write(&line.into_bytes())
        .expect("[_build_label] Write failed.");
}


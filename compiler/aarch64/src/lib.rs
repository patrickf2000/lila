//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr/*, LtacArg*/};

mod asm;

use asm::*;

pub fn compile(ltac_file : &LtacFile) -> io::Result<()> {
    // Translate the code
    let mut code : Vec<Arm64Instr> = Vec::new();
    translate_code(&mut code, &ltac_file.code);

    // Create the file
    let mut name = "/tmp/".to_string();
    name.push_str(&ltac_file.name);
    name.push_str(".asm");
    
    // Write it out
    let file = File::create(&name)?;
    let mut writer = BufWriter::new(file);
    
    write_data(&mut writer, &ltac_file.data);
    write_code(&mut writer, &mut code);
    
    Ok(())
}

// Write the data section
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
        .expect("[AARCH64_data] Write failed in .data");
}

// Translate the code section
fn translate_code(code : &mut Vec<Arm64Instr>, input : &Vec<LtacInstr>) {
    for ln in input {
        match ln.instr_type {
            LtacType::Extern => {
                let mut instr = create_arm64_instr(Arm64Type::Extern);
                instr.name = ln.name.clone();
                code.push(instr);
            },
            
            LtacType::Label => {
                let mut instr = create_arm64_instr(Arm64Type::Label);
                instr.name = ln.name.clone();
                code.push(instr);
            },
            
            LtacType::Func => {
                arm64_build_func(code, &ln);
            },
            
            _ => {},
        }
    }
}

// Write the code section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<Arm64Instr>) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[AArch64_code] Write failed");

    //let mut stack_size = 0;
    for ln in code {
        match ln.instr_type {
            Arm64Type::Extern => {
                let mut line = ".extern ".to_string();
                line.push_str(&ln.name);
                line.push_str("\n");
                
                writer.write(&line.into_bytes())
                    .expect("[AArch64_extern] Write failed.");
            },
            
            Arm64Type::Global => {
                let mut line = ".global ".to_string();
                line.push_str(&ln.name);
                line.push_str("\n");
                
                writer.write(&line.into_bytes())
                    .expect("[AArch64_global] Write failed.");
            },
            
            Arm64Type::Label => {
                let mut line = ln.name.clone();
                line.push_str(":\n");
                writer.write(&line.into_bytes())
                    .expect("[AArch64_label] Write failed.");
            },
            
            _ => {},
        }
    }
}

// Builds a function declaration
fn arm64_build_func(code : &mut Vec<Arm64Instr>, instr : &LtacInstr) {
    let mut glbl = create_arm64_instr(Arm64Type::Global);
    glbl.name = instr.name.clone();
    code.push(glbl);

    let mut instr2 = create_arm64_instr(Arm64Type::Label);
    instr2.name = instr.name.clone();
    code.push(instr2);
}

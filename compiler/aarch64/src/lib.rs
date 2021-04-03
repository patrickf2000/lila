//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

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
    let mut stack_size = 0;
    
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
                stack_size = ln.arg1_val;
            },
            
            LtacType::Ret => arm64_build_ret(code, stack_size),
            
            LtacType::MovB | LtacType::MovUB | LtacType::MovW | LtacType::MovUW
            | LtacType::Mov | LtacType::MovU | LtacType::MovQ | LtacType::MovUQ
            => arm64_build_mov(code, &ln),
            
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
            
            Arm64Type::Ret => {
                let line = "  ret\n".to_string();
                writer.write(&line.into_bytes())
                    .expect("[AArch64_ret] Write failed.");
            },
            
            _ => write_instr(writer, &ln),
        }
    }
}

fn write_instr(writer : &mut BufWriter<File>, ln : &Arm64Instr) {
    let mut line = "  ".to_string();
    
    match ln.instr_type {
        Arm64Type::Ldp => line.push_str("ldp "),
        Arm64Type::Stp => line.push_str("stp "),
        Arm64Type::Mov => line.push_str("mov "),
        _ => {},
    }
    
    line.push_str(&write_operand(&ln.arg1));
    line.push_str(", ");
    line.push_str(&write_operand(&ln.arg2));
    
    if ln.instr_type != Arm64Type::Mov {
        line.push_str(", ");
        line.push_str(&write_operand(&ln.arg3));
    }
    
    if ln.instr_type == Arm64Type::Ldp {
        line.push_str(", ");
        line.push_str(&write_operand(&ln.arg4));
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AArch64_instr] Write failed.");
}

// Writes an operand
fn write_operand(arg : &Arm64Arg) -> String {
    match arg {
        Arm64Arg::Mem(reg, val) => {
            let mut line = "[".to_string();
            line.push_str(&write_register(reg));
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("]");
            return line;
        },
        
        Arm64Arg::RegRef(reg) => {
            let mut line = "[".to_string();
            line.push_str(&write_register(reg));
            line.push_str("]");
            return line;
        },
        
        Arm64Arg::Imm32(val) => val.to_string(),
        
        Arm64Arg::Reg(reg) => write_register(reg),
        
        _ => String::new(),
    }
}

// Translates a register
fn write_register(reg : &Arm64Reg) -> String {
    match reg {
        Arm64Reg::SP => "sp".to_string(),
        
        Arm64Reg::X0 => "x0".to_string(),
        Arm64Reg::X29 => "x29".to_string(),
        Arm64Reg::X30 => "x30".to_string(),
        
        Arm64Reg::W0 => "w0".to_string(),
        
        _ => String::new(),
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
    
    // stp x29, x30, [sp, -size]
    let mut stp = create_arm64_instr(Arm64Type::Stp);
    stp.arg1 = Arm64Arg::Reg(Arm64Reg::X29);
    stp.arg2 = Arm64Arg::Reg(Arm64Reg::X30);
    stp.arg3 = Arm64Arg::Mem(Arm64Reg::SP, instr.arg1_val);
    code.push(stp);
    
    // mov x29, sp
    let mut mov = create_arm64_instr(Arm64Type::Mov);
    mov.arg1 = Arm64Arg::Reg(Arm64Reg::X29);
    mov.arg2 = Arm64Arg::Reg(Arm64Reg::SP);
    code.push(mov);
}

// Builds a function return
fn arm64_build_ret(code : &mut Vec<Arm64Instr>, stack_size : i32) {
    // ldp x29, x30, [sp], stack_size
    let mut ldp = create_arm64_instr(Arm64Type::Ldp);
    ldp.arg1 = Arm64Arg::Reg(Arm64Reg::X29);
    ldp.arg2 = Arm64Arg::Reg(Arm64Reg::X30);
    ldp.arg3 = Arm64Arg::RegRef(Arm64Reg::SP);
    ldp.arg4 = Arm64Arg::Imm32(stack_size);
    code.push(ldp);

    // ret
    let ret = create_arm64_instr(Arm64Type::Ret);
    code.push(ret);
}

// Builds a move instruction
fn arm64_build_mov(code : &mut Vec<Arm64Instr>, instr : &LtacInstr) {
    let mut mov = create_arm64_instr(Arm64Type::Mov);
    
    match instr.arg1 {
        /*LtacArg::Reg8(val) | LtacArg::Reg16(val)
        | LtacArg::Reg32(val) => mov.arg1 = arm64_arg_reg(val),*/
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8
        | LtacArg::RetRegI16 | LtacArg::RetRegU16
        | LtacArg::RetRegI32 | LtacArg::RetRegU32 => {
            mov.arg1 = Arm64Arg::Reg(Arm64Reg::W0);
        },
        
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => mov.arg1 = Arm64Arg::Reg(Arm64Reg::X0),
        
        _ => {},
    }
    
    match instr.arg2 {
        /*LtacArg::Reg8(val) | LtacArg::Reg16(val)
        | LtacArg::Reg32(val) => mov.arg1 = arm64_arg_reg(val),*/
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8
        | LtacArg::RetRegI16 | LtacArg::RetRegU16
        | LtacArg::RetRegI32 | LtacArg::RetRegU32 => {
            mov.arg2 = Arm64Arg::Reg(Arm64Reg::W0);
        },
        
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => mov.arg2 = Arm64Arg::Reg(Arm64Reg::X0),
        
        LtacArg::I32(val) => mov.arg2 = Arm64Arg::Imm32(val),
        LtacArg::U32(val) => mov.arg2 = Arm64Arg::Imm32(val as i32),
        
        _ => {},
    }
    
    code.push(mov);
}

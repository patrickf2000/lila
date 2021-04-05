//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr};

mod asm;
mod func;
mod instr;

use asm::*;
use func::*;
use instr::*;

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
                stack_size = ln.arg1_val + 16;
            },
            
            LtacType::Call => {
                let mut instr = create_arm64_instr(Arm64Type::Call);
                instr.name = ln.name.clone();
                code.push(instr);
            },
            
            LtacType::Ret => arm64_build_ret(code, stack_size),
            
            LtacType::PushArg => arm64_build_pusharg(code, &ln, stack_size, false),
            LtacType::KPushArg => arm64_build_pusharg(code, &ln, stack_size, true),
            
            LtacType::Syscall => {
                let instr = create_arm64_instr(Arm64Type::Svc);
                code.push(instr);
            },
            
            LtacType::LdArgI8 | LtacType::LdArgU8 | LtacType::LdArgI16 | LtacType::LdArgU16
            | LtacType::LdArgI32 | LtacType::LdArgU32 | LtacType::LdArgI64 | LtacType::LdArgU64
            | LtacType::LdArgPtr => arm64_build_ldarg(code, &ln, stack_size),
            
            LtacType::StrB | LtacType::StrUB | LtacType::StrW | LtacType::StrUW
            | LtacType::Str | LtacType::StrU | LtacType::StrQ | LtacType::StrUQ
            | LtacType::StrPtr => arm64_build_ld_str(code, &ln, stack_size),
            
            LtacType::LdB | LtacType::LdUB | LtacType::LdW | LtacType::LdUW
            | LtacType::Ld | LtacType::LdU | LtacType::LdQ | LtacType::LdUQ
            => arm64_build_ld_str(code, &ln, stack_size),
            
            LtacType::MovB | LtacType::MovUB | LtacType::MovW | LtacType::MovUW
            | LtacType::Mov | LtacType::MovU | LtacType::MovQ | LtacType::MovUQ
            => arm64_build_mov(code, &ln),
            
            LtacType::I32Add | LtacType::I32Sub | LtacType::I32Mul
            | LtacType::I32Div | LtacType::I32Mod
            | LtacType::And | LtacType::Or | LtacType::Xor
            | LtacType::Lsh | LtacType::Rsh
            | LtacType::I32Cmp
            => arm64_build_instr(code, &ln),
            
            LtacType::Br | LtacType::Be | LtacType::Bne
            | LtacType::Bl | LtacType::Ble
            | LtacType::Bg | LtacType::Bge => arm64_build_jump(code, &ln),
            
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
            
            Arm64Type::Call => {
                let mut line = "  bl ".to_string();
                line.push_str(&ln.name);
                line.push_str("\n");
                
                writer.write(&line.into_bytes())
                    .expect("[AArch64_call] Write failed.");
            },
            
            Arm64Type::Svc => {
                writer.write(b"  svc 0\n")
                    .expect("[AArch64_syscall] Write failed.");
            },
            
            Arm64Type::B
            | Arm64Type::Beq | Arm64Type::Bne
            | Arm64Type::Bl | Arm64Type::Ble
            | Arm64Type::Bg | Arm64Type::Bge => write_jump(writer, &ln),
            
            _ => write_instr(writer, &ln),
        }
    }
}

fn write_instr(writer : &mut BufWriter<File>, ln : &Arm64Instr) {
    let mut line = "  ".to_string();
    
    match ln.instr_type {
        Arm64Type::Ldp => line.push_str("ldp "),
        Arm64Type::Stp => line.push_str("stp "),
        Arm64Type::Adrp => line.push_str("adrp "),
        Arm64Type::Mov => line.push_str("mov "),
        Arm64Type::Str => line.push_str("str "),
        Arm64Type::Ldr => line.push_str("ldr "),
        Arm64Type::LdrSW => line.push_str("ldrsw "),
        Arm64Type::Add => line.push_str("add "),
        Arm64Type::Sub => line.push_str("sub "),
        Arm64Type::Mul => line.push_str("mul "),
        Arm64Type::SDiv => line.push_str("sdiv "),
        Arm64Type::MSub => line.push_str("msub "),
        Arm64Type::And => line.push_str("and "),
        Arm64Type::Orr => line.push_str("orr "),
        Arm64Type::Eor => line.push_str("eor "),
        Arm64Type::Lsl => line.push_str("lsl "),
        Arm64Type::Lsr => line.push_str("lsr "),
        Arm64Type::Cmp => line.push_str("cmp "),
        _ => {},
    }
    
    line.push_str(&write_operand(&ln.arg1, false));
    line.push_str(", ");
    line.push_str(&write_operand(&ln.arg2, false));
    
    if ln.arg3 != Arm64Arg::Empty {
        line.push_str(", ");
        line.push_str(&write_operand(&ln.arg3, true));
    }
    
    if ln.arg4 != Arm64Arg::Empty {
        line.push_str(", ");
        line.push_str(&write_operand(&ln.arg4, false));
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AArch64_instr] Write failed.");
}

// Writes a jump statement
fn write_jump(writer : &mut BufWriter<File>, ln : &Arm64Instr) {
    let mut line = "  ".to_string();
    
    match ln.instr_type {
        Arm64Type::B => line.push_str("b"),
        Arm64Type::Beq => line.push_str("beq"),
        Arm64Type::Bne => line.push_str("bne"),
        Arm64Type::Bl => line.push_str("bl"),
        Arm64Type::Ble => line.push_str("ble"),
        Arm64Type::Bg => line.push_str("bg"),
        Arm64Type::Bge => line.push_str("bge"),
        
        _ => return,
    }
    
    line.push_str(" ");
    line.push_str(&ln.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AArch64_jump] Write failed.");
}

// Writes an operand
fn write_operand(arg : &Arm64Arg, flag_mem : bool) -> String {
    match arg {
        Arm64Arg::Mem(reg, val) => {
            let mut line = "[".to_string();
            line.push_str(&write_register(reg));
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("]");
            
            if flag_mem {
                line.push_str("!");
            }
            
            return line;
        },
        
        Arm64Arg::RegRef(reg) => {
            let mut line = "[".to_string();
            line.push_str(&write_register(reg));
            line.push_str("]");
            return line;
        },
        
        Arm64Arg::Imm32(val) => val.to_string(),
        
        Arm64Arg::PtrLcl(ref val) => val.to_string(),
        
        Arm64Arg::PtrLclLow(ref val) => {
            let mut line = ":lo12:".to_string();
            line.push_str(val);
            return line;
        },
        
        Arm64Arg::Reg(reg) => write_register(reg),
        
        _ => String::new(),
    }
}

// Translates a register
fn write_register(reg : &Arm64Reg) -> String {
    match reg {
        Arm64Reg::SP => "sp".to_string(),
        
        Arm64Reg::X0 => "x0".to_string(),
        Arm64Reg::X1 => "x1".to_string(),
        Arm64Reg::X2 => "x2".to_string(),
        Arm64Reg::X3 => "x3".to_string(),
        Arm64Reg::X4 => "x4".to_string(),
        Arm64Reg::X5 => "x5".to_string(),
        Arm64Reg::X6 => "x6".to_string(),
        Arm64Reg::X7 => "x7".to_string(),
        Arm64Reg::X8 => "x8".to_string(),
        Arm64Reg::X29 => "x29".to_string(),
        Arm64Reg::X30 => "x30".to_string(),
        
        Arm64Reg::W0 => "w0".to_string(),
        Arm64Reg::W1 => "w1".to_string(),
        
        Arm64Reg::W9 => "w9".to_string(),
        Arm64Reg::W10 => "w10".to_string(),
        Arm64Reg::W11 => "w11".to_string(),
        Arm64Reg::W12 => "w12".to_string(),
        Arm64Reg::W13 => "w13".to_string(),
        Arm64Reg::W14 => "w14".to_string(),
        Arm64Reg::W15 => "w15".to_string(),
        Arm64Reg::W16 => "w16".to_string(),
        Arm64Reg::W17 => "w17".to_string(),
        
        _ => String::new(),
    }
}

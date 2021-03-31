//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::Command;

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

mod call;
mod flow;
mod func;
mod mov;
mod utils;

use call::*;
use flow::*;
use func::*;
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
pub fn link(all_names : &Vec<String>, output : &String, use_c : bool, is_lib : bool) {
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
    
    if use_c {
        if !is_lib {
            args.push("/usr/lib64/crti.o");
            args.push("/usr/lib64/crtn.o");
            args.push("/usr/lib64/crt1.o");
        }
        
        args.push("-lc");
    }
    
    args.push("-dynamic-linker");
    args.push("/lib64/ld-linux-riscv64-lp64d.so.1");
        
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
    
    let ld = Command::new("ld")
        .args(args.as_slice())
        .output()
        .expect("Fatal: Linking failed.");
    
    if !ld.status.success() {
        io::stdout().write_all(&ld.stdout).unwrap();
        io::stderr().write_all(&ld.stderr).unwrap();
    }
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
                line.push_str(": .word ");
                line.push_str(&data.val);
                line.push_str("\n");
            },
            
            LtacDataType::DoubleL => {
                line.push_str(&data.name);
                line.push_str(": .long ");
                line.push_str(&data.val);
                line.push_str("\n");
            },
        }
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[RISCV64_data] Write failed in .data");
}

// Write the code section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[RISCV_code] Write failed");
        
    let mut stack_size = 0;
    let mut cmp_instr : &LtacInstr = code.first().unwrap();

    for code in code.iter() {
        match &code.instr_type {
        
            // Basic function instructions
            LtacType::Extern => riscv64_build_extern(writer, &code),
            LtacType::Label => riscv64_build_label(writer, &code),
            LtacType::Ret => riscv64_build_ret(writer, stack_size),
            
            LtacType::Func => {
                riscv64_build_func(writer, &code);
                stack_size = code.arg1_val + 16;
            },
            
            // Used to load function arguments
            LtacType::LdArgI8 | LtacType::LdArgU8
            | LtacType::LdArgI16 | LtacType::LdArgU16
            | LtacType::LdArgI32 | LtacType::LdArgU32
            | LtacType::LdArgI64 | LtacType::LdArgU64
            | LtacType::LdArgF32 | LtacType::LdArgF64
            | LtacType::LdArgPtr => riscv64_build_ldarg(writer, &code, stack_size),
            
            // All the move instructions
            LtacType::MovF64 => {},
            LtacType::MovI32Vec => {},

            LtacType::MovB | LtacType::MovUB |
            LtacType::MovW | LtacType::MovUW |
            LtacType::Mov | LtacType::MovU |
            LtacType::MovQ | LtacType::MovUQ |
            LtacType::MovF32 |
            LtacType::MovF64Int => riscv64_build_mov(writer, &code),
            
            // Push/pop
            LtacType::Push => {},
            LtacType::Pop => {},
            
            // Argument and function call instructions
            LtacType::PushArg => riscv64_build_pusharg(writer, &code, false, stack_size),
            LtacType::KPushArg => riscv64_build_pusharg(writer, &code, true, stack_size),
            LtacType::Call => riscv64_build_call(writer, &code),
            LtacType::Syscall => riscv64_build_syscall(writer),
            
            // Comparison instructons
            LtacType::I8Cmp | LtacType::U8Cmp
            | LtacType::I16Cmp | LtacType::U16Cmp
            | LtacType::I32Cmp | LtacType::U32Cmp 
            | LtacType::I64Cmp | LtacType::U64Cmp => cmp_instr = code,
            LtacType::F32Cmp => {},
            LtacType::F64Cmp => {},
            LtacType::StrCmp => {},
            
            // Branching instructions
            LtacType::Br => riscv64_build_jump(writer, &code),
            LtacType::Be | LtacType::Bne
            | LtacType::Bl | LtacType::Ble
            | LtacType::Bg | LtacType::Bge => riscv64_build_cond_jump(writer, &cmp_instr, &code),
            LtacType::Bfl => {},
            LtacType::Bfle => {},
            LtacType::Bfg => {},
            LtacType::Bfge => {},
            
            // Signed 32-bit vector math operations
            LtacType::I32VAdd => {},
            
            // These are intrinsics if you will; they should never get down to a code generation layer
            LtacType::Exit => {},
            LtacType::Malloc => {},
            LtacType::Free => {},
            
            // These are specific to RISC machines
            // RISC Load instructions
            LtacType::LdF64 => {},

            LtacType::LdB | LtacType::LdUB |
            LtacType::LdW | LtacType::LdUW |
            LtacType::Ld | LtacType::LdU |
            LtacType::LdQ | LtacType::LdUQ |
            LtacType::LdF32 => riscv64_build_ld_str(writer, &code, stack_size, true),
            
            // RISC store instructions
            LtacType::StrF64 => {},
            LtacType::StrPtr => {},

            LtacType::StrB | LtacType::StrUB |
            LtacType::StrW | LtacType::StrUW |
            LtacType::Str | LtacType::StrU |
            LtacType::StrQ | LtacType::StrUQ | 
            LtacType::StrF32 => riscv64_build_ld_str(writer, &code, stack_size, false),

            // Misc instructions
            LtacType::CvtF32F64 => riscv64_build_cvt(writer, &code),
            
            // All else
            _ => riscv64_build_instr(writer, &code),
        }
    }
}

// A small utility function to see if we are using a multiply-divide instruction
fn riscv64_is_muldiv(instr_type : &LtacType) -> bool {
    match instr_type {
        LtacType::I8Mul | LtacType::U8Mul
        | LtacType::I8Div | LtacType::U8Div
        | LtacType::I8Mod | LtacType::U8Mod
        | LtacType::I16Mul | LtacType::U16Mul
        | LtacType::I16Div | LtacType::U16Div
        | LtacType::I16Mod | LtacType::U16Mod
        | LtacType::I32Mul | LtacType::U32Mul
        | LtacType::I32Div | LtacType::U32Div
        | LtacType::I32Mod | LtacType::U32Mod
        | LtacType::I64Mul | LtacType::U64Mul
        | LtacType::I64Div | LtacType::U64Div
        | LtacType::I64Mod | LtacType::U64Mod
        => return true,

        _ => return false,
    }
}

// Builds the base integer instructions
fn riscv64_build_instr(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut instr = String::new();
    let mut suffix = 'w';

    let is_muldiv = riscv64_is_muldiv(&code.instr_type);

    // Write the instruction type
    match &code.instr_type {
        LtacType::I8Add | LtacType::U8Add
        | LtacType::I16Add | LtacType::U16Add
        | LtacType::I32Add | LtacType::U32Add 
        | LtacType::I64Add | LtacType::U64Add => instr = "add".to_string(),
        
        LtacType::I8Sub | LtacType::I16Sub
        | LtacType::I32Sub | LtacType::I64Sub => instr = "sub".to_string(),

        LtacType::I8Mul | LtacType::U8Mul
        | LtacType::I16Mul | LtacType::U16Mul
        | LtacType::I32Mul | LtacType::U32Mul 
        | LtacType::I64Mul | LtacType::U64Mul => instr = "mul".to_string(),

        LtacType::I8Div | LtacType::U8Div
        | LtacType::I16Div | LtacType::U16Div
        | LtacType::I32Div | LtacType::U32Div 
        | LtacType::I64Div | LtacType::U64Div => instr = "div".to_string(),

        LtacType::I8Mod | LtacType::U8Mod
        | LtacType::I16Mod | LtacType::U16Mod
        | LtacType::I32Mod | LtacType::U32Mod 
        | LtacType::I64Mod | LtacType::U64Mod => instr = "rem".to_string(),

        LtacType::F32Add => {
            instr = "fadd.s".to_string();
            suffix = 0 as char;
        } ,
        
        LtacType::F32Sub => {
            instr = "fsub.s".to_string();
            suffix = 0 as char;
        },
        
        LtacType::F32Mul => {
            instr = "fmul.s".to_string();
            suffix = 0 as char;
        },
        
        LtacType::F32Div => {
            instr = "fdiv.s".to_string();
            suffix = 0 as char;
        },

        LtacType::And => {
            instr = "and".to_string();
            suffix = 0 as char;
        },
        
        LtacType::Or => {
            instr = "or".to_string();
            suffix = 0 as char;
        },
        
        LtacType::Xor => {
            instr = "xor".to_string();
            suffix = 0 as char;
        },
        
        LtacType::Lsh => instr = "sll".to_string(),
        LtacType::Rsh => instr = "srl".to_string(),
            
        _ => {},
    }

    // Check to see if we have an immediate as the second operand
    // TODO: Try to combine some of this
    match &code.arg2 {
        LtacArg::Byte(val) if is_muldiv => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::UByte(val) if is_muldiv => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
    
        LtacArg::I16(val) if is_muldiv => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::U16(val) if is_muldiv => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
        
        LtacArg::I32(val) if is_muldiv => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::U32(val) if is_muldiv => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::I64(val) if is_muldiv => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::U64(val) if is_muldiv => {
            line.push_str("  li s2, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },

        LtacArg::Byte(_v) if code.instr_type == LtacType::I8Sub => instr = "addi".to_string(),
        LtacArg::Byte(_v) => instr.push('i'),

        LtacArg::UByte(_v) => instr.push('i'),

        LtacArg::I16(_v) if code.instr_type == LtacType::I16Sub => instr = "addi".to_string(),
        LtacArg::I16(_v) => instr.push('i'),

        LtacArg::U16(_v) => instr.push('i'),
        
        LtacArg::I32(_v) if code.instr_type == LtacType::I32Sub => instr = "addi".to_string(),
        LtacArg::I32(_v) => instr.push('i'),

        LtacArg::U32(_v) => instr.push('i'),

        LtacArg::I64(_v) if code.instr_type == LtacType::I64Sub => instr = "addi".to_string(),
        LtacArg::I64(_v) => instr.push('i'),

        LtacArg::U64(_v) => instr.push('i'),

        LtacArg::F32(val) => {
            line.push_str("  lui s2, %hi(");
            line.push_str(val);
            line.push_str(")\n");

            line.push_str("  flw fs2, %lo(");
            line.push_str(val);
            line.push_str(")(s2)\n");
        },

        _ => {},
    }

    if suffix != (0 as char) {
        instr.push(suffix);
    }
    
    line.push_str("  ");
    line.push_str(&instr);
    line.push_str(" ");

    // Write the first operand
    match &code.arg1 {
        LtacArg::RetRegI8 | LtacArg::RetRegU8
        | LtacArg::RetRegI16 | LtacArg::RetRegU16
        | LtacArg::RetRegI32 | LtacArg::RetRegU32 
        | LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("a0, "),

        LtacArg::Reg8(pos) | LtacArg::Reg16(pos)
        | LtacArg::Reg32(pos) | LtacArg::Reg64(pos) => {
            let reg = riscv64_op_reg(*pos);

            line.push_str(&reg);
            line.push_str(", ");

            // TODO: Does this need to be here?
            if code.instr_type != LtacType::Mov {
                line.push_str(&reg);
                line.push_str(", ");
            }
        },

        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = riscv64_op_freg(*pos);
            
            line.push_str(&reg);
            line.push_str(", ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        _ => {},
    }

    // Write the second operand
    match &code.arg2 {
        LtacArg::Reg8(pos) | LtacArg::Reg16(pos)
        | LtacArg::Reg32(pos) | LtacArg::Reg64(pos) => {
            let reg = riscv64_op_reg(*pos);
            line.push_str(&reg);
        },

        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = riscv64_op_freg(*pos);
            line.push_str(&reg);
        },

        LtacArg::Byte(_v) if is_muldiv => line.push_str("s2"),
        LtacArg::UByte(_v) if is_muldiv => line.push_str("s2"),

        LtacArg::I16(_v) if is_muldiv => line.push_str("s2"),
        LtacArg::U16(_v) if is_muldiv => line.push_str("s2"),

        LtacArg::I32(_v) if is_muldiv => line.push_str("s2"),
        LtacArg::U32(_v) if is_muldiv => line.push_str("s2"),

        LtacArg::I64(_v) if is_muldiv => line.push_str("s2"),
        LtacArg::U64(_v) if is_muldiv => line.push_str("s2"),

        LtacArg::Byte(val) => {
            let mut num = *val;
            
            if code.instr_type == LtacType::I8Sub {
                if num > 0 {
                    line.push_str("-");
                } else {
                    num *= -1;
                }
            }
            
            line.push_str(&num.to_string());
        },

        LtacArg::I16(val) => {
            let mut num = *val;
            
            if code.instr_type == LtacType::I16Sub {
                if num > 0 {
                    line.push_str("-");
                } else {
                    num *= -1;
                }
            }
            
            line.push_str(&num.to_string());
        },
    
        LtacArg::I32(val) => {
            let mut num = *val;
            
            if code.instr_type == LtacType::I32Sub {
                if num > 0 {
                    line.push_str("-");
                } else {
                    num *= -1;
                }
            }
            
            line.push_str(&num.to_string());
        },

        LtacArg::I64(val) => {
            let mut num = *val;
            
            if code.instr_type == LtacType::I64Sub {
                if num > 0 {
                    line.push_str("-");
                } else {
                    num *= -1;
                }
            }
            
            line.push_str(&num.to_string());
        },

        LtacArg::UByte(val) => line.push_str(&val.to_string()),
        LtacArg::U16(val) => line.push_str(&val.to_string()),
        LtacArg::U32(val) => line.push_str(&val.to_string()),
        LtacArg::U64(val) => line.push_str(&val.to_string()),
        
        LtacArg::F32(_v) => line.push_str("fs2"),

        _ => {},
    }

    // Finish writing
    line.push_str("\n");

    writer.write(&line.into_bytes())
        .expect("[RISCV64_write_instr] Write failed.");
}

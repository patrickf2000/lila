use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::{Command, Output};

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

// Import local modules
mod call;
mod func;
mod utils;
mod vector;

use call::*;
use func::*;
use utils::*;
use vector::*;

pub fn compile(ltac_file : &LtacFile) -> io::Result<()> {
    let mut name = "/tmp/".to_string();
    name.push_str(&ltac_file.name);
    name.push_str(".asm");
    
    // Write it out
    let file = File::create(&name)?;
    let mut writer = BufWriter::new(file);
    
    //GNU AS specific
    writer.write(b".intel_syntax noprefix\n")
        .expect("[AMD64_setup] Write failed.");
    
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
            "/usr/lib/x86_64-linux-gnu/crti.o",
            "/usr/lib/x86_64-linux-gnu/crtn.o",
            "/usr/lib/x86_64-linux-gnu/crt1.o",
            &obj_name,
            "-dynamic-linker",
            "/lib64/ld-linux-x86-64.so.2",
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
        .expect("[AMD64_data] Write failed in .data");
}

// Writes the .text section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[AMD64_code] Write failed");

    for code in code.iter() {
        match &code.instr_type {
            LtacType::Extern => amd64_build_extern(writer, &code),
            LtacType::Label => amd64_build_label(writer, &code),
            LtacType::Func => amd64_build_func(writer, &code),
            LtacType::LdArgI32 => amd64_build_ldarg(writer, &code),
            LtacType::LdArgF32 => amd64_build_ldarg_float(writer, &code),
            LtacType::LdArgF64 => amd64_build_ldarg_float(writer, &code),
            LtacType::LdArgPtr => amd64_build_ldarg(writer, &code),
            LtacType::Ret => amd64_build_ret(writer),
            
            LtacType::Mov => amd64_build_instr(writer, &code),
            LtacType::MovB | LtacType::MovUB => amd64_build_instr(writer, &code),
            LtacType::MovW => amd64_build_instr(writer, &code),
            LtacType::MovF32 => amd64_build_instr(writer, &code),
            LtacType::MovF64 => amd64_build_instr(writer, &code),
            LtacType::MovOffImm => amd64_build_mov_offset(writer, &code),
            LtacType::MovOffMem => amd64_build_mov_offset(writer, &code),
            LtacType::MovI32Vec => amd64_build_vector_instr(writer, &code),
            
            LtacType::PushArg => amd64_build_pusharg(writer, &code, false),
            LtacType::KPushArg => amd64_build_pusharg(writer, &code, true),
            LtacType::Call => amd64_build_call(writer, &code),
            LtacType::Syscall => amd64_build_syscall(writer),
            
            LtacType::I8Cmp => amd64_build_instr(writer, &code),
            LtacType::I32Cmp => amd64_build_instr(writer, &code),
            LtacType::F32Cmp => amd64_build_instr(writer, &code),
            LtacType::F64Cmp => amd64_build_instr(writer, &code),
            LtacType::StrCmp => amd64_build_strcmp(writer, &code),
            
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
            
            LtacType::BMul => amd64_build_byte_mul(writer, &code),
            LtacType::BDiv | LtacType::BMod => amd64_build_byte_div(writer, &code),
            
            LtacType::BAdd | LtacType::I32Add => amd64_build_instr(writer, &code),
            LtacType::BSub | LtacType::I32Sub => amd64_build_instr(writer, &code),
            LtacType::I32Mul => amd64_build_instr(writer, &code),
            LtacType::I32Div => amd64_build_div(writer, &code),
            LtacType::I32Mod => amd64_build_div(writer, &code),
            
            LtacType::F32Add => amd64_build_instr(writer, &code),
            LtacType::F32Sub => amd64_build_instr(writer, &code),
            LtacType::F32Mul => amd64_build_instr(writer, &code),
            LtacType::F32Div => amd64_build_instr(writer, &code),
            
            LtacType::F64Add => amd64_build_instr(writer, &code),
            LtacType::F64Sub => amd64_build_instr(writer, &code),
            LtacType::F64Mul => amd64_build_instr(writer, &code),
            LtacType::F64Div => amd64_build_instr(writer, &code),
            
            LtacType::BAnd | LtacType::I32And => amd64_build_instr(writer, &code),
            LtacType::BOr | LtacType::I32Or => amd64_build_instr(writer, &code),
            LtacType::BXor | LtacType::I32Xor => amd64_build_instr(writer, &code),
            LtacType::BLsh | LtacType::I32Lsh => amd64_build_instr(writer, &code),
            LtacType::BRsh | LtacType::I32Rsh => amd64_build_instr(writer, &code),
            
            LtacType::I32VAdd => amd64_build_vector_instr(writer, &code),
            
            // We shouldn't generate any assembly for these
            LtacType::Exit => {},
            LtacType::Malloc => {},
            LtacType::Free => {},
            
            // These are RISC-specific. At some point, we should generate instructions for them
            LtacType::Ld => {},
            LtacType::LdB | LtacType::LdUB => {},
            LtacType::LdW => {},
            LtacType::Str => {},
            LtacType::StrB | LtacType::StrUB => {},
            LtacType::StrW => {},
            LtacType::StrPtr => {},
        }
    }
}

// Many instructions have common syntax
fn amd64_build_instr(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    // Specific for float literals
    match code.arg1_type {
        LtacArg::F32(ref val) => {
            line.push_str("  movss xmm1, DWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        LtacArg::F64(ref val) => {
            line.push_str("  movsd xmm1, QWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        _ => {},
    }
    
    match code.arg2_type {
        LtacArg::F32(ref val) => {
            line.push_str("  movss xmm0, DWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        LtacArg::F64(ref val) => {
            line.push_str("  movsd xmm0, QWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        _ => {},
    }
    
    // The instruction
    match &code.instr_type {
        LtacType::Mov | 
        LtacType::MovB | LtacType::MovUB |
        LtacType::MovW => line.push_str("  mov "),
        LtacType::MovF32 => line.push_str("  movss "),
        LtacType::MovF64 => line.push_str("  movsd "),
        
        LtacType::BAdd | LtacType::I32Add => line.push_str("  add "),
        LtacType::BSub | LtacType::I32Sub => line.push_str("  sub "),
        LtacType::I32Mul => line.push_str("  imul "),
        
        LtacType::F32Add => line.push_str("  addss "),
        LtacType::F32Sub => line.push_str("  subss "),
        LtacType::F32Mul => line.push_str("  mulss "),
        LtacType::F32Div => line.push_str("  divss "),
        
        LtacType::F64Add => line.push_str("  addsd "),
        LtacType::F64Sub => line.push_str("  subsd "),
        LtacType::F64Mul => line.push_str("  mulsd "),
        LtacType::F64Div => line.push_str("  divsd "),
        
        LtacType::BAnd | LtacType::I32And => line.push_str("  and "),
        LtacType::BOr | LtacType::I32Or => line.push_str("  or "),
        LtacType::BXor | LtacType::I32Xor => line.push_str("  xor "),
        LtacType::BLsh | LtacType::I32Lsh => line.push_str("  shl "),
        LtacType::BRsh | LtacType::I32Rsh => line.push_str("  shr "),
        
        LtacType::I8Cmp | LtacType::I32Cmp => line.push_str("  cmp "),
        LtacType::F32Cmp => line.push_str("  ucomiss "),
        LtacType::F64Cmp => line.push_str("  ucomisd "),
        
        _ => {},
    }

    // The arguments
    match &code.arg1_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg16(_p) => {},
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::RetRegI32 => line.push_str("eax, "),
        LtacArg::RetRegI64 => line.push_str("rax, "),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0, "),
        
        LtacArg::Mem(pos) => {
            match &code.arg2_type {
                LtacArg::Byte(_v) => line.push_str("BYTE PTR "),
                LtacArg::I16(_v) => line.push_str("WORD PTR "),
                LtacArg::I32(_v) => line.push_str("DWORD PTR "), 
                LtacArg::F32(_v) => line.push_str("DWORD PTR "),
                LtacArg::F64(_v) | LtacArg::PtrLcl(_v) => line.push_str("QWORD PTR "), 
                LtacArg::Ptr(_v) => line.push_str("QWORD PTR "),
                _ => {},
            }
            
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("], ");
        },
        
        LtacArg::Byte(_v) => {},
        LtacArg::I16(_v) => {},
        LtacArg::I32(_v) => {},
        
        // TODO: Combine
        LtacArg::F32(_v) => {
            line.push_str("xmm1, ");
        },
        
        LtacArg::F64(_v) => {
            line.push_str("xmm1, ");
        },
        
        LtacArg::Ptr(_v) => {},
        LtacArg::PtrLcl(_v) => {},
    }
    
    match &code.arg2_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::Reg16(_p) => {},
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI32 => line.push_str("eax"),
        LtacArg::RetRegI64 => line.push_str("rax"),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0"),
        
        LtacArg::Mem(pos) => {
            line.push_str("[rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]");
        },
        
        LtacArg::Byte(val) => {
            line.push_str(&val.to_string());
        },
        
        LtacArg::I16(val) => {
            line.push_str(&val.to_string());
        },
        
        LtacArg::I32(val) => {
            line.push_str(&val.to_string());
        },
        
        LtacArg::F32(_v) | LtacArg::F64(_v) => {
            line.push_str("xmm0\n");
        },
        
        LtacArg::Ptr(_p) => {},
        
        LtacArg::PtrLcl(ref val) => {
            line.push_str("OFFSET FLAT:");
            line.push_str(&val);
        },
    }
    
    // Write to the file
    line.push_str("\n");
    writer.write(&line.into_bytes())
        .expect("[AMD64_write_instr] Write failed.");
}

// Builds a move-offset instruction
fn amd64_build_mov_offset(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    // Needed if the source is an array index
    if code.arg2_offset > 0 && code.instr_type == LtacType::MovOffImm {
        match code.arg2_type {
            LtacArg::Mem(pos) => {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            },
            
            _ => {},
        }
        
        match code.arg1_type {
            LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15+"),
            _ => line.push_str("  mov r15d, DWORD PTR [r15+"),
        }
        
        line.push_str(&code.arg2_offset.to_string());
        line.push_str("]\n");
    } else if code.arg2_offset > 0 && code.instr_type == LtacType::MovOffMem {
        // Load the variable
        line.push_str("  mov r15d, DWORD PTR [rbp-");
        line.push_str(&code.arg2_offset.to_string());
        line.push_str("]\n");
        
        // Clear flags
        line.push_str("  cdqe\n");
        
        // Load the effective address
        line.push_str("  lea r14, [0+r15*");
        line.push_str(&code.arg2_offset_size.to_string());
        line.push_str("]\n");
        
        // Load the array
        match code.arg2_type {
            LtacArg::Mem(pos) => {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            },
            
            _ => {},
        }
        
        // Add to get the proper offset
        line.push_str("  add r15, r14\n");
        
        // Store
        line.push_str("  mov r15d, DWORD PTR [r15]\n");
    }
    
    // The arguments
    match &code.arg1_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg16(_p) => {},
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::FltReg(_p) => {},
        LtacArg::FltReg64(_p) => {},
        
        LtacArg::RetRegI32 => line.push_str("  mov eax, "),
        LtacArg::RetRegI64 => line.push_str("  mov rax, "),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0, "),
        
        LtacArg::Mem(pos) => {
            if code.arg1_offset > 0 && code.instr_type == LtacType::MovOffImm {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  add r15, ");
                line.push_str(&code.arg1_offset.to_string());
                line.push_str("\n");
                
                line.push_str("  mov ");
                match code.arg2_type {
                    LtacArg::I32(_v) => line.push_str("DWORD PTR "),
                    _ => {},
                };
                line.push_str("[r15], ");
            } else if code.arg1_offset > 0 && code.instr_type == LtacType::MovOffMem {
                // Load the variable
                line.push_str("  mov r15d, DWORD PTR [rbp-");
                line.push_str(&code.arg1_offset.to_string());
                line.push_str("]\n");
                
                // Clear flags
                line.push_str("  cdqe\n");
                
                // Load the effective address
                line.push_str("  lea r14, [0+r15*");
                line.push_str(&code.arg1_offset_size.to_string());
                line.push_str("]\n");
                
                // Load the array
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                // Add to get the proper offset
                line.push_str("  add r15, r14\n");
                
                // Now set up for the final move
                line.push_str("  mov ");
                match code.arg2_type {
                    LtacArg::I32(_v) => line.push_str("DWORD PTR "),
                    _ => {},
                }
                line.push_str("[r15], ");
            } else {
                match code.arg2_type {
                    LtacArg::I32(_v) => line.push_str("DWORD PTR "),
                    _ => line.push_str("  mov "),
                }
                
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("], ");
            }
        },
        
        LtacArg::Byte(_v) => {},
        LtacArg::I16(_v) => {},
        LtacArg::I32(_v) => {},
        LtacArg::F32(_v) => {},
        LtacArg::F64(_v) => {},
        LtacArg::Ptr(_v) => {},
        LtacArg::PtrLcl(_v) => {},
    }
    
    match &code.arg2_type {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(_p) => {},
        LtacArg::Reg16(_p) => {},
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::FltReg(_p) => {},
        LtacArg::FltReg64(_p) => {},
        
        LtacArg::RetRegI32 => line.push_str("eax"),
        LtacArg::RetRegI64 => line.push_str("rax"),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0"),
        
        LtacArg::Mem(pos) => {
            if code.instr_type == LtacType::MovOffImm || code.instr_type == LtacType::MovOffMem {
                match &code.arg1_type {
                    LtacArg::Reg8(_p) => line.push_str("r15b"),
                    _ => line.push_str("r15d"),
                }
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]");
            }
        },
        
        LtacArg::Byte(val) => {
            line.push_str(" BYTE PTR ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::I16(_v) => {},
        
        LtacArg::I32(val) => {
            line.push_str(&val.to_string());
        },
        
        LtacArg::F32(_v) => {},
        
        LtacArg::F64(_v) => {},
        
        LtacArg::Ptr(_v) => {},
        LtacArg::PtrLcl(_v) => {},
    }
    
    line.push_str("\n");

    writer.write(&line.into_bytes())
        .expect("[AMD64_writer_instr] Write failed.");
}

// Builds multiplication for byte values
// On x86 this is also a little strange...
fn amd64_build_byte_mul(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    line.push_str("  xor eax, eax\n");
    
    match &code.arg1_type {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  mov al, ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  mul ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  mul [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::Byte(val) => {
            line.push_str("  mov r15b, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  mul r15b\n");
        },
        
        _ => {},
    }
    
    // Move the result back to the proper register
    let reg = amd64_op_reg16(code.arg1_val);
    
    line.push_str("  mov ");
    line.push_str(&reg);
    line.push_str(", ax\n");
    
    // Write
    writer.write(&line.into_bytes())
        .expect("[AMD64_byte_mul] Write failed.");
}

// Builds division for byte values
fn amd64_build_byte_div(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut dest = String::new();
    
    line.push_str("  xor eax, eax\n");
    line.push_str("  xor edx, edx\n");
    
    match &code.arg1_type {
        LtacArg::Reg8(pos) => {
            dest = amd64_op_reg8(*pos);
            
            line.push_str("  mov al, ");
            line.push_str(&dest);
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  div ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  div BYTE PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::Byte(val) => {
            line.push_str("  mov r15b, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  div r15b\n");
        },
        
        _ => {},
    }
    
    // Move the result back to the proper register
    line.push_str("  mov ");
    line.push_str(&dest);
    line.push_str(", ");
    
    if code.instr_type == LtacType::BMod {
        line.push_str("ah\n");
    } else {
        line.push_str("al\n");
    }
    
    // Write
    writer.write(&line.into_bytes())
        .expect("[AMD64_byte_div] Write failed.");
}

// Builds the integer and modulus instructions
// On x86 these are a little weird...
fn amd64_build_div(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    let mut dest_line = String::new();
    
    line.push_str("  xor edx, edx\n");
    
    match &code.arg1_type {
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str("  mov eax, ");
            line.push_str(&reg);
            line.push_str("\n");
            
            dest_line.push_str("  mov ");
            dest_line.push_str(&reg);
            dest_line.push_str(", ");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  mov eax, DWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            dest_line.push_str("  mov DWORD PTR [rbp-");
            dest_line.push_str(&pos.to_string());
            dest_line.push_str("], ");
        },
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str("  idiv ");
            line.push_str(&reg);
            line.push_str("\n");
        },
        
        LtacArg::Mem(pos) => {
            line.push_str("  idiv DWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I32(val) => {
            line.push_str("  mov r15d, ");
            line.push_str(&val.to_string());
            line.push_str("\n");
            
            line.push_str("  idiv r15d\n");
        },
        
        _ => {},
    }
    
    line.push_str(&dest_line);
    
    if code.instr_type == LtacType::I32Mod {
        line.push_str("edx\n");
    } else {
        line.push_str("eax\n");
    }
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_div] Write failed.");
}

// Builds a branch (actually kinda called "jumps" in x86...)
fn amd64_build_jump(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  ".to_string();
    
    match &code.instr_type {
        LtacType::Br => line.push_str("jmp "),
        LtacType::Be => line.push_str("je "),
        LtacType::Bne => line.push_str("jne "),
        LtacType::Bl => line.push_str("jl "),
        LtacType::Ble => line.push_str("jle "),
        LtacType::Bfl => line.push_str("jb "),
        LtacType::Bfle => line.push_str("jbe "),
        LtacType::Bg => line.push_str("jg "),
        LtacType::Bge => line.push_str("jge "),
        LtacType::Bfg => line.push_str("ja "),
        LtacType::Bfge => line.push_str("jae "),
        _ => {},
    }
    
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_jump] Write failed.");
}

// Builds a string comparison
fn amd64_build_strcmp(writer : &mut BufWriter<File>, _code : &LtacInstr) {
    let mut line = String::new();
    line.push_str("  call strcmp\n");
    line.push_str("  cmp eax, 0\n");
    
    writer.write(&line.into_bytes())
        .expect("[AMD64_build_strcmp] Write failed.");
}


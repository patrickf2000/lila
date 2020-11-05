use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process::Command;

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

// Import local modules
mod call;
mod func;
mod math;
mod utils;
mod vector;

use call::*;
use func::*;
use math::*;
use utils::*;
use vector::*;

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
            args.push("/usr/lib/x86_64-linux-gnu/crti.o");
            args.push("/usr/lib/x86_64-linux-gnu/crtn.o");
            args.push("/usr/lib/x86_64-linux-gnu/crt1.o");
            
            args.push("-dynamic-linker");
            args.push("/lib64/ld-linux-x86-64.so.2");
        }
        
        args.push("-lc");
    }
        
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

// Writes the .text section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>, is_pic : bool) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[AMD64_code] Write failed");

    for code in code.iter() {
        match &code.instr_type {
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
            
            LtacType::MovOffImm => amd64_build_mov_offset(writer, &code),
            LtacType::MovOffMem => amd64_build_mov_offset(writer, &code),
            LtacType::MovI32Vec => amd64_build_vector_instr(writer, &code),
            
            LtacType::PushArg => amd64_build_pusharg(writer, &code, false, is_pic),
            LtacType::KPushArg => amd64_build_pusharg(writer, &code, true, is_pic),
            LtacType::Call => amd64_build_call(writer, &code),
            LtacType::Syscall => amd64_build_syscall(writer),
            
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
            LtacType::Ld => {},
            LtacType::LdB | LtacType::LdUB => {},
            LtacType::LdW => {},
            LtacType::Str => {},
            LtacType::StrB | LtacType::StrUB => {},
            LtacType::StrW => {},
            LtacType::StrPtr => {},
            
            // Everything else uses the common build instruction function
            _ => amd64_build_instr(writer, &code, is_pic),
        }
    }
}

// Many instructions have common syntax
fn amd64_build_instr(writer : &mut BufWriter<File>, code : &LtacInstr, is_pic : bool) {
    let mut line = String::new();
    
    // Specific for float literals
    match code.arg2 {
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
    // TODO: The unsigned multiplication should use "mul". This may require a separate function
    match &code.instr_type {
        LtacType::Mov | LtacType::MovU |
        LtacType::MovB | LtacType::MovUB |
        LtacType::MovW | LtacType::MovUW |
        LtacType::MovQ | LtacType::MovUQ => line.push_str("  mov "),
        LtacType::MovF32 => line.push_str("  movss "),
        LtacType::MovF64 => line.push_str("  movsd "),
        
        LtacType::I8Add | LtacType::U8Add |
        LtacType::I16Add | LtacType::U16Add |
        LtacType::I32Add | LtacType::U32Add |
        LtacType::I64Add | LtacType::U64Add => line.push_str("  add "),
        
        LtacType::I8Sub | LtacType::I16Sub |
        LtacType::I32Sub  | LtacType::I64Sub => line.push_str("  sub "),
        
        LtacType::I16Mul | LtacType::I32Mul |
        LtacType::I64Mul => line.push_str("  imul "),
        LtacType::U16Mul | LtacType::U32Mul |
        LtacType::U64Mul => line.push_str("  imul "),
        
        LtacType::F32Add => line.push_str("  addss "),
        LtacType::F32Sub => line.push_str("  subss "),
        LtacType::F32Mul => line.push_str("  mulss "),
        LtacType::F32Div => line.push_str("  divss "),
        
        LtacType::F64Add => line.push_str("  addsd "),
        LtacType::F64Sub => line.push_str("  subsd "),
        LtacType::F64Mul => line.push_str("  mulsd "),
        LtacType::F64Div => line.push_str("  divsd "),
        
        LtacType::BAnd | LtacType::WAnd |
        LtacType::I32And | LtacType::I64And => line.push_str("  and "),
        LtacType::BOr | LtacType::WOr |
        LtacType::I32Or | LtacType::I64Or => line.push_str("  or "),
        LtacType::BXor | LtacType::WXor |
        LtacType::I32Xor | LtacType::I64Xor => line.push_str("  xor "),
        LtacType::BLsh | LtacType::WLsh |
        LtacType::I32Lsh | LtacType::I64Lsh => line.push_str("  shl "),
        LtacType::BRsh | LtacType::WRsh |
        LtacType::I32Rsh | LtacType::I64Rsh => line.push_str("  shr "),
        
        LtacType::I8Cmp | LtacType::U8Cmp |
        LtacType::I16Cmp | LtacType::U16Cmp |
        LtacType::I32Cmp | LtacType::U32Cmp |
        LtacType::I64Cmp | LtacType::U64Cmp => line.push_str("  cmp "),
        LtacType::F32Cmp => line.push_str("  ucomiss "),
        LtacType::F64Cmp => line.push_str("  ucomisd "),
        
        _ => {},
    }

    // The arguments
    match &code.arg1 {
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
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
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8 => line.push_str("eax, "),
        LtacArg::RetRegI16 | LtacArg::RetRegU16 => line.push_str("eax, "),
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => line.push_str("eax, "),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("rax, "),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0, "),
        
        LtacArg::Mem(pos) => {
            match &code.arg2 {
                LtacArg::Byte(_v) => line.push_str("BYTE PTR "),
                LtacArg::UByte(_v) => line.push_str("BYTE PTR "),
                LtacArg::I16(_v) => line.push_str("WORD PTR "),
                LtacArg::U16(_v) => line.push_str("WORD PTR "),
                LtacArg::I32(_v) => line.push_str("DWORD PTR "),
                LtacArg::U32(_v) => line.push_str("DWORD PTR "),
                LtacArg::I64(_v) => line.push_str("QWORD PTR "),
                LtacArg::U64(_v) => line.push_str("QWORD PTR "),
                LtacArg::F32(_v) => line.push_str("DWORD PTR "),
                LtacArg::F64(_v) | LtacArg::PtrLcl(_v) => line.push_str("QWORD PTR "), 
                LtacArg::Ptr(_v) => line.push_str("QWORD PTR "),
                _ => {},
            }
            
            if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp], ");
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("], ");
            }
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            line.push_str(&reg);
        },
        
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
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8 => line.push_str("al"),
        LtacArg::RetRegI16 | LtacArg::RetRegU16 => line.push_str("ax"),
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => line.push_str("eax"),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("rax"),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0"),
        
        LtacArg::Mem(pos) => {
            if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]");
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]");
            }
        },
        
        LtacArg::Byte(val) => line.push_str(&val.to_string()),
        LtacArg::UByte(val) => line.push_str(&val.to_string()),
        
        LtacArg::I16(val) => {
            line.push_str(&val.to_string());
        },
        
        LtacArg::U16(val) => {
            line.push_str(&val.to_string());
        },
        
        LtacArg::I32(val) => line.push_str(&val.to_string()),
        LtacArg::U32(val) => line.push_str(&val.to_string()),
        
        LtacArg::I64(val) => line.push_str(&val.to_string()),
        LtacArg::U64(val) => line.push_str(&val.to_string()),
        
        LtacArg::F32(_v) | LtacArg::F64(_v) => {
            line.push_str("xmm0\n");
        },
        
        LtacArg::Ptr(_p) => {},
        
        LtacArg::PtrLcl(ref val) => {
            line.push_str("OFFSET FLAT:");
            line.push_str(&val);
        },
    }
    
    line.push_str("\n");
    
    // Special cases
    // Bytes
    if code.arg1 == LtacArg::RetRegI8 {
        line.push_str("  movsx eax, al\n");
    } else if code.arg1 == LtacArg::RetRegU8 {
        line.push_str("  movzx eax, al\n");
        
    // Short
    } else if code.arg1 == LtacArg::RetRegI16 {
        line.push_str("  movsx eax, ax\n");
    } else if code.arg1 == LtacArg::RetRegU16 {
        line.push_str("  movzx eax, ax\n");
    }
    
    // Write to the file
    writer.write(&line.into_bytes())
        .expect("[AMD64_write_instr] Write failed.");
}

// Builds a move-offset instruction
fn amd64_build_mov_offset(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    // Specific for float literals
    match code.arg2 {
        LtacArg::F32(ref val) => {
            line.push_str("  movss xmm2, DWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        LtacArg::F64(ref val) => {
            line.push_str("  movsd xmm2, QWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        _ => {},
    }
    
    // Needed if the source is an array index
    if code.instr_type == LtacType::MovOffImm {
        match code.arg2 {
            LtacArg::Mem(pos) => {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                match code.arg1 {
                    LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15+"),
                    LtacArg::Reg16(_p) => line.push_str("  mov r15w, WORD PTR [r15+"),
                    LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15+"),
                    LtacArg::FltReg(_p) => line.push_str("  movss xmm1, DWORD PTR [r15+"),
                    LtacArg::FltReg64(_p) => line.push_str("  movsd xmm1, QWORD PTR [r15+"),
                    _ => line.push_str("  mov r15d, DWORD PTR [r15+"),
                }
                
                line.push_str(&code.arg2_offset.to_string());
                line.push_str("]\n");
            },
            
            _ => {},
        }
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
        match code.arg2 {
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
        match &code.arg1 {
            LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15]\n"),
            LtacArg::FltReg(_p) => line.push_str("  movss xmm1, DWORD PTR [r15]\n"),
            LtacArg::FltReg64(_p) => line.push_str("  movsd xmm1, QWORD PTR [r15]\n"),
            _ => line.push_str("  mov r15d, DWORD PTR [r15]\n"),
        }
    }
    
    // The arguments
    match &code.arg1 {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
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
        
        LtacArg::FltReg(pos) => {
            let reg = amd64_op_flt(*pos);
            
            line.push_str("  movss ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            
            line.push_str("  movsd ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => line.push_str("  mov eax, "),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("  mov rax, "),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0, "),
        
        LtacArg::Mem(pos) => {
            if code.arg1_offset > 0 && code.instr_type == LtacType::MovOffImm {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  add r15, ");
                line.push_str(&code.arg1_offset.to_string());
                line.push_str("\n");
                
                match &code.arg2 {
                    LtacArg::I32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::U32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::I64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::U64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::F32(_v) => line.push_str("  movss DWORD PTR "),
                    LtacArg::F64(_v) => line.push_str("  movsd QWORD PTR "),
                    LtacArg::FltReg(_v) => line.push_str("  movss "),
                    LtacArg::FltReg64(_v) => line.push_str("  movsd "),
                    _ => line.push_str("  mov "),
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
                match &code.arg2 {
                    LtacArg::I32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::U32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::I64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::U64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::F32(_v) => line.push_str("  movss DWORD PTR "),
                    LtacArg::F64(_v) => line.push_str("  movsd QWORD PTR "),
                    LtacArg::FltReg(_v) => line.push_str("  movss "),
                    LtacArg::FltReg64(_v) => line.push_str("  movsd "),
                    _ => line.push_str("  mov "),
                }
                line.push_str("[r15], ");
            } else {
                match code.arg2 {
                    LtacArg::I32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::U32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::I64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::U64(_v) => line.push_str("  mov QWORD PTR "),
                    _ => line.push_str("  mov "),
                }
                
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("], ");
            }
        },
        
        _ => {},
    }
    
    match &code.arg2 {
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
        
        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8 => line.push_str("eax"),
        LtacArg::RetRegI16 | LtacArg::RetRegU16 => line.push_str("eax"),
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => line.push_str("eax"),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("rax"),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0"),
        
        LtacArg::Mem(pos) => {
            if code.instr_type == LtacType::MovOffImm || code.instr_type == LtacType::MovOffMem {
                match &code.arg1 {
                    LtacArg::Reg8(_p) => line.push_str("r15b"),
                    LtacArg::Reg16(_p) => line.push_str("r15w"),
                    LtacArg::Reg64(_p) => line.push_str("r15"),
                    LtacArg::FltReg(_p) | LtacArg::FltReg64(_p) => line.push_str("xmm1"),
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
        
        LtacArg::UByte(val) => {
            line.push_str(" BYTE PTR ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::I16(val) => {
            line.push_str(" WORD PTR ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::U16(val) => {
            line.push_str(" WORD PTR ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::I32(val) => line.push_str(&val.to_string()),
        LtacArg::U32(val) => line.push_str(&val.to_string()),
        
        LtacArg::I64(val) => line.push_str(&val.to_string()),
        LtacArg::U64(val) => line.push_str(&val.to_string()),
        
        LtacArg::F32(_v) | LtacArg::F64(_v) => {
            line.push_str("xmm2");
        },
        
        LtacArg::Ptr(_v) => {},
        LtacArg::PtrLcl(_v) => {},
    }
    
    line.push_str("\n");

    writer.write(&line.into_bytes())
        .expect("[AMD64_writer_instr] Write failed.");
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


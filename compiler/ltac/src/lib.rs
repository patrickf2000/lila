
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use parser::ltac::{LtacFile, LtacData, LtacDataType, LtacType, LtacInstr, LtacArg};

mod func;

use func::*;

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
        line.push_str(&data.name);
        
        match &data.data_type {
            LtacDataType::StringL => {
                line.push_str(" .string ");
                line.push_str("\"");
                line.push_str(&data.val);
                line.push_str("\"");
            },
            
            LtacDataType::FloatL => {
                line.push_str(" .float ");
                line.push_str(&data.val);
            },
            
            LtacDataType::DoubleL => {
                line.push_str(" .double ");
                line.push_str(&data.val);
            },
        }
        
        line.push_str("\n");
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[_data] Write failed in .data");
}

// Write the code section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[LTAC_code] Write failed");

    for code in code.iter() {
        match &code.instr_type {
            LtacType::Extern => ltac_build_extern(writer, code),
            LtacType::Label => ltac_build_label(writer, code),
            LtacType::Func => ltac_build_func(writer, code),
            LtacType::Ret => ltac_build_ret(writer),
            
            LtacType::LdArgI8 => ltac_build_ldarg(writer, code),
            LtacType::LdArgI32 => ltac_build_ldarg(writer, code),
            LtacType::LdArgF32 => ltac_build_ldarg(writer, code),
            LtacType::LdArgF64 => ltac_build_ldarg(writer, code),
            LtacType::LdArgPtr => ltac_build_ldarg(writer, code),
            
            LtacType::Mov => ltac_build_instr(writer, code),
            LtacType::MovB => ltac_build_instr(writer, code),
            LtacType::MovUB => ltac_build_instr(writer, code),
            LtacType::MovW => ltac_build_instr(writer, code),
            LtacType::MovF32 => ltac_build_instr(writer, code),
            LtacType::MovF64 => ltac_build_instr(writer, code),
            LtacType::MovOffImm => ltac_build_instr(writer, code),
            LtacType::MovOffMem => ltac_build_instr(writer, code),
            LtacType::MovI32Vec => ltac_build_instr(writer, code),
            
            LtacType::PushArg => ltac_build_instr(writer, code),
            LtacType::KPushArg => ltac_build_instr(writer, code),
            LtacType::Call => ltac_build_call(writer, code),
            LtacType::Syscall => ltac_build_cmd(writer, code),
            
            LtacType::I8Cmp => ltac_build_instr(writer, code),
            LtacType::I32Cmp => ltac_build_instr(writer, code),
            LtacType::F32Cmp => ltac_build_instr(writer, code),
            LtacType::F64Cmp => ltac_build_instr(writer, code),
            LtacType::StrCmp => ltac_build_strcmp(writer),
            
            LtacType::Br => ltac_build_jump(writer, code),
            LtacType::Be => ltac_build_jump(writer, code),
            LtacType::Bne => ltac_build_jump(writer, code),
            LtacType::Bl => ltac_build_jump(writer, code),
            LtacType::Ble => ltac_build_jump(writer, code),
            LtacType::Bfl => ltac_build_jump(writer, code),
            LtacType::Bfle => ltac_build_jump(writer, code),
            LtacType::Bg => ltac_build_jump(writer, code),
            LtacType::Bge => ltac_build_jump(writer, code),
            LtacType::Bfg => ltac_build_jump(writer, code),
            LtacType::Bfge => ltac_build_jump(writer, code),
            
            LtacType::BAdd => ltac_build_instr(writer, code),
            LtacType::BSub => ltac_build_instr(writer, code),
            LtacType::BMul => ltac_build_instr(writer, code),
            LtacType::BDiv => ltac_build_instr(writer, code),
            LtacType::BMod => ltac_build_instr(writer, code),
            
            LtacType::BAnd => ltac_build_instr(writer, code),
            LtacType::BOr => ltac_build_instr(writer, code),
            LtacType::BXor => ltac_build_instr(writer, code),
            LtacType::BLsh => ltac_build_instr(writer, code),
            LtacType::BRsh => ltac_build_instr(writer, code),
            
            LtacType::I32Add => ltac_build_instr(writer, code),
            LtacType::I32Sub => ltac_build_instr(writer, code),
            LtacType::I32Mul => ltac_build_instr(writer, code),
            LtacType::I32Div => ltac_build_instr(writer, code),
            LtacType::I32Mod => ltac_build_instr(writer, code),
            
            LtacType::I32And => ltac_build_instr(writer, code),
            LtacType::I32Or => ltac_build_instr(writer, code),
            LtacType::I32Xor => ltac_build_instr(writer, code),
            LtacType::I32Lsh => ltac_build_instr(writer, code),
            LtacType::I32Rsh => ltac_build_instr(writer, code),
            
            LtacType::I32VAdd => ltac_build_instr(writer, code),
            
            LtacType::F32Add => ltac_build_instr(writer, code),
            LtacType::F32Sub => ltac_build_instr(writer, code),
            LtacType::F32Mul => ltac_build_instr(writer, code),
            LtacType::F32Div => ltac_build_instr(writer, code),
            
            LtacType::F64Add => ltac_build_instr(writer, code),
            LtacType::F64Sub => ltac_build_instr(writer, code),
            LtacType::F64Mul => ltac_build_instr(writer, code),
            LtacType::F64Div => ltac_build_instr(writer, code),
            
            // These are intrinsics if you will; they should never get down to a code generation layer
            LtacType::Exit => ltac_build_cmd(writer, code),
            LtacType::Malloc => ltac_build_cmd(writer, code),
            LtacType::Free => ltac_build_cmd(writer, code),
            
            // These are specific to RISC machines
            // Load instructions
            LtacType::Ld => ltac_build_instr(writer, code),
            LtacType::LdB => ltac_build_instr(writer, code),
            LtacType::LdUB => ltac_build_instr(writer, code),
            LtacType::LdW => ltac_build_instr(writer, code),
            
            //Store instructions
            LtacType::Str => ltac_build_instr(writer, code),
            LtacType::StrB => ltac_build_instr(writer, code),
            LtacType::StrUB => ltac_build_instr(writer, code),
            LtacType::StrW => ltac_build_instr(writer, code),
            LtacType::StrPtr => ltac_build_instr(writer, code),
        }
    }
}

// Commands (may or may not map directly to an instruction)
fn ltac_build_cmd(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();

    match &code.instr_type {
        LtacType::Exit => line.push_str("  exit"),
        LtacType::Malloc => line.push_str("  malloc"),
        LtacType::Free => line.push_str("  free"),
        LtacType::Syscall => line.push_str("  syscall"),
        
        _ => {},
    }
    
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[LTAC_build_intrinsics] Write failed.");
}

// String comparisons
fn ltac_build_strcmp(writer : &mut BufWriter<File>) {
    writer.write(b"  str.cmp")
        .expect("[LTAC_build_strcmp] Write failed.");
}

// Builds the jump instructions
fn ltac_build_jump(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    match &code.instr_type {
        LtacType::Br => line.push_str("  br "),
        LtacType::Be => line.push_str("  be "),
        LtacType::Bne => line.push_str("  bne "),
        LtacType::Bl => line.push_str("  bl "),
        LtacType::Ble => line.push_str("  ble "),
        LtacType::Bfl => line.push_str("  bfl "),
        LtacType::Bfle => line.push_str("  bfle "),
        LtacType::Bg => line.push_str("  bg "),
        LtacType::Bge => line.push_str("  bge "),
        LtacType::Bfg => line.push_str("  bfg "),
        LtacType::Bfge => line.push_str("  bfge "),
        
        _ => {},
    }
    
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[LTAC_build_jump] Write failed.");
}

// Builds common instructions
fn ltac_build_instr(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    match &code.instr_type {
        // Move instructions
        LtacType::Mov => line.push_str("  mov "),
        LtacType::MovB => line.push_str("  mov.b "),
        LtacType::MovUB => line.push_str("  mov.ub "),
        LtacType::MovW => line.push_str("  mov.w "),
        LtacType::MovF32 => line.push_str("  mov.f32 "),
        LtacType::MovF64 => line.push_str("  mov.f64 "),
        LtacType::MovOffImm => line.push_str("  mov.imm "),
        LtacType::MovOffMem => line.push_str("  mov.mem "),
        LtacType::MovI32Vec => line.push_str("  mov.i32.vec "),
        
        // Load instructions
        LtacType::Ld => line.push_str("  i32.ld "),
        LtacType::LdB => line.push_str("  i8.ld "),
        LtacType::LdUB => line.push_str("  u8.ld "),
        LtacType::LdW => line.push_str("  i16.ld "),
        
        //Store instructions
        LtacType::Str => line.push_str("  i32.str "),
        LtacType::StrB => line.push_str("  i8.str "),
        LtacType::StrUB => line.push_str("  u8.str "),
        LtacType::StrW => line.push_str("  i16.str "),
        LtacType::StrPtr => line.push_str("  ptr.str "),
        
        // Byte (i8) operations
        LtacType::BAdd => line.push_str("  i8.add "),
        LtacType::BSub => line.push_str("  i8.sub "),
        LtacType::BMul => line.push_str("  i8.mul "),
        LtacType::BDiv => line.push_str("  i8.div "),
        LtacType::BMod => line.push_str("  i8.mod "),
        
        LtacType::BAnd => line.push_str("  i8.and "),
        LtacType::BOr => line.push_str("  i8.or "),
        LtacType::BXor => line.push_str("  i8.xor "),
        LtacType::BLsh => line.push_str("  i8.lsh "),
        LtacType::BRsh => line.push_str("  i8.rsh "),
        
        // Integer (i32) operations
        LtacType::I32Add => line.push_str("  i32.add "),
        LtacType::I32Sub => line.push_str("  i32.sub "),
        LtacType::I32Mul => line.push_str("  i32.mul "),
        LtacType::I32Div => line.push_str("  i32.div "),
        LtacType::I32Mod => line.push_str("  i32.mod "),
        
        LtacType::I32And => line.push_str("  i32.and "),
        LtacType::I32Or => line.push_str("  i32.or "),
        LtacType::I32Xor => line.push_str("  i32.xor "),
        LtacType::I32Lsh => line.push_str("  i32.lsh "),
        LtacType::I32Rsh => line.push_str("  i32.rsh "),
        
        // Integer (i32) vector operations
        LtacType::I32VAdd => line.push_str("  i32.vadd "),
        
        // Single-precision float operations
        LtacType::F32Add => line.push_str("  f32.add "),
        LtacType::F32Sub => line.push_str("  f32.sub "),
        LtacType::F32Mul => line.push_str("  f32.mul "),
        LtacType::F32Div => line.push_str("  f32.div "),
        
        // Double-precision float operations
        LtacType::F64Add => line.push_str("  f64.add "),
        LtacType::F64Sub => line.push_str("  f64.sub "),
        LtacType::F64Mul => line.push_str("  f64.mul "),
        LtacType::F64Div => line.push_str("  f64.div "),
        
        // Comparisons
        LtacType::I8Cmp => line.push_str("  i8.cmp "),
        LtacType::I32Cmp => line.push_str("  i32.cmp "),
        LtacType::F32Cmp => line.push_str("  f32.cmp "),
        LtacType::F64Cmp => line.push_str("  f64.cmp "),
        
        // Argument push
        LtacType::PushArg => line.push_str("  pusharg "),
        LtacType::KPushArg => line.push_str("  kpusharg "),
        
        _ => {},
    }
    
    match &code.arg1_type {
            LtacArg::Empty => line.push_str(" "),
            
            LtacArg::Reg8(val) => {
                line.push_str("i8.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::Reg16(val) => {
                line.push_str("i16.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::Reg32(val) => {
                line.push_str("i32.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::Reg64(val) => {
                line.push_str("i64.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::FltReg(val) => {
                line.push_str("f32.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::FltReg64(val) => {
                line.push_str("f64.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::RetRegI32 => line.push_str("i32.ret"),
            LtacArg::RetRegI64 => line.push_str("i64.ret"),
            LtacArg::RetRegF32 => line.push_str("f32.ret"),
            LtacArg::RetRegF64 => line.push_str("f64.ret"),
            
            LtacArg::Mem(val) => {
                if code.arg1_offset > 0 && code.arg1_offset_size > 0 {
                    line.push_str("[bp-");
                    line.push_str(&val.to_string());
                    line.push_str("+(");
                    line.push_str(&code.arg1_offset.to_string());
                    line.push_str("*");
                    line.push_str(&code.arg1_offset_size.to_string());
                    line.push_str(")]");
                } else if code.arg1_offset > 0 {
                    line.push_str("[bp-");
                    line.push_str(&val.to_string());
                    line.push_str("+");
                    line.push_str(&code.arg1_offset.to_string());
                    line.push_str("]");
                } else {
                    line.push_str("[bp-");
                    line.push_str(&val.to_string());
                    line.push_str("]");
                }
            },
            
            LtacArg::Byte(val) => line.push_str(&val.to_string()),
            LtacArg::UByte(val) => line.push_str(&val.to_string()),
            LtacArg::I16(val) => line.push_str(&val.to_string()),
            LtacArg::I32(val) => line.push_str(&val.to_string()),
            LtacArg::F32(ref val) => line.push_str(&val.to_string()),
            LtacArg::F64(ref val) => line.push_str(&val.to_string()),
            
            LtacArg::Ptr(val) => {
                line.push_str("[bp-");
                line.push_str(&val.to_string());
                line.push_str("]");
            },
            
            LtacArg::PtrLcl(val) => line.push_str(&val.to_string()),
        }
        
        match &code.arg2_type {
            LtacArg::Empty => line.push_str(""),
            
            LtacArg::Reg8(val) => {
                line.push_str(", i8.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::Reg16(val) => {
                line.push_str(", i16.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::Reg32(val) => {
                line.push_str(", i32.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::Reg64(val) => {
                line.push_str(", i64.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::FltReg(val) => {
                line.push_str(", f32.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::FltReg64(val) => {
                line.push_str(", f64.r");
                line.push_str(&val.to_string());
            },
            
            LtacArg::RetRegI32 => line.push_str(", i32.ret"),
            LtacArg::RetRegI64 => line.push_str(", i64.ret"),
            LtacArg::RetRegF32 => line.push_str(", f32.ret"),
            LtacArg::RetRegF64 => line.push_str(", f64.ret"),
            
            LtacArg::Mem(val) => {
                if code.arg2_offset > 0 && code.arg2_offset_size > 0 {
                    line.push_str(", [bp-");
                    line.push_str(&val.to_string());
                    line.push_str("+(");
                    line.push_str(&code.arg2_offset.to_string());
                    line.push_str("*");
                    line.push_str(&code.arg2_offset_size.to_string());
                    line.push_str(")]");
                } else if code.arg2_offset > 0 {
                    line.push_str(", [bp-");
                    line.push_str(&val.to_string());
                    line.push_str("+");
                    line.push_str(&code.arg2_offset.to_string());
                    line.push_str("]");
                } else {
                    line.push_str(", [bp-");
                    line.push_str(&val.to_string());
                    line.push_str("]");
                }
            },
            
            LtacArg::Byte(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::UByte(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::I16(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::I32(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::F32(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::F64(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::Ptr(val) => {
                line.push_str(", [bp-");
                line.push_str(&val.to_string());
                line.push_str("]");
            },
            
            LtacArg::PtrLcl(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
        }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[_build_label] Write failed.");
}


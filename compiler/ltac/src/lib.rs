
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
            
            LtacType::LdArgI8 | LtacType::LdArgU8 => ltac_build_ldarg(writer, code),
            LtacType::LdArgI16 | LtacType::LdArgU16 => ltac_build_ldarg(writer, code),
            LtacType::LdArgI32 | LtacType::LdArgU32 => ltac_build_ldarg(writer, code),
            LtacType::LdArgI64 | LtacType::LdArgU64 => ltac_build_ldarg(writer, code),
            LtacType::LdArgF32 => ltac_build_ldarg(writer, code),
            LtacType::LdArgF64 => ltac_build_ldarg(writer, code),
            LtacType::LdArgPtr => ltac_build_ldarg(writer, code),
            
            LtacType::Call => ltac_build_call(writer, code),
            LtacType::Syscall => ltac_build_cmd(writer, code),
            
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
            
            // These are intrinsics if you will; they should never get down to a code generation layer
            LtacType::Exit => ltac_build_cmd(writer, code),
            LtacType::Malloc => ltac_build_cmd(writer, code),
            LtacType::Free => ltac_build_cmd(writer, code),
            
            _ => ltac_build_instr(writer, code),
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
        LtacType::MovB => line.push_str("  mov.b "),
        LtacType::MovUB => line.push_str("  mov.ub "),
        LtacType::MovW => line.push_str("  mov.w "),
        LtacType::MovUW => line.push_str("  mov.uw "),
        LtacType::Mov => line.push_str("  mov "),
        LtacType::MovU => line.push_str("  mov.u "),
        LtacType::MovQ => line.push_str("  mov.q "),
        LtacType::MovUQ => line.push_str("  mov.uq "),
        LtacType::MovF32 => line.push_str("  mov.f32 "),
        LtacType::MovF64 => line.push_str("  mov.f64 "),
        LtacType::MovI32Vec => line.push_str("  mov.i32.vec "),
        
        // Push and pop
        LtacType::Push => line.push_str("  push "),
        LtacType::Pop => line.push_str("  pop "),
        
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
        LtacType::I8Add => line.push_str("  i8.add "),
        LtacType::I8Sub => line.push_str("  i8.sub "),
        LtacType::I8Mul => line.push_str("  i8.mul "),
        LtacType::I8Div => line.push_str("  i8.div "),
        LtacType::I8Mod => line.push_str("  i8.mod "),
        
        // Unsigned byte (u8) operations
        LtacType::U8Add => line.push_str("  u8.add "),
        LtacType::U8Mul => line.push_str("  u8.mul "),
        LtacType::U8Div => line.push_str("  u8.div "),
        LtacType::U8Mod => line.push_str("  u8.mod "),
        
        // Byte bitwise operations
        LtacType::BAnd => line.push_str("  b.and "),
        LtacType::BOr => line.push_str("  b.or "),
        LtacType::BXor => line.push_str("  b.xor "),
        LtacType::BLsh => line.push_str("  b.lsh "),
        LtacType::BRsh => line.push_str("  b.rsh "),
        
        // Signed short (i16) operations
        LtacType::I16Add => line.push_str("  i16.add "),
        LtacType::I16Sub => line.push_str("  i16.sub "),
        LtacType::I16Mul => line.push_str("  i16.mul "),
        LtacType::I16Div => line.push_str("  i16.div "),
        LtacType::I16Mod => line.push_str("  i16.mod "),
        
        // Unsigned short (u16) operations
        LtacType::U16Add => line.push_str("  u16.add "),
        LtacType::U16Mul => line.push_str("  u16.mul "),
        LtacType::U16Div => line.push_str("  u16.div "),
        LtacType::U16Mod => line.push_str("  u16.mod "),
        
        // Short bitwise operations
        LtacType::WAnd => line.push_str("  w.and "),
        LtacType::WOr => line.push_str("  w.or "),
        LtacType::WXor => line.push_str("  w.xor "),
        LtacType::WLsh => line.push_str("  w.lsh "),
        LtacType::WRsh => line.push_str("  w.rsh "),
        
        // Integer (i32) operations
        LtacType::I32Add => line.push_str("  i32.add "),
        LtacType::I32Sub => line.push_str("  i32.sub "),
        LtacType::I32Mul => line.push_str("  i32.mul "),
        LtacType::I32Div => line.push_str("  i32.div "),
        LtacType::I32Mod => line.push_str("  i32.mod "),
        
        // Unsigned integer (u32) operations
        LtacType::U32Add => line.push_str("  u32.add "),
        LtacType::U32Mul => line.push_str("  u32.mul "),
        LtacType::U32Div => line.push_str("  u32.div "),
        LtacType::U32Mod => line.push_str("  u32.mod "),
        
        // Integer bitwise operations
        LtacType::I32And => line.push_str("  i32.and "),
        LtacType::I32Or => line.push_str("  i32.or "),
        LtacType::I32Xor => line.push_str("  i32.xor "),
        LtacType::I32Lsh => line.push_str("  i32.lsh "),
        LtacType::I32Rsh => line.push_str("  i32.rsh "),
        
        // Integer (i32) vector operations
        LtacType::I32VAdd => line.push_str("  i32.vadd "),
        
        // Signed 64-bit integer math operations
        LtacType::I64Add => line.push_str("  i64.add "),
        LtacType::I64Sub => line.push_str("  i64.sub "),
        LtacType::I64Mul => line.push_str("  i64.mul "),
        LtacType::I64Div => line.push_str("  i64.div "),
        LtacType::I64Mod => line.push_str("  i64.mod "),
        
        // Signed 64-bit integer math operations
        LtacType::U64Add => line.push_str("  u64.add "),
        LtacType::U64Mul => line.push_str("  u64.mul "),
        LtacType::U64Div => line.push_str("  u64.div "),
        LtacType::U64Mod => line.push_str("  u64.mod "),
        
        // 64-bit integer bitwise operations
        LtacType::I64And => line.push_str("  i64.and "),
        LtacType::I64Or => line.push_str("  i64.or "),
        LtacType::I64Xor => line.push_str("  i64.xor "),
        LtacType::I64Lsh => line.push_str("  i64.lsh "),
        LtacType::I64Rsh => line.push_str("  i64.rsh "),
        
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
        LtacType::U8Cmp => line.push_str("  u8.cmp "),
        LtacType::I16Cmp => line.push_str("  i16.cmp "),
        LtacType::U16Cmp => line.push_str("  u16.cmp "),
        LtacType::I32Cmp => line.push_str("  i32.cmp "),
        LtacType::U32Cmp => line.push_str("  u32.cmp "),
        LtacType::I64Cmp => line.push_str("  i64.cmp "),
        LtacType::U64Cmp => line.push_str("  u64.cmp "),
        LtacType::F32Cmp => line.push_str("  f32.cmp "),
        LtacType::F64Cmp => line.push_str("  f64.cmp "),
        
        // Argument push
        LtacType::PushArg => line.push_str("  pusharg "),
        LtacType::KPushArg => line.push_str("  kpusharg "),
        
        _ => {},
    }
    
    match &code.arg1 {
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
            
            LtacArg::RetRegI8 => line.push_str("i8.ret"),
            LtacArg::RetRegU8 => line.push_str("u8.ret"),
            LtacArg::RetRegI16 => line.push_str("i16.ret"),
            LtacArg::RetRegU16 => line.push_str("u16.ret"),
            LtacArg::RetRegI32 => line.push_str("i32.ret"),
            LtacArg::RetRegU32 => line.push_str("u32.ret"),
            LtacArg::RetRegI64 => line.push_str("i64.ret"),
            LtacArg::RetRegU64 => line.push_str("u64.ret"),
            LtacArg::RetRegF32 => line.push_str("f32.ret"),
            LtacArg::RetRegF64 => line.push_str("f64.ret"),
            
            LtacArg::Mem(val) => {
                line.push_str("[bp-");
                line.push_str(&val.to_string());
                line.push_str("]");
            },
            
            LtacArg::MemOffsetImm(pos, offset_pos) => {
                line.push_str("[bp-");
                line.push_str(&pos.to_string());
                line.push_str("+");
                line.push_str(&offset_pos.to_string());
                line.push_str("]");
            },
            
            LtacArg::MemOffsetMem(pos, offset_pos, offset_size) => {
                line.push_str("[bp-");
                line.push_str(&pos.to_string());
                line.push_str("+(");
                line.push_str(&offset_pos.to_string());
                line.push_str("*");
                line.push_str(&offset_size.to_string());
                line.push_str(")]");
            },
            
            LtacArg::MemOffsetReg(pos, reg, offset_size) => {
                line.push_str("[bp-");
                line.push_str(&pos.to_string());
                line.push_str("+(r");
                line.push_str(&reg.to_string());
                line.push_str("*");
                line.push_str(&offset_size.to_string());
                line.push_str(")]");
            },
            
            LtacArg::Byte(val) => line.push_str(&val.to_string()),
            LtacArg::UByte(val) => line.push_str(&val.to_string()),
            LtacArg::I16(val) => line.push_str(&val.to_string()),
            LtacArg::U16(val) => line.push_str(&val.to_string()),
            LtacArg::I32(val) => line.push_str(&val.to_string()),
            LtacArg::U32(val) => line.push_str(&val.to_string()),
            LtacArg::I64(val) => line.push_str(&val.to_string()),
            LtacArg::U64(val) => line.push_str(&val.to_string()),
            LtacArg::F32(ref val) => line.push_str(&val.to_string()),
            LtacArg::F64(ref val) => line.push_str(&val.to_string()),
            
            LtacArg::Ptr(val) => {
                line.push_str("[bp-");
                line.push_str(&val.to_string());
                line.push_str("]");
            },
            
            LtacArg::PtrLcl(val) => line.push_str(&val.to_string()),
        }
        
        match &code.arg2 {
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
            
            LtacArg::RetRegI8 => line.push_str(", i8.ret"),
            LtacArg::RetRegU8 => line.push_str(", u8.ret"),
            LtacArg::RetRegI16 => line.push_str(", i16.ret"),
            LtacArg::RetRegU16 => line.push_str(", u16.ret"),
            LtacArg::RetRegI32 => line.push_str(", i32.ret"),
            LtacArg::RetRegU32 => line.push_str(", u32.ret"),
            LtacArg::RetRegI64 => line.push_str(", i64.ret"),
            LtacArg::RetRegU64 => line.push_str(", u64.ret"),
            LtacArg::RetRegF32 => line.push_str(", f32.ret"),
            LtacArg::RetRegF64 => line.push_str(", f64.ret"),
            
            LtacArg::Mem(val) => {
                line.push_str(", [bp-");
                line.push_str(&val.to_string());
                line.push_str("]");
            },
            
            LtacArg::MemOffsetImm(pos, offset_pos) => {
                line.push_str(", [bp-");
                line.push_str(&pos.to_string());
                line.push_str("+");
                line.push_str(&offset_pos.to_string());
                line.push_str("]");
            },
            
            LtacArg::MemOffsetMem(pos, offset_pos, offset_size) => {
                line.push_str(", [bp-");
                line.push_str(&pos.to_string());
                line.push_str("+(");
                line.push_str(&offset_pos.to_string());
                line.push_str("*");
                line.push_str(&offset_size.to_string());
                line.push_str(")]");
            },
            
            LtacArg::MemOffsetReg(pos, reg, offset_size) => {
                line.push_str(", [bp-");
                line.push_str(&pos.to_string());
                line.push_str("+(r");
                line.push_str(&reg.to_string());
                line.push_str("*");
                line.push_str(&offset_size.to_string());
                line.push_str(")]");
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
            
            LtacArg::U16(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::I32(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::U32(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::I64(val) => {
                line.push_str(", ");
                line.push_str(&val.to_string());
            },
            
            LtacArg::U64(val) => {
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


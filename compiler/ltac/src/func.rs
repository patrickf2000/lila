
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use parser::ltac::{LtacType, LtacInstr};

pub fn ltac_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "extern ".to_string();
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[LTAC_build_extern] Write failed.");
}

pub fn ltac_build_label(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "lbl ".to_string();
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[LTAC_build_label] Write failed.");
}

pub fn ltac_build_func(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "\nfunc ".to_string();
    line.push_str(&code.name);
    line.push_str("\n  setup ");
    line.push_str(&code.arg1_val.to_string());
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[LTAC_build_func] Write failed.");
}

pub fn ltac_build_ret(writer : &mut BufWriter<File>) {
    writer.write(b"\n  ret\n\n")
        .expect("[LTAC_build_ret] Write failed.");
}

pub fn ltac_build_ldarg(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  ".to_string();
    
    match &code.instr_type {
        LtacType::LdArgI8 => line.push_str("i8.ldarg"),
        LtacType::LdArgU8 => line.push_str("u8.ldarg"),
        LtacType::LdArgI16 => line.push_str("i16.ldarg"),
        LtacType::LdArgU16 => line.push_str("u16.ldarg"),
        LtacType::LdArgI32 => line.push_str("i32.ldarg"),
        LtacType::LdArgU32 => line.push_str("u32.ldarg"),
        LtacType::LdArgI64 => line.push_str("i64.ldarg"),
        LtacType::LdArgU64 => line.push_str("u64.ldarg"),
        LtacType::LdArgF32 => line.push_str("f32.ldarg"),
        LtacType::LdArgF64 => line.push_str("f64.ldarg"),
        LtacType::LdArgPtr => line.push_str("ptr.ldarg"),
        _ => {},
    }
    
    line.push_str(" [bp-");
    line.push_str(&code.arg1_val.to_string());
    line.push_str("], r");
    line.push_str(&code.arg2_val.to_string());
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[LTAC_build_ldarg] Write failed.");
}

pub fn ltac_build_call(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  call ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[LTAC_build_call] Write failed.");
}



use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use parser::ltac::{LtacFile, LtacData, /*LtacDataType,*/ LtacType, LtacInstr/*, LtacArg*/};

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
fn write_data(_writer : &mut BufWriter<File>, _data : &Vec<LtacData>) {

}

// Write the code section
fn write_code(writer : &mut BufWriter<File>, code : &Vec<LtacInstr>) {
    let line = ".text\n".to_string();
    writer.write(&line.into_bytes())
        .expect("[_code] Write failed");

    for code in code.iter() {
        match &code.instr_type {
            LtacType::Extern => {},
            LtacType::Label => {},
            LtacType::Func => {},
            LtacType::Ret => {},
            
            LtacType::LdArgI8 => {},
            LtacType::LdArgI32 => {},
            LtacType::LdArgF32 => {},
            LtacType::LdArgF64 => {},
            LtacType::LdArgPtr => {},
            
            LtacType::Mov => {},
            LtacType::MovB => {},
            LtacType::MovUB => {},
            LtacType::MovW => {},
            LtacType::MovF32 => {},
            LtacType::MovF64 => {},
            LtacType::MovOffImm => {},
            LtacType::MovOffMem => {},
            LtacType::MovI32Vec => {},
            
            LtacType::PushArg => {},
            LtacType::KPushArg => {},
            LtacType::Call => {},
            LtacType::Syscall => {},
            
            LtacType::I8Cmp => {},
            LtacType::I32Cmp => {},
            LtacType::F32Cmp => {},
            LtacType::F64Cmp => {},
            LtacType::StrCmp => {},
            
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
            
            LtacType::BAdd => {},
            LtacType::BSub => {},
            LtacType::BMul => {},
            LtacType::BDiv => {},
            LtacType::BMod => {},
            
            LtacType::BAnd => {},
            LtacType::BOr => {},
            LtacType::BXor => {},
            LtacType::BLsh => {},
            LtacType::BRsh => {},
            
            LtacType::I32Add => {},
            LtacType::I32Sub => {},
            LtacType::I32Mul => {},
            LtacType::I32Div => {},
            LtacType::I32Mod => {},
            
            LtacType::I32And => {},
            LtacType::I32Or => {},
            LtacType::I32Xor => {},
            LtacType::I32Lsh => {},
            LtacType::I32Rsh => {},
            
            LtacType::I32VAdd => {},
            
            LtacType::F32Add => {},
            LtacType::F32Sub => {},
            LtacType::F32Mul => {},
            LtacType::F32Div => {},
            
            LtacType::F64Add => {},
            LtacType::F64Sub => {},
            LtacType::F64Mul => {},
            LtacType::F64Div => {},
            
            // These are intrinsics if you will; they should never get down to a code generation layer
            LtacType::Exit => {},
            LtacType::Malloc => {},
            LtacType::Free => {},
            
            // These are specific to RISC machines
            // Load instructions
            LtacType::Ld => {},
            LtacType::LdB => {},
            LtacType::LdUB => {},
            LtacType::LdW => {},
            LtacType::Str => {},
            LtacType::StrB => {},
            LtacType::StrUB => {},
            LtacType::StrW => {},
            LtacType::StrPtr => {},
        }
    }
}

/*fn ltac_build_label(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(&code.name);
    line.push_str(":\n");
    
    writer.write(&line.into_bytes())
        .expect("[_build_label] Write failed.");
}*/


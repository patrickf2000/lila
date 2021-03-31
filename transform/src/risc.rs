//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use crate::ltac;
use crate::ltac::{LtacFile, LtacType, LtacArg};

fn is_move(instr : &LtacType) -> bool {
    match instr {
        LtacType::MovB | LtacType::MovUB |
        LtacType::MovW | LtacType::MovUW |
        LtacType::Mov | LtacType::MovU |
        LtacType::MovQ | LtacType::MovUQ |
        LtacType::MovF32 | LtacType::MovF64
            => return true,
            
        _ => return false,
    }
}

fn has_mem(arg : &LtacArg) -> bool {
    match arg {
        LtacArg::Mem(_n) => return true,
        LtacArg::MemOffsetImm(_n1, _n2) => return true,
        LtacArg::MemOffsetMem(_n1, _n2, _n3) |
        LtacArg::MemOffsetReg(_n1, _n2, _n3) => return true,
        
        _ => return false,
    }
}

// Returns the proper load instruction for a given move
fn load_for_mov(instr : &LtacType) -> LtacType {
    match instr {
        LtacType::I8Add | LtacType::I8Sub | LtacType::I8Mul |
        LtacType::I8Div | LtacType::I8Mod |
        LtacType::I8Cmp => return LtacType::LdB,
        
        LtacType::I16Add | LtacType::I16Sub | LtacType::I16Mul |
        LtacType::I16Div | LtacType::I16Mod |
        LtacType::I16Cmp => return LtacType::LdW,
        
        LtacType::I64Add | LtacType::I64Sub | LtacType::I64Mul |
        LtacType::I64Div | LtacType::I64Mod => return LtacType::LdQ,
        
        LtacType::U64Add | LtacType::U64Mul |
        LtacType::U64Div | LtacType::U64Mod => return LtacType::LdUQ,
        
        LtacType::MovB => return LtacType::LdB,
        LtacType::MovUB => return LtacType::LdUB,
        LtacType::MovW => return LtacType::LdW,
        LtacType::MovUW => return LtacType::LdUW,
        LtacType::MovU => return LtacType::LdU,
        LtacType::MovQ => return LtacType::LdQ,
        LtacType::MovUQ => return LtacType::LdUQ,
        
        LtacType::F32Add | LtacType::F32Sub |
        LtacType::F32Mul | LtacType::F32Div => return LtacType::LdF32,
        
        LtacType::F64Add | LtacType::F64Sub |
        LtacType::F64Mul | LtacType::F64Div => return LtacType::LdF64,
        
        LtacType::MovF32 => return LtacType::LdF32,
        LtacType::MovF64 => return LtacType::LdF64,
        
        _ => return LtacType::Ld,
    }
}

// Returns the proper store instruction for a given move
fn store_for_mov(instr : &LtacType) -> LtacType {
    match instr {
        LtacType::MovB => return LtacType::StrB,
        LtacType::MovUB => return LtacType::StrUB,
        LtacType::MovW => return LtacType::StrW,
        LtacType::MovUW => return LtacType::StrUW,
        LtacType::MovU => return LtacType::StrU,
        LtacType::MovQ => return LtacType::StrQ,
        LtacType::MovUQ => return LtacType::StrUQ,
        LtacType::MovF32 => return LtacType::StrF32,
        LtacType::MovF64 => return LtacType::StrF64,
        _ => return LtacType::Str,
    }
}

// Returns a register for a given move statement
fn reg_for_mov(instr : &LtacType, pos : i32) -> LtacArg {
    match instr {
        LtacType::LdB | LtacType::LdUB |
        LtacType::MovB | LtacType::MovUB => return LtacArg::Reg8(pos),
        LtacType::LdW | LtacType::LdUW |
        LtacType::MovW | LtacType::MovUW => return LtacArg::Reg16(pos),
        LtacType::LdQ | LtacType::LdUQ |
        LtacType::MovQ | LtacType::MovUQ => return LtacArg::Reg64(pos),
        LtacType::LdF32 | LtacType::MovF32 => return LtacArg::FltReg(pos),
        LtacType::LdF64 | LtacType::MovF64 => return LtacArg::FltReg64(pos),
        _ => return LtacArg::Reg32(pos),
    }
}

// The main RISC optimizer loop
pub fn risc_optimize(file : &LtacFile) -> Result<LtacFile, ()> {
    let mut file2 = LtacFile {
        name : file.name.clone(),
        data : file.data.clone(),
        code : Vec::new(),
    };
    
    let code = file.code.clone();
    
    for line in code.iter() {
        let mut instr2 = line.clone();
        
        if is_move(&line.instr_type) {
            if has_mem(&line.arg1) {
                let instr_type = store_for_mov(&line.instr_type);
                let mut store = ltac::create_instr(instr_type);
                store.arg1 = instr2.arg1.clone();
                store.arg2 = reg_for_mov(&line.instr_type, 3);
                
                instr2.arg1 = reg_for_mov(&line.instr_type, 3);
                
                file2.code.push(instr2);
                file2.code.push(store);
            } else if has_mem(&line.arg2) {
                let instr_type = load_for_mov(&line.instr_type);
                let mut load = ltac::create_instr(instr_type);
                load.arg1 = instr2.arg2.clone();
                load.arg2 = reg_for_mov(&line.instr_type, 3);
                
                instr2.arg2 = reg_for_mov(&line.instr_type, 3);
                
                file2.code.push(load);
                file2.code.push(instr2);
            } else {
                file2.code.push(instr2);
            }
        } else {
            if has_mem(&line.arg2) && line.instr_type != LtacType::PushArg {
                let instr_type = load_for_mov(&line.instr_type);
                let mut load = ltac::create_instr(instr_type.clone());
                load.arg1 = instr2.arg2.clone();
                load.arg2 = reg_for_mov(&instr_type, 3);
                
                instr2.arg2 = reg_for_mov(&instr_type, 3);
                
                file2.code.push(load);
            }
            
            file2.code.push(instr2);
        }
    }
    
    Ok(file2)
}


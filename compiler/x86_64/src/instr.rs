
// This file is part of the Lila compiler
// Copyright (C) 2020-2021 Patrick Flynn
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::asm::*;

// Builds a branch (actually kinda called "jumps" in x86...)
pub fn amd64_build_jump(x86_code : &mut Vec<X86Instr>, code : &LtacInstr) {
    let instr_type : X86Type;
    
    match &code.instr_type {
        LtacType::Be => instr_type = X86Type::Je,
        LtacType::Bne => instr_type = X86Type::Jne,
        LtacType::Bl => instr_type = X86Type::Jl,
        LtacType::Ble => instr_type = X86Type::Jle,
        LtacType::Bfl => instr_type = X86Type::Jb,
        LtacType::Bfle => instr_type = X86Type::Jbe,
        LtacType::Bg => instr_type = X86Type::Jg,
        LtacType::Bge => instr_type = X86Type::Jge,
        LtacType::Bfg => instr_type = X86Type::Ja,
        LtacType::Bfge => instr_type = X86Type::Jae,
        _ => instr_type = X86Type::Jmp,
    }
    
    let mut instr = create_x86instr(instr_type);
    instr.name = code.name.clone();
    x86_code.push(instr);
}

// Builds a string comparison
pub fn amd64_build_strcmp(x86_code : &mut Vec<X86Instr>) {
    let mut instr2 = create_x86instr(X86Type::Call);
    instr2.name = "strcmp".to_string();
    x86_code.push(instr2.clone());
    
    instr2 = create_x86instr(X86Type::Cmp);
    instr2.arg1 = X86Arg::Reg32(X86Reg::RAX);
    instr2.arg2 = X86Arg::Imm32(1);
    x86_code.push(instr2);
}

fn amd64_build_offset_mem(x86_code : &mut Vec<X86Instr>, pos : i32, offset : i32, size : i32, _is_pic : bool) {
    // Load the variable
    // TODO: PIC
    let mut instr2 = create_x86instr(X86Type::Mov);
    instr2.arg1 = X86Arg::Reg32(X86Reg::R15);
    instr2.arg2 = X86Arg::DwordMem(X86Reg::RBP, offset);
    x86_code.push(instr2.clone());
    
    // Load the effective address
    //TODO: PIC
    instr2 = create_x86instr(X86Type::Lea);
    instr2.arg1 = X86Arg::Reg64(X86Reg::R14);
    instr2.arg2 = X86Arg::ScaleMem(0, X86Reg::R15, size);
    x86_code.push(instr2.clone());
    
    // Load the array
    // TODO: PIC
    instr2 = create_x86instr(X86Type::Mov);
    instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
    instr2.arg2 = X86Arg::QwordMem(X86Reg::RBP, pos);
    x86_code.push(instr2.clone());
    
    // Add to get the proper offset
    instr2 = create_x86instr(X86Type::Add);
    instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
    instr2.arg2 = X86Arg::Reg64(X86Reg::R14);
    x86_code.push(instr2.clone());
}

fn amd64_build_offset_reg(x86_code : &mut Vec<X86Instr>, pos : i32, reg : i32, size : i32, _is_pic : bool) {
    // Determine the right register
    let src_reg : X86Reg;
    
        match reg {
            0 => src_reg = X86Reg::RBX,
            1 => src_reg = X86Reg::RCX,
            2 => src_reg = X86Reg::R10,
            3 => src_reg = X86Reg::R11,
            4 => src_reg = X86Reg::R12,
            _ => src_reg = X86Reg::RAX,
        };
    
    // Load the effective address
    //TODO: PIC
    let mut instr2 = create_x86instr(X86Type::Lea);
    instr2.arg1 = X86Arg::Reg64(X86Reg::R14);
    instr2.arg2 = X86Arg::ScaleMem(0, src_reg, size);
    x86_code.push(instr2.clone());
    
    // Load the array
    // TODO: PIC
    instr2 = create_x86instr(X86Type::Mov);
    instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
    instr2.arg2 = X86Arg::QwordMem(X86Reg::RBP, pos);
    x86_code.push(instr2.clone());
    
    // Add to get the proper offset
    instr2 = create_x86instr(X86Type::Add);
    instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
    instr2.arg2 = X86Arg::Reg64(X86Reg::R14);
    x86_code.push(instr2.clone());
}

fn amd64_check_arg1(x86_code : &mut Vec<X86Instr>, arg1 : &LtacArg, offset : i32) -> X86Arg {
    // Store
    let mut instr2 = create_x86instr(X86Type::Mov);
    let arg2 : X86Arg;
    
    match &arg1 {
        LtacArg::Reg8(_p) => {
            instr2.arg1 = X86Arg::Reg8(X86Reg::R15);
            instr2.arg2 = X86Arg::BwordMem(X86Reg::R15, offset * -1);
            
            arg2 = X86Arg::Reg8(X86Reg::R15);
        }
        
        LtacArg::Reg16(_p) => {
            instr2.arg1 = X86Arg::Reg16(X86Reg::R15);
            instr2.arg2 = X86Arg::WordMem(X86Reg::R15, offset * -1);
            
            arg2 = X86Arg::Reg16(X86Reg::R15);
        }
        
        LtacArg::Reg64(_p) => {
            instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
            instr2.arg2 = X86Arg::QwordMem(X86Reg::R15, offset * -1);
            
            arg2 = X86Arg::Reg64(X86Reg::R15);
        },
        
        _ => {
            instr2.arg1 = X86Arg::Reg32(X86Reg::R15);
            instr2.arg2 = X86Arg::DwordMem(X86Reg::R15, offset * -1);
            
            arg2 = X86Arg::Reg32(X86Reg::R15);
        },
    }
    
    x86_code.push(instr2);
    arg2
}

// Many instructions have common syntax
pub fn amd64_build_instr(x86_code : &mut Vec<X86Instr>, code : &LtacInstr, is_pic : bool) {
    
    // The instruction
    // TODO: The unsigned multiplication should use "mul". This may require a separate function
    let mut instr : X86Instr;
    
    match &code.instr_type {
        LtacType::Mov | LtacType::MovU |
        LtacType::MovB | LtacType::MovUB |
        LtacType::MovW | LtacType::MovUW |
        LtacType::MovQ | LtacType::MovUQ => {
            /*match &code.arg2 {
                LtacArg::PtrLcl(ref val) if is_pic => {
                    line.push_str("  lea r15, ");
                    line.push_str(&val);
                    line.push_str("[rip]\n");
                },
                _ => {},
            }
            
            line.push_str("  mov ");*/
            instr = create_x86instr(X86Type::Mov);
        },
        
        LtacType::MovF32 => {
            match &code.arg1 {
                LtacArg::MemOffsetImm(_p, _o) => instr = create_x86instr(X86Type::Mov),
                LtacArg::MemOffsetMem(_p, _o, _s) |
                LtacArg::MemOffsetReg(_p, _o, _s) => instr = create_x86instr(X86Type::Mov),
                _ => instr = create_x86instr(X86Type::MovSS),
            }
        },
        LtacType::MovF64 => {
            match &code.arg1 {
                LtacArg::MemOffsetImm(_p, _o) => instr = create_x86instr(X86Type::Mov),
                LtacArg::MemOffsetMem(_p, _o, _s) |
                LtacArg::MemOffsetReg(_p, _o, _s) => instr = create_x86instr(X86Type::Mov),
                _ => instr = create_x86instr(X86Type::MovSD),
            }
        },
        
        LtacType::LdAddr => instr = create_x86instr(X86Type::Lea),
        
        LtacType::I8Add | LtacType::U8Add |
        LtacType::I16Add | LtacType::U16Add |
        LtacType::I32Add | LtacType::U32Add |
        LtacType::I64Add | LtacType::U64Add => instr = create_x86instr(X86Type::Add),
        
        LtacType::I8Sub | LtacType::I16Sub |
        LtacType::I32Sub  | LtacType::I64Sub => instr = create_x86instr(X86Type::Sub),
        
        LtacType::I16Mul | LtacType::I32Mul |
        LtacType::I64Mul => instr = create_x86instr(X86Type::IMul),
        LtacType::U16Mul | LtacType::U32Mul |
        LtacType::U64Mul => instr = create_x86instr(X86Type::IMul),     // TODO: Should be "mul"
        
        LtacType::F32Add => instr = create_x86instr(X86Type::AddSS),
        LtacType::F32Sub => instr = create_x86instr(X86Type::SubSS),
        LtacType::F32Mul => instr = create_x86instr(X86Type::MulSS),
        LtacType::F32Div => instr = create_x86instr(X86Type::DivSS),
        
        LtacType::F64Add => instr = create_x86instr(X86Type::AddSD),
        LtacType::F64Sub => instr = create_x86instr(X86Type::SubSD),
        LtacType::F64Mul => instr = create_x86instr(X86Type::MulSD),
        LtacType::F64Div => instr = create_x86instr(X86Type::DivSD),
        
        LtacType::BAnd | LtacType::WAnd |
        LtacType::I32And | LtacType::I64And => instr = create_x86instr(X86Type::And),
        LtacType::BOr | LtacType::WOr |
        LtacType::I32Or | LtacType::I64Or => instr = create_x86instr(X86Type::Or),
        LtacType::BXor | LtacType::WXor |
        LtacType::I32Xor | LtacType::I64Xor => instr = create_x86instr(X86Type::Xor),
        LtacType::BLsh | LtacType::WLsh |
        LtacType::I32Lsh | LtacType::I64Lsh => instr = create_x86instr(X86Type::Shl),
        LtacType::BRsh | LtacType::WRsh |
        LtacType::I32Rsh | LtacType::I64Rsh => instr = create_x86instr(X86Type::Shr),
        
        LtacType::I8Cmp | LtacType::U8Cmp |
        LtacType::I16Cmp | LtacType::U16Cmp |
        LtacType::I32Cmp | LtacType::U32Cmp |
        LtacType::I64Cmp | LtacType::U64Cmp => instr = create_x86instr(X86Type::Cmp),
        LtacType::F32Cmp => instr = create_x86instr(X86Type::Ucomiss),
        LtacType::F64Cmp => instr = create_x86instr(X86Type::Ucomisd),
        
        _ => instr = create_x86instr(X86Type::Nop),
    }

    // The arguments
    match &code.arg1 {
        
        LtacArg::Reg8(pos) => instr.arg1 = amd64_op_reg8(*pos),
        LtacArg::Reg16(pos) => instr.arg1 = amd64_op_reg16(*pos),
        LtacArg::Reg32(pos) => instr.arg1 = amd64_op_reg32(*pos),
        LtacArg::Reg64(pos) => instr.arg1 = amd64_op_reg64(*pos),
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8 => instr.arg1 = X86Arg::Reg8(X86Reg::RAX),
        LtacArg::RetRegI16 | LtacArg::RetRegU16 => instr.arg1 = X86Arg::Reg16(X86Reg::RAX),
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => instr.arg1 = X86Arg::Reg32(X86Reg::RAX),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => instr.arg1 = X86Arg::Reg64(X86Reg::RAX),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => instr.arg1 = X86Arg::Xmm(0),
        
        LtacArg::Mem(pos) => {
            match &code.arg2 {
                LtacArg::Byte(_v) => instr.arg1 = X86Arg::BwordMem(X86Reg::RBP, *pos),
                LtacArg::UByte(_v) => instr.arg1 = X86Arg::BwordMem(X86Reg::RBP, *pos),
                LtacArg::I16(_v) => instr.arg1 = X86Arg::WordMem(X86Reg::RBP, *pos),
                LtacArg::U16(_v) => instr.arg1 = X86Arg::WordMem(X86Reg::RBP, *pos),
                LtacArg::I32(_v) => instr.arg1 = X86Arg::DwordMem(X86Reg::RBP, *pos),
                LtacArg::U32(_v) => instr.arg1 = X86Arg::DwordMem(X86Reg::RBP, *pos),
                LtacArg::I64(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::RBP, *pos),
                LtacArg::U64(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::RBP, *pos),
                LtacArg::PtrLcl(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::RBP, *pos),
                LtacArg::Ptr(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::RBP, *pos),
                _ => instr.arg1 = X86Arg::Mem(X86Reg::RBP, *pos),
            }
            
            /*if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp], ");
            } else {*/
                /*line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("], ");*/
            //}
        },
        
        LtacArg::MemOffsetImm(pos, offset) => {
            // Load array
            // TODO: PIC
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
            instr2.arg2 = X86Arg::QwordMem(X86Reg::RBP, *pos);
            x86_code.push(instr2.clone());
            
            instr2 = create_x86instr(X86Type::Add);
            instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*offset);
            x86_code.push(instr2.clone());
            
            match &code.arg2 {
                LtacArg::Byte(_v) => instr.arg1 = X86Arg::BwordMem(X86Reg::R15, 0),
                LtacArg::UByte(_v) => instr.arg1 = X86Arg::BwordMem(X86Reg::R15, 0),
                LtacArg::I16(_v) => instr.arg1 = X86Arg::WordMem(X86Reg::R15, 0),
                LtacArg::U16(_v) => instr.arg1 = X86Arg::WordMem(X86Reg::R15, 0),
                LtacArg::I64(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::R15, 0),
                LtacArg::U64(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::R15, 0),
                LtacArg::Reg64(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::R15, 0),
                _ => instr.arg1 = X86Arg::DwordMem(X86Reg::R15, 0),
            };
        },
        
        LtacArg::MemOffsetMem(pos, offset, size) => {
            amd64_build_offset_mem(x86_code, *pos, *offset, *size, is_pic);
            
            // Now set up for the final move
            match &code.arg2 {
                LtacArg::Reg8(_v) => instr.arg1 = X86Arg::BwordMem(X86Reg::R15, 0),
                LtacArg::Reg16(_v) => instr.arg1 = X86Arg::WordMem(X86Reg::R15, 0),
                LtacArg::Byte(_v) => instr.arg1 = X86Arg::BwordMem(X86Reg::R15, 0),
                LtacArg::UByte(_v) => instr.arg1 = X86Arg::BwordMem(X86Reg::R15, 0),
                LtacArg::I16(_v) => instr.arg1 = X86Arg::WordMem(X86Reg::R15, 0),
                LtacArg::U16(_v) => instr.arg1 = X86Arg::WordMem(X86Reg::R15, 0),
                LtacArg::I64(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::R15, 0),
                LtacArg::U64(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::R15, 0),
                LtacArg::Reg64(_v) => instr.arg1 = X86Arg::QwordMem(X86Reg::R15, 0),
                _ => instr.arg1 = X86Arg::DwordMem(X86Reg::R15, 0),
            }
        },
        
        LtacArg::MemOffsetReg(pos, reg, size) => {
            amd64_build_offset_reg(x86_code, *pos, *reg, *size, is_pic);
            instr.arg1 = X86Arg::DwordMem(X86Reg::R15, 0);
        },
        
        _ => {},
    }
    
    // Build the second operand
    match &code.arg2 {
        LtacArg::Reg8(pos) => instr.arg2 = amd64_op_reg8(*pos),
        LtacArg::Reg16(pos) => instr.arg2 = amd64_op_reg16(*pos),
        LtacArg::Reg32(pos) => instr.arg2 = amd64_op_reg32(*pos),
        LtacArg::Reg64(pos) => instr.arg2 = amd64_op_reg64(*pos),
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8 => instr.arg2 = X86Arg::Reg8(X86Reg::RAX),
        LtacArg::RetRegI16 | LtacArg::RetRegU16 => instr.arg2 = X86Arg::Reg16(X86Reg::RAX),
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => instr.arg2 = X86Arg::Reg32(X86Reg::RAX),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => instr.arg2 = X86Arg::Reg64(X86Reg::RAX),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => instr.arg2 = X86Arg::Xmm(0),
        
        LtacArg::Mem(pos) => {
            /*if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]");
            } else {*/
                instr.arg2 = X86Arg::Mem(X86Reg::RBP, *pos);
            //}
        },
        
        LtacArg::MemOffsetImm(pos, offset) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
            instr2.arg2 = X86Arg::QwordMem(X86Reg::RBP, *pos);
            x86_code.push(instr2.clone());
            
            instr.arg2 = amd64_check_arg1(x86_code, &code.arg1, *offset);
        },
        
        LtacArg::MemOffsetMem(pos, offset, size) => {
            amd64_build_offset_mem(x86_code, *pos, *offset, *size, is_pic);
            instr.arg2 = amd64_check_arg1(x86_code, &code.arg1, 0);
        },
        
        LtacArg::MemOffsetReg(pos, reg, size) => {
            amd64_build_offset_reg(x86_code, *pos, *reg, *size, is_pic);
            instr.arg2 = amd64_check_arg1(x86_code, &code.arg1, 0);
        },
        
        LtacArg::Byte(val) => instr.arg2 = X86Arg::Imm32(*val as i32),
        LtacArg::UByte(val) => instr.arg2 = X86Arg::Imm32(*val as i32),
        
        LtacArg::I16(val) => instr.arg2 = X86Arg::Imm32(*val as i32),
        LtacArg::U16(val) => instr.arg2 = X86Arg::Imm32(*val as i32),
        
        LtacArg::I32(val) => instr.arg2 = X86Arg::Imm32(*val),
        LtacArg::U32(val) => instr.arg2 = X86Arg::Imm32(*val as i32),
        
        LtacArg::I64(val) => instr.arg2 = X86Arg::Imm64(*val),
        LtacArg::U64(val) => instr.arg2 = X86Arg::Imm64(*val as i64),
        
        LtacArg::PtrLcl(ref val) => {
            if is_pic {
                instr.arg2 = X86Arg::Reg64(X86Reg::R15);
            } else {
                instr.arg2 = X86Arg::LclMem(val.to_string());
            }
        },
        
        _ => {},
    }
    
    // Special cases
    // Bytes
    if code.arg1 == LtacArg::RetRegI8 {
        let mut instr2 = create_x86instr(X86Type::MovSX);
        instr2.arg1 = X86Arg::Reg32(X86Reg::RAX);
        instr2.arg2 = X86Arg::Reg8(X86Reg::RAX);
        x86_code.push(instr2);
    } else if code.arg1 == LtacArg::RetRegU8 {
        let mut instr2 = create_x86instr(X86Type::MovZX);
        instr2.arg1 = X86Arg::Reg32(X86Reg::RAX);
        instr2.arg2 = X86Arg::Reg8(X86Reg::RAX);
        x86_code.push(instr2);
        
    // Short
    } else if code.arg1 == LtacArg::RetRegI16 {
        let mut instr2 = create_x86instr(X86Type::MovSX);
        instr2.arg1 = X86Arg::Reg32(X86Reg::RAX);
        instr2.arg2 = X86Arg::Reg16(X86Reg::RAX);
        x86_code.push(instr2);
    } else if code.arg1 == LtacArg::RetRegU16 {
        let mut instr2 = create_x86instr(X86Type::MovZX);
        instr2.arg1 = X86Arg::Reg32(X86Reg::RAX);
        instr2.arg2 = X86Arg::Reg16(X86Reg::RAX);
        x86_code.push(instr2);
    }
        
   x86_code.push(instr);
}

// Builds the integer and modulus instructions
// On x86 these are a little weird...
pub fn amd64_build_div(x86_code : &mut Vec<X86Instr>, code : &LtacInstr) {
    //Clear the RDX register
    let mut xor = create_x86instr(X86Type::Xor);
    xor.arg1 = X86Arg::Reg64(X86Reg::RDX);
    xor.arg2 = X86Arg::Reg64(X86Reg::RDX);
    x86_code.push(xor);

    // Create and build the instruction
    let mut instr = create_x86instr(X86Type::IDiv);
    let mut dest_instr = create_x86instr(X86Type::Mov);
    
    match code.instr_type {
        LtacType::U8Div | LtacType::U8Mod
        | LtacType::U16Div | LtacType::U16Mod
        | LtacType::U32Div | LtacType::U32Mod
        | LtacType::U64Div | LtacType::U64Mod => instr = create_x86instr(X86Type::Div),
        
        _ => {},
    }
    
    match &code.arg1 {
        LtacArg::Reg8(pos) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg8(X86Reg::RAX);
            instr2.arg2 = amd64_op_reg8(*pos);
            x86_code.push(instr2);
            
            dest_instr.arg1 = amd64_op_reg8(*pos);
        },
        
        LtacArg::Reg16(pos) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg16(X86Reg::RAX);
            instr2.arg2 = amd64_op_reg16(*pos);
            x86_code.push(instr2);
            
            dest_instr.arg1 = amd64_op_reg16(*pos);
        },
        
        LtacArg::Reg32(pos) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg32(X86Reg::RAX);
            instr2.arg2 = amd64_op_reg32(*pos);
            x86_code.push(instr2);
            
            dest_instr.arg1 = amd64_op_reg32(*pos);
        },
        
        LtacArg::Reg64(pos) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg64(X86Reg::RAX);
            instr2.arg2 = amd64_op_reg64(*pos);
            x86_code.push(instr2);
            
            dest_instr.arg1 = amd64_op_reg64(*pos);
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Reg8(pos) => instr.arg1 = amd64_op_reg8(*pos),
        LtacArg::Reg16(pos) => instr.arg1 = amd64_op_reg16(*pos),
        LtacArg::Reg32(pos) => instr.arg1 = amd64_op_reg32(*pos),
        LtacArg::Reg64(pos) => instr.arg1 = amd64_op_reg64(*pos),
        
        LtacArg::Mem(pos) => {
            if code.instr_type == LtacType::I8Div || code.instr_type == LtacType::I8Mod {
                instr.arg1 = X86Arg::BwordMem(X86Reg::RBP, *pos);
            } else if code.instr_type == LtacType::I16Div || code.instr_type == LtacType::I16Mod {
                instr.arg1 = X86Arg::WordMem(X86Reg::RBP, *pos);
            } else if code.instr_type == LtacType::I64Div || code.instr_type == LtacType::I64Mod {
                instr.arg1 = X86Arg::QwordMem(X86Reg::RBP, *pos);
            } else {
                instr.arg1 = X86Arg::DwordMem(X86Reg::RBP, *pos);
            }
        },
        
        LtacArg::Byte(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg8(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg8(X86Reg::R15);
        },
        
        LtacArg::UByte(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg8(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg8(X86Reg::R15);
        },
        
        LtacArg::I16(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg16(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg16(X86Reg::R15);
        },
        
        LtacArg::U16(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg16(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg16(X86Reg::R15);
        },
        
        LtacArg::I32(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg32(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg32(X86Reg::R15);
        },
        
        LtacArg::U32(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg32(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg32(X86Reg::R15);
        },
        
        LtacArg::I64(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg64(X86Reg::R15);
        },
        
        LtacArg::U64(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg64(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg64(X86Reg::R15);
        },
        
        _ => {},
    }
    
    match &code.instr_type {
        LtacType::I8Div | LtacType::U8Div => dest_instr.arg2 = X86Arg::Reg8(X86Reg::AL),
        LtacType::I16Div | LtacType::U16Div => dest_instr.arg2 = X86Arg::Reg16(X86Reg::RAX),
        LtacType::I32Div | LtacType::U32Div => dest_instr.arg2 = X86Arg::Reg32(X86Reg::RAX),
        LtacType::I64Div | LtacType::U64Div => dest_instr.arg2 = X86Arg::Reg64(X86Reg::RAX),
        
        LtacType::I8Mod | LtacType::U8Mod => dest_instr.arg2 = X86Arg::Reg8(X86Reg::AH),
        LtacType::I16Mod | LtacType::U16Mod => dest_instr.arg2 = X86Arg::Reg16(X86Reg::RDX),
        LtacType::I32Mod | LtacType::U32Mod => dest_instr.arg2 = X86Arg::Reg32(X86Reg::RDX),
        LtacType::I64Mod | LtacType::U64Mod => dest_instr.arg2 = X86Arg::Reg64(X86Reg::RDX),
        
        _ => {},
    }
    
    x86_code.push(instr);
    x86_code.push(dest_instr);
}

// Builds multiplication for byte values
// On x86 this is also a little strange...
pub fn amd64_build_byte_mul(x86_code : &mut Vec<X86Instr>, code : &LtacInstr) {
    //Clear the EAX register
    let mut xor = create_x86instr(X86Type::Xor);
    xor.arg1 = X86Arg::Reg32(X86Reg::RAX);
    xor.arg2 = X86Arg::Reg32(X86Reg::RAX);
    x86_code.push(xor);

    // Create and build the instruction
    let mut instr = create_x86instr(X86Type::IMul8);
    
    let mut dest_instr = create_x86instr(X86Type::Mov);
    dest_instr.arg2 = X86Arg::Reg16(X86Reg::RAX);
    
    if code.instr_type == LtacType::U8Mul {
        instr = create_x86instr(X86Type::Mul8);
    }
    
    match &code.arg1 {
        LtacArg::Reg8(pos) => {
            dest_instr.arg1 = amd64_op_reg16(*pos);
            
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg8(X86Reg::RAX);
            instr2.arg2 = amd64_op_reg8(*pos);
            x86_code.push(instr2);
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Reg8(pos) => instr.arg1 = amd64_op_reg8(*pos),
        LtacArg::Mem(pos) => instr.arg1 = X86Arg::BwordMem(X86Reg::RBP, *pos),
        
        LtacArg::Byte(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg8(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg8(X86Reg::R15);
        },
        
        LtacArg::UByte(val) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
            instr2.arg1 = X86Arg::Reg8(X86Reg::R15);
            instr2.arg2 = X86Arg::Imm32(*val as i32);
            x86_code.push(instr2);
            
            instr.arg1 = X86Arg::Reg8(X86Reg::R15);
        },
        
        _ => {},
    }
    
    // Write
    x86_code.push(instr);
    x86_code.push(dest_instr);
}



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

// TODO: I would like to eventually merge this with another file

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

// Many instructions have common syntax
pub fn amd64_build_instr(x86_code : &mut Vec<X86Instr>, code : &LtacInstr, _is_pic : bool) {
    /*let mut line = String::new();
    
    // Specific for float literals
    match code.arg2 {
        LtacArg::F32(ref val) => {
            line.push_str("  movss xmm8, DWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        LtacArg::F64(ref val) => {
            line.push_str("  movsd xmm8, QWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        _ => {},
    }
    
    // Need if any parts represent a memory offset (ie, array access)
    match code.arg2 {
        LtacArg::MemOffsetImm(pos, offset) if !is_risc => {
            line.push_str("  mov r15, QWORD PTR [rbp-");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
            
            match code.arg1 {
                LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15+"),
                LtacArg::Reg16(_p) => line.push_str("  mov r15w, WORD PTR [r15+"),
                LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15+"),
                LtacArg::FltReg(_p) => line.push_str("  movss xmm9, DWORD PTR [r15+"),
                LtacArg::FltReg64(_p) => line.push_str("  movsd xmm9, QWORD PTR [r15+"),
                _ => line.push_str("  mov r15d, DWORD PTR [r15+"),
            }
            
            line.push_str(&offset.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::MemOffsetMem(pos, offset, size) if !is_risc => {
            // Load the variable
            line.push_str("  mov r15d, DWORD PTR ");
            
            if is_pic {
                line.push_str("-");
                line.push_str(&offset.to_string());
                line.push_str("[rbp]\n");
            } else {
                line.push_str("[rbp-");
                line.push_str(&offset.to_string());
                line.push_str("]\n");
            }
            
            // Load the effective address
            line.push_str("  lea r14, ");
            
            if is_pic {
                line.push_str("0");
            }
            
            line.push_str("[0+r15*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
            
            // Load the array
            line.push_str("  mov r15, QWORD PTR ");
            
            if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]\n");
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            }
            
            // Add to get the proper offset
            line.push_str("  add r15, r14\n");
            
            // Store
            match &code.arg1 {
                LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15]\n"),
                LtacArg::Reg16(_p) => line.push_str("  mov r15w, WORD PTR [r15]\n"),
                LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15]\n"),
                LtacArg::FltReg(_p) => line.push_str("  movss xmm9, DWORD PTR [r15]\n"),
                LtacArg::FltReg64(_p) => line.push_str("  movsd xmm9, QWORD PTR [r15]\n"),
                _ => line.push_str("  mov r15d, DWORD PTR [r15]\n"),
            }
        },
        
        LtacArg::MemOffsetReg(pos, reg, size) if !is_risc => {
            // Determine the right register
            let src_reg = amd64_op_reg32(reg);
            
            // Load the array
            line.push_str("  mov r15, QWORD PTR ");
            
            if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]\n");
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            }
            
            // Load the effective address
            line.push_str("  lea r14, ");
            
            if is_pic {
                line.push_str("0");
            }
            
            line.push_str("[0+");
            line.push_str(&src_reg);
            line.push_str("*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
            
            // Add to get the proper offset
            line.push_str("  add r15, r14\n");
            
            // Store
            match &code.arg1 {
                LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15]\n"),
                LtacArg::Reg16(_p) => line.push_str("  mov r15w, WORD PTR [r15]\n"),
                LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15]\n"),
                LtacArg::FltReg(_p) => line.push_str("  movss xmm9, DWORD PTR [r15]\n"),
                LtacArg::FltReg64(_p) => line.push_str("  movsd xmm9, QWORD PTR [r15]\n"),
                _ => line.push_str("  mov r15d, DWORD PTR [r15]\n"),
            }
        },
        
        _ => {},
    }*/
    
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
        
        //LtacType::LdAddr => line.push_str("  lea "),
        
        LtacType::I8Add | LtacType::U8Add |
        LtacType::I16Add | LtacType::U16Add |
        LtacType::I32Add | LtacType::U32Add |
        LtacType::I64Add | LtacType::U64Add => instr = create_x86instr(X86Type::Add),
        
        LtacType::I8Sub | LtacType::I16Sub |
        LtacType::I32Sub  | LtacType::I64Sub => instr = create_x86instr(X86Type::Sub),
        
        LtacType::I16Mul | LtacType::I32Mul |
        LtacType::I64Mul => instr = create_x86instr(X86Type::IMul),
        LtacType::U16Mul | LtacType::U32Mul |
        LtacType::U64Mul => instr = create_x86instr(X86Type::Mul),
        
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
        
        /*LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },*/
        
        LtacArg::Reg32(pos) => instr.arg1 = amd64_op_reg32(*pos),
        
        /*LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            line.push_str(&reg);
            line.push_str(", ");
        },
        */
        LtacArg::RetRegI8 | LtacArg::RetRegU8 => instr.arg1 = X86Arg::Reg8(X86Reg::RAX),
        LtacArg::RetRegI16 | LtacArg::RetRegU16 => instr.arg1 = X86Arg::Reg16(X86Reg::RAX),
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => instr.arg1 = X86Arg::Reg32(X86Reg::RAX),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => instr.arg1 = X86Arg::Reg64(X86Reg::RAX),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => instr.arg1 = X86Arg::Xmm(0),
        
        LtacArg::Mem(pos) => {
            match &code.arg2 {
                /*LtacArg::Byte(_v) => line.push_str("BYTE PTR "),
                LtacArg::UByte(_v) => line.push_str("BYTE PTR "),
                LtacArg::I16(_v) => line.push_str("WORD PTR "),
                LtacArg::U16(_v) => line.push_str("WORD PTR "),
                LtacArg::I32(_v) => line.push_str("DWORD PTR "),
                LtacArg::U32(_v) => line.push_str("DWORD PTR "),
                LtacArg::I64(_v) => line.push_str("QWORD PTR "),
                LtacArg::U64(_v) => line.push_str("QWORD PTR "),
                LtacArg::F32(_v) => line.push_str("DWORD PTR "),
                LtacArg::F64(_v) | LtacArg::PtrLcl(_v) => line.push_str("QWORD PTR "), 
                LtacArg::Ptr(_v) => line.push_str("QWORD PTR "),*/
                _ => instr.arg1 = X86Arg::DwordMem(X86Reg::RBP, *pos),
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
        
        /*LtacArg::MemOffsetImm(pos, offset) if !is_risc => {
            line.push_str("r15, QWORD PTR ");
            
            if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]\n");
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            }
            
            line.push_str("  add r15, ");
            line.push_str(&offset.to_string());
            line.push_str("\n");
            
            match &code.arg2 {
                LtacArg::Byte(_v) => line.push_str("  mov BYTE PTR "),
                LtacArg::UByte(_v) => line.push_str("  mov BYTE PTR "),
                LtacArg::I16(_v) => line.push_str("  mov WORD PTR "),
                LtacArg::U16(_v) => line.push_str("  mov WORD PTR "),
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
        },
        
        LtacArg::MemOffsetMem(pos, offset, size) if !is_risc => {
            // Load the variable
            line.push_str("r15d, DWORD PTR ");
            
            if is_pic {
                line.push_str("-");
                line.push_str(&offset.to_string());
                line.push_str("[rbp]\n");
            } else {
                line.push_str("[rbp-");
                line.push_str(&offset.to_string());
                line.push_str("]\n");
            }
            
            // Load the effective address
            line.push_str("  lea r14, ");
            
            if is_pic {
                line.push_str("0");
            }
            
            line.push_str("[0+r15*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
            
            // Load the array
            line.push_str("  mov r15, QWORD PTR "); 
            
            if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]\n");
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            }
            
            // Add to get the proper offset
            line.push_str("  add r15, r14\n");
            
            // Now set up for the final move
            match &code.arg2 {
                LtacArg::Reg8(_v) => line.push_str("  mov BYTE PTR "),
                LtacArg::Reg16(_v) => line.push_str("  mov WORD PTR "),
                LtacArg::Byte(_v) => line.push_str("  mov BYTE PTR "),
                LtacArg::UByte(_v) => line.push_str("  mov BYTE PTR "),
                LtacArg::I16(_v) => line.push_str("  mov WORD PTR "),
                LtacArg::U16(_v) => line.push_str("  mov WORD PTR "),
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
        },
        
        // If we can clean this up, especially the first match, that would be nice
        LtacArg::MemOffsetReg(pos, reg, size) if !is_risc => {
            // Determine the right register
            let src_reg = amd64_op_reg32(*reg);
            let size_mod : String;
            let mov_instr : String;
            
            match &code.arg2 {
                LtacArg::Reg8(_v) => { size_mod = "BYTE PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::Reg16(_v) => { size_mod = "WORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::Byte(_v) => { size_mod = "BYTE PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::UByte(_v) => { size_mod = "BYTE PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::I16(_v) => { size_mod = "WORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::U16(_v) => { size_mod = "WORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::I32(_v) => { size_mod = "DWORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::U32(_v) => { size_mod = "DWORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::I64(_v) => { size_mod = "QWORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::U64(_v) => { size_mod = "QWORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
                LtacArg::F32(_v) => { size_mod = "DWORD PTR".to_string(); mov_instr = "  movss ".to_string(); },
                LtacArg::F64(_v) => { size_mod = "QWORD PTR".to_string(); mov_instr = "  movsd ".to_string(); },
                LtacArg::FltReg(_v) => { size_mod = "DWORD PTR".to_string(); mov_instr = "  movss ".to_string(); },
                LtacArg::FltReg64(_v) => { size_mod = "QWORD PTR".to_string(); mov_instr = "  movsd ".to_string(); },
                _ => { size_mod = "DWORD PTR".to_string(); mov_instr = "  mov ".to_string(); },
            }
        
            // Load the array
            line.push_str("r15, QWORD PTR ");
            
            if is_pic {
                line.push_str("-");
                line.push_str(&pos.to_string());
                line.push_str("[rbp]\n");
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            }
            
            // Load the effective address
            line.push_str("  lea r14, ");
            
            if is_pic {
                line.push_str("0");
            }
            
            line.push_str("[0+");
            line.push_str(&src_reg);
            line.push_str("*");
            line.push_str(&size.to_string());
            line.push_str("]\n");
            
            // Add to get the proper offset
            line.push_str("  add r15, r14\n");
            
            // Now set up for the final move
            line.push_str(&mov_instr);
            line.push_str(&size_mod);
            line.push_str("[r15], ");
        },*/
        
        _ => {},
    }
    
    // Build the second operand
    match &code.arg2 {
        /*LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            line.push_str(&reg);
        },*/
        
        LtacArg::Reg32(pos) => instr.arg2 = amd64_op_reg32(*pos),
        
        /*LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            line.push_str(&reg);
        },*/
        
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
        
        /*LtacArg::MemOffsetImm(_p, _o) if !is_risc => {
            match &code.arg1 {
                LtacArg::Reg8(_p) => line.push_str("r15b"),
                LtacArg::Reg16(_p) => line.push_str("r15w"),
                LtacArg::Reg64(_p) => line.push_str("r15"),
                LtacArg::FltReg(_p) | LtacArg::FltReg64(_p) => line.push_str("xmm9"),
                _ => line.push_str("r15d"),
            }
        },
        
        LtacArg::MemOffsetMem(_p, _o, _s) | LtacArg::MemOffsetReg(_p, _o, _s) if !is_risc => {
            match &code.arg1 {
                LtacArg::Reg8(_p) => line.push_str("r15b"),
                LtacArg::Reg16(_p) => line.push_str("r15w"),
                LtacArg::Reg64(_p) => line.push_str("r15"),
                LtacArg::FltReg(_p) | LtacArg::FltReg64(_p) => line.push_str("xmm9"),
                _ => line.push_str("r15d"),
            }
        },
        
        LtacArg::Byte(val) => line.push_str(&val.to_string()),
        LtacArg::UByte(val) => line.push_str(&val.to_string()),
        
        LtacArg::I16(val) => line.push_str(&val.to_string()),
        LtacArg::U16(val) => line.push_str(&val.to_string()),*/
        
        LtacArg::I32(val) => instr.arg2 = X86Arg::Imm32(*val),
        LtacArg::U32(val) => instr.arg2 = X86Arg::Imm32(*val as i32),
        
        /*LtacArg::I64(val) => line.push_str(&val.to_string()),
        LtacArg::U64(val) => line.push_str(&val.to_string()),
        
        LtacArg::F32(_v) | LtacArg::F64(_v) => line.push_str("xmm8\n"),
        
        LtacArg::PtrLcl(ref val) => {
            if is_pic {
                line.push_str("r15");
            } else {
                line.push_str("OFFSET FLAT:");
                line.push_str(&val);
            }
        },*/
        
        _ => {},
    }
    
    /*line.push_str("\n");
    
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
        .expect("[AMD64_write_instr] Write failed.");*/
        
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
    
    if code.instr_type == LtacType::U32Div || code.instr_type == LtacType::U32Mod
        || code.instr_type == LtacType::U64Div || code.instr_type == LtacType::U64Mod {
        instr = create_x86instr(X86Type::Div);
    }
    
    match &code.arg1 {
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
        
        LtacArg::Mem(pos) => {
            let mut instr2 = create_x86instr(X86Type::Mov);
        
            if code.instr_type == LtacType::I64Div || code.instr_type == LtacType::I64Mod {
                instr2.arg1 = X86Arg::Reg64(X86Reg::RAX);
                instr2.arg2 = X86Arg::QwordMem(X86Reg::RBP, *pos);
                x86_code.push(instr2);
                
                dest_instr.arg1 = X86Arg::QwordMem(X86Reg::RBP, *pos);
            } else {
                instr2.arg1 = X86Arg::Reg32(X86Reg::RAX);
                instr2.arg2 = X86Arg::DwordMem(X86Reg::RBP, *pos);
                x86_code.push(instr2);
                
                dest_instr.arg1 = X86Arg::DwordMem(X86Reg::RBP, *pos);
            }
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Reg32(pos) => instr.arg1 = amd64_op_reg32(*pos),
        LtacArg::Reg64(pos) => instr.arg1 = amd64_op_reg64(*pos),
        
        LtacArg::Mem(pos) => {
            if code.instr_type == LtacType::I64Div || code.instr_type == LtacType::I64Mod {
                instr.arg1 = X86Arg::QwordMem(X86Reg::RBP, *pos);
            } else {
                instr.arg1 = X86Arg::DwordMem(X86Reg::RBP, *pos);
            }
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
        LtacType::I32Div | LtacType::U32Div => dest_instr.arg2 = X86Arg::Reg32(X86Reg::RAX),
        LtacType::I64Div | LtacType::U64Div => dest_instr.arg2 = X86Arg::Reg64(X86Reg::RAX),
        
        LtacType::I32Mod | LtacType::U32Mod => dest_instr.arg2 = X86Arg::Reg32(X86Reg::RDX),
        LtacType::I64Mod | LtacType::U64Mod => dest_instr.arg2 = X86Arg::Reg64(X86Reg::RDX),
        
        _ => {},
    }
    
    x86_code.push(instr);
    x86_code.push(dest_instr);
}


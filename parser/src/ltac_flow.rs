
// This file is part of the Lila compiler
// Copyright (C) 2020 Patrick Flynn
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

use crate::ltac_builder::*;
use crate::ltac_utils::*;

use crate::ast::{AstStmt, AstStmtType, AstArgType};
use crate::ltac;
use crate::ltac::{LtacType, LtacInstr, LtacArg};

// Break out of a current loop
pub fn build_break(builder : &mut LtacBuilder) {
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = builder.end_labels.last().unwrap().to_string();
    builder.file.code.push(br);
}

// Continue through the rest of the loop
pub fn build_continue(builder : &mut LtacBuilder) {
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = builder.loop_labels.last().unwrap().to_string();
    builder.file.code.push(br);
}

// A utility function to create a label
fn create_label(builder : &mut LtacBuilder, is_top : bool) {
    let lbl_pos = builder.str_pos.to_string();
    builder.str_pos += 1;
    
    let mut name = "L".to_string();
    name.push_str(&lbl_pos);
    
    if is_top {
        builder.top_label_stack.push(name);
    } else {
        builder.label_stack.push(name);
    }
}

// Builds a conditional statement
fn build_cmp(builder : &mut LtacBuilder, line : &AstStmt) -> Vec<LtacInstr> {
    let mut block : Vec<LtacInstr> = Vec::new();
    let mut cmp = ltac::create_instr(LtacType::U32Cmp);
    
    // Build the conditional statement
    let arg1 = match line.args.iter().nth(0) {
        Some(a) => a,
        None => return block,
    };
    
    let mut arg2 = match line.args.iter().nth(2) {
        Some(a) => a,
        None => return block,
    };
    
    // Set to true if we have a signed byte, short, or int variable
    let mut signed_variant = false;
    let mut negate = false;
    
    if arg2.arg_type == AstArgType::OpNeg {
        negate = true;
        arg2 = match line.args.iter().nth(3) {
            Some(a) => a,
            None => return block,
        };
    }
    
    // Although we assume its integer comparison by default, the first operand
    // determines the comparison type
    match &arg1.arg_type {
        AstArgType::ByteL => {
            let mut mov = ltac::create_instr(LtacType::MovUB);
            mov.arg1 = LtacArg::Reg8(0);
            mov.arg2 = LtacArg::UByte(arg1.u8_val);
            block.push(mov);
            
            cmp = ltac::create_instr(LtacType::U8Cmp);
            cmp.arg1 = LtacArg::Reg8(0);
        },
        
        AstArgType::ShortL => {
            let mut mov = ltac::create_instr(LtacType::MovUW);
            mov.arg1 = LtacArg::Reg16(0);
            mov.arg2 = LtacArg::U16(arg1.u16_val);
            block.push(mov);
            
            cmp = ltac::create_instr(LtacType::U16Cmp);
            cmp.arg1 = LtacArg::Reg16(0);
        },
        
        AstArgType::IntL => {
            if negate {
                let val : i32 = 0 - (arg1.u64_val as i32);
                
                let mut mov = ltac::create_instr(LtacType::Mov);
                mov.arg1 = LtacArg::Reg32(0);
                mov.arg2  = LtacArg::I32(val);
                block.push(mov);
                
                cmp = ltac::create_instr(LtacType::I32Cmp);
                signed_variant = true;
            } else {
                let mut mov = ltac::create_instr(LtacType::MovU);
                mov.arg1 = LtacArg::Reg32(0);
                mov.arg2  = LtacArg::U32(arg1.u64_val as u32);
                block.push(mov);
            }
            
            cmp.arg1 = LtacArg::Reg32(0);
        },
        
        AstArgType::FloatL => {
            let name = builder.build_float(arg1.f64_val, false, false);
            let mut mov = ltac::create_instr(LtacType::MovF32);
            mov.arg1 = LtacArg::FltReg(0);
            mov.arg2 = LtacArg::F32(name);
            block.push(mov);
            
            cmp = ltac::create_instr(LtacType::F32Cmp);
            cmp.arg1 = LtacArg::FltReg(0);
        },
        
        AstArgType::StringL => {
            let name = builder.build_string(arg1.str_val.clone());
            
            let mut instr2 = ltac::create_instr(LtacType::PushArg);
            instr2.arg1 = LtacArg::PtrLcl(name);
            instr2.arg2_val = 1;
            builder.file.code.push(instr2);
            
            cmp = ltac::create_instr(LtacType::StrCmp);
        },
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::MovU);
            mov.arg1 = LtacArg::Reg32(0);
            
            match &builder.vars.get(&arg1.str_val) {
                Some(v) => {
                    // String comparisons
                    if v.data_type == DataType::Str {
                        cmp = ltac::create_instr(LtacType::StrCmp);
                        
                        mov = ltac::create_instr(LtacType::PushArg);
                        mov.arg1 = LtacArg::Ptr(v.pos);
                        mov.arg2_val = 1;
                        
                    // Float-32 comparisons
                    } else if v.data_type == DataType::Float {
                        mov = ltac::create_instr(LtacType::MovF32);
                        mov.arg1 = LtacArg::FltReg(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp = ltac::create_instr(LtacType::F32Cmp);
                        cmp.arg1 = LtacArg::FltReg(0);
                        
                    // Float-64 comparisons
                    } else if v.data_type == DataType::Double {
                        mov = ltac::create_instr(LtacType::MovF64);
                        mov.arg1 = LtacArg::FltReg64(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp = ltac::create_instr(LtacType::F64Cmp);
                        cmp.arg1 = LtacArg::FltReg64(0);
                        
                    // Byte comparisons
                    } else if v.data_type == DataType::Byte {
                        cmp = ltac::create_instr(LtacType::I8Cmp);
                        cmp.arg1 = LtacArg::Reg8(0);
                        
                        mov = ltac::create_instr(LtacType::MovB);
                        mov.arg1 = LtacArg::Reg8(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        signed_variant = true;
                        
                    // Characters
                    } else if v.data_type == DataType::Char {
                        cmp = ltac::create_instr(LtacType::I8Cmp);
                        cmp.arg1 = LtacArg::Reg8(0);
                        
                        mov = ltac::create_instr(LtacType::MovB);
                        mov.arg1 = LtacArg::Reg8(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                    // Unsigned byte comparisons
                    } else if v.data_type == DataType::UByte {
                        cmp.instr_type = LtacType::U8Cmp;
                        cmp.arg1 = LtacArg::Reg8(0);
                        
                        mov = ltac::create_instr(LtacType::MovUB);
                        mov.arg1 = LtacArg::Reg8(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                    // Short comparisons
                    } else if v.data_type == DataType::Short {
                        cmp= ltac::create_instr(LtacType::I16Cmp);
                        cmp.arg1 = LtacArg::Reg16(0);
                        
                        mov = ltac::create_instr(LtacType::MovW);
                        mov.arg1 = LtacArg::Reg16(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        signed_variant = true;
                    
                    // Unsigned short comparisons
                    } else if v.data_type == DataType::UShort {
                        cmp.instr_type = LtacType::U16Cmp;
                        cmp.arg1 = LtacArg::Reg16(0);
                        
                        mov = ltac::create_instr(LtacType::MovUW);
                        mov.arg1 = LtacArg::Reg16(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                    // Signed int64 comparisons
                    } else if v.data_type == DataType::Int64 {
                        cmp.instr_type = LtacType::I64Cmp;
                        cmp.arg1 = LtacArg::Reg64(0);
                        
                        mov = ltac::create_instr(LtacType::MovQ);
                        mov.arg1 = LtacArg::Reg64(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                    
                    // Unsigned int64 comparisons
                    } else if v.data_type == DataType::UInt64 {
                        cmp.instr_type = LtacType::U64Cmp;
                        cmp.arg1 = LtacArg::Reg64(0);
                        
                        mov = ltac::create_instr(LtacType::MovUQ);
                        mov.arg1 = LtacArg::Reg64(0);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                    // Integer comparisons
                    } else {
                        if v.data_type == DataType::Int {
                            mov.instr_type = LtacType::Mov;
                            cmp.instr_type = LtacType::I32Cmp;
                            signed_variant = true;
                        }
                        
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp.arg1 = LtacArg::Reg32(0);
                    }
                    
                    block.push(mov);
                },
                
                None => mov.arg2_val = 0,
            }
        },
        
        _ => {},
    }
    
    match &arg2.arg_type {
        AstArgType::CharL => {
            cmp.arg2 = LtacArg::Byte(arg2.char_val as i8);
        },
        
        AstArgType::ByteL => {
            if signed_variant {
                cmp.arg2 = LtacArg::Byte(arg2.u8_val as i8);
            } else {
                cmp.arg2 = LtacArg::UByte(arg2.u8_val);
            }
        },
        
        AstArgType::ShortL => {
            if signed_variant {
                cmp.arg2 = LtacArg::I16(arg2.u16_val as i16);
            } else {
                cmp.arg2 = LtacArg::U16(arg2.u16_val);
            }
        },
    
        AstArgType::IntL => {
            if signed_variant {
                let mut val = arg2.u64_val as i64;
                if negate {
                    val = 0 - val;
                }
                
                if cmp.instr_type == LtacType::I64Cmp {
                    cmp.arg2 = LtacArg::I64(val);
                } else {
                    cmp.arg2 = LtacArg::I32(val as i32);
                }
            } else {
                if cmp.instr_type == LtacType::I8Cmp {
                    cmp.arg2 = LtacArg::Byte(arg2.u64_val as i8);
                } else if cmp.instr_type == LtacType::U64Cmp {
                    cmp.arg2 = LtacArg::U64(arg2.u64_val);
                } else {
                    cmp.arg2 = LtacArg::U32(arg2.u64_val as u32);
                }
            }
        },
        
        AstArgType::FloatL => {
            if cmp.instr_type == LtacType::F64Cmp {
                let name = builder.build_float(arg2.f64_val, true, false);
                cmp.arg2 = LtacArg::F64(name);
            } else {
                let name = builder.build_float(arg2.f64_val, false, false);
                cmp.arg2 = LtacArg::F32(name);
            }
        },
        
        AstArgType::StringL => {
            let name = builder.build_string(arg2.str_val.clone());
            
            let mut instr2 = ltac::create_instr(LtacType::PushArg);
            instr2.arg1 = LtacArg::PtrLcl(name);
            instr2.arg2_val = 2;
            builder.file.code.push(instr2); 
        },
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1 = LtacArg::Reg32(1);
            
            match &builder.vars.get(&arg2.str_val) {
                Some(v) => {
                    // Strings
                    if v.data_type == DataType::Str {
                        mov = ltac::create_instr(LtacType::PushArg);
                        mov.arg1 = LtacArg::Ptr(v.pos);
                        mov.arg2_val = 2;
                        
                    // Single-precision floats
                    } else if v.data_type == DataType::Float {
                        mov = ltac::create_instr(LtacType::MovF32);
                        mov.arg1 = LtacArg::FltReg(1);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp.arg2 = LtacArg::FltReg(1);
                        
                    // Double-precision floats
                    } else if v.data_type == DataType::Double {
                        mov = ltac::create_instr(LtacType::MovF64);
                        mov.arg1 = LtacArg::FltReg64(1);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        match cmp.arg1 {
                            LtacArg::FltReg(pos) => {
                                block.pop();
                                
                                let name = builder.build_float(arg1.f64_val, true, false);
                                let mut mov = ltac::create_instr(LtacType::MovF64);
                                mov.arg1 = LtacArg::FltReg64(pos);
                                mov.arg2 = LtacArg::F64(name);
                                block.push(mov);
                                
                                cmp = ltac::create_instr(LtacType::F64Cmp);
                                cmp.arg1 = LtacArg::FltReg64(pos);
                            },
                            
                            _ => {},
                        }
                        
                        cmp.arg2 = LtacArg::FltReg64(1);
                        
                    // Bytes
                    } else if v.data_type == DataType::Byte {
                        if arg1.arg_type == AstArgType::ByteL {
                            block.pop();
                            mov = ltac::create_instr(LtacType::MovB);
                            mov.arg1 = LtacArg::Reg8(0);
                            mov.arg2 = LtacArg::Byte(arg1.u8_val as i8);
                        
                            cmp = ltac::create_instr(LtacType::I8Cmp);
                            cmp.arg1 = LtacArg::Reg8(0);
                        } else {
                            mov.arg1 = LtacArg::Empty;
                        }
                        
                        cmp.arg2 = LtacArg::Mem(v.pos);
                    
                    // Unsigned bytes
                    } else if v.data_type == DataType::UByte {
                        mov = ltac::create_instr(LtacType::MovUB);
                        mov.arg1 = LtacArg::Reg8(1);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp.arg2 = LtacArg::Reg8(1);
                        
                    // Characters
                    } else if v.data_type == DataType::Char {
                        if arg1.arg_type == AstArgType::IntL {
                            // TODO: Fix
                            let val : i8 = 0;
                        
                            block.pop();
                            mov = ltac::create_instr(LtacType::MovB);
                            mov.arg1 = LtacArg::Reg8(0);
                            mov.arg2 = LtacArg::Byte(val);
                            
                            cmp = ltac::create_instr(LtacType::I8Cmp);
                            cmp.arg1 = LtacArg::Reg8(0);
                        } else {
                            mov.arg1 = LtacArg::Empty;
                        }
                        
                        cmp.arg2 = LtacArg::Mem(v.pos);
                        
                    // Shorts
                    } else if v.data_type == DataType::Short {
                        if arg1.arg_type == AstArgType::ShortL {
                            block.pop();
                            mov = ltac::create_instr(LtacType::MovW);
                            mov.arg1 = LtacArg::Reg16(0);
                            mov.arg2 = LtacArg::I16(arg1.u16_val as i16);
                        
                            cmp = ltac::create_instr(LtacType::I16Cmp);
                            cmp.arg1 = LtacArg::Reg16(0);
                        } else {
                            mov.arg1 = LtacArg::Empty;
                        }
                        
                        cmp.arg2 = LtacArg::Mem(v.pos);
                    
                    // Unsigned short
                    } else if v.data_type == DataType::UShort {
                        mov = ltac::create_instr(LtacType::MovUW);
                        mov.arg1 = LtacArg::Reg16(1);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp.arg2 = LtacArg::Reg16(1);
                        
                    // Int-64
                    } else if v.data_type == DataType::Int64 {
                        if arg1.arg_type == AstArgType::IntL {
                            block.pop();
                            let mut mov2 = ltac::create_instr(LtacType::MovQ);
                            mov2.arg1 = LtacArg::Reg64(0);
                            mov2.arg2 = LtacArg::I64(arg1.u64_val as i64);
                            block.push(mov2);
                            
                            cmp = ltac::create_instr(LtacType::I64Cmp);
                            cmp.arg1 = LtacArg::Reg64(0);
                        }
                        
                        mov = ltac::create_instr(LtacType::MovQ);
                        mov.arg1 = LtacArg::Reg64(1);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp.arg2 = LtacArg::Reg64(1);
                    
                    // Unsigned int-64
                    } else if v.data_type == DataType::UInt64 {
                        if arg1.arg_type == AstArgType::IntL {
                            block.pop();
                            let mut mov2 = ltac::create_instr(LtacType::MovUQ);
                            mov2.arg1 = LtacArg::Reg64(0);
                            mov2.arg2 = LtacArg::U64(arg1.u64_val);
                            block.push(mov2);
                            
                            cmp = ltac::create_instr(LtacType::U64Cmp);
                            cmp.arg1 = LtacArg::Reg64(0);
                        }
                        
                        mov = ltac::create_instr(LtacType::MovUQ);
                        mov.arg1 = LtacArg::Reg64(1);
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp.arg2 = LtacArg::Reg64(1);
                        
                    // Integers
                    } else if v.data_type == DataType::Int {
                        if arg1.arg_type == AstArgType::IntL {
                            block.pop();
                            mov = ltac::create_instr(LtacType::Mov);
                            mov.arg1 = LtacArg::Reg32(0);
                            mov.arg2 = LtacArg::I32(arg1.u64_val as i32);
                        
                            cmp = ltac::create_instr(LtacType::I32Cmp);
                            cmp.arg1 = LtacArg::Reg32(0);
                        } else {
                            mov.arg1 = LtacArg::Empty;
                        }
                        
                        cmp.arg2 = LtacArg::Mem(v.pos);
                        
                    } else {
                        mov.arg2 = LtacArg::Mem(v.pos);
                        
                        cmp.arg2 = LtacArg::Reg32(1);
                    }
                    
                    if mov.arg1 != LtacArg::Empty {
                        block.push(mov);
                    }
                },
                
                None => mov.arg2_val = 0,
            }
        },
        
        _ => {},
    }
    
    block.push(cmp);
    block
}

// Builds an LTAC conditional block (specific for if-else)
pub fn build_cond(builder : &mut LtacBuilder, line : &AstStmt) {
    if line.stmt_type == AstStmtType::If {
        builder.block_layer += 1;
        create_label(builder, true);
        
        // A dummy placeholder
        let code_block : Vec<LtacInstr> = Vec::new();
        builder.code_stack.push(code_block);
    } else {
        let mut jmp = ltac::create_instr(LtacType::Br);
        jmp.name = builder.top_label_stack.last().unwrap().to_string();
        builder.file.code.push(jmp);
        
        let mut label = ltac::create_instr(LtacType::Label);
        match builder.label_stack.pop() {
            Some(name) => {
                label.name = name;
                builder.file.code.push(label);
            },
            
            None => {},
        }
    }
    
    create_label(builder, false);
    
    let cmp_block = build_cmp(builder, line);
    for ln in cmp_block.iter() {
        builder.file.code.push(ln.clone());
    }
    
    if line.stmt_type == AstStmtType::Else {
        return;
    }
    
    // Add the instruction
    let cmp = cmp_block.last().unwrap();
    let cmp_type = cmp.instr_type.clone();
    
    // Now the operator
    let op = &line.args.iter().nth(1).unwrap();
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = builder.label_stack.last().unwrap().to_string();
    
    match &op.arg_type {
        AstArgType::OpEq => br.instr_type = LtacType::Bne,
        AstArgType::OpNeq => br.instr_type = LtacType::Be,
        
        AstArgType::OpLt 
            if (cmp_type == LtacType::F32Cmp || cmp_type == LtacType::F64Cmp)
            => br.instr_type = LtacType::Bfge,
        AstArgType::OpLt => br.instr_type = LtacType::Bge,
        
        AstArgType::OpLte
            if (cmp_type == LtacType::F32Cmp || cmp_type == LtacType::F64Cmp)
            => br.instr_type = LtacType::Bfg,
        AstArgType::OpLte => br.instr_type = LtacType::Bg,
        
        AstArgType::OpGt
            if (cmp_type == LtacType::F32Cmp || cmp_type == LtacType::F64Cmp)
            => br.instr_type = LtacType::Bfle,
        AstArgType::OpGt => br.instr_type = LtacType::Ble,
        
        AstArgType::OpGte
            if (cmp_type == LtacType::F32Cmp || cmp_type == LtacType::F64Cmp)
            => br.instr_type = LtacType::Bfl,
        AstArgType::OpGte => br.instr_type = LtacType::Bl,
        
        _ => {},
    }
    
    builder.file.code.push(br);
}

// Builds a while loop block
pub fn build_while(builder : &mut LtacBuilder, line : &AstStmt) {
    builder.block_layer += 1;
    builder.loop_layer += 1;
    
    create_label(builder, false);    // Goes at the very end
    create_label(builder, false);    // Add a comparison label
    create_label(builder, false);   // Add a loop label
    
    let end_label = builder.label_stack.pop().unwrap();
    let loop_label = builder.label_stack.pop().unwrap();
    let cmp_label = builder.label_stack.pop().unwrap();
    
    builder.loop_labels.push(cmp_label.clone());
    builder.end_labels.push(end_label.clone());
    
    // Jump to the comparsion label, and add the loop label
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = cmp_label.clone();
    builder.file.code.push(br);
    
    let mut lbl = ltac::create_instr(LtacType::Label);
    lbl.name = loop_label.clone();
    builder.file.code.push(lbl);
    
    // Now build the comparison
    let mut cmp_block : Vec<LtacInstr> = Vec::new();
    
    let mut lbl2 = ltac::create_instr(LtacType::Label);
    lbl2.name = cmp_label.clone();
    cmp_block.push(lbl2);
    
    // Build the conditional statement
    let block = build_cmp(builder, line);
    for ln in block.iter() {
        cmp_block.push(ln.clone());
    }
    
    // Now the operator
    let op = &line.args.iter().nth(1).unwrap();
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = loop_label.clone();
    
    match &op.arg_type {
        AstArgType::OpEq => br.instr_type = LtacType::Be,
        AstArgType::OpNeq => br.instr_type = LtacType::Bne,
        AstArgType::OpLt => br.instr_type = LtacType::Bl,
        AstArgType::OpLte => br.instr_type = LtacType::Ble,
        AstArgType::OpGt => br.instr_type = LtacType::Bg,
        AstArgType::OpGte => br.instr_type = LtacType::Bge,
        _ => {},
    }
    
    cmp_block.push(br);
    
    // The end label
    let mut end_lbl = ltac::create_instr(LtacType::Label);
    end_lbl.name = end_label.clone();
    cmp_block.push(end_lbl);
    
    builder.code_stack.push(cmp_block);
}

// Builds a for loop block
pub fn build_for_loop(builder : &mut LtacBuilder, line : &AstStmt) {
    builder.block_layer += 1;
    builder.loop_layer += 1;
    
    create_label(builder, false);    // Goes at the very end
    create_label(builder, false);    // Add a comparison label
    create_label(builder, false);   // Add a loop label
    
    if line.args.len() == 4 {
        build_range_for_loop(builder, line);
    } else {
        build_foreach_loop(builder, line);
    }
}

// Builds a range-based for loop
fn build_range_for_loop(builder : &mut LtacBuilder, line : &AstStmt)  {
    let end_label = builder.label_stack.pop().unwrap();
    let loop_label = builder.label_stack.pop().unwrap();
    let cmp_label = builder.label_stack.pop().unwrap();
    
    builder.loop_labels.push(cmp_label.clone());
    builder.end_labels.push(end_label.clone());
    
    // Create the variable
    let index_var = line.args.first().unwrap();
    let name = index_var.str_val.clone();
    
    builder.stack_pos += 4;
    let pos = builder.stack_pos;
    
    let index = Var {
        pos : pos,
        data_type : DataType::Int,
        sub_type : DataType::None,
        is_param : false,
    };
    
    builder.vars.insert(name, index);
    
    // Determine the type of loop
    let start_pos = line.args.iter().nth(1).unwrap();
    let end_arg = line.args.iter().nth(3).unwrap();
    
    // Set the variable equal to the start
    // TODO: Other types
    match start_pos.arg_type {
        AstArgType::IntL => {
            let mut instr = ltac::create_instr(LtacType::Mov);
            instr.arg1 = LtacArg::Mem(pos);
            instr.arg2 = LtacArg::I32(start_pos.u64_val as i32);
            builder.file.code.push(instr);
        },
        
        _ => {},
    }
    
    // Start the loop
    let mut lbl = ltac::create_instr(LtacType::Label);
    lbl.name = loop_label.clone();
    builder.file.code.push(lbl);
    
    // Now build the comparison
    // We create a separate block since this will go at the end of the loop
    let mut cmp_block : Vec<LtacInstr> = Vec::new();
    
    // Increment the counter variable
    let mut lbl2 = ltac::create_instr(LtacType::Label);
    lbl2.name = cmp_label.clone();
    cmp_block.push(lbl2);
    
    let mut mov2 = ltac::create_instr(LtacType::Mov);
    mov2.arg1 = LtacArg::Reg32(0);
    mov2.arg2 = LtacArg::Mem(pos);
    cmp_block.push(mov2);
    
    let mut inc_counter = ltac::create_instr(LtacType::I32Add);
    inc_counter.arg1 = LtacArg::Reg32(0);
    inc_counter.arg2 = LtacArg::I32(1);
    cmp_block.push(inc_counter);
    
    let mut mov3 = ltac::create_instr(LtacType::Mov);
    mov3.arg1 = LtacArg::Mem(pos);
    mov3.arg2 = LtacArg::Reg32(0);
    cmp_block.push(mov3);
    
    // Build the conditional statement
    // TODO: Other types
    let mut cmp_instr = ltac::create_instr(LtacType::I32Cmp);
    cmp_instr.arg1 = LtacArg::Reg32(0);
    
    match end_arg.arg_type {
        AstArgType::IntL => cmp_instr.arg2 = LtacArg::I32(end_arg.u64_val as i32),
        _ => {},
    }
    
    cmp_block.push(cmp_instr);
    
    // Now the operator
    let mut br = ltac::create_instr(LtacType::Bl);
    br.name = loop_label.clone();
    cmp_block.push(br);
    
    // The end label
    let mut end_lbl = ltac::create_instr(LtacType::Label);
    end_lbl.name = end_label.clone();
    cmp_block.push(end_lbl);
    
    builder.code_stack.push(cmp_block);
}

// Builds a foreach loop
// Overall logic behind a for each loop
// Two extra variables need
//      -> Index is the user-specified one to hold the current element
//      -> Pos is an internal one used to check the current index against the loop size
//
// mov pos, 0
// jmp CMP
// LOOP
// mov index, array[pos]
// ~~~~
// ~~~~
// add pos, 1
// CMP
// cmp pos, array_size
// jl LOOP
//
fn build_foreach_loop(builder : &mut LtacBuilder, line : &AstStmt)  {
    let end_label = builder.label_stack.pop().unwrap();
    let loop_label = builder.label_stack.pop().unwrap();
    let cmp_label = builder.label_stack.pop().unwrap();
    
    builder.loop_labels.push(cmp_label.clone());
    builder.end_labels.push(end_label.clone());
    
    // First, build the index variable
    let index_var = line.args.first().unwrap();
    let array_var = line.args.last().unwrap();
    
    let index_name = index_var.str_val.clone();     // The name of the user's index variable
    let array_name = array_var.str_val.clone();     // The name of the array we are searching
    
    let array = match builder.get_var(&array_name) {
        Ok(v) => v,
        Err(_e) => {
            //TODO: Syntax error
            return;
        },
    };
    
    let data_type = array.sub_type.clone();
    let array_pos = array.pos;
    let array_size_pos = array_pos - 8;
    
    match data_type {
        DataType::Byte | DataType::UByte => builder.stack_pos += 1,
        DataType::Short | DataType::UShort => builder.stack_pos += 2,
        DataType::Int | DataType::UInt => builder.stack_pos += 4,
        DataType::Int64 | DataType::UInt64 => builder.stack_pos += 8,
        DataType::Char => builder.stack_pos += 1,
        DataType::Str => builder.stack_pos += 8,
        
        _ => {
            // TODO: Syntax error
            return;
        },
    }
    
    builder.stack_pos += 4;
    let index_pos = builder.stack_pos;
    
    let index = Var {
        pos : index_pos,
        data_type : data_type.clone(),
        sub_type : DataType::None,
        is_param : false,
    };
    
    builder.vars.insert(index_name, index);
    
    // Build another index variable to keep track of the size
    builder.stack_pos += 4;
    let size_pos = builder.stack_pos;
    
    let mut instr = ltac::create_instr(LtacType::Mov);
    instr.arg1 = LtacArg::Mem(size_pos);
    instr.arg2 = LtacArg::I32(0);
    builder.file.code.push(instr.clone());
    
    instr = ltac::create_instr(LtacType::Br);
    instr.name = cmp_label.clone();
    builder.file.code.push(instr.clone());
    
    // Start the loop
    let mut lbl = ltac::create_instr(LtacType::Label);
    lbl.name = loop_label.clone();
    builder.file.code.push(lbl);
    
    ///////////////////////////////////////
    // Load the index variable
    // mov r0, array[size_pos]
    // mov index, r0
    //
    if data_type == DataType::Str {
        instr = ltac::create_instr(LtacType::MovQ);
        instr.arg1 = LtacArg::Reg64(0);
        instr.arg2 = LtacArg::MemOffsetMem(array_pos, size_pos, 8);
        builder.file.code.push(instr.clone());
        
        instr.arg1 = LtacArg::Mem(index_pos);
        instr.arg2 = LtacArg::Reg64(0);
        builder.file.code.push(instr.clone());
    } else {
        let reg = reg_for_type(&data_type, &DataType::None, 0);
        
        instr = mov_for_type(&data_type, &DataType::None);
        instr.arg1 = reg.clone();
        instr.arg2 = LtacArg::MemOffsetMem(array_pos, size_pos, 4);     // TODO: Size should not be hard-coded
        builder.file.code.push(instr.clone());
        
        instr = mov_for_type(&data_type, &DataType::None);
        instr.arg1 = LtacArg::Mem(index_pos);
        instr.arg2 = reg.clone();
        builder.file.code.push(instr.clone());
    }
    
    ///////////////////////////////////////
    // Build the bottom of the loop block
    let mut cmp_block : Vec<LtacInstr> = Vec::new();
    
    // Increment the counter
    // mov r0, [size_pos]
    // add r0, 1
    // mov [size_pos], r0
    //
    instr = ltac::create_instr(LtacType::Mov);
    instr.arg1 = LtacArg::Reg32(0);
    instr.arg2 = LtacArg::Mem(size_pos);
    cmp_block.push(instr.clone());
    
    instr = ltac::create_instr(LtacType::I32Add);
    instr.arg1 = LtacArg::Reg32(0);
    instr.arg2 = LtacArg::I32(1);
    cmp_block.push(instr.clone());
    
    instr = ltac::create_instr(LtacType::Mov);
    instr.arg1 = LtacArg::Mem(size_pos);
    instr.arg2 = LtacArg::Reg32(0);
    cmp_block.push(instr.clone());
    
    // Comparison label
    let mut lbl2 = ltac::create_instr(LtacType::Label);
    lbl2.name = cmp_label.clone();
    cmp_block.push(lbl2);
    
    // Load the counter variable and the array size variables and compare    
    // mov r0, [size_pos]
    // cmp r0, [array_size_pos]
    //
    instr = ltac::create_instr(LtacType::Mov);
    instr.arg1 = LtacArg::Reg32(0);
    instr.arg2 = LtacArg::Mem(size_pos);
    cmp_block.push(instr.clone());
    
    let mut cmp_instr = ltac::create_instr(LtacType::I32Cmp);
    cmp_instr.arg1 = LtacArg::Reg32(0);
    cmp_instr.arg2 = LtacArg::Mem(array_size_pos);
    cmp_block.push(cmp_instr);
    
    // Now the branch instruction
    let mut br = ltac::create_instr(LtacType::Bl);
    br.name = loop_label.clone();
    cmp_block.push(br);
    
    // The end label
    let mut end_lbl = ltac::create_instr(LtacType::Label);
    end_lbl.name = end_label.clone();
    cmp_block.push(end_lbl);
    
    builder.code_stack.push(cmp_block);
}



// This file is part of the Dash compiler
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
use crate::ltac;
use crate::ltac::{LtacType, LtacArg};
use crate::ast::{AstStmt, AstArgType};

use crate::ltac_array::*;

// Builds an LTAC function call
pub fn build_func_call(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    let mut arg_type = LtacType::PushArg;
    let mut call_type = LtacType::Call;
    
    if line.name == "syscall" {
        arg_type = LtacType::KPushArg;
        call_type = LtacType::Syscall;
    }
    
    // Represents the current argument position
    let mut arg_no : i32 = 1;
    let mut flt_arg_no : i32 = 1;

    // Build the arguments
    for arg in line.args.iter() {
        match &arg.arg_type {
            AstArgType::ByteL => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1 = LtacArg::UByte(arg.u8_val);
                push.arg2_val = arg_no;
                builder.file.code.push(push);
                
                arg_no += 1;
            },
            
            AstArgType::ShortL => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1 = LtacArg::U16(arg.u16_val);
                push.arg2_val = arg_no;
                builder.file.code.push(push);
                
                arg_no += 1;
            },
            
            AstArgType::IntL => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1 = LtacArg::U32(arg.u64_val as u32);
                push.arg2_val = arg_no;
                builder.file.code.push(push);
                
                arg_no += 1;
            },
            
            AstArgType::FloatL if call_type == LtacType::Syscall => {
                builder.syntax.ltac_error(line, "Only integers and strings are valid in system calls.".to_string());
                return false;
            },
            
            AstArgType::FloatL => {
                let mut push = ltac::create_instr(LtacType::PushArg);
                let name = builder.build_float(arg.f64_val, false, false);
                push.arg1 = LtacArg::F32(name);
                push.arg2_val = flt_arg_no;
                builder.file.code.push(push);
                
                flt_arg_no += 1;  
            },
            
            AstArgType::StringL => {
                let name = builder.build_string(arg.str_val.clone());
                
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1 = LtacArg::PtrLcl(name);
                push.arg2_val = arg_no;
                builder.file.code.push(push);
                
                arg_no += 1;
            },
            
            AstArgType::Id => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg2_val = arg_no;
                
                // Check variables
                match &builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        push.arg1 = LtacArg::Mem(v.pos);
                        
                        if v.data_type == DataType::Byte || v.data_type == DataType::Char {
                            push.arg2 = LtacArg::Byte(0);
                            
                        } else if v.data_type == DataType::UByte {
                            push.arg2 = LtacArg::UByte(0);
                            
                        } else if v.data_type == DataType::Short {
                            push.arg2 = LtacArg::I16(0);
                            
                        } else if v.data_type == DataType::UShort {
                            push.arg2 = LtacArg::U16(0);
                            
                        } else if v.data_type == DataType::ByteDynArray || v.data_type == DataType::IntDynArray
                                || v.data_type == DataType::Str {
                            push.arg1 = LtacArg::Ptr(v.pos);
                            
                        } else if v.data_type == DataType::Int64 {
                            push.arg2 = LtacArg::I64(0);
                            
                        } else if v.data_type == DataType::UInt64 {
                            push.arg2 = LtacArg::U64(0);
                            
                        } else if v.data_type == DataType::Float {
                            push.arg2 = LtacArg::FltReg(flt_arg_no);
                            
                        } else if v.data_type == DataType::Double {
                            push.arg2 = LtacArg::FltReg64(flt_arg_no);
                        }
                        
                        // For the proper registers
                        if v.data_type == DataType::Float || v.data_type == DataType::Double {
                            push.arg2_val = flt_arg_no;
                            flt_arg_no += 1;
                        } else {
                            push.arg2_val = arg_no;
                            arg_no += 1;
                        }
                    },
                    
                    None => push.arg1 = LtacArg::Empty,
                }
                
                if push.arg1 != LtacArg::Empty {
                    builder.file.code.push(push);
                    continue;
                }
                
                // Check constants
                match builder.clone().global_consts.get(&arg.str_val) {
                    Some(c) => {
                        match c {
                            LtacArg::F32(_p) | LtacArg::F64(_p) => {
                                push.arg2_val = flt_arg_no;
                                flt_arg_no += 1;
                            },
                            
                            _ => {
                                push.arg2_val = arg_no;
                                arg_no += 1;
                            },
                        }
                        
                        push.arg1 = c.clone();
                        builder.file.code.push(push);
                    },
                    
                    None => {
                        let mut msg = "Invalid constant or variable name: ".to_string();
                        msg.push_str(&arg.str_val);
                        
                        builder.syntax.ltac_error(line, msg);
                        return false;
                    },
                }
            },
            
            _ => {},
        }
    }
    
    // Build the call
    let mut fc = ltac::create_instr(call_type);
    fc.name = line.name.clone();
    builder.file.code.push(fc);
    
    true
}

// Builds a function return
pub fn build_return(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    if line.args.len() > 0 && builder.current_type == DataType::Void {
        let mut msg = "Cannot return value in void function: ".to_string();
        msg.push_str(&builder.current_func);
         
        builder.syntax.ltac_error(line, msg);
        return false;
    }
    
    let mut to_ignore = String::new();
    if line.args.len() == 1 {
        let arg1 = line.args.first().unwrap();
        if arg1.arg_type == AstArgType::Id {
            to_ignore = arg1.str_val.clone();
        }
    }

    free_arrays(builder, to_ignore);

    if line.args.len() == 1 {
        let arg1 = line.args.first().unwrap();
        let mut mov = ltac::create_instr(LtacType::Mov);
        
        match &builder.current_type {
            DataType::Byte => {
                mov = ltac::create_instr(LtacType::MovB);
                mov.arg1 = LtacArg::RetRegI8;
            },
            
            DataType::UByte => {
                mov = ltac::create_instr(LtacType::MovUB);
                mov.arg1 = LtacArg::RetRegU8;
            },
            
            DataType::Short => {
                mov = ltac::create_instr(LtacType::MovW);
                mov.arg1 = LtacArg::RetRegI16;
            },
            
            DataType::UShort => {
                mov = ltac::create_instr(LtacType::MovUW);
                mov.arg1 = LtacArg::RetRegU16;
            },
            
            DataType::UInt => {
                mov = ltac::create_instr(LtacType::MovU);
                mov.arg1 = LtacArg::RetRegU32;
            },
            
            DataType::Int64 => {
                mov = ltac::create_instr(LtacType::MovQ);
                mov.arg1 = LtacArg::RetRegI64;
            },
            
            DataType::UInt64 => {
                mov = ltac::create_instr(LtacType::MovUQ);
                mov.arg1 = LtacArg::RetRegU64;
            },
            
            DataType::Float => {
                mov = ltac::create_instr(LtacType::MovF32);
                mov.arg1 = LtacArg::RetRegF32;
            },
            
            DataType::Double => {
                mov = ltac::create_instr(LtacType::MovF64);
                mov.arg1 = LtacArg::RetRegF64;
            },
            
            // TODO: We may want a different move for this
            DataType::Str => {
                mov = ltac::create_instr(LtacType::Mov);
                mov.arg1 = LtacArg::RetRegI64;
            },
            
            _ => mov.arg1 = LtacArg::RetRegI32,
        }
        
        match &arg1.arg_type {
            AstArgType::ByteL => {
                if builder.current_type == DataType::UByte {
                    mov.arg2 = LtacArg::UByte(arg1.u8_val);
                } else {
                    mov.arg2 = LtacArg::Byte(arg1.u8_val as i8);
                }
            },
            
            AstArgType::ShortL => {
                if builder.current_type == DataType::UShort {
                    mov.arg2 = LtacArg::U16(arg1.u16_val);
                } else {
                    mov.arg2 = LtacArg::I16(arg1.u16_val as i16);
                }
            },
        
            AstArgType::IntL => {
                match builder.current_type {
                    DataType::Int => mov.arg2 = LtacArg::I32(arg1.u64_val as i32),
                    DataType::UInt => mov.arg2 = LtacArg::U32(arg1.u64_val as u32),
                    DataType::Int64 => mov.arg2 = LtacArg::I64(arg1.u64_val as i64),
                    DataType::UInt64 => mov.arg2 = LtacArg::U64(arg1.u64_val),
                    
                    _ => {},
                }
            },
            
            AstArgType::FloatL => {
                if builder.current_type == DataType::Float {
                    let name = builder.build_float(arg1.f64_val, false, false);
                    mov.arg2 = LtacArg::F32(name);
                } else {
                    let name = builder.build_float(arg1.f64_val, true, false);
                    mov.arg2 = LtacArg::F64(name);
                }
            }
            
            AstArgType::StringL => {},
            
            AstArgType::Id => {
                match builder.vars.get(&arg1.str_val) {
                    Some(v) => mov.arg2 = LtacArg::Mem(v.pos),
                    None => {/* TODO: Syntax error */},
                }
            },
            
            _ => {},
        }
        
        builder.file.code.push(mov);
    } else if line.args.len() > 1 {
        // TODO
    }
    
    let ret = ltac::create_instr(LtacType::Ret);
    builder.file.code.push(ret);
    
    true
}

// Builds the exit keyword
pub fn build_exit(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    free_arrays(builder, String::new());
    
    let mut instr = ltac::create_instr(LtacType::Exit);
    instr.arg1 = LtacArg::I32(0);
    
    if line.args.len() == 1 {
        //TODO
    } else if line.args.len() > 1 {
        builder.syntax.ltac_error(line, "You can only have one argument in the \"exit\" statement.".to_string());
        return false;
    }
    
    builder.file.code.push(instr);

    true
}

// Builds the end of a block
pub fn build_end(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    if builder.block_layer == 0 {
        let last = builder.file.code.last().unwrap();
        
        if last.instr_type != LtacType::Ret && last.instr_type != LtacType::Exit {
            free_arrays(builder, String::new());
            
            // See if there was supposed to be a return instruction
            if builder.current_type != DataType::Void {
                let mut msg = "Expected return in function: ".to_string();
                msg.push_str(&builder.current_func);
                
                builder.syntax.ltac_error(line, msg);
                return false;
            }
            
            // Otherwise, create a void instruction
            let ret = ltac::create_instr(LtacType::Ret);
            builder.file.code.push(ret);
        }
    } else {
        builder.block_layer -= 1;
        
        if builder.loop_layer > 0 {
            builder.loop_layer -= 1;
            
            builder.end_labels.pop();
            builder.loop_labels.pop();
        }
        
        if builder.label_stack.len() > 0 {
            let mut label = ltac::create_instr(LtacType::Label);
            label.name = builder.label_stack.pop().unwrap();
            builder.file.code.push(label);
        }
        
        if builder.top_label_stack.len() > 0 {
            let mut label = ltac::create_instr(LtacType::Label);
            label.name = builder.top_label_stack.pop().unwrap();
            builder.file.code.push(label);
        }
        
        if builder.code_stack.len() > 0 {
            let sub_block = builder.code_stack.pop().unwrap();
            
            for item in sub_block.iter() {
                builder.file.code.push(item.clone());
            }
        }
    }
    
    true
}


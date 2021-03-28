
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


use crate::ltac_builder::*;
use crate::ltac;
use crate::ltac::{LtacType, LtacInstr, LtacArg};
use crate::ast::{DataType, AstStmt, AstArgType};

use crate::ltac_expr::*;

// Assigns a value to an array
pub fn build_array_assign(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    let var : Var;
    match builder.vars.get(&line.name) {
        Some(v) => var = v.clone(),
        None => return false,
    }
    
    let code = build_var_math(builder, &line, &var);
    
    code
}

// An internal function to free any dynamic arrays in the current context
pub fn free_arrays(builder : &mut LtacBuilder, ignore : String) {
    for (name, var) in &builder.vars {
        if var.data_type == DataType::Ptr && !var.is_param && *name != ignore {
            let mut pusharg = ltac::create_instr(LtacType::PushArg);
            pusharg.arg1 = LtacArg::Ptr(var.pos);
            pusharg.arg2_val = 1;
            builder.file.code.push(pusharg);
            
            let call = ltac::create_instr(LtacType::Free);
            builder.file.code.push(call);
        }
    }
}

// Initializes a an array in the heap
pub fn build_dyn_array(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) -> bool {
    let sub_args = &line.sub_args;
    let mut code = true;
    
    // Setup the store instruction, which holds the array size
    let mut size_instr = ltac::create_instr(LtacType::Mov);
    size_instr.arg1 = LtacArg::Mem(var.pos - 8);
    
    // Create the array
    if sub_args.len() == 1 && sub_args.last().unwrap().arg_type == AstArgType::IntL {
        let arg = sub_args.last().unwrap();
        let mut size = 4;
        
        if var.sub_type == DataType::Byte || var.sub_type == DataType::UByte {
            size = 1;
        } else if var.sub_type == DataType::Short || var.sub_type == DataType::UShort {
            size = 2;
        } else if  var.sub_type == DataType::Int64 || var.sub_type == DataType::UInt64
                || var.sub_type == DataType::Double || var.sub_type == DataType::Str {
            size = 8;
        }
        
        let mut pusharg = ltac::create_instr(LtacType::PushArg);
        pusharg.arg1 = LtacArg::I32((arg.u64_val as i32) * size);
        pusharg.arg2_val = 1;
        
        builder.file.code.push(pusharg);
        
        let mut instr = ltac::create_instr(LtacType::Malloc);
        builder.file.code.push(instr);
        
        // Move the return register back to the variable
        instr = ltac::create_instr(LtacType::MovQ);
        instr.arg1 = LtacArg::Mem(var.pos);
        instr.arg2 = LtacArg::RetRegI64;
        builder.file.code.push(instr);
        
        // Store the size
        size_instr.arg2 = LtacArg::I32(arg.u64_val as i32);
        builder.file.code.push(size_instr);
        
    // An array with a variable as the size
    } else if sub_args.len() == 1 && sub_args.last().unwrap().arg_type == AstArgType::Id {
        let arg = sub_args.last().unwrap();
        let size = 1;
        let data_type : DataType;
        let pos : i32;
        
        match &builder.vars.get(&arg.str_val) {
            Some(v) => {
                data_type = v.data_type.clone();
                pos = v.pos;
            }
            
            None => {
                builder.syntax.ltac_error(line, "Invalid identifier".to_string());
                return false;
            },
        }
        
        if data_type != DataType::Int && data_type != DataType::UInt {
            builder.syntax.ltac_error(line, "Array size can only be set with integer values or variables.".to_string());
            return false;
        }
        
        // Instruction syntax:
        // mov u32.r0, [pos]
        // imul u32.r0, size
        // pusharg u32
        // call malloc
        
        let mut instr : LtacInstr;
        
        if size > 1 {
            instr = ltac::create_instr(LtacType::MovU);
            instr.arg1 = LtacArg::Reg32(0);
            instr.arg2 = LtacArg::Mem(pos);
            builder.file.code.push(instr.clone());
            
            instr = ltac::create_instr(LtacType::U32Mul);
            instr.arg1 = LtacArg::Reg32(0);
            instr.arg2 = LtacArg::U32(size);
            builder.file.code.push(instr.clone());
            
            // Store the size
            size_instr.arg2 = LtacArg::Reg32(0);
            builder.file.code.push(size_instr);
            
            // Prepare for the call
            instr = ltac::create_instr(LtacType::PushArg);
            instr.arg1 = LtacArg::Reg32(0);
            instr.arg2_val = 1;
            builder.file.code.push(instr.clone());
        } else {
            // Store the size
            let mut instr2 = ltac::create_instr(LtacType::Mov);
            instr2.arg1 = LtacArg::Reg32(0);
            instr2.arg2 = LtacArg::Mem(pos);
            builder.file.code.push(instr2);
            
            size_instr.arg2 = LtacArg::Reg32(0);
            builder.file.code.push(size_instr);
        
            // Prepare for the call
            instr = ltac::create_instr(LtacType::PushArg);
            instr.arg1 = LtacArg::Mem(pos);
            instr.arg2 = LtacArg::Reg32(0);
            instr.arg2_val = 1;
            builder.file.code.push(instr.clone());
        }
        
        instr = ltac::create_instr(LtacType::Malloc);
        builder.file.code.push(instr);
        
        // Move the return register back to the variable
        instr = ltac::create_instr(LtacType::MovQ);
        instr.arg1 = LtacArg::Mem(var.pos);
        instr.arg2 = LtacArg::RetRegI64;
        builder.file.code.push(instr);
        
    // Vector math
    } else if sub_args.len() == 0 && line.args.len() > 1 {
        code = build_i32array_vector_math(builder, line, var);
    } else {
        //TODO
    }
    
    code
}

// Builds integer vector math
pub fn build_i32array_vector_math(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) -> bool {
    let mut instr = ltac::create_instr(LtacType::MovI32Vec);
    instr.arg1 = LtacArg::Reg32(0);
    
    // The last loaded memory position
    let mut last_pos = 0;

    // TODO: This code is kind of awful
    for arg in line.args.iter() {
        match &arg.arg_type {
            AstArgType::Id => {
                match &builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        instr.arg2 = LtacArg::Mem(v.pos);
                        
                        if arg.sub_args.len() > 0 {
                            let first_arg = arg.sub_args.last().unwrap();
                            
                            if arg.sub_args.len() == 1 {
                                if first_arg.arg_type == AstArgType::IntL {
                                    let offset = (first_arg.u64_val as i32) * 4;
                                    instr.arg2 = LtacArg::MemOffsetImm(v.pos, offset);
                                } else if first_arg.arg_type == AstArgType::Id {
                                    match &builder.vars.get(&first_arg.str_val) {
                                        Some(v2) => instr.arg2 = LtacArg::MemOffsetMem(v.pos, v2.pos, 4),
                                        None => {
                                            builder.syntax.ltac_error(line, "Invalid offset variable.".to_string());
                                            return false;
                                        },
                                    }
                                }
                            } else {
                                //TODO
                            }
                        }
                        
                        // This is a flag, sort of; if we set it, we do not want to reload
                        // the memory location
                        if v.pos == last_pos {
                            instr.arg2_val = -1;
                        }
                        
                        last_pos = v.pos;
                    },
                    
                    None => {
                        builder.syntax.ltac_error(line, "Invalid variable.".to_string());
                        return false;
                    },
                }
                
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::OpAdd => {
                instr = ltac::create_instr(LtacType::I32VAdd);
                instr.arg1 = LtacArg::Reg32(0);
            },
            
            _ => {
                builder.syntax.ltac_error(line, "Invalid expression for vector math.".to_string());
                return false;
            }
        }
    }
    
    // The final move instruction
    instr = ltac::create_instr(LtacType::MovI32Vec);
    instr.arg1 = LtacArg::Mem(var.pos);
    instr.arg2 = LtacArg::Reg32(0);
    
    builder.file.code.push(instr.clone());
    
    true
}


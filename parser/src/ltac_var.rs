//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use crate::ltac_builder::*;
use crate::ast;
use crate::ast::*;
use crate::ltac;
use crate::ltac::{LtacInstr, LtacType, LtacArg};

use crate::ltac_expr::*;
use crate::ltac_array::*;
use crate::ltac_func::*;
use crate::ltac_utils::*;

// Builds an LTAC variable declaration
// Note for array sizes:
//    Array sizes are 12 bytes long.
//    The first 8 bytes hold the pointer, and the second 4 hold the size
//
pub fn build_var_dec(builder : &mut LtacBuilder, line : &AstStmt, arg_no_o : i32, flt_arg_no_o : i32) -> (bool, i32, i32) {
    let mut arg_no = arg_no_o;
    let mut flt_arg_no = flt_arg_no_o;
    
    let name = line.name.clone();
    let data_type = line.data_type.clone();
    let sub_type = line.sub_type.clone();
    
    match &line.data_type {
        DataType::Byte | DataType::UByte => builder.stack_pos += 1,
        
        DataType::Short | DataType::UShort => builder.stack_pos += 2,
    
        DataType::Int | DataType::UInt => builder.stack_pos += 4,

        DataType::Int64 | DataType::UInt64 => builder.stack_pos += 8,
        
        DataType::Float => builder.stack_pos += 4,
        DataType::Double => builder.stack_pos += 8,
        
        DataType::Char => builder.stack_pos += 1,
        DataType::Str => builder.stack_pos += 8,
        
        DataType::Ptr => builder.stack_pos += 12,
        
        // TODO: We will need better type detection
        DataType::Enum(_) => builder.stack_pos += 4,
        
        DataType::Void => {},
        
        // Do we need an error here? Really, it should never get to this pointer
        DataType::None => return (false, arg_no, flt_arg_no),
    }
    
    let mut is_param = false;
    if arg_no > 0 {
        is_param = true;
    }
    
    let v = Var {
        pos : builder.stack_pos,
        data_type : data_type,
        sub_type : sub_type,
        is_param : is_param,
    };
    
    builder.vars.insert(name, v);
    
    // If we have a function argument, add the load instruction
    if is_param {
        let data_type = line.data_type.clone();
        let mem = LtacArg::Mem(builder.stack_pos);
        let ld : LtacInstr;
        
        if data_type == DataType::Float || data_type == DataType::Double {
            ld = ldarg_for_type(&data_type, mem, flt_arg_no);
            flt_arg_no += 1;
        } else {
            ld = ldarg_for_type(&data_type, mem, arg_no);
            arg_no += 1;
            
            // If we have a pointer, make sure to load the size
            if data_type == DataType::Ptr {
                let mut arg2 = ltac::create_instr(LtacType::LdArgI32);
                arg2.arg1 = LtacArg::Mem(builder.stack_pos - 8);
                arg2.arg2_val = arg_no;
                builder.file.code.push(arg2);
                
                arg_no += 1;
            }
        }
        
        builder.file.code.push(ld);
    } else {
        if !build_var_assign(builder, line) {
            return (false, arg_no, flt_arg_no);
        }
    }
    
    (true, arg_no, flt_arg_no)
}

// Builds an LTAC variable assignment
pub fn build_var_assign(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    let var : Var;
    match builder.vars.get(&line.name) {
        Some(v) => var = v.clone(),
        None => {
            builder.syntax.ltac_error(line, "Invalid variable.".to_string());
            return false;
        },
    }
    
    let code : bool;
    
    if var.data_type == DataType::Ptr {
        code = build_dyn_array(builder, &line, &var);
    } else if var.data_type == DataType::Str {
        code = build_str_assign(builder, &line, &var);
    } else {
        code = build_var_math(builder, &line, &var);
    }
    
    code
}

// Builds a string variable assignment
// TODO: I want to consider merging this with the rest of the expression builder
pub fn build_str_assign(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) -> bool {
    let mut instr = ltac::create_instr(LtacType::MovQ);
    
    if line.args.len() == 1 {
        let arg = line.args.first().unwrap();
        
        instr.arg1 = LtacArg::Mem(var.pos);
        
        match &arg.arg_type {
            AstArgType::StringL => {
                let name = builder.build_string(arg.str_val.clone());
                instr.arg2 = LtacArg::PtrLcl(name);
            },
            
            // Build an ID value based on a variable
            AstArgType::Id if builder.var_exists(&arg.str_val) => {
                let v = match &builder.get_var(&arg.str_val) {
                    Ok(v) => v.clone(),
                    Err(_e) => return false,
                };
            
                if v.data_type != DataType::Str && v.sub_type != DataType::Str
                        && v.sub_type != DataType::Byte && v.sub_type != DataType::UByte {
                    builder.syntax.ltac_error(line, "You can only assign a string to a string.".to_string());
                    return false;
                } else if v.data_type == DataType::Ptr && v.sub_type == DataType::Str {
                    let mut instr2 = ltac::create_instr(LtacType::MovQ);
                    instr2.arg1 = LtacArg::Reg64(0);
                    instr2.arg2 = LtacArg::Mem(v.pos);
                    
                    if arg.sub_args.len() > 0 {
                        let first_arg = arg.sub_args.last().unwrap();
                        let size = 8;
                    
                        if arg.sub_args.len() == 1 {
                            if first_arg.arg_type == AstArgType::IntL {
                                let offset = (first_arg.u64_val as i32) * size;
                                instr2.arg2 = LtacArg::MemOffsetImm(v.pos, offset);
                            } else if first_arg.arg_type == AstArgType::Id {
                                match &builder.get_var(&first_arg.str_val) {
                                    Ok(v2) => instr2.arg2 = LtacArg::MemOffsetMem(v.pos, v2.pos, size),
                                    Err(_e) => {
                                        builder.syntax.ltac_error2("Invalid offset variable.".to_string());
                                        return false;
                                    },
                                };
                            }
                        }
                    }
                    
                    builder.file.code.push(instr2);
                } else {
                    let mut instr2 = ltac::create_instr(LtacType::MovQ);
                    instr2.arg1 = LtacArg::Reg64(0);
                    instr2.arg2 = LtacArg::Mem(v.pos);
                    builder.file.code.push(instr2);
                }
                
                instr.arg2 = LtacArg::Reg64(0);
            },
            
            AstArgType::Id => {
                match &builder.clone().functions.get(&arg.str_val) {
                    Some(t) => {
                        // TODO: Better detection with whether its byte or ubyte
                        if **t != DataType::Str && **t != DataType::Ptr {
                            builder.syntax.ltac_error(line, "You can only assign string or byte arrays to string variables.".to_string());
                            return false;
                        }
                        
                        instr.arg2 = LtacArg::RetRegI64;
                        
                        // Create a statement to build the rest of the function call
                        let mut stmt = ast::create_orphan_stmt(AstStmtType::FuncCall);
                        stmt.name = arg.str_val.clone();
                        stmt.args = arg.sub_args.clone();
                        build_func_call(builder, &stmt);
                    },
                    
                    None => {
                        builder.syntax.ltac_error(line, "Invalid string variable.".to_string());
                        return false;
                    },
                }
            },
            
            _ => {
                builder.syntax.ltac_error(line, "Invalid string assignment.".to_string());
                return false;
            },
        }
    } else {
        //TODO
    }
    
    builder.file.code.push(instr);
    
    true
}

    

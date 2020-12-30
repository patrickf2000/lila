
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


// The main expression builder for the LTAC layer

use std::mem;

use crate::ast;
use crate::ast::{AstStmt, AstArg, AstStmtType, AstArgType};
use crate::ltac;
use crate::ltac::{LtacType, LtacArg, LtacInstr};

use crate::ltac_builder::*;
use crate::ltac_func::*;
use crate::ltac_utils::*;

// Builds assignments for numerical variables
pub fn build_var_math(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) -> bool {
    builder.syntax.set_data(line);

    let args = &line.args;
    let first_type = args.first().unwrap().arg_type.clone();
    let reg_no = 1;
    
    if !build_var_expr(builder, args, var, reg_no) {
        return false;
    }
    
    let mut instr : LtacInstr;
    
    //Store the result back
    // If it was a single assign (no math), compact the instructions
    if line.args.len() == 1 && first_type != AstArgType::Id {
        let top = builder.file.code.pop().unwrap();
        
        instr = ltac::create_instr(top.instr_type);
        instr.arg1 = LtacArg::Mem(var.pos);
        instr.arg2 = top.arg2;
        instr.arg2_val = top.arg2_val;
        
    } else {
        instr = mov_for_type(&var.data_type, &var.sub_type);
        instr.arg1 = LtacArg::Mem(var.pos);
        instr.arg2 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
    }
    
    // If we have an array, there's additional work
    if line.sub_args.len() > 0 && var.data_type == DataType::Ptr {
        let first_arg = line.sub_args.last().unwrap();
        let mut offset_size = 4;
        
        if var.sub_type == DataType::Byte|| var.sub_type == DataType::UByte {
            offset_size = 1;
        } else if var.sub_type == DataType::Short || var.sub_type == DataType::UShort {
            offset_size = 2;
        } else if var.sub_type == DataType::Int64 || var.sub_type == DataType::UInt64
            || var.sub_type == DataType::Double || var.sub_type == DataType::Str {
            offset_size = 8;
        }
        
        if line.sub_args.len() == 1 {
            if first_arg.arg_type == AstArgType::IntL {
                let offset = (first_arg.u64_val as i32) * offset_size;
                instr.arg1 = LtacArg::MemOffsetImm(var.pos, offset);
            } else if first_arg.arg_type == AstArgType::Id {
                match builder.vars.get(&first_arg.str_val) {
                    Some(v) => instr.arg1 = LtacArg::MemOffsetMem(var.pos, v.pos, offset_size),
                    None => {
                        builder.syntax.ltac_error2("Invalid offset variable.".to_string());
                        return false;
                    },
                }
            }
        } else {
            // We create a dummy variable so the positional math is done as integers
            let var2 = Var {
                pos : 0,
                data_type : DataType::Int,
                sub_type : DataType::None,
                is_param : false,
            };
            
            build_var_expr(builder, &line.sub_args, &var2, 0);
            instr.arg1 = LtacArg::MemOffsetReg(var.pos, 0, offset_size);
        }
    }
    
    builder.file.code.push(instr);
    
    true
}

// TODO: I would eventually like to get rid of the "line" parameter
// Doing so may require work in the ltac_builder module.
fn build_var_expr(builder : &mut LtacBuilder, args : &Vec<AstArg>, var : &Var, reg_no : i32) -> bool {

    // The control variable for negatives
    let mut negate_next = false;
    
    let mut instr = mov_for_type(&var.data_type, &var.sub_type);
    instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
    
    // Control variables for sub-expressions
    let mut is_sub_expr = false;
    let mut sub_expr : Vec<AstArg> = Vec::new();
    let mut layer = 1;
    
    for arg in args.iter() {
        if is_sub_expr {
            if arg.arg_type == AstArgType::OpLParen {
                layer += 1;
                sub_expr.push(arg.clone());
                continue;
            } else if arg.arg_type == AstArgType::OpRParen && layer > 1 {
                layer -= 1;
                sub_expr.push(arg.clone());
                continue;
            } else if arg.arg_type != AstArgType::OpRParen {
                sub_expr.push(arg.clone());
                continue;
            }
        }
    
        match &arg.arg_type {
            // Parantheses
            // Left paren- increment working register
            AstArgType::OpLParen => is_sub_expr = true,
            
            // Right paren- decrement working register
            AstArgType::OpRParen => {
                build_var_expr(builder, &sub_expr, var, reg_no+1);
                
                instr.arg2 = reg_for_type(&var.data_type, &var.sub_type, reg_no+1);
                builder.file.code.push(instr.clone());
                
                sub_expr = Vec::new();
                is_sub_expr = false;
            },
        
            // Assign byte literals
            AstArgType::ByteL => {
                if negate_next {
                    builder.syntax.ltac_error2("Negation invalid for this type.".to_string());
                    return false;
                }
            
                if var.data_type == DataType::Byte || var.sub_type == DataType::Byte {
                    instr.arg2 = LtacArg::Byte(arg.u8_val as i8);
                } else if var.data_type == DataType::UByte || var.sub_type == DataType::UByte {
                    instr.arg2 = LtacArg::UByte(arg.u8_val);
                } else {
                    builder.syntax.ltac_error2("Invalid use of byte literal.".to_string());
                    return false;
                }
                
                builder.file.code.push(instr.clone());
            },
            
            // Assign short literals
            AstArgType::ShortL => {
                if negate_next {
                    builder.syntax.ltac_error2("Negation invalid for this type.".to_string());
                    return false;
                }
                
                if var.data_type == DataType::Short || var.sub_type == DataType::Short {
                    instr.arg2 = LtacArg::I16(arg.u16_val as i16);
                } else if var.data_type == DataType::UShort || var.sub_type == DataType::UShort {
                    instr.arg2 = LtacArg::U16(arg.u16_val);
                } else {
                    builder.syntax.ltac_error2("Invalid use of short literal.".to_string());
                    return false;
                }
                    
                builder.file.code.push(instr.clone());
            },
        
            // ===============================================================
            // Assign integer literals
            
            AstArgType::IntL => {
                // Bytes
                if var.data_type == DataType::Byte || var.data_type == DataType::Char
                    || var.sub_type == DataType::Byte {
                    let val = arg.u64_val as i32;
                    
                    // TODO: Why the hell is this an error? Its getting thrown when using char values
                    /*if mem::size_of::<i8>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into byte.".to_string());
                        return false;
                    }*/
                    
                    let parts = unsafe { mem::transmute::<i32, [i8; 4]>(val) };
                    let mut result = parts[0];
                    
                    if negate_next {
                        result = -result;
                        negate_next = false;
                    }
                    
                    instr.arg2 = LtacArg::Byte(result);
                    builder.file.code.push(instr.clone());
                    
                // UByte
                } else if var.data_type == DataType::UByte || var.sub_type == DataType::UByte {
                    let val = arg.u64_val as u32;
                    
                    // TODO: Why are we getting this error?
                    /*if mem::size_of::<u8>() < (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into ubyte.".to_string());
                        return false;
                    }*/
                    
                    let parts = unsafe { mem::transmute::<u32, [u8; 4]>(val) };
                    let result = parts[0];
                    
                    instr.arg2 = LtacArg::UByte(result);
                    builder.file.code.push(instr.clone());
                    
                // Short
                } else if var.data_type == DataType::Short || var.sub_type == DataType::Short {
                    let val = arg.u64_val as i32;
                    
                    /*if mem::size_of::<u16>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into short.".to_string());
                        return false;
                    }*/
                    
                    let parts = unsafe { mem::transmute::<i32, [i16; 2]>(val) };
                    let mut result = parts[0];
                    
                    if negate_next {
                        result = -result;
                        negate_next = false;
                    }
                    
                    instr.arg2 = LtacArg::I16(result);
                    builder.file.code.push(instr.clone());
                    
                // UShort
                } else if var.data_type == DataType::UShort || var.sub_type == DataType::UShort {
                    let val = arg.u64_val as u32;
                    
                    /*if mem::size_of::<u16>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into ushort.".to_string());
                        return false;
                    }*/
                    
                    let parts = unsafe { mem::transmute::<u32, [u16; 2]>(val) };
                    let result = parts[0];
                    
                    instr.arg2 = LtacArg::U16(result);
                    builder.file.code.push(instr.clone());
                    
                // Integers and integer arrays
                } else if var.data_type == DataType::Int || var.sub_type == DataType::Int {
                    let mut val = arg.u64_val as i32;
                    
                    if negate_next {
                        val = -val;
                        negate_next = false;
                    }
                    
                    instr.arg2 = LtacArg::I32(val);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::UInt || var.sub_type == DataType::UInt {
                    instr.arg2 = LtacArg::U32(arg.u64_val as u32);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::Int64  || var.sub_type == DataType::Int64 {
                    let mut val = arg.u64_val as i64;
                    
                    if negate_next {
                        val = -val;
                        negate_next = false;
                    }
                    
                    instr.arg2 = LtacArg::I64(val);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::UInt64 || var.sub_type == DataType::UInt64 {
                    instr.arg2 = LtacArg::U64(arg.u64_val);
                    builder.file.code.push(instr.clone());
                    
                // Invalid
                } else {
                    builder.syntax.ltac_error2("Invalid use of integer.".to_string());
                    return false;
                }
                
                // If the negate flag is still active at this point, we used it in the wrong place.
                if negate_next {
                    builder.syntax.ltac_error2("Negation invalid for this type.".to_string());
                    return false;
                }
            },
            
            // ===============================================================
            // Assign float literals
            
            AstArgType::FloatL => {
                if var.data_type == DataType::Float || var.sub_type == DataType::Float {
                    let name = builder.build_float(arg.f64_val, false, negate_next);
                    instr.arg2 = LtacArg::F32(name);
                    builder.file.code.push(instr.clone());
                
                } else if var.data_type == DataType::Double || var.sub_type == DataType::Double {
                    let name = builder.build_float(arg.f64_val, true, negate_next);
                    instr.arg2 = LtacArg::F64(name);
                    builder.file.code.push(instr.clone());
                    
                } else {
                    builder.syntax.ltac_error2("Invalid use of float literal.".to_string());
                    return false;
                }
                
                negate_next = false;
            },
            
            // ===============================================================
            // Strings and characters
            
            AstArgType::CharL => {
                if var.data_type == DataType::Char || var.data_type == DataType::Byte {
                    instr.arg2 = LtacArg::Byte(arg.char_val as i8);
                    builder.file.code.push(instr.clone());
                    
                } else {
                    builder.syntax.ltac_error2("Invalid use of char literal.".to_string());
                }
            },
            
            AstArgType::StringL => {
                let name = builder.build_string(arg.str_val.clone());
                
                let mut instr2 = ltac::create_instr(LtacType::MovQ);
                instr2.arg1 = LtacArg::Reg64(0);
                instr2.arg2 = LtacArg::PtrLcl(name); 
                builder.file.code.push(instr2);
                
                instr.arg2 = LtacArg::Reg64(0);
                builder.file.code.push(instr.clone());
            },
            
            // ===============================================================
            // Variables and functions
            
            AstArgType::Id if builder.var_exists(&arg.str_val) => {
                if !build_expr_var(builder, &arg, &var, reg_no, negate_next, &mut instr) {
                    return false;
                }
                  
                negate_next = false;  
            },
            
            // System calls
            AstArgType::Id if arg.str_val == "syscall" => {
                
                let mut stmt = ast::create_orphan_stmt(AstStmtType::FuncCall);
                stmt.name = arg.str_val.clone();
                stmt.args = arg.sub_args.clone();
                build_func_call(builder, &stmt);
                
                if var.data_type == DataType::Int || var.data_type == DataType::UInt {
                    instr.arg2 = LtacArg::RetRegI32;
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64 {
                    instr.arg2 = LtacArg::RetRegI64;
                } else {
                    builder.syntax.ltac_error2("You can only assign system call returns to integers.".to_string());
                    return false;
                }
                
                builder.file.code.push(instr.clone());
            },
            
            // Function calls
            AstArgType::Id if builder.function_exists(&arg.str_val) => {
                if !build_expr_func_call(builder, &arg, &var, reg_no, &mut instr) {
                    return false;
                }
            },
            
            // Constants
            AstArgType::Id if builder.const_exists(&arg.str_val) => {
                // Check constants
                let const_arg = match builder.get_const(&arg.str_val) {
                    Ok(c) => c.clone(),
                    Err(_e) => return false,             // We shouldn't get here
                };
                
                instr.arg2 = const_arg;
                builder.file.code.push(instr.clone());
            }
            
            // Check enumerations, and throw an error if there is no such thing
            AstArgType::Id => {
                
                // Check enumerated values
                // TODO: I don't like this
                match var.data_type {
                    DataType::Enum(ref name) => {
                        match builder.clone().enums.get(name) {
                            Some(t) => {
                                let num = match t.values.get(&arg.str_val) {
                                    Some(n) => *n,
                                    None => 0,
                                };
                                
                                instr.arg2 = LtacArg::I32(num);
                            },
                    
                            None => instr.arg2 = LtacArg::Empty,
                        }
                    },
                    
                    _ => {},
                }
                
                if instr.arg2 != LtacArg::Empty {
                    builder.file.code.push(instr.clone());
                    continue;
                }
                
                // If we get to this point, throw an error
                let mut msg = "Invalid function, constant, or variable name: ".to_string();
                msg.push_str(&arg.str_val);
            
                builder.syntax.ltac_error2(msg);
                return false;
            },
            
            // Ldarg statement
            // Format position (sub_arg[0]), data_type (sub_modifiers[0])
            
            AstArgType::LdArg => {
                let position_arg = arg.sub_args.first().unwrap();
                let position = position_arg.u64_val as i32;
                
                let ast_data_type = arg.sub_modifiers.first().unwrap();
                let (data_type, sub_type) = ast_to_datatype(&ast_data_type);
                let reg = reg_for_type(&data_type, &sub_type, reg_no+1);
                
                let ld_instr = ldarg_for_type(&data_type, reg.clone(), position);
                builder.file.code.push(ld_instr);
                
                instr.arg2 = reg;
                builder.file.code.push(instr.clone());
            },
            
            // Sizeof statement
            // To get the size, get the array variable, and the size is stored in the upper 4 bytes
            
            AstArgType::Sizeof => {
                let name_arg = arg.sub_args.first().unwrap();
                let array_var = match builder.get_var(&name_arg.str_val) {
                    Ok(v) if v.data_type == DataType::Ptr => v,
                    
                    Ok(_v) => {
                        builder.syntax.ltac_error2("Sizeof can only be used with arrays and strings.".to_string());
                        return false;
                    },
                    
                    Err(_e) => {
                        builder.syntax.ltac_error2("Unknown array or string.".to_string());
                        return false;
                    },
                };
                
                let pos = array_var.pos - 8;
                let reg = reg_for_type(&var.data_type, &DataType::None, reg_no);
                
                let mut instr2 = mov_for_type(&var.data_type, &DataType::None);
                instr2.arg1 = reg.clone();
                instr2.arg2 = LtacArg::Mem(pos);
                builder.file.code.push(instr2);
                
                instr.arg2 = reg;
                builder.file.code.push(instr.clone());
            },
            
            // Addrof statement
            
            AstArgType::AddrOf => {
                let name_arg = arg.sub_args.first().unwrap();
                let ref_var = match builder.get_var(&name_arg.str_val) {
                    Ok(v) => v,
                    
                    Err(_e) => {
                        builder.syntax.ltac_error2("Unknown variable reference.".to_string());
                        return false;
                    },
                };
                
                let mut instr2 = ltac::create_instr(LtacType::LdAddr);
                instr2.arg1 = LtacArg::Reg64(reg_no);
                instr2.arg2 = LtacArg::Mem(ref_var.pos);
                builder.file.code.push(instr2);
                
                instr.arg2 = LtacArg::Reg64(reg_no);
                builder.file.code.push(instr.clone());
            },
            
            // Negate operator
            // Basically, we set a control variable. That way, if the next AST node is a literal, we simply
            // negate it here. If its a variable, we can create a subtraction operations
            
            AstArgType::OpNeg => negate_next = true,
            
            // Addition
            
            AstArgType::OpAdd => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Add),
                    DataType::UByte => instr = ltac::create_instr(LtacType::U8Add),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Add),
                    DataType::UShort => instr = ltac::create_instr(LtacType::U16Add),
                    DataType::Int => instr = ltac::create_instr(LtacType::I32Add),
                    DataType::UInt => instr = ltac::create_instr(LtacType::U32Add),
                    DataType::Int64 => instr = ltac::create_instr(LtacType::I64Add),
                    DataType::UInt64 => instr = ltac::create_instr(LtacType::U64Add),
                    DataType::Float => instr = ltac::create_instr(LtacType::F32Add),
                    DataType::Double => instr = ltac::create_instr(LtacType::F64Add),
                    
                    DataType::Ptr if var.sub_type == DataType::Byte => instr = ltac::create_instr(LtacType::I8Add),
                    DataType::Ptr if var.sub_type == DataType::UByte => instr = ltac::create_instr(LtacType::U8Add),
                    DataType::Ptr if var.sub_type == DataType::Short => instr = ltac::create_instr(LtacType::I16Add),
                    DataType::Ptr if var.sub_type == DataType::UShort => instr = ltac::create_instr(LtacType::U16Add),
                    DataType::Ptr if var.sub_type == DataType::Int => instr = ltac::create_instr(LtacType::I32Add),
                    DataType::Ptr if var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::U32Add),
                    DataType::Ptr if var.sub_type == DataType::Int64 => instr = ltac::create_instr(LtacType::I64Add),
                    DataType::Ptr if var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::U64Add),
                    DataType::Ptr if var.sub_type == DataType::Float => instr = ltac::create_instr(LtacType::F32Add),
                    DataType::Ptr if var.sub_type == DataType::Double => instr = ltac::create_instr(LtacType::F64Add),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of addition operator.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Subtraction
            
            AstArgType::OpSub => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Sub),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Sub),
                    DataType::Int => instr = ltac::create_instr(LtacType::I32Sub),
                    DataType::Int64 => instr = ltac::create_instr(LtacType::I64Sub),
                    DataType::Float => instr = ltac::create_instr(LtacType::F32Sub),
                    DataType::Double => instr = ltac::create_instr(LtacType::F64Sub),
                    
                    DataType::Ptr if var.sub_type == DataType::Byte => instr = ltac::create_instr(LtacType::I8Sub),
                    DataType::Ptr if var.sub_type == DataType::Short => instr = ltac::create_instr(LtacType::I16Sub),
                    DataType::Ptr if var.sub_type == DataType::Int => instr = ltac::create_instr(LtacType::I32Sub),
                    DataType::Ptr if var.sub_type == DataType::Int64 => instr = ltac::create_instr(LtacType::I64Sub),
                    DataType::Ptr if var.sub_type == DataType::Float => instr = ltac::create_instr(LtacType::F32Sub),
                    DataType::Ptr if var.sub_type == DataType::Double => instr = ltac::create_instr(LtacType::F64Sub),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of subtraction operator.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Multiplication
            
            AstArgType::OpMul => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Mul),
                    DataType::UByte => instr = ltac::create_instr(LtacType::U8Mul),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Mul),
                    DataType::UShort => instr = ltac::create_instr(LtacType::U16Mul),
                    DataType::Int => instr = ltac::create_instr(LtacType::I32Mul),
                    DataType::UInt => instr = ltac::create_instr(LtacType::U32Mul),
                    DataType::Int64 => instr = ltac::create_instr(LtacType::I64Mul),
                    DataType::UInt64 => instr = ltac::create_instr(LtacType::U64Mul),
                    DataType::Float => instr = ltac::create_instr(LtacType::F32Mul),
                    DataType::Double => instr = ltac::create_instr(LtacType::F64Mul),
                    
                    DataType::Ptr if var.sub_type == DataType::Byte => instr = ltac::create_instr(LtacType::I8Mul),
                    DataType::Ptr if var.sub_type == DataType::UByte => instr = ltac::create_instr(LtacType::U8Mul),
                    DataType::Ptr if var.sub_type == DataType::Short => instr = ltac::create_instr(LtacType::I16Mul),
                    DataType::Ptr if var.sub_type == DataType::UShort => instr = ltac::create_instr(LtacType::U16Mul),
                    DataType::Ptr if var.sub_type == DataType::Int => instr = ltac::create_instr(LtacType::I32Mul),
                    DataType::Ptr if var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::U32Mul),
                    DataType::Ptr if var.sub_type == DataType::Int64 => instr = ltac::create_instr(LtacType::I64Mul),
                    DataType::Ptr if var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::U64Mul),
                    DataType::Ptr if var.sub_type == DataType::Float => instr = ltac::create_instr(LtacType::F32Mul),
                    DataType::Ptr if var.sub_type == DataType::Double => instr = ltac::create_instr(LtacType::F64Mul),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of multiplication operator.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Division
            
            AstArgType::OpDiv => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Div),
                    DataType::UByte => instr = ltac::create_instr(LtacType::U8Div),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Div),
                    DataType::UShort => instr = ltac::create_instr(LtacType::U16Div),
                    DataType::Int => instr = ltac::create_instr(LtacType::I32Div),
                    DataType::UInt => instr = ltac::create_instr(LtacType::U32Div),
                    DataType::Int64 => instr = ltac::create_instr(LtacType::I64Div),
                    DataType::UInt64 => instr = ltac::create_instr(LtacType::U64Div),
                    DataType::Float => instr = ltac::create_instr(LtacType::F32Div),
                    DataType::Double => instr = ltac::create_instr(LtacType::F64Div),
                    
                    DataType::Ptr if var.sub_type == DataType::Byte => instr = ltac::create_instr(LtacType::I8Div),
                    DataType::Ptr if var.sub_type == DataType::UByte => instr = ltac::create_instr(LtacType::U8Div),
                    DataType::Ptr if var.sub_type == DataType::Short => instr = ltac::create_instr(LtacType::I16Div),
                    DataType::Ptr if var.sub_type == DataType::UShort => instr = ltac::create_instr(LtacType::U16Div),
                    DataType::Ptr if var.sub_type == DataType::Int => instr = ltac::create_instr(LtacType::I32Div),
                    DataType::Ptr if var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::U32Div),
                    DataType::Ptr if var.sub_type == DataType::Int64 => instr = ltac::create_instr(LtacType::I64Div),
                    DataType::Ptr if var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::U64Div),
                    DataType::Ptr if var.sub_type == DataType::Float => instr = ltac::create_instr(LtacType::F32Div),
                    DataType::Ptr if var.sub_type == DataType::Double => instr = ltac::create_instr(LtacType::F64Div),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of division operator.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Modulo
            
            AstArgType::OpMod => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Mod),
                    DataType::UByte => instr = ltac::create_instr(LtacType::U8Mod),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Mod),
                    DataType::UShort => instr = ltac::create_instr(LtacType::U16Mod),
                    DataType::Int => instr = ltac::create_instr(LtacType::I32Mod),
                    DataType::UInt => instr = ltac::create_instr(LtacType::U32Mod),
                    DataType::Int64 => instr = ltac::create_instr(LtacType::I64Mod),
                    DataType::UInt64 => instr = ltac::create_instr(LtacType::U64Mod),
                    
                    DataType::Ptr if var.sub_type == DataType::Byte => instr = ltac::create_instr(LtacType::I8Mod),
                    DataType::Ptr if var.sub_type == DataType::UByte => instr = ltac::create_instr(LtacType::U8Mod),
                    DataType::Ptr if var.sub_type == DataType::Short => instr = ltac::create_instr(LtacType::I16Mod),
                    DataType::Ptr if var.sub_type == DataType::UShort => instr = ltac::create_instr(LtacType::U16Mod),
                    DataType::Ptr if var.sub_type == DataType::Int => instr = ltac::create_instr(LtacType::I32Mod),
                    DataType::Ptr if var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::U32Mod),
                    DataType::Ptr if var.sub_type == DataType::Int64 => instr = ltac::create_instr(LtacType::I64Mod),
                    DataType::Ptr if var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::U64Mod),
                    
                    _ => {
                        builder.syntax.ltac_error2("Modulo is only valid with integer values.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Logical AND
            
            AstArgType::OpAnd => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BAnd),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WAnd),
                    DataType::Int | DataType::UInt => instr = ltac::create_instr(LtacType::I32And),
                    DataType::Int64 | DataType::UInt64 => instr = ltac::create_instr(LtacType::I64And),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int || var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::I32And),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int64 || var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::I64And),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of logical and.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Logical OR
            
            AstArgType::OpOr => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BOr),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WOr),
                    DataType::Int | DataType::UInt => instr = ltac::create_instr(LtacType::I32Or),
                    DataType::Int64 | DataType::UInt64 => instr = ltac::create_instr(LtacType::I64Or),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int || var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::I32Or),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int64 || var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::I64Or),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of logical or.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Logical XOR
            
            AstArgType::OpXor => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BXor),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WXor),
                    DataType::Int | DataType::UInt => instr = ltac::create_instr(LtacType::I32Xor),
                    DataType::Int64 | DataType::UInt64 => instr = ltac::create_instr(LtacType::I64Xor),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int || var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::I32Xor),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int64 || var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::I64Xor),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of logical xor.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Left shift
            
            AstArgType::OpLeftShift => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BLsh),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WLsh),
                    DataType::Int | DataType::UInt => instr = ltac::create_instr(LtacType::I32Lsh),
                    DataType::Int64 | DataType::UInt64 => instr = ltac::create_instr(LtacType::I64Lsh),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int || var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::I32Lsh),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int64 || var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::I64Lsh),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of left shift.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            // Right shift
            
            AstArgType::OpRightShift => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BRsh),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WRsh),
                    DataType::Int | DataType::UInt => instr = ltac::create_instr(LtacType::I32Rsh),
                    DataType::Int64 | DataType::UInt64 => instr = ltac::create_instr(LtacType::I64Rsh),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int || var.sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::I32Rsh),
                    
                    DataType::Ptr
                    if var.sub_type == DataType::Int64 || var.sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::I64Rsh),
                    
                    _ => {
                        builder.syntax.ltac_error2("Invalid use of right shift.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, &var.sub_type, reg_no);
            },
            
            _ => {},
        }
    }
    
    true
}

// Builds a variable reference within an expression
pub fn build_expr_var(builder : &mut LtacBuilder, arg : &AstArg, var : &Var, reg_no : i32, negate_next : bool, instr : &mut LtacInstr) -> bool {
    let v = match builder.get_var(&arg.str_val) {
        Ok(v) => v.clone(),
        Err(_e) => return false,    // This really shouldn't happen
    };
    
    let zero = builder.build_float(0.0, false, false);      // I don't love having this here, but it won't work in the match
    let mut pop_float = true;
    
    instr.arg2 = LtacArg::Mem(v.pos);
    
    let mut size = 1;
    if v.sub_type == DataType::Short || v.sub_type == DataType::UShort {
        size = 2;
    } else if v.sub_type == DataType::Int || v.sub_type == DataType::UInt
        || v.sub_type == DataType::Float {
        size = 4;
    } else if  v.sub_type == DataType::Int64 || v.sub_type == DataType::UInt64
        || v.sub_type == DataType::Double || var.sub_type == DataType::Str {
        size = 8;
    }
    
    if arg.sub_args.len() > 0 {
        let first_arg = arg.sub_args.last().unwrap();
        
        if arg.sub_args.len() == 1 {
            if first_arg.arg_type == AstArgType::IntL {
                let offset = (first_arg.u64_val as i32) * size;
                instr.arg2 = LtacArg::MemOffsetImm(v.pos, offset);
            } else if first_arg.arg_type == AstArgType::Id {
                let mut instr2 = mov_for_type(&v.data_type, &v.sub_type);
                
                match builder.vars.get(&first_arg.str_val) {
                    Some(v2) => instr2.arg2 = LtacArg::MemOffsetMem(v.pos, v2.pos, size),
                    None => {
                        builder.syntax.ltac_error2("Invalid offset variable.".to_string());
                        return false;
                    },
                };
                
                // Choose the proper registers
                instr2.arg1 = reg_for_type(&v.data_type, &v.sub_type, reg_no);
                instr.arg2 = reg_for_type(&v.data_type, &v.sub_type, reg_no);
                
                builder.file.code.push(instr2);
            }
        } else {
            // Create a dummy variable so the types stay correct
            let var2 = Var {
                pos : 0,
                data_type : DataType::Int,
                sub_type : DataType::None,
                is_param : false,
            };
            
            build_var_expr(builder, &arg.sub_args, &var2, 0);
            
            let mut instr2 = mov_for_type(&v.data_type, &v.sub_type);
            instr2.arg1 = reg_for_type(&v.data_type, &v.sub_type, 0);
            instr2.arg2 = LtacArg::MemOffsetReg(v.pos, 0, size);
            builder.file.code.push(instr2);
            
            instr.arg2 = reg_for_type(&v.data_type, &v.sub_type, 0);
        }
    }
    
    // Negate variable if needed
    // Variable negation is simply the value subtracted from 0
    //
    // instr: mov r1, 0
    //        sub r1, mem
    if negate_next {
        instr.arg2 = reg_for_type(&v.data_type, &v.sub_type, 0);
        
        // The first argument is the same register
        let mut instr2 = mov_for_type(&v.data_type, &v.sub_type);
        instr2.arg1 = reg_for_type(&v.data_type, &v.sub_type, 0);
        
        match v.data_type {
            DataType::Byte => {
                instr2.arg2 = LtacArg::Byte(0);
                builder.file.code.push(instr2.clone());
                
                instr2.instr_type = LtacType::I8Sub;
            },
            
            DataType::Short => {
                instr2.arg2 = LtacArg::I16(0);
                builder.file.code.push(instr2.clone());
                
                instr2.instr_type = LtacType::I16Sub;
            },
            
            DataType::Int => {
                instr2.arg2 = LtacArg::I32(0);
                builder.file.code.push(instr2.clone());
                
                instr2.instr_type = LtacType::I32Sub;
            },
            
            DataType::Int64 => {
                instr2.arg2 = LtacArg::I64(0);
                builder.file.code.push(instr2.clone());
                
                instr2.instr_type = LtacType::I64Sub;
            },
            
            DataType::Float => {
                instr2.arg2 = LtacArg::F32(zero);
                builder.file.code.push(instr2.clone());
                
                instr2.instr_type = LtacType::F32Sub;
                pop_float = false;
            },
            
            DataType::Double => {
                instr2.arg2 = LtacArg::F64(zero);
                builder.file.code.push(instr2.clone());
                
                instr2.instr_type = LtacType::F64Sub;
                pop_float = false;
            },
            
            _ => {
                builder.syntax.ltac_error2("Invalid use of negation operator.".to_string());
                return false;
            },
        }
        
        // Set the memory and push the second operand
        instr2.arg2 = LtacArg::Mem(v.pos);
        builder.file.code.push(instr2);
    }
    
    // Pop the extra float we created at the top if we don't need it
    if pop_float {
        builder.file.data.pop();
    }
    
    // Add the instruction
    builder.file.code.push(instr.clone());
    true
}

// Builds a function call within an expression
pub fn build_expr_func_call(builder : &mut LtacBuilder, arg : &AstArg, var : &Var, reg_no : i32, instr : &mut LtacInstr) -> bool {
    let t = match builder.get_function(&arg.str_val) {
        Ok(t) => t.clone(),
        Err(_e) => return false,
    };

    // First, push the current register
    let mut store = mov_for_type(&t, &DataType::None);        // TODO: Replace this
    store.arg1 = LtacArg::Mem(var.pos);
    store.arg2 = reg_for_type(&t, &DataType::None, reg_no);    // TODO: Replace this
    builder.file.code.push(store.clone());

    // Create a statement to build the rest of the function call
    let mut stmt = ast::create_orphan_stmt(AstStmtType::FuncCall);
    stmt.name = arg.str_val.clone();
    stmt.args = arg.sub_args.clone();
    build_func_call(builder, &stmt);
           
    //Restore the current register
    store.arg1 = reg_for_type(&t, &DataType::None, reg_no);        // TODO: Replace this
    store.arg2 = LtacArg::Mem(var.pos);
    builder.file.code.push(store);

    match t {
        DataType::Byte => instr.arg2 = LtacArg::RetRegI8,
        DataType::UByte => instr.arg2 = LtacArg::RetRegU8,
        DataType::Short => instr.arg2 = LtacArg::RetRegI16,
        DataType::UShort => instr.arg2 = LtacArg::RetRegU16,
        DataType::Int => instr.arg2 = LtacArg::RetRegI32,
        DataType::UInt => instr.arg2 = LtacArg::RetRegU32,
        DataType::Int64 => instr.arg2 = LtacArg::RetRegI64,
        DataType::UInt64 => instr.arg2 = LtacArg::RetRegU64,
        DataType::Float => instr.arg2 = LtacArg::RetRegF32,
        DataType::Double => instr.arg2 = LtacArg::RetRegF64,
        
        _ => {
            builder.syntax.ltac_error2("Invalid return.".to_string());
            return false;
        },
    }

    // Add the line
    builder.file.code.push(instr.clone());
    true
}


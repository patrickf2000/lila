// The main expression builder for the LTAC layer

use std::mem;

use crate::ltac_builder::*;
use crate::ast;
use crate::ast::{AstStmt, AstArg, AstStmtType, AstArgType};
use crate::ltac;
use crate::ltac::{LtacType, LtacArg, LtacInstr};

use crate::ltac_func::*;

// Builds assignments for numerical variables
pub fn build_var_math(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) -> bool {
    let args = &line.args;
    let first_type = args.first().unwrap().arg_type.clone();
    let reg_no = 1;
    
    if !build_var_expr(builder, args, line, var, reg_no) {
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
        instr.arg2_offset = top.arg2_offset;
        instr.arg2_offset_size = top.arg2_offset_size;
        
    } else {
        instr = mov_for_type(&var.data_type);
        instr.arg1 = LtacArg::Mem(var.pos);
        instr.arg2 = reg_for_type(&var.data_type, reg_no);
    }
    
    // If we have an array, there's additional work
    if line.sub_args.len() > 0 {
        let first_arg = line.sub_args.last().unwrap();
        let mut offset_size = 4;
        
        if var.data_type == DataType::ByteDynArray || var.data_type == DataType::UByteDynArray {
            offset_size = 1;
        } else if var.data_type == DataType::ShortDynArray || var.data_type == DataType::UShortDynArray {
            offset_size = 2;
        } else if var.data_type == DataType::I64DynArray || var.data_type == DataType::U64DynArray 
            || var.data_type == DataType::DoubleDynArray {
            offset_size = 8;
        }
        
        if line.sub_args.len() == 1 {
            if first_arg.arg_type == AstArgType::IntL {
                instr.instr_type = LtacType::MovOffImm;
                instr.arg1_offset = (first_arg.u64_val as i32) * offset_size;
            } else if first_arg.arg_type == AstArgType::Id {
                instr.instr_type = LtacType::MovOffMem;
                instr.arg1_offset_size = offset_size;
                
                match builder.vars.get(&first_arg.str_val) {
                    Some(v) => instr.arg1_offset = v.pos,
                    None => instr.arg1_offset = 0,
                }
            }
        }
    }
    
    builder.file.code.push(instr);
    
    true
}

// TODO: I would eventually like to get rid of the "line" parameter
// Doing so may require work in the ltac_builder module.
fn build_var_expr(builder : &mut LtacBuilder, args : &Vec<AstArg>, line : &AstStmt, var : &Var, reg_no : i32) -> bool {

    // The control variable for negatives
    let mut negate_next = false;
    
    let mut instr = mov_for_type(&var.data_type);
    instr.arg1 = reg_for_type(&var.data_type, reg_no);
    
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
                build_var_expr(builder, &sub_expr, line, var, reg_no+1);
                
                instr.arg2 = reg_for_type(&var.data_type, reg_no+1);
                builder.file.code.push(instr.clone());
                
                sub_expr = Vec::new();
                is_sub_expr = false;
            },
        
            // Assign byte literals
            AstArgType::ByteL => {
                if negate_next {
                    builder.syntax.ltac_error(&line, "Negation invalid for this type.".to_string());
                    return false;
                }
            
                if var.data_type == DataType::Byte || var.data_type == DataType::ByteDynArray {
                    instr.arg2 = LtacArg::Byte(arg.u8_val as i8);
                } else if var.data_type == DataType::UByte || var.data_type == DataType::UByteDynArray {
                    instr.arg2 = LtacArg::UByte(arg.u8_val);
                } else {
                    builder.syntax.ltac_error(&line, "Invalid use of byte literal.".to_string());
                    return false;
                }
                
                builder.file.code.push(instr.clone());
            },
            
            // Assign short literals
            AstArgType::ShortL => {
                if negate_next {
                    builder.syntax.ltac_error(&line, "Negation invalid for this type.".to_string());
                    return false;
                }
                
                if var.data_type == DataType::Short || var.data_type == DataType::ShortDynArray {
                    instr.arg2 = LtacArg::I16(arg.u16_val as i16);
                } else if var.data_type == DataType::UShort || var.data_type == DataType::UShortDynArray {
                    instr.arg2 = LtacArg::U16(arg.u16_val);
                } else {
                    builder.syntax.ltac_error(&line, "Invalid use of short literal.".to_string());
                    return false;
                }
                    
                builder.file.code.push(instr.clone());
            },
        
            // ===============================================================
            // Assign integer literals
            
            AstArgType::IntL => {
                // Bytes
                if var.data_type == DataType::Byte || var.data_type == DataType::Char {
                    let val = arg.u64_val as i32;
                    
                    if mem::size_of::<i8>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into byte.".to_string());
                        return false;
                    }
                    
                    let parts = unsafe { mem::transmute::<i32, [i8; 4]>(val) };
                    let mut result = parts[0];
                    
                    if negate_next {
                        result = -result;
                        negate_next = false;
                    }
                    
                    instr.arg2 = LtacArg::Byte(result);
                    builder.file.code.push(instr.clone());
                    
                // UByte
                } else if var.data_type == DataType::UByte {
                    let val = arg.u64_val as u32;
                    
                    if mem::size_of::<u8>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into ubyte.".to_string());
                        return false;
                    }
                    
                    let parts = unsafe { mem::transmute::<u32, [u8; 4]>(val) };
                    let result = parts[0];
                    
                    instr.arg2 = LtacArg::UByte(result);
                    builder.file.code.push(instr.clone());
                    
                // Short
                } else if var.data_type == DataType::Short {
                    let val = arg.u64_val as i32;
                    
                    if mem::size_of::<u16>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into short.".to_string());
                        return false;
                    }
                    
                    let parts = unsafe { mem::transmute::<i32, [i16; 2]>(val) };
                    let mut result = parts[0];
                    
                    if negate_next {
                        result = -result;
                        negate_next = false;
                    }
                    
                    instr.arg2 = LtacArg::I16(result);
                    builder.file.code.push(instr.clone());
                    
                // UShort
                } else if var.data_type == DataType::UShort {
                    let val = arg.u64_val as u32;
                    
                    if mem::size_of::<u16>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into ushort.".to_string());
                        return false;
                    }
                    
                    let parts = unsafe { mem::transmute::<u32, [u16; 2]>(val) };
                    let result = parts[0];
                    
                    instr.arg2 = LtacArg::U16(result);
                    builder.file.code.push(instr.clone());
                    
                // Integers and integer arrays
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    let mut val = arg.u64_val as i32;
                    
                    if negate_next {
                        val = -val;
                        negate_next = false;
                    }
                    
                    instr.arg2 = LtacArg::I32(val);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr.arg2 = LtacArg::U32(arg.u64_val as u32);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::Int64  || var.data_type == DataType::I64DynArray {
                    let mut val = arg.u64_val as i64;
                    
                    if negate_next {
                        val = -val;
                        negate_next = false;
                    }
                    
                    instr.arg2 = LtacArg::I64(val);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::UInt64 || var.data_type == DataType::U64DynArray {
                    instr.arg2 = LtacArg::U64(arg.u64_val);
                    builder.file.code.push(instr.clone());
                    
                // Invalid
                } else {
                    builder.syntax.ltac_error(&line, "Invalid use of integer.".to_string());
                    return false;
                }
                
                // If the negate flag is still active at this point, we used it in the wrong place.
                if negate_next {
                    builder.syntax.ltac_error(&line, "Negation invalid for this type.".to_string());
                    return false;
                }
            },
            
            // ===============================================================
            // Assign float literals
            
            AstArgType::FloatL => {
                if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    let name = builder.build_float(arg.f64_val, false, negate_next);
                    instr.arg2 = LtacArg::F32(name);
                    builder.file.code.push(instr.clone());
                
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    let name = builder.build_float(arg.f64_val, true, negate_next);
                    instr.arg2 = LtacArg::F64(name);
                    builder.file.code.push(instr.clone());
                    
                } else {
                    builder.syntax.ltac_error(&line, "Invalid use of float literal.".to_string());
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
                    builder.syntax.ltac_error(&line, "Invalid use of char literal.".to_string());
                }
            },
            
            AstArgType::StringL => {},
            
            // ===============================================================
            // Variables and functions
            
            // TODO: This is TERRIBLE. Please clean up
            
            AstArgType::Id => {
                let zero = builder.build_float(0.0, false, false);      // I don't love having this here, but it won't work in the match
                let mut pop_float = true;
                
                match builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        instr.arg2 = LtacArg::Mem(v.pos);
                        
                        let mut size = 1;
                        if v.data_type == DataType::ShortDynArray || v.data_type == DataType::UShortDynArray {
                            size = 2;
                        } else if v.data_type == DataType::IntDynArray  || v.data_type == DataType::UIntDynArray
                            || v.data_type == DataType::FloatDynArray {
                            size = 4;
                        } else if  v.data_type == DataType::I64DynArray || v.data_type == DataType::U64DynArray
                            || v.data_type == DataType::DoubleDynArray {
                            size = 8;
                        }
                        
                        if arg.sub_args.len() > 0 {
                            let first_arg = arg.sub_args.last().unwrap();
                            
                            if arg.sub_args.len() == 1 {
                                if first_arg.arg_type == AstArgType::IntL {
                                    instr.instr_type = LtacType::MovOffImm;
                                    instr.arg2_offset = (first_arg.u64_val as i32) * size;
                                } else if first_arg.arg_type == AstArgType::Id {
                                    let mut instr2 = ltac::create_instr(LtacType::MovOffMem);
                                    
                                    instr2.arg2 = LtacArg::Mem(v.pos);
                                    instr2.arg2_offset_size = size;
                                    
                                    match builder.vars.get(&first_arg.str_val) {
                                        Some(v) => instr2.arg2_offset = v.pos,
                                        None => instr2.arg2_offset = 0,
                                    };
                                    
                                    // Choose the proper registers
                                    instr2.arg1 = reg_for_type(&v.data_type, reg_no);
                                    instr.arg2 = reg_for_type(&v.data_type, reg_no);
                                    
                                    builder.file.code.push(instr2);
                                }
                            }
                        }
                        
                        // Negate variable if needed
                        // Variable negation is simply the value subtracted from 0
                        //
                        // instr: mov r1, 0
                        //        sub r1, mem
                        if negate_next {
                            instr.arg2 = reg_for_type(&v.data_type, 0);
                            
                            // The first argument is the same register
                            let mut instr2 = mov_for_type(&v.data_type);
                            instr2.arg1 = reg_for_type(&v.data_type, 0);
                            
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
                                    builder.syntax.ltac_error(line, "Invalid use of negation operator.".to_string());
                                    return false;
                                },
                            }
                            
                            // Set the memory and push the second operand
                            instr2.arg2 = LtacArg::Mem(v.pos);
                            builder.file.code.push(instr2);
                            
                            negate_next = false;
                        }
                        
                        // Pop the extra float we created at the top if we don't need it
                        if pop_float {
                            builder.file.data.pop();
                        }
                    },
                    
                    None => {
                        match builder.clone().functions.get(&arg.str_val) {
                            Some(t) => {
                                // Create a statement to build the rest of the function call
                                let mut stmt = ast::create_orphan_stmt(AstStmtType::FuncCall);
                                stmt.name = arg.str_val.clone();
                                stmt.args = arg.sub_args.clone();
                                build_func_call(builder, &stmt);
                                
                                match *t {
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
                                        builder.syntax.ltac_error(line, "Invalid return.".to_string());
                                        return false;
                                    },
                                }
                            },
                            
                            None => {
                                match builder.clone().global_consts.get(&arg.str_val) {
                                    Some(c) => {
                                        instr.arg2 = c.clone();
                                    },
                                    
                                    None => {
                                        let mut msg = "Invalid function, constant, or variable name: ".to_string();
                                        msg.push_str(&arg.str_val);
                                    
                                        builder.syntax.ltac_error(line, msg);
                                        return false;
                                    },
                                }
                            },
                        }
                    }
                }
                
                builder.file.code.push(instr.clone());
            },
            
            // Ldarg statement
            // Format position (sub_arg[0]), data_type (sub_modifiers[0])
            
            AstArgType::LdArg => {
                let position_arg = arg.sub_args.first().unwrap();
                let position = position_arg.u64_val as i32;
                
                let ast_data_type = arg.sub_modifiers.first().unwrap();
                let data_type = ast_to_datatype(&ast_data_type);
                let reg = reg_for_type(&data_type, reg_no+1);
                
                let ld_instr = ldarg_for_type(&data_type, reg.clone(), position);
                builder.file.code.push(ld_instr);
                
                instr.arg2 = reg;
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
                    DataType::Int | DataType::IntDynArray => instr = ltac::create_instr(LtacType::I32Add),
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::U32Add),
                    DataType::Int64 | DataType::I64DynArray => instr = ltac::create_instr(LtacType::I64Add),
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::U64Add),
                    DataType::Float | DataType::FloatDynArray => instr = ltac::create_instr(LtacType::F32Add),
                    DataType::Double | DataType::DoubleDynArray => instr = ltac::create_instr(LtacType::F64Add),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of addition operator.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Subtraction
            
            AstArgType::OpSub => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Sub),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Sub),
                    DataType::Int | DataType::IntDynArray => instr = ltac::create_instr(LtacType::I32Sub),
                    DataType::Int64 | DataType::I64DynArray => instr = ltac::create_instr(LtacType::I64Sub),
                    DataType::Float | DataType::FloatDynArray => instr = ltac::create_instr(LtacType::F32Sub),
                    DataType::Double | DataType::DoubleDynArray => instr = ltac::create_instr(LtacType::F64Sub),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of subtraction operator.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Multiplication
            
            AstArgType::OpMul => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Mul),
                    DataType::UByte => instr = ltac::create_instr(LtacType::U8Mul),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Mul),
                    DataType::UShort => instr = ltac::create_instr(LtacType::U16Mul),
                    DataType::Int | DataType::IntDynArray => instr = ltac::create_instr(LtacType::I32Mul),
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::U32Mul),
                    DataType::Int64 | DataType::I64DynArray => instr = ltac::create_instr(LtacType::I64Mul),
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::U64Mul),
                    DataType::Float | DataType::FloatDynArray => instr = ltac::create_instr(LtacType::F32Mul),
                    DataType::Double | DataType::DoubleDynArray => instr = ltac::create_instr(LtacType::F64Mul),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of multiplication operator.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Division
            
            AstArgType::OpDiv => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Div),
                    DataType::UByte => instr = ltac::create_instr(LtacType::U8Div),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Div),
                    DataType::UShort => instr = ltac::create_instr(LtacType::U16Div),
                    DataType::Int | DataType::IntDynArray => instr = ltac::create_instr(LtacType::I32Div),
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::U32Div),
                    DataType::Int64 | DataType::I64DynArray => instr = ltac::create_instr(LtacType::I64Div),
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::U64Div),
                    DataType::Float | DataType::FloatDynArray => instr = ltac::create_instr(LtacType::F32Div),
                    DataType::Double | DataType::DoubleDynArray => instr = ltac::create_instr(LtacType::F64Div),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of division operator.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Modulo
            
            AstArgType::OpMod => {
                match var.data_type {
                    DataType::Byte => instr = ltac::create_instr(LtacType::I8Mod),
                    DataType::UByte => instr = ltac::create_instr(LtacType::U8Mod),
                    DataType::Short => instr = ltac::create_instr(LtacType::I16Mod),
                    DataType::UShort => instr = ltac::create_instr(LtacType::U16Mod),
                    DataType::Int | DataType::IntDynArray => instr = ltac::create_instr(LtacType::I32Mod),
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::U32Mod),
                    DataType::Int64 | DataType::I64DynArray => instr = ltac::create_instr(LtacType::I64Mod),
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::U64Mod),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Modulo is only valid with integer values.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Logical AND
            
            AstArgType::OpAnd => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BAnd),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WAnd),
                    DataType::Int | DataType::IntDynArray |
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::I32And),
                    DataType::Int64 | DataType::I64DynArray |
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::I64And),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of logical and.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Logical OR
            
            AstArgType::OpOr => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BOr),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WOr),
                    DataType::Int | DataType::IntDynArray |
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::I32Or),
                    DataType::Int64 | DataType::I64DynArray |
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::I64Or),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of logical or.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Logical XOR
            
            AstArgType::OpXor => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BXor),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WXor),
                    DataType::Int | DataType::IntDynArray |
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::I32Xor),
                    DataType::Int64 | DataType::I64DynArray |
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::I64Xor),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of logical xor.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Left shift
            
            AstArgType::OpLeftShift => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BLsh),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WLsh),
                    DataType::Int | DataType::IntDynArray |
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::I32Lsh),
                    DataType::Int64 | DataType::I64DynArray |
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::I64Lsh),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of left shift.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            // Right shift
            
            AstArgType::OpRightShift => {
                match var.data_type {
                    DataType::Byte | DataType::UByte => instr = ltac::create_instr(LtacType::BRsh),
                    DataType::Short | DataType::UShort => instr = ltac::create_instr(LtacType::WRsh),
                    DataType::Int | DataType::IntDynArray |
                    DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::I32Rsh),
                    DataType::Int64 | DataType::I64DynArray |
                    DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::I64Rsh),
                    
                    _ => {
                        builder.syntax.ltac_error(line, "Invalid use of right shift.".to_string());
                        return false;
                    },
                }
                
                instr.arg1 = reg_for_type(&var.data_type, reg_no);
            },
            
            _ => {},
        }
    }
    
    true
}


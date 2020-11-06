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
                    
                    // TODO:
                    // This was getting triggered for odd reasons
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
            
            AstArgType::Id => {
                let zero = builder.build_float(0.0, false, false);      // I don't love having this here, but it won't work in the match
                
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
                                },
                                
                                DataType::Double => {
                                    instr2.arg2 = LtacArg::F64(zero);
                                    builder.file.code.push(instr2.clone());
                                    
                                    instr2.instr_type = LtacType::F64Sub;
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
                                let mut msg = "Invalid function or variable name: ".to_string();
                                msg.push_str(&arg.str_val);
                            
                                builder.syntax.ltac_error(line, msg);
                                return false;
                            },
                        }
                    }
                }
                
                builder.file.code.push(instr.clone());
            },
            
            // Negate operator
            // Basically, we set a control variable. That way, if the next AST node is a literal, we simply
            // negate it here. If its a variable, we can create a subtraction operations
            
            AstArgType::OpNeg => negate_next = true,
            
            // Addition
            
            AstArgType::OpAdd => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::I8Add);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::U8Add);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Add);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::U16Add);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Add);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::U32Add);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Add);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else if var.data_type == DataType::UInt64 || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::U64Add);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    instr = ltac::create_instr(LtacType::F32Add);
                    instr.arg1 = LtacArg::FltReg(reg_no);
                    
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    instr = ltac::create_instr(LtacType::F64Add);
                    instr.arg1 = LtacArg::FltReg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of addition operator.".to_string());
                    return false;
                }
            },
            
            // Subtraction
            
            AstArgType::OpSub => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::I8Sub);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Sub);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Sub);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Sub);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    instr = ltac::create_instr(LtacType::F32Sub);
                    instr.arg1 = LtacArg::FltReg(reg_no);
                    
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    instr = ltac::create_instr(LtacType::F64Sub);
                    instr.arg1 = LtacArg::FltReg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of subtraction operator.".to_string());
                    return false;
                }
            },
            
            // Multiplication
            
            AstArgType::OpMul => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::I8Mul);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::U8Mul);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Mul);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::U16Mul);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Mul);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::U32Mul);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Mul);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else if var.data_type == DataType::UInt64 || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::U64Mul);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    instr = ltac::create_instr(LtacType::F32Mul);
                    instr.arg1 = LtacArg::FltReg(reg_no);
                    
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    instr = ltac::create_instr(LtacType::F64Mul);
                    instr.arg1 = LtacArg::FltReg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of multiplication operator.".to_string());
                    return false;
                }
            },
            
            // Division
            
            AstArgType::OpDiv => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::I8Div);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::U8Div);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Div);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::U16Div);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Div);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::U32Div);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Div);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else if var.data_type == DataType::UInt64 || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::U64Div);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    instr = ltac::create_instr(LtacType::F32Div);
                    instr.arg1 = LtacArg::FltReg(reg_no);
                    
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    instr = ltac::create_instr(LtacType::F64Div);
                    instr.arg1 = LtacArg::FltReg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of addition operator.".to_string());
                    return false;
                }
            },
            
            // Modulo
            
            AstArgType::OpMod => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::I8Mod);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::U8Mod);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Mod);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::U16Mod);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Mod);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::U32Mod);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Mod);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else if var.data_type == DataType::UInt64 || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::U64Mod);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Modulo is only valid with integer values.".to_string());
                    return false;
                }
            },
            
            // Logical AND
            
            AstArgType::OpAnd => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BAnd);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WAnd);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32And);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64
                     || var.data_type == DataType::I64DynArray || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::I64And);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of logical and.".to_string());
                    return false;
                }
            },
            
            // Logical OR
            
            AstArgType::OpOr => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BOr);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WOr);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32Or);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64 
                         || var.data_type == DataType::I64DynArray || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::I64Or);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of logical or.".to_string());
                    return false;
                }
            },
            
            // Logical XOR
            
            AstArgType::OpXor => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BXor);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WXor);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32Xor);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64
                         || var.data_type == DataType::I64DynArray || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::I64Xor);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of logical xor.".to_string());
                    return false;
                }
            },
            
            // Left shift
            
            AstArgType::OpLeftShift => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BLsh);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WLsh);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32Lsh);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64
                         || var.data_type == DataType::I64DynArray || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::I64Lsh);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of left shift.".to_string());
                    return false;
                }
            },
            
            // Right shift
            
            AstArgType::OpRightShift => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BRsh);
                    instr.arg1 = LtacArg::Reg8(reg_no);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WRsh);
                    instr.arg1 = LtacArg::Reg16(reg_no);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32Rsh);
                    instr.arg1 = LtacArg::Reg32(reg_no);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64
                         || var.data_type == DataType::I64DynArray || var.data_type == DataType::U64DynArray {
                    instr = ltac::create_instr(LtacType::I64Rsh);
                    instr.arg1 = LtacArg::Reg64(reg_no);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of right shift.".to_string());
                    return false;
                }
            },
            
            _ => {},
        }
    }
    
    true
}


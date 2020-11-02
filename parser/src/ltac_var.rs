
use std::mem;

use crate::ltac_builder::*;
use crate::ast;
use crate::ast::{AstStmt, AstStmtType, AstModType, AstArgType};
use crate::ltac;
use crate::ltac::{LtacType, LtacArg};

use crate::ltac_array::*;
use crate::ltac_func::*;

// Builds an LTAC variable declaration
pub fn build_var_dec(builder : &mut LtacBuilder, line : &AstStmt, arg_no_o : i32, flt_arg_no_o : i32) -> (bool, i32, i32) {
    let mut arg_no = arg_no_o;
    let mut flt_arg_no = flt_arg_no_o;
    
    let name = line.name.clone();
    let ast_data_type = &line.modifiers[0];
    let data_type : DataType;
    
    match &ast_data_type.mod_type {
        AstModType::Byte => {
            data_type = DataType::Byte;
            builder.stack_pos += 1;
        },
        
        AstModType::ByteDynArray => {
            data_type = DataType::ByteDynArray;
            builder.stack_pos += 8
        },
        
        AstModType::UByte => {
            data_type = DataType::UByte;
            builder.stack_pos += 1;
        },
        
        AstModType::UByteDynArray => {
            data_type = DataType::UByteDynArray;
            builder.stack_pos += 8;
        },
        
        AstModType::Short => {
            data_type = DataType::Short;
            builder.stack_pos += 2;
        },
        
        AstModType::UShort => {
            data_type = DataType::UShort;
            builder.stack_pos += 2;
        },
        
        AstModType::ShortDynArray => {
            data_type = DataType::ShortDynArray;
            builder.stack_pos += 8;
        },
        
        AstModType::UShortDynArray => {
            data_type = DataType::UShortDynArray;
            builder.stack_pos += 8;
        },
    
        AstModType::Int => {
            data_type = DataType::Int;
            builder.stack_pos += 4;
        },
        
        AstModType::UInt => {
            data_type = DataType::UInt;
            builder.stack_pos += 4;
        },
        
        AstModType::IntDynArray => {
            data_type = DataType::IntDynArray;
            builder.stack_pos += 8
        },
        
        AstModType::UIntDynArray => {
            data_type = DataType::UIntDynArray;
            builder.stack_pos += 8;
        },
        
        AstModType::Int64 => {
            data_type = DataType::Int64;
            builder.stack_pos += 8;
        },
        
        AstModType::UInt64 => {
            data_type = DataType::UInt64;
            builder.stack_pos += 8;
        },
        
        AstModType::I64DynArray => {
            data_type = DataType::I64DynArray;
            builder.stack_pos += 8;
        },
        
        AstModType::Float => {
            data_type = DataType::Float;
            builder.stack_pos += 4;
        },
        
        AstModType::Double => {
            data_type = DataType::Double;
            builder.stack_pos += 8;
        },
        
        AstModType::FloatDynArray => {
            data_type = DataType::FloatDynArray;
            builder.stack_pos += 8;
        },
        
        AstModType::DoubleDynArray => {
            data_type = DataType::DoubleDynArray;
            builder.stack_pos += 8;
        },
        
        AstModType::Str => {
            data_type = DataType::Str;
            builder.stack_pos += 8;
        },
        
        // Do we need an error here? Really, it should never get to this pointer
        AstModType::None => return (false, arg_no, flt_arg_no),
    }
    
    let mut is_param = false;
    if arg_no > 0 {
        is_param = true;
    }
    
    let v = Var {
        pos : builder.stack_pos,
        data_type : data_type,
        is_param : is_param,
    };
    
    builder.vars.insert(name, v);
    
    // If we have a function argument, add the load instruction
    if is_param {
        let mut ld = ltac::create_instr(LtacType::LdArgI32);
        
        if ast_data_type.mod_type == AstModType::Float {
            ld = ltac::create_instr(LtacType::LdArgF32);
            ld.arg2_val = flt_arg_no;
            flt_arg_no += 1;
            
        } else if ast_data_type.mod_type == AstModType::Double {
            ld = ltac::create_instr(LtacType::LdArgF64);
            ld.arg2_val = flt_arg_no;
            flt_arg_no += 1;
            
        } else if ast_data_type.mod_type == AstModType::IntDynArray
            || ast_data_type.mod_type == AstModType::Str {
            ld = ltac::create_instr(LtacType::LdArgPtr);
            ld.arg2_val = arg_no;
            arg_no += 1;
            
        } else if ast_data_type.mod_type == AstModType::Byte {
            ld = ltac::create_instr(LtacType::LdArgI8);
            ld.arg2_val = arg_no;
            arg_no += 1;
            
        } else if ast_data_type.mod_type == AstModType::UByte {
            ld = ltac::create_instr(LtacType::LdArgU8);
            ld.arg2_val = arg_no;
            arg_no += 1;
            
        } else if ast_data_type.mod_type == AstModType::Short {
            ld = ltac::create_instr(LtacType::LdArgI16);
            ld.arg2_val = arg_no;
            arg_no += 1;
        
        } else if ast_data_type.mod_type == AstModType::UShort {
            ld = ltac::create_instr(LtacType::LdArgU16);
            ld.arg2_val = arg_no;
            arg_no += 1;
            
        } else if ast_data_type.mod_type == AstModType::UInt {
            ld = ltac::create_instr(LtacType::LdArgU32);
            ld.arg2_val = arg_no;
            arg_no += 1;
            
        } else if ast_data_type.mod_type == AstModType::Int64 {
            ld = ltac::create_instr(LtacType::LdArgI64);
            ld.arg2_val = arg_no;
            arg_no += 1;
        
        } else if ast_data_type.mod_type == AstModType::UInt64 {
            ld = ltac::create_instr(LtacType::LdArgU64);
            ld.arg2_val = arg_no;
            arg_no += 1;
            
        } else {
            ld.arg2_val = arg_no;
            arg_no += 1;
        }
        
        ld.arg1_val = builder.stack_pos;
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
        None => return false,
    }
    
    let code : bool;
    
    if var.data_type == DataType::ByteDynArray || var.data_type == DataType::UByteDynArray ||
       var.data_type == DataType::ShortDynArray || var.data_type == DataType::UShortDynArray ||
       var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray ||
       var.data_type == DataType::I64DynArray ||
       var.data_type == DataType::FloatDynArray || var.data_type == DataType::DoubleDynArray {
        code = build_dyn_array(builder, &line, &var);
    } else if var.data_type == DataType::Str {
        code = build_str_assign(builder, &line, &var);
    } else {
        code = build_var_math(builder, &line, &var);
    }
    
    code
}

// Builds assignments for numerical variables
pub fn build_var_math(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) -> bool {
    let args = &line.args;
    let first_type = args.first().unwrap().arg_type.clone();

    let mut instr = ltac::create_instr(LtacType::Mov);
    instr.arg1_type = LtacArg::Reg32(0);
    
    // The byte types
    if var.data_type == DataType::Byte {
        instr = ltac::create_instr(LtacType::MovB);
        instr.arg1_type = LtacArg::Reg8(0);
        
    } else if var.data_type == DataType::UByte {
        instr = ltac::create_instr(LtacType::MovUB);
        instr.arg1_type = LtacArg::Reg8(0);
    
    // The short types
    } else if var.data_type == DataType::Short {
        instr = ltac::create_instr(LtacType::MovW);
        instr.arg1_type = LtacArg::Reg16(0);
        
    } else if var.data_type == DataType::UShort {
        instr = ltac::create_instr(LtacType::MovUW);
        instr.arg1_type = LtacArg::Reg16(0);
        
    // Unsigned integer
    } else if var.data_type == DataType::UInt {
        instr = ltac::create_instr(LtacType::MovU);
        instr.arg1_type = LtacArg::Reg32(0);
        
    // The int64 types
    } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
        instr = ltac::create_instr(LtacType::MovQ);
        instr.arg1_type = LtacArg::Reg64(0);
        
    } else if var.data_type == DataType::UInt64 {
        instr = ltac::create_instr(LtacType::MovUQ);
        instr.arg1_type = LtacArg::Reg64(0);
        
    // The float types
    } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
        instr = ltac::create_instr(LtacType::MovF32);
        instr.arg1_type = LtacArg::FltReg(0);
        
    } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
        instr = ltac::create_instr(LtacType::MovF64);
        instr.arg1_type = LtacArg::FltReg64(0);
    }
    
    for arg in args.iter() {
        match &arg.arg_type {
            // Assign byte literals
            AstArgType::ByteL => {
                if var.data_type == DataType::Byte || var.data_type == DataType::ByteDynArray {
                    instr.arg2_type = LtacArg::Byte(arg.u8_val as i8);
                } else if var.data_type == DataType::UByte || var.data_type == DataType::UByteDynArray {
                    instr.arg2_type = LtacArg::UByte(arg.u8_val);
                } else {
                    builder.syntax.ltac_error(&line, "Invalid use of byte literal.".to_string());
                    return false;
                }
                
                builder.file.code.push(instr.clone());
            },
            
            // Assign short literals
            AstArgType::ShortL => {
                if var.data_type == DataType::Short || var.data_type == DataType::ShortDynArray {
                    instr.arg2_type = LtacArg::I16(arg.u16_val as i16);
                } else if var.data_type == DataType::UShort || var.data_type == DataType::UShortDynArray {
                    instr.arg2_type = LtacArg::U16(arg.u16_val);
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
                if var.data_type == DataType::Byte {
                    let val = arg.u64_val as i32;
                    
                    if mem::size_of::<u8>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into byte.".to_string());
                        return false;
                    }
                    
                    let parts = unsafe { mem::transmute::<i32, [i8; 4]>(val) };
                    let result = parts[0];
                    
                    instr.arg2_type = LtacArg::Byte(result);
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
                    
                    instr.arg2_type = LtacArg::UByte(result);
                    builder.file.code.push(instr.clone());
                    
                // Short
                } else if var.data_type == DataType::Short {
                    let val = arg.u64_val as i32;
                    
                    if mem::size_of::<u16>() > (val as usize) {
                        builder.syntax.ltac_error(&line, "Integer is too big to fit into short.".to_string());
                        return false;
                    }
                    
                    let parts = unsafe { mem::transmute::<i32, [i16; 2]>(val) };
                    let result = parts[0];
                    
                    instr.arg2_type = LtacArg::I16(result);
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
                    
                    instr.arg2_type = LtacArg::U16(result);
                    builder.file.code.push(instr.clone());
                    
                // Integers and integer arrays
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr.arg2_type = LtacArg::I32(arg.u64_val as i32);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr.arg2_type = LtacArg::U32(arg.u64_val as u32);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::Int64  || var.data_type == DataType::I64DynArray {
                    instr.arg2_type = LtacArg::I64(arg.u64_val as i64);
                    builder.file.code.push(instr.clone());
                    
                } else if var.data_type == DataType::UInt64 {
                    instr.arg2_type = LtacArg::U64(arg.u64_val);
                    builder.file.code.push(instr.clone());
                    
                // Invalid
                } else {
                    builder.syntax.ltac_error(&line, "Invalid use of integer.".to_string());
                    return false;
                }
            },
            
            // ===============================================================
            // Assign float literals
            
            AstArgType::FloatL => {
                if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    let name = builder.build_float(arg.f64_val, false);
                    instr.arg2_type = LtacArg::F32(name);
                    builder.file.code.push(instr.clone());
                
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    let name = builder.build_float(arg.f64_val, true);
                    instr.arg2_type = LtacArg::F64(name);
                    builder.file.code.push(instr.clone());
                    
                } else {
                    builder.syntax.ltac_error(&line, "Invalid use of float literal.".to_string());
                    return false;
                }
            },
            
            AstArgType::StringL => {},
            
            AstArgType::Id => {
                match builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        instr.arg2_type = LtacArg::Mem(v.pos);
                        
                        let mut size = 1;
                        if v.data_type == DataType::ShortDynArray || v.data_type == DataType::UShortDynArray {
                            size = 2;
                        } else if v.data_type == DataType::IntDynArray  || v.data_type == DataType::UIntDynArray
                            || v.data_type == DataType::FloatDynArray {
                            size = 4;
                        } else if  v.data_type == DataType::I64DynArray
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
                                    
                                    instr2.arg2_type = LtacArg::Mem(v.pos);
                                    instr2.arg2_offset_size = size;
                                    
                                    match builder.vars.get(&first_arg.str_val) {
                                        Some(v) => instr2.arg2_offset = v.pos,
                                        None => instr2.arg2_offset = 0,
                                    };
                                    
                                    // Choose the proper registers
                                    if v.data_type == DataType::IntDynArray || v.data_type == DataType::UIntDynArray {
                                        instr2.arg1_type = LtacArg::Reg32(0);
                                        instr.arg2_type = LtacArg::Reg32(0);
                                    } else if v.data_type == DataType::I64DynArray {
                                        instr2.arg1_type = LtacArg::Reg64(0);
                                        instr.arg2_type = LtacArg::Reg64(0);
                                    } else if v.data_type == DataType::FloatDynArray {
                                        instr2.arg1_type = LtacArg::FltReg(0);
                                        instr.arg2_type = LtacArg::FltReg(0);
                                    } else if v.data_type == DataType::DoubleDynArray {
                                        instr2.arg1_type = LtacArg::FltReg64(0);
                                        instr.arg2_type = LtacArg::FltReg64(0);
                                    }
                                    
                                    builder.file.code.push(instr2);
                                }
                            }
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
                                    DataType::Byte => instr.arg2_type = LtacArg::RetRegI8,
                                    DataType::UByte => instr.arg2_type = LtacArg::RetRegU8,
                                    DataType::Short => instr.arg2_type = LtacArg::RetRegI16,
                                    DataType::UShort => instr.arg2_type = LtacArg::RetRegU16,
                                    DataType::Int => instr.arg2_type = LtacArg::RetRegI32,
                                    DataType::UInt => instr.arg2_type = LtacArg::RetRegU32,
                                    DataType::Int64 => instr.arg2_type = LtacArg::RetRegI64,
                                    DataType::UInt64 => instr.arg2_type = LtacArg::RetRegU64,
                                    DataType::Float => instr.arg2_type = LtacArg::RetRegF32,
                                    DataType::Double => instr.arg2_type = LtacArg::RetRegF64,
                                    
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
            
            // Addition
            
            AstArgType::OpAdd => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::BAdd);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::U8Add);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Add);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::U16Add);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Add);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::U32Add);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Add);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else if var.data_type == DataType::UInt64 {
                    instr = ltac::create_instr(LtacType::U64Add);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    instr = ltac::create_instr(LtacType::F32Add);
                    instr.arg1_type = LtacArg::FltReg(0);
                    
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    instr = ltac::create_instr(LtacType::F64Add);
                    instr.arg1_type = LtacArg::FltReg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of addition operator.".to_string());
                    return false;
                }
            },
            
            // Subtraction
            
            AstArgType::OpSub => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::BSub);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Sub);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Sub);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Sub);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    instr = ltac::create_instr(LtacType::F32Sub);
                    instr.arg1_type = LtacArg::FltReg(0);
                    
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    instr = ltac::create_instr(LtacType::F64Sub);
                    instr.arg1_type = LtacArg::FltReg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of subtraction operator.".to_string());
                    return false;
                }
            },
            
            // Multiplication
            
            AstArgType::OpMul => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::BMul);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::U8Mul);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Mul);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::U16Mul);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Mul);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::U32Mul);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Mul);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else if var.data_type == DataType::UInt64 {
                    instr = ltac::create_instr(LtacType::U64Mul);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    instr = ltac::create_instr(LtacType::F32Mul);
                    instr.arg1_type = LtacArg::FltReg(0);
                    
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    instr = ltac::create_instr(LtacType::F64Mul);
                    instr.arg1_type = LtacArg::FltReg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of multiplication operator.".to_string());
                    return false;
                }
            },
            
            // Division
            
            AstArgType::OpDiv => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::BDiv);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::U8Div);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Div);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::U16Div);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Div);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::U32Div);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Div);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else if var.data_type == DataType::UInt64 {
                    instr = ltac::create_instr(LtacType::U64Div);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
                    instr = ltac::create_instr(LtacType::F32Div);
                    instr.arg1_type = LtacArg::FltReg(0);
                    
                } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
                    instr = ltac::create_instr(LtacType::F64Div);
                    instr.arg1_type = LtacArg::FltReg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of addition operator.".to_string());
                    return false;
                }
            },
            
            // Modulo
            
            AstArgType::OpMod => {
                if var.data_type == DataType::Byte {
                    instr = ltac::create_instr(LtacType::BMod);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::U8Mod);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short {
                    instr = ltac::create_instr(LtacType::I16Mod);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::U16Mod);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray {
                    instr = ltac::create_instr(LtacType::I32Mod);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::UInt || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::U32Mod);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Mod);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else if var.data_type == DataType::UInt64 {
                    instr = ltac::create_instr(LtacType::U64Mod);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Modulo is only valid with integer values.".to_string());
                    return false;
                }
            },
            
            // Logical AND
            
            AstArgType::OpAnd => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BAnd);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WAnd);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32And);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64
                     || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64And);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of logical and.".to_string());
                    return false;
                }
            },
            
            // Logical OR
            
            AstArgType::OpOr => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BOr);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WOr);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32Or);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64 
                         || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Or);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of logical or.".to_string());
                    return false;
                }
            },
            
            // Logical XOR
            
            AstArgType::OpXor => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BXor);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WXor);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32Xor);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64
                         || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Xor);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of logical xor.".to_string());
                    return false;
                }
            },
            
            // Left shift
            
            AstArgType::OpLeftShift => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BLsh);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WLsh);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32Lsh);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64
                         || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Lsh);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of left shift.".to_string());
                    return false;
                }
            },
            
            // Right shift
            
            AstArgType::OpRightShift => {
                if var.data_type == DataType::Byte || var.data_type == DataType::UByte {
                    instr = ltac::create_instr(LtacType::BRsh);
                    instr.arg1_type = LtacArg::Reg8(0);
                    
                } else if var.data_type == DataType::Short || var.data_type == DataType::UShort {
                    instr = ltac::create_instr(LtacType::WRsh);
                    instr.arg1_type = LtacArg::Reg16(0);
                    
                } else if var.data_type == DataType::Int || var.data_type == DataType::UInt ||
                        var.data_type == DataType::IntDynArray || var.data_type == DataType::UIntDynArray {
                    instr = ltac::create_instr(LtacType::I32Rsh);
                    instr.arg1_type = LtacArg::Reg32(0);
                    
                } else if var.data_type == DataType::Int64 || var.data_type == DataType::UInt64
                         || var.data_type == DataType::I64DynArray {
                    instr = ltac::create_instr(LtacType::I64Rsh);
                    instr.arg1_type = LtacArg::Reg64(0);
                    
                } else {
                    builder.syntax.ltac_error(line, "Invalid use of right shift.".to_string());
                    return false;
                }
            },
            
            _ => {},
        }
    }
    
    //Store the result back
    // If it was a single assign (no math), compact the instructions
    if line.args.len() == 1 && first_type != AstArgType::Id {
        let top = builder.file.code.pop().unwrap();
        
        instr = ltac::create_instr(top.instr_type);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = top.arg2_type;
        instr.arg2_val = top.arg2_val;
        instr.arg2_offset = top.arg2_offset;
        instr.arg2_offset_size = top.arg2_offset_size;
        
    // Store back a byte
    } else if var.data_type == DataType::Byte {
        instr = ltac::create_instr(LtacType::MovB);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::Reg8(0);
        
    // Store back a ubyte
    } else if var.data_type == DataType::UByte {
        instr = ltac::create_instr(LtacType::MovUB);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::Reg8(0);
        
    // Store back a short
    } else if var.data_type == DataType::Short {
        instr = ltac::create_instr(LtacType::MovW);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::Reg16(0);
        
    // Store back an unsigned short
    } else if var.data_type == DataType::UShort {
        instr = ltac::create_instr(LtacType::MovUW);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::Reg16(0);
        
    // Store back an unsigned integer
    } else if var.data_type == DataType::UInt {
        instr = ltac::create_instr(LtacType::MovU);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::Reg32(0);
        
    // Store back a 64-bit integer
    } else if var.data_type == DataType::Int64  || var.data_type == DataType::I64DynArray {
        instr = ltac::create_instr(LtacType::MovQ);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::Reg64(0);
        
    // Store back an unsigned 64-bit integer
    } else if var.data_type == DataType::UInt64 {
        instr = ltac::create_instr(LtacType::MovUQ);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::Reg64(0);
        
    // Store back a float
    } else if var.data_type == DataType::Float || var.data_type == DataType::FloatDynArray {
        instr = ltac::create_instr(LtacType::MovF32);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::FltReg(0);
        
    // Store back a double
    } else if var.data_type == DataType::Double || var.data_type == DataType::DoubleDynArray {
        instr = ltac::create_instr(LtacType::MovF64);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::FltReg64(0);
        
    // Store back everything else
    } else {
        instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::Reg32(0);
    }
    
    if line.sub_args.len() > 0 {
        let first_arg = line.sub_args.last().unwrap();
        let mut offset_size = 4;
        
        if var.data_type == DataType::ByteDynArray || var.data_type == DataType::UByteDynArray {
            offset_size = 1;
        } else if var.data_type == DataType::ShortDynArray || var.data_type == DataType::UShortDynArray {
            offset_size = 2;
        } else if var.data_type == DataType::I64DynArray 
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

// Builds a string variable assignment
pub fn build_str_assign(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) -> bool {
    let mut instr = ltac::create_instr(LtacType::Mov);
    
    if line.args.len() == 1 {
        let arg = line.args.first().unwrap();
        
        instr.arg1_type = LtacArg::Mem(var.pos);
        
        match &arg.arg_type {
            AstArgType::StringL => {
                let name = builder.build_string(arg.str_val.clone());
                instr.arg2_type = LtacArg::PtrLcl(name);
            },
            
            AstArgType::Id => {
                match &builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        if v.data_type != DataType::Str {
                            builder.syntax.ltac_error(line, "You can only assign a string to a string.".to_string());
                            return false;
                        }
                        
                        instr.arg2_type = LtacArg::Reg64(0);
                        
                        let mut instr2 = ltac::create_instr(LtacType::Mov);
                        instr2.arg1_type = LtacArg::Reg64(0);
                        instr2.arg2_type = LtacArg::Mem(v.pos);
                        builder.file.code.push(instr2);
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

    

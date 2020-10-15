
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
        
        AstModType::UByte => {
            data_type = DataType::UByte;
            builder.stack_pos += 1;
        },
        
        AstModType::Short => {
            data_type = DataType::Short;
            builder.stack_pos += 2;
        },
    
        AstModType::Int => {
            data_type = DataType::Int;
            builder.stack_pos += 4;
        },
        
        AstModType::IntDynArray => {
            data_type = DataType::IntDynArray;
            builder.stack_pos += 8
        },
        
        AstModType::Float => {
            data_type = DataType::Float;
            builder.stack_pos += 4;
        },
        
        AstModType::Double => {
            data_type = DataType::Double;
            builder.stack_pos += 8;
        },
        
        AstModType::Str => {
            data_type = DataType::Str;
            builder.stack_pos += 8;
        },
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
    
    if var.data_type == DataType::IntDynArray {
        code = build_i32dyn_array(builder, &line, &var);
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
        
    // The float types
    } else if var.data_type == DataType::Float {
        instr = ltac::create_instr(LtacType::MovF32);
        instr.arg1_type = LtacArg::FltReg;
        instr.arg1_val = 0;
        
    } else if var.data_type == DataType::Double {
        instr = ltac::create_instr(LtacType::MovF64);
        instr.arg1_type = LtacArg::FltReg64;
        instr.arg1_val = 0;
    }
    
    for arg in args.iter() {
        match &arg.arg_type {
            AstArgType::ByteL if var.data_type == DataType::Byte => {
                instr.arg2_type = LtacArg::Byte(arg.u8_val as i8);
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::ByteL => {
                builder.syntax.ltac_error(&line, "Invalid use of byte literal.".to_string());
                return false;
            },
            
            AstArgType::ShortL if var.data_type == DataType::Short => {
                instr.arg2_type = LtacArg::I16;
                instr.arg2_wval = arg.u16_val;
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::ShortL => {
                builder.syntax.ltac_error(&line, "Invalid use of short literal.".to_string());
                return false;
            },
        
            AstArgType::IntL if var.data_type == DataType::Int || var.data_type == DataType::IntDynArray => {
                instr.arg2_type = LtacArg::I32(arg.i32_val);
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::IntL if var.data_type == DataType::Byte => {
                let val = arg.i32_val;
                
                if mem::size_of::<u8>() > (val as usize) {
                    builder.syntax.ltac_error(&line, "Integer is too big to fit into byte.".to_string());
                    return false;
                }
                
                let parts = unsafe { mem::transmute::<i32, [i8; 4]>(val) };
                let result = parts[0];
                
                instr.arg2_type = LtacArg::Byte(result);
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::IntL => {
                builder.syntax.ltac_error(&line, "Invalid use of integer.".to_string());
                return false;
            },
            
            AstArgType::FloatL if var.data_type == DataType::Float => {
                instr.arg2_type = LtacArg::F32;
                instr.arg2_sval = builder.build_float(arg.f64_val, false);
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::FloatL if var.data_type == DataType::Double => {
                instr.arg2_type = LtacArg::F64;
                instr.arg2_sval = builder.build_float(arg.f64_val, true);
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::FloatL => {},
            
            AstArgType::StringL => {},
            
            AstArgType::Id => {
                match builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        instr.arg2_type = LtacArg::Mem;
                        instr.arg2_val = v.pos;
                        
                        let mut size = 1;
                        if v.data_type == DataType::IntDynArray {
                            size = 4;
                        }
                        
                        if arg.sub_args.len() > 0 {
                            let first_arg = arg.sub_args.last().unwrap();
                            
                            if arg.sub_args.len() == 1 {
                                if first_arg.arg_type == AstArgType::IntL {
                                    instr.instr_type = LtacType::MovOffImm;
                                    instr.arg2_offset = first_arg.i32_val * size;
                                } else if first_arg.arg_type == AstArgType::Id {
                                    let mut instr2 = ltac::create_instr(LtacType::MovOffMem);
                                    instr2.arg1_type = LtacArg::Reg32(0);
                                    
                                    instr2.arg2_type = LtacArg::Mem;
                                    instr2.arg2_val = instr.arg2_val;
                                    instr2.arg2_offset_size = size;
                                    
                                    match builder.vars.get(&first_arg.str_val) {
                                        Some(v) => instr2.arg2_offset = v.pos,
                                        None => instr2.arg2_offset = 0,
                                    };
                                    
                                    builder.file.code.push(instr2);
                                    
                                    instr.arg2_type = LtacArg::Reg32(0);
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
                                
                                if *t == DataType::Int {
                                    instr.arg2_type = LtacArg::RetRegI32;
                                } else if *t == DataType::Float {
                                    instr.arg2_type = LtacArg::RetRegF32;
                                } else if *t == DataType::Double {
                                    instr.arg2_type = LtacArg::RetRegF64;
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
            
            AstArgType::OpAdd if var.data_type == DataType::Float => {
                instr = ltac::create_instr(LtacType::F32Add);
                instr.arg1_type = LtacArg::FltReg;
                instr.arg1_val = 0;
            },
            
            AstArgType::OpAdd if var.data_type == DataType::Double => {
                instr = ltac::create_instr(LtacType::F64Add);
                instr.arg1_type = LtacArg::FltReg64;
                instr.arg1_val = 0;
            },
            
            AstArgType::OpAdd if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BAdd);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpAdd 
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Add);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpAdd => {
                builder.syntax.ltac_error(line, "Invalid use of addition operator.".to_string());
                return false;
            },
            
            // Subtraction
            
            AstArgType::OpSub if var.data_type == DataType::Float => {
                instr = ltac::create_instr(LtacType::F32Sub);
                instr.arg1_type = LtacArg::FltReg;
                instr.arg1_val = 0;
            },
            
            AstArgType::OpSub if var.data_type == DataType::Double => {
                instr = ltac::create_instr(LtacType::F64Sub);
                instr.arg1_type = LtacArg::FltReg64;
                instr.arg1_val = 0;
            },
            
            AstArgType::OpSub if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BSub);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpSub
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Sub);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpSub => {
                builder.syntax.ltac_error(line, "Invalid use of subtraction operator.".to_string());
                return false;
            },
            
            // Multiplication
            
            AstArgType::OpMul if var.data_type == DataType::Float => {
                instr = ltac::create_instr(LtacType::F32Mul);
                instr.arg1_type = LtacArg::FltReg;
                instr.arg1_val = 0;
            },
            
            AstArgType::OpMul if var.data_type == DataType::Double => {
                instr = ltac::create_instr(LtacType::F64Mul);
                instr.arg1_type = LtacArg::FltReg64;
                instr.arg1_val = 0;
            },
            
            AstArgType::OpMul if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BMul);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpMul
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Mul);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpMul => {
                builder.syntax.ltac_error(line, "Invalid use of multiplication operator.".to_string());
                return false;
            },
            
            // Division
            
            AstArgType::OpDiv if var.data_type == DataType::Float => {
                instr = ltac::create_instr(LtacType::F32Div);
                instr.arg1_type = LtacArg::FltReg;
                instr.arg1_val = 0;
            },
            
            AstArgType::OpDiv if var.data_type == DataType::Double => {
                instr = ltac::create_instr(LtacType::F64Div);
                instr.arg1_type = LtacArg::FltReg64;
                instr.arg1_val = 0;
            },
            
            AstArgType::OpDiv
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Div);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpDiv if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BDiv);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpDiv => {
                builder.syntax.ltac_error(line, "Invalid use of addition operator.".to_string());
                return false;
            },
            
            // Modulo
            
            AstArgType::OpMod if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BMod);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpMod
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Mod);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpMod => {
                builder.syntax.ltac_error(line, "Modulo is only valid with integer values.".to_string());
                return false;
            },
            
            // Logical and
            
            AstArgType::OpAnd if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BAnd);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpAnd
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32And);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpAnd => {
                builder.syntax.ltac_error(line, "Invalid use of logical and.".to_string());
                return false;
            },
            
            // Logical OR
            
            AstArgType::OpOr if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BOr);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpOr
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Or);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpOr => {
                builder.syntax.ltac_error(line, "Invalid use of logical or.".to_string());
                return false;
            },
            
            // Logical XOR
            
            AstArgType::OpXor if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BXor);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpXor
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Xor);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpXor => {
                builder.syntax.ltac_error(line, "Invalid use of logical xor.".to_string());
                return false;
            },
            
            // Left shift
            
            AstArgType::OpLeftShift if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BLsh);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpLeftShift
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Lsh);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpLeftShift => {
                builder.syntax.ltac_error(line, "Invalid use of left shift.".to_string());
                return false;
            },
            
            // Right shift
            
            AstArgType::OpRightShift if var.data_type == DataType::Byte => {
                instr = ltac::create_instr(LtacType::BRsh);
                instr.arg1_type = LtacArg::Reg8(0);
            },
            
            AstArgType::OpRightShift
            if (var.data_type == DataType::Int || var.data_type == DataType::IntDynArray) => {
                instr = ltac::create_instr(LtacType::I32Rsh);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            AstArgType::OpRightShift => {
                builder.syntax.ltac_error(line, "Invalid use of right shift.".to_string());
                return false;
            },
            
            _ => {},
        }
    }
    
    //Store the result back
    // If it was a single assign (no math), compact the instructions
    if line.args.len() == 1 && first_type != AstArgType::Id {
        let top = builder.file.code.pop().unwrap();
        
        instr = ltac::create_instr(top.instr_type);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        instr.arg2_type = top.arg2_type;
        instr.arg2_val = top.arg2_val;
        instr.arg2_wval = top.arg2_wval;
        instr.arg2_sval = top.arg2_sval;
        instr.arg2_offset = top.arg2_offset;
        instr.arg2_offset_size = top.arg2_offset_size;
        
    // Store back a byte
    } else if var.data_type == DataType::Byte {
        instr = ltac::create_instr(LtacType::MovB);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        instr.arg2_type = LtacArg::Reg8(0);
        
    // Store back a float
    } else if var.data_type == DataType::Float {
        instr = ltac::create_instr(LtacType::MovF32);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        instr.arg2_type = LtacArg::FltReg;
        instr.arg2_val = 0;
        
    // Store back a double
    } else if var.data_type == DataType::Double {
        instr = ltac::create_instr(LtacType::MovF64);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        instr.arg2_type = LtacArg::FltReg64;
        instr.arg2_val = 0;
        
    // Store back everything else
    } else {
        instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        instr.arg2_type = LtacArg::Reg32(0);
    }
    
    if line.sub_args.len() > 0 {
        let first_arg = line.sub_args.last().unwrap();
        
        if line.sub_args.len() == 1 {
            if first_arg.arg_type == AstArgType::IntL {
                instr.instr_type = LtacType::MovOffImm;
                instr.arg1_offset = first_arg.i32_val * 4;
            } else if first_arg.arg_type == AstArgType::Id {
                instr.instr_type = LtacType::MovOffMem;
                instr.arg1_offset_size = 4;
                
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
        
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        
        match &arg.arg_type {
            AstArgType::StringL => {
                let name = builder.build_string(arg.str_val.clone());
                
                instr.arg2_type = LtacArg::Ptr;
                instr.arg2_sval = name;
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
                        instr2.arg2_type = LtacArg::Mem;
                        instr2.arg2_val = v.pos;
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

    

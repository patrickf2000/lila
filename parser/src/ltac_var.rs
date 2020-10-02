
use crate::ltac_builder::*;
use crate::ast;
use crate::ast::{AstStmt, AstStmtType, AstModType, AstArg, AstArgType};
use crate::ltac;
use crate::ltac::{LtacType, LtacArg};

use crate::ltac_array::*;
use crate::ltac_func::*;

// Builds an LTAC variable declaration
pub fn build_var_dec(builder : &mut LtacBuilder, line : &AstStmt, arg_no : i32) {
    let name = line.name.clone();
    let ast_data_type = &line.modifiers[0];
    let data_type : DataType;
    
    match &ast_data_type.mod_type {
        AstModType::Int => {
            data_type = DataType::Int;
            builder.stack_pos += 4;
        },
        
        AstModType::IntDynArray => {
            data_type = DataType::IntDynArray;
            builder.stack_pos += 8
        },
    }
    
    let v = Var {
        pos : builder.stack_pos,
        data_type : data_type,
    };
    
    builder.vars.insert(name, v);
    
    // If we have a function argument, add the load instruction
    if arg_no > 0 {
        let mut ld = ltac::create_instr(LtacType::LdArgI32);
        ld.arg1_val = builder.stack_pos;
        ld.arg2_val = arg_no;
        builder.file.code.push(ld);
    } else {
        build_var_assign(builder, line);
    }
}

// Builds an LTAC variable assignment
pub fn build_var_assign(builder : &mut LtacBuilder, line : &AstStmt) {
    let var : Var;
    match builder.vars.get(&line.name) {
        Some(v) => var = v.clone(),
        None => return,
    }
    
    if var.data_type == DataType::Int {
        if line.args.len() == 1 {
            build_i32var_single_assign(builder, &line.args, &var);
        } else {
            build_i32var_math(builder, &line, &var);
        }
    } else if var.data_type == DataType::IntDynArray {
        build_i32dyn_array(builder, &line.sub_args, &var);
    }
}

// Builds a single int32 variable assignment
pub fn build_i32var_single_assign(builder : &mut LtacBuilder, args : &Vec<AstArg>, var : &Var) {
    let arg = &args[0];
    
    let mut instr = ltac::create_instr(LtacType::Mov);
    instr.arg1_type = LtacArg::Mem;
    instr.arg1_val = var.pos;
    
    match &arg.arg_type {
        AstArgType::IntL => {
            instr.arg2_type = LtacArg::I32;
            instr.arg2_val = arg.i32_val;
        },
        
        AstArgType::Id => {
            let mut size = 1;
        
            match builder.vars.get(&arg.str_val) {
                Some(v) => {
                    instr.arg2_val = v.pos;
                    
                    if v.data_type == DataType::IntDynArray {
                        size = 4;
                    }
                },
                
                None => {
                    if builder.functions.contains(&arg.str_val) {
                        // Create a statement to build the rest of the function call
                        let mut stmt = ast::create_stmt(AstStmtType::FuncCall);
                        stmt.name = arg.str_val.clone();
                        stmt.args = arg.sub_args.clone();
                        build_func_call(builder, &stmt);
                        
                        instr.arg2_type = LtacArg::RetRegI32;
                        builder.file.code.push(instr);
                        return;
                    }
                },
            }
            
            instr.arg2_type = LtacArg::Mem;
            
            // TODO: Add the rest of the variations
            if arg.sub_args.len() > 0 {
                let first_arg = arg.sub_args.last().unwrap();
                
                if arg.sub_args.len() == 1 {
                    if first_arg.arg_type == AstArgType::IntL {
                        instr.instr_type = LtacType::MovOffImm;
                        instr.arg2_offset = first_arg.i32_val * size;
                    } else if first_arg.arg_type == AstArgType::Id {
                        let mut instr2 = ltac::create_instr(LtacType::MovOffMem);
                        instr2.arg1_type = LtacArg::Reg;
                        instr2.arg1_val = 0;
                        
                        instr2.arg2_type = LtacArg::Mem;
                        instr2.arg2_val = instr.arg2_val;
                        instr2.arg2_offset_size = size;
                        
                        match builder.vars.get(&first_arg.str_val) {
                            Some(v) => instr2.arg2_offset = v.pos,
                            None => instr2.arg2_offset = 0,
                        };
                        
                        builder.file.code.push(instr2);
                        
                        instr.arg2_type = LtacArg::Reg;
                        instr.arg2_val = 0;
                    }
                }
            }
        },
            
        _ => { /* TODO ERROR */ },
    }
    
    builder.file.code.push(instr);
}

// Builds an int32 math assignment
pub fn build_i32var_math(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) {
    let args = &line.args;

    let mut instr = ltac::create_instr(LtacType::Mov);
    instr.arg1_type = LtacArg::Reg;
    instr.arg1_val = 1;
    
    for arg in args.iter() {
        match &arg.arg_type {
            AstArgType::IntL => {
                instr.arg2_type = LtacArg::I32;
                instr.arg2_val = arg.i32_val;
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::StringL => {},
            
            AstArgType::Id => {
                match builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        instr.arg2_type = LtacArg::Mem;
                        instr.arg2_val = v.pos;
                
                        // TODO: Add the rest of the variations
                        if arg.sub_args.len() > 0 {
                            let first_arg = arg.sub_args.last().unwrap();
                            
                            if arg.sub_args.len() == 1 && arg.arg_type == AstArgType::IntL {
                                instr.instr_type = LtacType::MovOffImm;
                                instr.arg2_offset = first_arg.i32_val;
                            }
                        }
                    },
                    
                    None => {
                        if builder.functions.contains(&arg.str_val) {
                            // Create a statement to build the rest of the function call
                            let mut stmt = ast::create_stmt(AstStmtType::FuncCall);
                            stmt.name = arg.str_val.clone();
                            stmt.args = arg.sub_args.clone();
                            build_func_call(builder, &stmt);
                            
                            instr.arg2_type = LtacArg::RetRegI32;
                        }
                    }
                }
                
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::OpAdd => {
                instr = ltac::create_instr(LtacType::I32Add);
                instr.arg1_type = LtacArg::Reg;
                instr.arg1_val = 1;
            },
            
            AstArgType::OpMul => {
                instr = ltac::create_instr(LtacType::I32Mul);
                instr.arg1_type = LtacArg::Reg;
                instr.arg1_val = 1;
            },
            
            _ => {},
        }
    }
    
    //Store the result back
    instr = ltac::create_instr(LtacType::Mov);
    instr.arg1_type = LtacArg::Mem;
    instr.arg1_val = var.pos;
    instr.arg2_type = LtacArg::Reg;
    instr.arg2_val = 1;
    
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
}
    

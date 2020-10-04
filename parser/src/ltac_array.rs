
use crate::ltac_builder::*;
use crate::ltac;
use crate::ltac::{LtacType, LtacArg, LtacInstr};
use crate::ast::{AstStmt, AstArg, AstArgType};

use crate::ltac_var::*;

// Assigns a value to an array
pub fn build_array_assign(builder : &mut LtacBuilder, line : &AstStmt) {
    let var : Var;
    match builder.vars.get(&line.name) {
        Some(v) => var = v.clone(),
        None => return,
    }
    
    if var.data_type == DataType::IntDynArray {
        if line.args.len() == 1 {
            build_i32array_single_assign(builder, &line, &var);
        } else {
            build_i32var_math(builder, &line, &var);
        }
    }
}

// An internal function to free any dynamic arrays in the current context
pub fn free_arrays(builder : &mut LtacBuilder) {
    for (_name, var) in &builder.vars {
        if var.data_type == DataType::IntDynArray && !var.is_param {
            let mut pusharg = ltac::create_instr(LtacType::PushArg);
            pusharg.arg1_type = LtacArg::Ptr;
            pusharg.arg1_val = var.pos;
            pusharg.arg2_val = 1;
            builder.file.code.push(pusharg);
            
            let call = ltac::create_instr(LtacType::Free);
            builder.file.code.push(call);
        }
    }
}

// Initializes a 32-bit integer array in the heap
pub fn build_i32dyn_array(builder : &mut LtacBuilder, args : &Vec<AstArg>, var : &Var) {
    if args.len() == 1 && args.last().unwrap().arg_type == AstArgType::IntL {
        let arg = args.last().unwrap();
        
        let mut pusharg = ltac::create_instr(LtacType::PushArg);
        pusharg.arg1_type = LtacArg::I32;
        pusharg.arg1_val = arg.i32_val * 4;
        pusharg.arg2_val = 1;
        
        builder.file.code.push(pusharg);
    } else {
        //TODO
    }
    
    let mut instr = ltac::create_instr(LtacType::Malloc);
    builder.file.code.push(instr);
    
    // Move the return register back to the variable
    instr = ltac::create_instr(LtacType::Mov);
    instr.arg1_type = LtacArg::Mem;
    instr.arg1_val = var.pos;
    instr.arg2_type = LtacArg::RetRegI64;
    builder.file.code.push(instr);
}

// Builds a single int32 array assignment
pub fn build_i32array_single_assign(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) {
    let arg = &line.args[0];
    let mut instr : LtacInstr;
    
    if line.sub_args.len() == 1 {
        let sub_arg = line.sub_args.last().unwrap();
        
        if sub_arg.arg_type == AstArgType::IntL {
            instr = ltac::create_instr(LtacType::MovOffImm);
            instr.arg1_type = LtacArg::Mem;
            instr.arg1_val = var.pos;
            instr.arg1_offset = sub_arg.i32_val * 4;
        } else if sub_arg.arg_type == AstArgType::Id {
            instr = ltac::create_instr(LtacType::MovOffMem);
            instr.arg1_type = LtacArg::Mem;
            instr.arg1_val = var.pos;
            instr.arg1_offset_size = 4;
            
            match builder.vars.get(&sub_arg.str_val) {
                Some(v) => instr.arg1_offset = v.pos,
                None => instr.arg1_offset = 0,
            }
        } else {
            // TODO: This is wrong
            instr = ltac::create_instr(LtacType::Mov);
        }
    } else {
        // TODO: This is wrong
        instr = ltac::create_instr(LtacType::Mov);
    }
    
    match &arg.arg_type {
        AstArgType::IntL => {
            instr.arg2_type = LtacArg::I32;
            instr.arg2_val = arg.i32_val;
        },
        
        AstArgType::Id => {
            let mut instr2 = ltac::create_instr(LtacType::Mov);
            instr2.arg1_type = LtacArg::Reg;
            instr2.arg1_val = 0;
            instr2.arg2_type = LtacArg::Mem;
            
            match builder.vars.get(&arg.str_val) {
                Some(v) => instr2.arg2_val = v.pos,
                None => instr2.arg2_val = 0,
            }
            
            builder.file.code.push(instr2);
            
            instr.arg2_type = LtacArg::Reg;
            instr.arg2_val = 0;
        },
        _ => { /* TODO ERROR */ },
    }
    
    builder.file.code.push(instr);
}


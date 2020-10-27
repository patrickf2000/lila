
use crate::ltac_builder::*;
use crate::ltac;
use crate::ltac::{LtacType, LtacArg};
use crate::ast::{AstStmt, AstArgType};

use crate::ltac_var::*;

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
pub fn free_arrays(builder : &mut LtacBuilder) {
    for (_name, var) in &builder.vars {
        if (var.data_type == DataType::ByteDynArray || var.data_type == DataType::UByteDynArray ||
            var.data_type == DataType::IntDynArray)
            && !var.is_param {
            
            let mut pusharg = ltac::create_instr(LtacType::PushArg);
            pusharg.arg1_type = LtacArg::Ptr(var.pos);
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
    
    // Create the array
    if sub_args.len() == 1 && sub_args.last().unwrap().arg_type == AstArgType::IntL {
        let arg = sub_args.last().unwrap();
        let mut size = 4;
        
        if var.data_type == DataType::ByteDynArray || var.data_type == DataType::UByteDynArray {
            size = 1;
        }
        
        let mut pusharg = ltac::create_instr(LtacType::PushArg);
        pusharg.arg1_type = LtacArg::I32(arg.i32_val * size);
        pusharg.arg2_val = 1;
        
        builder.file.code.push(pusharg);
        
        let mut instr = ltac::create_instr(LtacType::Malloc);
        builder.file.code.push(instr);
        
        // Move the return register back to the variable
        instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Mem(var.pos);
        instr.arg2_type = LtacArg::RetRegI64;
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
    instr.arg1_type = LtacArg::Reg32(0);

    for arg in line.args.iter() {
        match &arg.arg_type {
            AstArgType::Id => {
                match &builder.vars.get(&arg.str_val) {
                    Some(v) => instr.arg2_type = LtacArg::Mem(v.pos),
                    None => {
                        builder.syntax.ltac_error(line, "Invalid variable.".to_string());
                        return false;
                    },
                }
                
                instr.arg2_offset = 0;
                instr.arg2_offset_size = 4;
                builder.file.code.push(instr.clone());
            },
            
            AstArgType::OpAdd => {
                instr = ltac::create_instr(LtacType::I32VAdd);
                instr.arg1_type = LtacArg::Reg32(0);
            },
            
            _ => {
                builder.syntax.ltac_error(line, "Invalid expression for vector math.".to_string());
                return false;
            }
        }
    }
    
    // The final move instruction
    instr = ltac::create_instr(LtacType::MovI32Vec);
    instr.arg1_type = LtacArg::Mem(var.pos);
    instr.arg2_type = LtacArg::Reg32(0);
    instr.arg2_offset_size = 4;
    
    builder.file.code.push(instr.clone());
    
    true
}


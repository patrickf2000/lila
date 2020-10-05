
use crate::ltac_builder::*;
use crate::ltac;
use crate::ltac::{LtacType, LtacArg};
use crate::ast::{AstStmt, AstArgType};

use crate::ltac_array::*;

// Builds an LTAC function call
pub fn build_func_call(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    let mut arg_type = LtacType::PushArg;
    let mut call_type = LtacType::Call;
    
    if line.name == "syscall" {
        arg_type = LtacType::KPushArg;
        call_type = LtacType::Syscall;
    }
    
    // Represents the current argument position
    let mut arg_no : i32 = 1;

    // Build the arguments
    for arg in line.args.iter() {
        match &arg.arg_type {
            AstArgType::IntL => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1_type = LtacArg::I32;
                push.arg1_val = arg.i32_val.clone();
                push.arg2_val = arg_no;
                builder.file.code.push(push);
            },
            
            AstArgType::StringL => {
                let name = builder.build_string(arg.str_val.clone());
                
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1_type = LtacArg::Ptr;
                push.arg1_sval = name;
                push.arg2_val = arg_no;
                builder.file.code.push(push);
            },
            
            AstArgType::Id => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg2_val = arg_no;
                push.arg1_type = LtacArg::Mem;
                
                match &builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        push.arg1_val = v.pos;
                        
                        if v.data_type == DataType::IntDynArray {
                            push.arg1_type = LtacArg::Ptr;
                        }
                    },
                    
                    None => {
                        let mut msg = "Invalid variable name: ".to_string();
                        msg.push_str(&arg.str_val);
                        
                        builder.syntax.ltac_error(line, msg);
                        return false;
                    },
                }
                
                builder.file.code.push(push);
            },
            
            _ => {},
        }
        
        arg_no = arg_no + 1;
    }
    
    // Build the call
    let mut fc = ltac::create_instr(call_type);
    fc.name = line.name.clone();
    builder.file.code.push(fc);
    
    true
}

// Builds a function return
pub fn build_return(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    if line.args.len() > 0 && builder.current_type == DataType::Void {
        let mut msg = "Cannot return value in void function: ".to_string();
        msg.push_str(&builder.current_func);
         
        builder.syntax.ltac_error(line, msg);
        return false;
    }

    free_arrays(builder);

    if line.args.len() == 1 {
        let arg1 = line.args.first().unwrap();
        let mut mov = ltac::create_instr(LtacType::Mov);
        mov.arg1_type = LtacArg::RetRegI32;
        
        match &arg1.arg_type {
            AstArgType::IntL => {
                mov.arg2_type = LtacArg::I32;
                mov.arg2_val = arg1.i32_val;
            },
            
            AstArgType::StringL => {},
            
            AstArgType::Id => {
                mov.arg2_type = LtacArg::Mem;
                
                match builder.vars.get(&arg1.str_val) {
                    Some(v) => mov.arg2_val = v.pos,
                    None => mov.arg2_val = 0,
                }
            },
            
            _ => {},
        }
        
        builder.file.code.push(mov);
    } else if line.args.len() > 1 {
        // TODO
    }
    
    let ret = ltac::create_instr(LtacType::Ret);
    builder.file.code.push(ret);
    
    true
}

// Builds the end of a block
pub fn build_end(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    if builder.block_layer == 0 {
        let last = builder.file.code.last().unwrap();
        
        if last.instr_type != LtacType::Ret {
            free_arrays(builder);
            
            // See if there was supposed to be a return instruction
            if builder.current_type != DataType::Void {
                let mut msg = "Expected return in function: ".to_string();
                msg.push_str(&builder.current_func);
                
                builder.syntax.ltac_error(line, msg);
                return false;
            }
            
            // Otherwise, create a void instruction
            let ret = ltac::create_instr(LtacType::Ret);
            builder.file.code.push(ret);
        }
    } else {
        builder.block_layer -= 1;
        
        if builder.loop_layer > 0 {
            builder.loop_layer -= 1;
            
            builder.end_labels.pop();
            builder.loop_labels.pop();
        }
        
        if builder.label_stack.len() > 0 {
            let mut label = ltac::create_instr(LtacType::Label);
            label.name = builder.label_stack.pop().unwrap();
            builder.file.code.push(label);
        }
        
        if builder.top_label_stack.len() > 0 {
            let mut label = ltac::create_instr(LtacType::Label);
            label.name = builder.top_label_stack.pop().unwrap();
            builder.file.code.push(label);
        }
        
        if builder.code_stack.len() > 0 {
            let sub_block = builder.code_stack.pop().unwrap();
            
            for item in sub_block.iter() {
                builder.file.code.push(item.clone());
            }
        }
    }
    
    true
}


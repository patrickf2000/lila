
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
    let mut flt_arg_no : i32 = 1;

    // Build the arguments
    for arg in line.args.iter() {
        match &arg.arg_type {
            AstArgType::ByteL => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1_type = LtacArg::UByte(arg.u8_val);
                push.arg2_val = arg_no;
                builder.file.code.push(push);
                
                arg_no += 1;
            },
            
            AstArgType::ShortL => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1_type = LtacArg::U16(arg.u16_val);
                push.arg2_val = arg_no;
                builder.file.code.push(push);
                
                arg_no += 1;
            },
            
            AstArgType::IntL => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1_type = LtacArg::U32(arg.u32_val);
                push.arg2_val = arg_no;
                builder.file.code.push(push);
                
                arg_no += 1;
            },
            
            AstArgType::FloatL if call_type == LtacType::Syscall => {
                builder.syntax.ltac_error(line, "Only integers and strings are valid in system calls.".to_string());
                return false;
            },
            
            AstArgType::FloatL => {
                let mut push = ltac::create_instr(LtacType::PushArg);
                let name = builder.build_float(arg.f64_val, false);
                push.arg1_type = LtacArg::F32(name);
                push.arg2_val = flt_arg_no;
                builder.file.code.push(push);
                
                flt_arg_no += 1;  
            },
            
            AstArgType::StringL => {
                let name = builder.build_string(arg.str_val.clone());
                
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg1_type = LtacArg::PtrLcl(name);
                push.arg2_val = arg_no;
                builder.file.code.push(push);
                
                arg_no += 1;
            },
            
            AstArgType::Id => {
                let mut push = ltac::create_instr(arg_type.clone());
                push.arg2_val = arg_no;
                
                match &builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        push.arg1_type = LtacArg::Mem(v.pos);
                        
                        if v.data_type == DataType::Byte {
                            push.arg2_type = LtacArg::Byte(0);
                            
                        } else if v.data_type == DataType::UByte {
                            push.arg2_type = LtacArg::UByte(0);
                            
                        } else if v.data_type == DataType::Short {
                            push.arg2_type = LtacArg::I16(0);
                            
                        } else if v.data_type == DataType::UShort {
                            push.arg2_type = LtacArg::U16(0);
                            
                        } else if v.data_type == DataType::IntDynArray || v.data_type == DataType::Str {
                            push.arg1_type = LtacArg::Ptr(v.pos);
                            
                        } else if v.data_type == DataType::Float {
                            push.arg2_type = LtacArg::FltReg(flt_arg_no);
                            
                        } else if v.data_type == DataType::Double {
                            push.arg2_type = LtacArg::FltReg64(flt_arg_no);
                        }
                        
                        // For the proper registers
                        if v.data_type == DataType::Float || v.data_type == DataType::Double {
                            flt_arg_no += 1;
                        } else {
                            push.arg2_val = arg_no;
                            arg_no += 1;
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
        
        match &builder.current_type {
            DataType::Byte => {
                mov = ltac::create_instr(LtacType::MovB);
                mov.arg1_type = LtacArg::RetRegI8;
            },
            
            DataType::UByte => {
                mov = ltac::create_instr(LtacType::MovUB);
                mov.arg1_type = LtacArg::RetRegU8;
            },
            
            DataType::Short => {
                mov = ltac::create_instr(LtacType::MovW);
                mov.arg1_type = LtacArg::RetRegI16;
            },
            
            DataType::UShort => {
                mov = ltac::create_instr(LtacType::MovUW);
                mov.arg1_type = LtacArg::RetRegU16;
            },
            
            DataType::UInt => {
                mov = ltac::create_instr(LtacType::MovU);
                mov.arg1_type = LtacArg::RetRegU32;
            },
            
            DataType::Float => {
                mov = ltac::create_instr(LtacType::MovF32);
                mov.arg1_type = LtacArg::RetRegF32;
            },
            
            DataType::Double => {
                mov = ltac::create_instr(LtacType::MovF64);
                mov.arg1_type = LtacArg::RetRegF64;
            },
            
            _ => mov.arg1_type = LtacArg::RetRegI32,
        }
        
        match &arg1.arg_type {
            AstArgType::ByteL => {
                if builder.current_type == DataType::UByte {
                    mov.arg2_type = LtacArg::UByte(arg1.u8_val);
                } else {
                    mov.arg2_type = LtacArg::Byte(arg1.u8_val as i8);
                }
            },
            
            AstArgType::ShortL => {
                if builder.current_type == DataType::UShort {
                    mov.arg2_type = LtacArg::U16(arg1.u16_val);
                } else {
                    mov.arg2_type = LtacArg::I16(arg1.u16_val as i16);
                }
            },
        
            AstArgType::IntL => {
                mov.arg2_type = LtacArg::I32(arg1.u32_val as i32);
            },
            
            AstArgType::FloatL => {
                if builder.current_type == DataType::Float {
                    let name = builder.build_float(arg1.f64_val, false);
                    mov.arg2_type = LtacArg::F32(name);
                } else {
                    let name = builder.build_float(arg1.f64_val, true);
                    mov.arg2_type = LtacArg::F64(name);
                }
            }
            
            AstArgType::StringL => {},
            
            AstArgType::Id => {
                match builder.vars.get(&arg1.str_val) {
                    Some(v) => mov.arg2_type = LtacArg::Mem(v.pos),
                    None => {/* TODO: Syntax error */},
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

// Builds the exit keyword
pub fn build_exit(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    free_arrays(builder);
    
    let mut instr = ltac::create_instr(LtacType::Exit);
    instr.arg1_type = LtacArg::I32(0);
    
    if line.args.len() == 1 {
        //TODO
    } else if line.args.len() > 1 {
        builder.syntax.ltac_error(line, "You can only have one argument in the \"exit\" statement.".to_string());
        return false;
    }
    
    builder.file.code.push(instr);

    true
}

// Builds the end of a block
pub fn build_end(builder : &mut LtacBuilder, line : &AstStmt) -> bool {
    if builder.block_layer == 0 {
        let last = builder.file.code.last().unwrap();
        
        if last.instr_type != LtacType::Ret && last.instr_type != LtacType::Exit {
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



use crate::ltac_builder::*;
use crate::ast::{AstStmt, AstModType, AstArgType};
use crate::ltac;
use crate::ltac::{LtacInstr, LtacType, LtacArg};

use crate::ltac_expr::*;
use crate::ltac_array::*;

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
        
        AstModType::U64DynArray => {
            data_type = DataType::U64DynArray;
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
        
        AstModType::Char => {
            data_type = DataType::Char;
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
        let data_type = ast_to_datatype(ast_data_type);
        let mem = LtacArg::Mem(builder.stack_pos);
        let ld : LtacInstr;
        
        if data_type == DataType::Float || data_type == DataType::Double {
            ld = ldarg_for_type(&data_type, mem, flt_arg_no);
            flt_arg_no += 1;
        } else {
            ld = ldarg_for_type(&data_type, mem, arg_no);
            arg_no += 1;
        }
        
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
       var.data_type == DataType::I64DynArray || var.data_type == DataType::U64DynArray ||
       var.data_type == DataType::FloatDynArray || var.data_type == DataType::DoubleDynArray {
        code = build_dyn_array(builder, &line, &var);
    } else if var.data_type == DataType::Str {
        code = build_str_assign(builder, &line, &var);
    } else {
        code = build_var_math(builder, &line, &var);
    }
    
    code
}

// Builds a string variable assignment
pub fn build_str_assign(builder : &mut LtacBuilder, line : &AstStmt, var : &Var) -> bool {
    let mut instr = ltac::create_instr(LtacType::Mov);
    
    if line.args.len() == 1 {
        let arg = line.args.first().unwrap();
        
        instr.arg1 = LtacArg::Mem(var.pos);
        
        match &arg.arg_type {
            AstArgType::StringL => {
                let name = builder.build_string(arg.str_val.clone());
                instr.arg2 = LtacArg::PtrLcl(name);
            },
            
            AstArgType::Id => {
                match &builder.vars.get(&arg.str_val) {
                    Some(v) => {
                        if v.data_type != DataType::Str {
                            builder.syntax.ltac_error(line, "You can only assign a string to a string.".to_string());
                            return false;
                        }
                        
                        instr.arg2 = LtacArg::Reg64(0);
                        
                        let mut instr2 = ltac::create_instr(LtacType::Mov);
                        instr2.arg1 = LtacArg::Reg64(0);
                        instr2.arg2 = LtacArg::Mem(v.pos);
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

    

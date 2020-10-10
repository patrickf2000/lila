
use crate::ltac_builder::*;

use crate::ast::{AstStmt, AstStmtType, AstArgType};
use crate::ltac;
use crate::ltac::{LtacType, LtacInstr, LtacArg};

// Break out of a current loop
pub fn build_break(builder : &mut LtacBuilder) {
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = builder.end_labels.last().unwrap().to_string();
    builder.file.code.push(br);
}

// Continue through the rest of the loop
pub fn build_continue(builder : &mut LtacBuilder) {
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = builder.loop_labels.last().unwrap().to_string();
    builder.file.code.push(br);
}

// A utility function to create a label
fn create_label(builder : &mut LtacBuilder, is_top : bool) {
    let lbl_pos = builder.str_pos.to_string();
    builder.str_pos += 1;
    
    let mut name = "L".to_string();
    name.push_str(&lbl_pos);
    
    if is_top {
        builder.top_label_stack.push(name);
    } else {
        builder.label_stack.push(name);
    }
}

// Builds an LTAC conditional block
pub fn build_cond(builder : &mut LtacBuilder, line : &AstStmt) {
    if line.stmt_type == AstStmtType::If {
        builder.block_layer += 1;
        create_label(builder, true);
        
        // A dummy placeholder
        let code_block : Vec<LtacInstr> = Vec::new();
        builder.code_stack.push(code_block);
    } else {
        let mut jmp = ltac::create_instr(LtacType::Br);
        jmp.name = builder.top_label_stack.last().unwrap().to_string();
        builder.file.code.push(jmp);
    
        let mut label = ltac::create_instr(LtacType::Label);
        label.name = builder.label_stack.pop().unwrap();
        builder.file.code.push(label);
        
        if line.stmt_type == AstStmtType::Else {
            return;
        }
    }
    
    create_label(builder, false);
    
    // Build the conditional statement
    let arg1 = &line.args.iter().nth(0).unwrap();
    let arg2 = &line.args.iter().nth(2).unwrap();
    
    let mut cmp = ltac::create_instr(LtacType::I32Cmp);
    
    // Although we assume its integer comparison by default, the first operand
    // determines the comparison type
    match &arg1.arg_type {
        AstArgType::IntL => {
            cmp.arg1_type = LtacArg::I32;
            cmp.arg1_val = arg1.i32_val;
        },
        
        AstArgType::FloatL => {
            cmp = ltac::create_instr(LtacType::F32Cmp);
            cmp.arg1_type = LtacArg::F32;
            cmp.arg1_sval = builder.build_float(arg1.f64_val, false);
        },
        
        AstArgType::StringL => {},
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg;
            mov.arg1_val = 0;
            mov.arg2_type = LtacArg::Mem;
            
            match &builder.vars.get(&arg1.str_val) {
                Some(v) => {
                    // String comparisons
                    if v.data_type == DataType::Str {
                        cmp = ltac::create_instr(LtacType::StrCmp);
                        
                        mov = ltac::create_instr(LtacType::PushArg);
                        mov.arg1_type = LtacArg::Ptr;
                        mov.arg1_val = v.pos;
                        mov.arg2_val = 1;
                        
                    // Float-32 comparisons
                    } else if v.data_type == DataType::Float {
                        mov = ltac::create_instr(LtacType::MovF32);
                        mov.arg1_type = LtacArg::FltReg;
                        mov.arg1_val = 0;
                        mov.arg2_type = LtacArg::Mem;
                        mov.arg2_val = v.pos;
                        
                        cmp = ltac::create_instr(LtacType::F32Cmp);
                        cmp.arg1_type = LtacArg::FltReg;
                        cmp.arg1_val = 0;
                        
                    // Float-64 comparisons
                    } else if v.data_type == DataType::Double {
                        mov = ltac::create_instr(LtacType::MovF64);
                        mov.arg1_type = LtacArg::FltReg64;
                        mov.arg1_val = 0;
                        mov.arg2_type = LtacArg::Mem;
                        mov.arg2_val = v.pos;
                        
                        cmp = ltac::create_instr(LtacType::F64Cmp);
                        cmp.arg1_type = LtacArg::FltReg64;
                        cmp.arg1_val = 0;
                        
                    // Integer comparisons
                    } else {
                        mov.arg2_val = v.pos;
                        
                        cmp.arg1_type = LtacArg::Reg;
                        cmp.arg1_val = 0;
                    }
                    
                    builder.file.code.push(mov);
                },
                
                None => mov.arg2_val = 0,
            }
        },
        
        _ => {},
    }
    
    match &arg2.arg_type {
        AstArgType::IntL => {
            cmp.arg2_type = LtacArg::I32;
            cmp.arg2_val = arg2.i32_val;
        },
        
        AstArgType::FloatL => {
            if cmp.instr_type == LtacType::F64Cmp {
                cmp.arg2_type = LtacArg::F64;
                cmp.arg2_sval = builder.build_float(arg2.f64_val, true);
            } else {
                cmp.arg2_type = LtacArg::F32;
                cmp.arg2_sval = builder.build_float(arg2.f64_val, false);
            }
        },
        
        AstArgType::StringL => {},
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg;
            mov.arg1_val = 1;
            mov.arg2_type = LtacArg::Mem;
            
            match &builder.vars.get(&arg2.str_val) {
                Some(v) => {
                    if v.data_type == DataType::Str {
                        mov = ltac::create_instr(LtacType::PushArg);
                        mov.arg1_type = LtacArg::Ptr;
                        mov.arg1_val = v.pos;
                        mov.arg2_val = 2;
                        
                    } else if v.data_type == DataType::Float {
                        mov = ltac::create_instr(LtacType::MovF32);
                        mov.arg1_type = LtacArg::FltReg;
                        mov.arg1_val = 1;
                        mov.arg2_type = LtacArg::Mem;
                        mov.arg2_val = v.pos;
                        
                        cmp.arg2_type = LtacArg::FltReg;
                        cmp.arg2_val = 1;
                        
                    } else if v.data_type == DataType::Double {
                        mov = ltac::create_instr(LtacType::MovF64);
                        mov.arg1_type = LtacArg::FltReg64;
                        mov.arg1_val = 1;
                        mov.arg2_type = LtacArg::Mem;
                        mov.arg2_val = v.pos;
                        
                        if cmp.arg1_type == LtacArg::F32 {
                            cmp = ltac::create_instr(LtacType::F64Cmp);
                            cmp.arg1_type = LtacArg::F64;
                            cmp.arg1_sval = builder.build_float(arg1.f64_val, true);
                        }
                        
                        cmp.arg2_type = LtacArg::FltReg64;
                        cmp.arg2_val = 1;
                        
                    } else {
                        mov.arg2_val = v.pos;
                        
                        cmp.arg1_type = LtacArg::Reg;
                        cmp.arg1_val = 0;
                    }
                    
                    builder.file.code.push(mov);
                },
                
                None => mov.arg2_val = 0,
            }
        },
        
        _ => {},
    }
    
    let cmp_type = cmp.instr_type.clone();
    builder.file.code.push(cmp);
    
    // Now the operator
    let op = &line.args.iter().nth(1).unwrap();
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = builder.label_stack.last().unwrap().to_string();
    
    match &op.arg_type {
        AstArgType::OpEq => br.instr_type = LtacType::Bne,
        AstArgType::OpNeq => br.instr_type = LtacType::Be,
        
        AstArgType::OpLt 
            if (cmp_type == LtacType::F32Cmp || cmp_type == LtacType::F64Cmp)
            => br.instr_type = LtacType::Bfge,
        AstArgType::OpLt => br.instr_type = LtacType::Bge,
        
        AstArgType::OpLte
            if (cmp_type == LtacType::F32Cmp || cmp_type == LtacType::F64Cmp)
            => br.instr_type = LtacType::Bfg,
        AstArgType::OpLte => br.instr_type = LtacType::Bg,
        
        AstArgType::OpGt
            if (cmp_type == LtacType::F32Cmp || cmp_type == LtacType::F64Cmp)
            => br.instr_type = LtacType::Bfle,
        AstArgType::OpGt => br.instr_type = LtacType::Ble,
        
        AstArgType::OpGte
            if (cmp_type == LtacType::F32Cmp || cmp_type == LtacType::F64Cmp)
            => br.instr_type = LtacType::Bfl,
        AstArgType::OpGte => br.instr_type = LtacType::Bl,
        
        _ => {},
    }
    
    builder.file.code.push(br);
}

// Builds a while loop block
pub fn build_while(builder : &mut LtacBuilder, line : &AstStmt) {
    builder.block_layer += 1;
    builder.loop_layer += 1;
    
    create_label(builder, false);    // Goes at the very end
    create_label(builder, false);    // Add a comparison label
    create_label(builder, false);   // Add a loop label
    
    let end_label = builder.label_stack.pop().unwrap();
    let loop_label = builder.label_stack.pop().unwrap();
    let cmp_label = builder.label_stack.pop().unwrap();
    
    builder.loop_labels.push(cmp_label.clone());
    builder.end_labels.push(end_label.clone());
    
    // Jump to the comparsion label, and add the loop label
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = cmp_label.clone();
    builder.file.code.push(br);
    
    let mut lbl = ltac::create_instr(LtacType::Label);
    lbl.name = loop_label.clone();
    builder.file.code.push(lbl);
    
    // Now build the comparison
    let mut cmp_block : Vec<LtacInstr> = Vec::new();
    
    let mut lbl2 = ltac::create_instr(LtacType::Label);
    lbl2.name = cmp_label.clone();
    cmp_block.push(lbl2);
    
    // Now for the arguments
    let arg1 = &line.args.iter().nth(0).unwrap();
    let arg2 = &line.args.iter().nth(2).unwrap();
    
    let mut cmp = ltac::create_instr(LtacType::I32Cmp);
    
    match &arg1.arg_type {
        AstArgType::IntL => {
            cmp.arg1_type = LtacArg::I32;
            cmp.arg1_val = arg1.i32_val;
        },
        
        AstArgType::StringL => {},
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg;
            mov.arg1_val = 0;
            mov.arg2_type = LtacArg::Mem;
            
            match &builder.vars.get(&arg1.str_val) {
                Some(v) => mov.arg2_val = v.pos,
                None => mov.arg2_val = 0,
            }
            
            cmp_block.push(mov);
            
            cmp.arg1_type = LtacArg::Reg;
            cmp.arg1_val = 0;
        },
        
        _ => {},
    }
    
    match &arg2.arg_type {
        AstArgType::IntL => {
            cmp.arg2_type = LtacArg::I32;
            cmp.arg2_val = arg2.i32_val;
        },
        
        AstArgType::StringL => {},
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg;
            mov.arg1_val = 1;
            mov.arg2_type = LtacArg::Mem;
            
            match &builder.vars.get(&arg2.str_val) {
                Some(v) => mov.arg2_val = v.pos,
                None => mov.arg2_val = 0,
            }
            
            cmp_block.push(mov);
            
            cmp.arg2_type = LtacArg::Reg;
            cmp.arg2_val = 1;
        },
        
        _ => {},
    }
    
    cmp_block.push(cmp);
    
    // Now the operator
    let op = &line.args.iter().nth(1).unwrap();
    let mut br = ltac::create_instr(LtacType::Br);
    br.name = loop_label.clone();
    
    match &op.arg_type {
        AstArgType::OpEq => br.instr_type = LtacType::Be,
        AstArgType::OpNeq => br.instr_type = LtacType::Bne,
        AstArgType::OpLt => br.instr_type = LtacType::Bl,
        AstArgType::OpLte => br.instr_type = LtacType::Ble,
        AstArgType::OpGt => br.instr_type = LtacType::Bg,
        AstArgType::OpGte => br.instr_type = LtacType::Bge,
        _ => {},
    }
    
    cmp_block.push(br);
    
    // The end label
    let mut end_lbl = ltac::create_instr(LtacType::Label);
    end_lbl.name = end_label.clone();
    cmp_block.push(end_lbl);
    
    builder.code_stack.push(cmp_block);
}


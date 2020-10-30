
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
    
    // Set to true if we have a signed byte, short, or int variable
    let mut signed_variant = false;
    
    // Although we assume its integer comparison by default, the first operand
    // determines the comparison type
    match &arg1.arg_type {
        AstArgType::ByteL => {
            let mut mov = ltac::create_instr(LtacType::MovUB);
            mov.arg1_type = LtacArg::Reg8(0);
            mov.arg2_type = LtacArg::UByte(arg1.u8_val);
            builder.file.code.push(mov);
            
            cmp = ltac::create_instr(LtacType::U8Cmp);
            cmp.arg1_type = LtacArg::Reg8(0);
        },
        
        AstArgType::ShortL => {
            let mut mov = ltac::create_instr(LtacType::MovUW);
            mov.arg1_type = LtacArg::Reg16(0);
            mov.arg2_type = LtacArg::U16(arg1.u16_val);
            builder.file.code.push(mov);
            
            cmp = ltac::create_instr(LtacType::U16Cmp);
            cmp.arg1_type = LtacArg::Reg16(0);
        },
        
        AstArgType::IntL => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg32(0);
            mov.arg2_type  = LtacArg::I32(arg1.i32_val);
            builder.file.code.push(mov);
            
            cmp.arg1_type = LtacArg::Reg32(0);
        },
        
        AstArgType::FloatL => {
            let name = builder.build_float(arg1.f64_val, false);
            let mut mov = ltac::create_instr(LtacType::MovF32);
            mov.arg1_type = LtacArg::FltReg(0);
            mov.arg2_type = LtacArg::F32(name);
            builder.file.code.push(mov);
            
            cmp = ltac::create_instr(LtacType::F32Cmp);
            cmp.arg1_type = LtacArg::FltReg(0);
        },
        
        AstArgType::StringL => {},
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg32(0);
            
            match &builder.vars.get(&arg1.str_val) {
                Some(v) => {
                    // String comparisons
                    if v.data_type == DataType::Str {
                        cmp = ltac::create_instr(LtacType::StrCmp);
                        
                        mov = ltac::create_instr(LtacType::PushArg);
                        mov.arg1_type = LtacArg::Ptr(v.pos);
                        mov.arg2_val = 1;
                        
                    // Float-32 comparisons
                    } else if v.data_type == DataType::Float {
                        mov = ltac::create_instr(LtacType::MovF32);
                        mov.arg1_type = LtacArg::FltReg(0);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        cmp = ltac::create_instr(LtacType::F32Cmp);
                        cmp.arg1_type = LtacArg::FltReg(0);
                        
                    // Float-64 comparisons
                    } else if v.data_type == DataType::Double {
                        mov = ltac::create_instr(LtacType::MovF64);
                        mov.arg1_type = LtacArg::FltReg64(0);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        cmp = ltac::create_instr(LtacType::F64Cmp);
                        cmp.arg1_type = LtacArg::FltReg64(0);
                        
                    // Byte comparisons
                    } else if v.data_type == DataType::Byte {
                        cmp= ltac::create_instr(LtacType::I8Cmp);
                        cmp.arg1_type = LtacArg::Reg8(0);
                        
                        mov = ltac::create_instr(LtacType::MovB);
                        mov.arg1_type = LtacArg::Reg8(0);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        signed_variant = true;
                        
                    // Unsigned byte comparisons
                    } else if v.data_type == DataType::UByte {
                        cmp.instr_type = LtacType::U8Cmp;
                        cmp.arg1_type = LtacArg::Reg8(0);
                        
                        mov = ltac::create_instr(LtacType::MovUB);
                        mov.arg1_type = LtacArg::Reg8(0);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                    // Short comparisons
                    } else if v.data_type == DataType::Short {
                        cmp= ltac::create_instr(LtacType::I16Cmp);
                        cmp.arg1_type = LtacArg::Reg16(0);
                        
                        mov = ltac::create_instr(LtacType::MovW);
                        mov.arg1_type = LtacArg::Reg16(0);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        signed_variant = true;
                    
                    // Unsigned short comparisons
                    } else if v.data_type == DataType::UShort {
                        cmp.instr_type = LtacType::U16Cmp;
                        cmp.arg1_type = LtacArg::Reg16(0);
                        
                        mov = ltac::create_instr(LtacType::MovUW);
                        mov.arg1_type = LtacArg::Reg16(0);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                    // Integer comparisons
                    } else {
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        cmp.arg1_type = LtacArg::Reg32(0);
                    }
                    
                    builder.file.code.push(mov);
                },
                
                None => mov.arg2_val = 0,
            }
        },
        
        _ => {},
    }
    
    match &arg2.arg_type {
        AstArgType::ByteL => {
            if signed_variant {
                cmp.arg2_type = LtacArg::Byte(arg2.u8_val as i8);
            } else {
                cmp.arg2_type = LtacArg::UByte(arg2.u8_val);
            }
        },
        
        AstArgType::ShortL => {
            if signed_variant {
                cmp.arg2_type = LtacArg::I16(arg2.u16_val as i16);
            } else {
                cmp.arg2_type = LtacArg::U16(arg2.u16_val);
            }
        },
    
        AstArgType::IntL => {
            cmp.arg2_type = LtacArg::I32(arg2.i32_val);
        },
        
        AstArgType::FloatL => {
            if cmp.instr_type == LtacType::F64Cmp {
                let name = builder.build_float(arg2.f64_val, true);
                cmp.arg2_type = LtacArg::F64(name);
            } else {
                let name = builder.build_float(arg2.f64_val, false);
                cmp.arg2_type = LtacArg::F32(name);
            }
        },
        
        AstArgType::StringL => {},
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg32(1);
            
            match &builder.vars.get(&arg2.str_val) {
                Some(v) => {
                    // Strings
                    if v.data_type == DataType::Str {
                        mov = ltac::create_instr(LtacType::PushArg);
                        mov.arg1_type = LtacArg::Ptr(v.pos);
                        mov.arg2_val = 2;
                        
                    // Single-precision floats
                    } else if v.data_type == DataType::Float {
                        mov = ltac::create_instr(LtacType::MovF32);
                        mov.arg1_type = LtacArg::FltReg(1);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        cmp.arg2_type = LtacArg::FltReg(1);
                        
                    // Double-precision floats
                    } else if v.data_type == DataType::Double {
                        mov = ltac::create_instr(LtacType::MovF64);
                        mov.arg1_type = LtacArg::FltReg64(1);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        match cmp.arg1_type {
                            LtacArg::FltReg(pos) => {
                                builder.file.code.pop();
                                
                                let name = builder.build_float(arg1.f64_val, true);
                                let mut mov = ltac::create_instr(LtacType::MovF64);
                                mov.arg1_type = LtacArg::FltReg64(pos);
                                mov.arg2_type = LtacArg::F64(name);
                                builder.file.code.push(mov);
                                
                                cmp = ltac::create_instr(LtacType::F64Cmp);
                                cmp.arg1_type = LtacArg::FltReg64(pos);
                            },
                            
                            _ => {},
                        }
                        
                        cmp.arg2_type = LtacArg::FltReg64(1);
                        
                    // Bytes
                    } else if v.data_type == DataType::Byte {
                        if arg1.arg_type == AstArgType::ByteL {
                            builder.file.code.pop();
                            mov = ltac::create_instr(LtacType::MovB);
                            mov.arg1_type = LtacArg::Reg8(0);
                            mov.arg2_type = LtacArg::Byte(arg1.u8_val as i8);
                        
                            cmp = ltac::create_instr(LtacType::I8Cmp);
                            cmp.arg1_type = LtacArg::Reg8(0);
                        } else {
                            mov.arg1_type = LtacArg::Empty;
                        }
                        
                        cmp.arg2_type = LtacArg::Mem(v.pos);
                    
                    // Unsigned bytes
                    } else if v.data_type == DataType::UByte {
                        mov = ltac::create_instr(LtacType::MovUB);
                        mov.arg1_type = LtacArg::Reg8(1);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        cmp.arg2_type = LtacArg::Reg8(1);
                        
                    // Shorts
                    } else if v.data_type == DataType::Short {
                        if arg1.arg_type == AstArgType::ShortL {
                            builder.file.code.pop();
                            mov = ltac::create_instr(LtacType::MovW);
                            mov.arg1_type = LtacArg::Reg16(0);
                            mov.arg2_type = LtacArg::I16(arg1.u16_val as i16);
                        
                            cmp = ltac::create_instr(LtacType::I16Cmp);
                            cmp.arg1_type = LtacArg::Reg16(0);
                        } else {
                            mov.arg1_type = LtacArg::Empty;
                        }
                        
                        cmp.arg2_type = LtacArg::Mem(v.pos);
                    
                    // Unsigned short
                    } else if v.data_type == DataType::UShort {
                        mov = ltac::create_instr(LtacType::MovUW);
                        mov.arg1_type = LtacArg::Reg16(1);
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        cmp.arg2_type = LtacArg::Reg16(1);
                        
                    // Other things
                    } else {
                        mov.arg2_type = LtacArg::Mem(v.pos);
                        
                        cmp.arg2_type = LtacArg::Reg32(1);
                    }
                    
                    if mov.arg1_type != LtacArg::Empty {
                        builder.file.code.push(mov);
                    }
                },
                
                None => mov.arg2_val = 0,
            }
        },
        
        _ => {},
    }
    
    // Add the instruction
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
            cmp.arg1_type = LtacArg::I32(arg1.i32_val);
        },
        
        AstArgType::StringL => {},
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg32(0);
            
            match &builder.vars.get(&arg1.str_val) {
                Some(v) => mov.arg2_type = LtacArg::Mem(v.pos),
                None => {/* TODO: Syntax error */},
            }
            
            cmp_block.push(mov);
            
            cmp.arg1_type = LtacArg::Reg32(0);
        },
        
        _ => {},
    }
    
    match &arg2.arg_type {
        AstArgType::IntL => {
            cmp.arg2_type = LtacArg::I32(arg2.i32_val);
        },
        
        AstArgType::StringL => {},
        
        AstArgType::Id => {
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::Reg32(1);
            
            match &builder.vars.get(&arg2.str_val) {
                Some(v) => mov.arg2_type = LtacArg::Mem(v.pos),
                None => {/* TODO: Syntax error */},
            }
            
            cmp_block.push(mov);
            
            cmp.arg2_type = LtacArg::Reg32(1);
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


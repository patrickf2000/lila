//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use crate::ltac_builder::*;
use crate::ltac_utils::*;

use crate::ast::{DataType, AstStmt, AstArgType};
use crate::ltac;
use crate::ltac::{LtacType, LtacInstr, LtacArg};

// Builds a for loop block
pub fn build_for_loop(builder : &mut LtacBuilder, line : &AstStmt) {
    builder.block_layer += 1;
    builder.loop_layer += 1;
    
    create_label2(builder, false);    // Goes at the very end
    create_label2(builder, false);    // Add a comparison label
    create_label2(builder, false);   // Add a loop label
    
    if line.args.len() == 4 {
        build_range_for_loop(builder, line);
    } else {
        build_foreach_loop(builder, line);
    }
}

// Builds a range-based for loop
fn build_range_for_loop(builder : &mut LtacBuilder, line : &AstStmt)  {
    let end_label = builder.label_stack.pop().unwrap();
    let loop_label = builder.label_stack.pop().unwrap();
    let cmp_label = builder.label_stack.pop().unwrap();
    
    builder.loop_labels.insert(builder.block_layer, cmp_label.clone());
    builder.end_labels.insert(builder.block_layer, end_label.clone());
    
    // Create the variable
    let index_var = line.args.first().unwrap();
    let name = index_var.str_val.clone();
    
    builder.stack_pos += 4;
    let pos = builder.stack_pos;
    
    let index = Var {
        pos : pos,
        data_type : DataType::Int,
        sub_type : DataType::None,
        is_param : false,
    };
    
    builder.vars.insert(name, index);
    
    // Determine the type of loop
    let start_pos = line.args.iter().nth(1).unwrap();
    let end_arg = line.args.iter().nth(3).unwrap();
    
    // Set the variable equal to the start
    // TODO: Other types
    match start_pos.arg_type {
        AstArgType::IntL => {
            let mut instr = ltac::create_instr(LtacType::Mov);
            instr.arg1 = LtacArg::Mem(pos);
            instr.arg2 = LtacArg::I32(start_pos.u64_val as i32);
            builder.file.code.push(instr);
        },
        
        _ => {},
    }
    
    // Start the loop
    let mut lbl = ltac::create_instr(LtacType::Label);
    lbl.name = loop_label.clone();
    builder.file.code.push(lbl);
    
    // Now build the comparison
    // We create a separate block since this will go at the end of the loop
    let mut cmp_block : Vec<LtacInstr> = Vec::new();
    
    // Increment the counter variable
    let mut lbl2 = ltac::create_instr(LtacType::Label);
    lbl2.name = cmp_label.clone();
    cmp_block.push(lbl2);
    
    let mut mov2 = ltac::create_instr(LtacType::Mov);
    mov2.arg1 = LtacArg::Reg32(0);
    mov2.arg2 = LtacArg::Mem(pos);
    cmp_block.push(mov2);
    
    let mut inc_counter = ltac::create_instr(LtacType::I32Add);
    inc_counter.arg1 = LtacArg::Reg32(0);
    inc_counter.arg2 = LtacArg::I32(1);
    cmp_block.push(inc_counter);
    
    let mut mov3 = ltac::create_instr(LtacType::Mov);
    mov3.arg1 = LtacArg::Mem(pos);
    mov3.arg2 = LtacArg::Reg32(0);
    cmp_block.push(mov3);
    
    // Build the conditional statement
    // TODO: Other types
    let mut cmp_instr = ltac::create_instr(LtacType::I32Cmp);
    cmp_instr.arg1 = LtacArg::Reg32(0);
    
    match end_arg.arg_type {
        AstArgType::IntL => cmp_instr.arg2 = LtacArg::I32(end_arg.u64_val as i32),
        
        AstArgType::Id => {
            let v = match builder.get_var(&end_arg.str_val) {
                Ok(v) => v,
                Err(_e) => {
                    // TODO: Syntax error
                    return;
                },
            };
            
            cmp_instr.arg2 = LtacArg::Mem(v.pos);
        },
        
        _ => {},
    }
    
    cmp_block.push(cmp_instr);
    
    // Now the operator
    let mut br = ltac::create_instr(LtacType::Bl);
    br.name = loop_label.clone();
    cmp_block.push(br);
    
    // The end label
    let mut end_lbl = ltac::create_instr(LtacType::Label);
    end_lbl.name = end_label.clone();
    cmp_block.push(end_lbl);
    
    builder.code_stack.push(cmp_block);
}

// Builds a foreach loop
// Overall logic behind a for each loop
// Two extra variables need
//      -> Index is the user-specified one to hold the current element
//      -> Pos is an internal one used to check the current index against the loop size
//
// mov pos, 0
// jmp CMP
// LOOP
// mov index, array[pos]
// ~~~~
// ~~~~
// add pos, 1
// CMP
// cmp pos, array_size
// jl LOOP
//
fn build_foreach_loop(builder : &mut LtacBuilder, line : &AstStmt)  {
    let end_label = builder.label_stack.pop().unwrap();
    let loop_label = builder.label_stack.pop().unwrap();
    let cmp_label = builder.label_stack.pop().unwrap();
    
    builder.loop_labels.insert(builder.block_layer, cmp_label.clone());
    builder.end_labels.insert(builder.block_layer, end_label.clone());
    
    // First, build the index variable
    let index_var = line.args.first().unwrap();
    let array_var = line.args.last().unwrap();
    
    let index_name = index_var.str_val.clone();     // The name of the user's index variable
    let array_name = array_var.str_val.clone();     // The name of the array we are searching
    
    let array = match builder.get_var(&array_name) {
        Ok(v) => v,
        Err(_e) => {
            //TODO: Syntax error
            return;
        },
    };
    
    let data_type = array.sub_type.clone();
    let data_type_size : i32 = size_for_type(&data_type);
    
    let array_pos = array.pos;
    let array_size_pos = array_pos - 8;
    
    builder.stack_pos += 4 + data_type_size;
    let index_pos = builder.stack_pos;
    
    let index = Var {
        pos : index_pos,
        data_type : data_type.clone(),
        sub_type : DataType::None,
        is_param : false,
    };
    
    builder.vars.insert(index_name, index);
    
    // Build another index variable to keep track of the size
    builder.stack_pos += 4;
    let size_pos = builder.stack_pos;
    
    let mut instr = ltac::create_instr(LtacType::Mov);
    instr.arg1 = LtacArg::Mem(size_pos);
    instr.arg2 = LtacArg::I32(0);
    builder.file.code.push(instr.clone());
    
    instr = ltac::create_instr(LtacType::Br);
    instr.name = cmp_label.clone();
    builder.file.code.push(instr.clone());
    
    // Start the loop
    let mut lbl = ltac::create_instr(LtacType::Label);
    lbl.name = loop_label.clone();
    builder.file.code.push(lbl);
    
    ///////////////////////////////////////
    // Load the index variable
    // mov r0, array[size_pos]
    // mov index, r0
    //
    if data_type == DataType::Str {
        instr = ltac::create_instr(LtacType::MovQ);
        instr.arg1 = LtacArg::Reg64(0);
        instr.arg2 = LtacArg::MemOffsetMem(array_pos, size_pos, 8);
        builder.file.code.push(instr.clone());
        
        instr.arg1 = LtacArg::Mem(index_pos);
        instr.arg2 = LtacArg::Reg64(0);
        builder.file.code.push(instr.clone());
    } else {
        let reg = reg_for_type(&data_type, &DataType::None, 0);
        
        instr = mov_for_type(&data_type, &DataType::None);
        instr.arg1 = reg.clone();
        instr.arg2 = LtacArg::MemOffsetMem(array_pos, size_pos, data_type_size);
        builder.file.code.push(instr.clone());
        
        instr = mov_for_type(&data_type, &DataType::None);
        instr.arg1 = LtacArg::Mem(index_pos);
        instr.arg2 = reg.clone();
        builder.file.code.push(instr.clone());
    }
    
    ///////////////////////////////////////
    // Build the bottom of the loop block
    let mut cmp_block : Vec<LtacInstr> = Vec::new();
    
    // Increment the counter
    // mov r0, [size_pos]
    // add r0, 1
    // mov [size_pos], r0
    //
    instr = ltac::create_instr(LtacType::Mov);
    instr.arg1 = LtacArg::Reg32(0);
    instr.arg2 = LtacArg::Mem(size_pos);
    cmp_block.push(instr.clone());
    
    instr = ltac::create_instr(LtacType::I32Add);
    instr.arg1 = LtacArg::Reg32(0);
    instr.arg2 = LtacArg::I32(1);
    cmp_block.push(instr.clone());
    
    instr = ltac::create_instr(LtacType::Mov);
    instr.arg1 = LtacArg::Mem(size_pos);
    instr.arg2 = LtacArg::Reg32(0);
    cmp_block.push(instr.clone());
    
    // Comparison label
    let mut lbl2 = ltac::create_instr(LtacType::Label);
    lbl2.name = cmp_label.clone();
    cmp_block.push(lbl2);
    
    // Load the counter variable and the array size variables and compare    
    // mov r0, [size_pos]
    // cmp r0, [array_size_pos]
    //
    instr = ltac::create_instr(LtacType::Mov);
    instr.arg1 = LtacArg::Reg32(0);
    instr.arg2 = LtacArg::Mem(size_pos);
    cmp_block.push(instr.clone());
    
    let mut cmp_instr = ltac::create_instr(LtacType::I32Cmp);
    cmp_instr.arg1 = LtacArg::Reg32(0);
    cmp_instr.arg2 = LtacArg::Mem(array_size_pos);
    cmp_block.push(cmp_instr);
    
    // Now the branch instruction
    let mut br = ltac::create_instr(LtacType::Bl);
    br.name = loop_label.clone();
    cmp_block.push(br);
    
    // The end label
    let mut end_lbl = ltac::create_instr(LtacType::Label);
    end_lbl.name = end_label.clone();
    cmp_block.push(end_lbl);
    
    builder.code_stack.push(cmp_block);
}


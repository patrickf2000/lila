
// This file is part of the Lila compiler
// Copyright (C) 2020-2021 Patrick Flynn
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use crate::ltac_builder::*;
use crate::ltac;
use crate::ast::*;
use crate::ltac::*;

// A utility function to create a label
pub fn create_label(builder : &mut LtacBuilder, is_top : bool) {
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

// Returns the size for a given type
pub fn size_for_type(data_type : &DataType) -> i32 {
    match data_type {
        DataType::Byte | DataType::UByte => 1,
        DataType::Char => 1,
        DataType::Short | DataType::UShort => 2,
        DataType::Int | DataType::UInt => 4,
        DataType::Int64 | DataType::UInt64 => 8,
        DataType::Str | DataType::Ptr => 8,
        DataType::Float => 4,
        DataType::Double => 8,
        _ => 0,
    }
}

// Return: Base Type, Sub Type
pub fn ast_to_datatype(ast_mod : &AstMod) -> (DataType, DataType) {
    match &ast_mod.mod_type {
        AstModType::Byte => return (DataType::Byte, DataType::None),
        AstModType::UByte => return (DataType::UByte, DataType::None),
        AstModType::ByteDynArray => return (DataType::Ptr, DataType::Byte),
        AstModType::UByteDynArray => return (DataType::Ptr, DataType::UByte),
        
        AstModType::Short => return (DataType::Short, DataType::None),
        AstModType::UShort => return (DataType::UShort, DataType::None),
        AstModType::ShortDynArray => return (DataType::Ptr, DataType::Short),
        AstModType::UShortDynArray => return (DataType::Ptr, DataType::UShort),
        
        AstModType::Int => return (DataType::Int, DataType::None),
        AstModType::UInt => return (DataType::UInt, DataType::None),
        AstModType::IntDynArray => return (DataType::Ptr, DataType::Int),
        AstModType::UIntDynArray => return (DataType::Ptr, DataType::UInt),
        
        AstModType::Int64 => return (DataType::Int64, DataType::None),
        AstModType::UInt64 => return (DataType::UInt64, DataType::None),
        AstModType::I64DynArray => return (DataType::Ptr, DataType::Int64),
        AstModType::U64DynArray => return (DataType::Ptr, DataType::UInt64),
        
        AstModType::Float => return (DataType::Float, DataType::None),
        AstModType::Double => return (DataType::Double, DataType::None),
        AstModType::FloatDynArray => return (DataType::Ptr, DataType::Float),
        AstModType::DoubleDynArray => return (DataType::Ptr, DataType::Double),
        
        AstModType::Char => return (DataType::Char, DataType::None),
        AstModType::Str => return (DataType::Str, DataType::None),
        AstModType::StrDynArray => return (DataType::Ptr, DataType::Str),
        AstModType::Enum(_v) => return (DataType::Int,  DataType::None),       // TODO: We will need better type detection
        
        // Do we need an error here? Really, it should never get to this pointer
        AstModType::None => return (DataType::Void, DataType::None),
    }
}

// Returns a move statement for a given type
pub fn mov_for_type(data_type : &DataType, sub_type : &DataType) -> LtacInstr {
    let mut instr = ltac::create_instr(LtacType::Mov);
    
    match data_type {
        // Bytes
        DataType::Byte => instr = ltac::create_instr(LtacType::MovB),
        DataType::UByte => instr = ltac::create_instr(LtacType::MovUB),
        
        DataType::Ptr if *sub_type == DataType::Byte => instr = ltac::create_instr(LtacType::MovB),
        DataType::Ptr if *sub_type == DataType::UByte => instr = ltac::create_instr(LtacType::MovUB),
        
        // Short
        DataType::Short => instr = ltac::create_instr(LtacType::MovW),
        DataType::UShort => instr = ltac::create_instr(LtacType::MovUW),
        
        DataType::Ptr if *sub_type == DataType::Short => instr = ltac::create_instr(LtacType::MovW),
        DataType::Ptr if *sub_type == DataType::UShort => instr = ltac::create_instr(LtacType::MovUW),
        
        // Int
        DataType::Int => instr = ltac::create_instr(LtacType::Mov),
        DataType::UInt => instr = ltac::create_instr(LtacType::MovU),
        
        DataType::Ptr if *sub_type == DataType::Int => instr = ltac::create_instr(LtacType::Mov),
        DataType::Ptr if *sub_type == DataType::UInt => instr = ltac::create_instr(LtacType::MovU),
        
        // Int64
        DataType::Int64 => instr = ltac::create_instr(LtacType::MovQ),
        DataType::UInt64 => instr = ltac::create_instr(LtacType::MovUQ),
        
        DataType::Ptr if *sub_type == DataType::Int64 => instr = ltac::create_instr(LtacType::MovQ),
        DataType::Ptr if *sub_type == DataType::UInt64 => instr = ltac::create_instr(LtacType::MovUQ),
        
        // Double
        DataType::Float => instr = ltac::create_instr(LtacType::MovF32),
        DataType::Double => instr = ltac::create_instr(LtacType::MovF64),
        
        DataType::Ptr if *sub_type == DataType::Float => instr = ltac::create_instr(LtacType::MovF32),
        DataType::Ptr if *sub_type == DataType::Double => instr = ltac::create_instr(LtacType::MovF64),
        
        // String
        DataType::Char | DataType::Str => instr = ltac::create_instr(LtacType::MovB),
        
        DataType::Ptr if *sub_type == DataType::Str => instr = ltac::create_instr(LtacType::MovQ),
        
        _ => {},
    }
    
    instr
}

// Returns a register for a given type
pub fn reg_for_type(data_type : &DataType, sub_type : &DataType, reg_no : i32) -> LtacArg {
    let mut arg = LtacArg::Reg32(reg_no);
    
    match data_type {
        // Byte
        DataType::Byte => arg = LtacArg::Reg8(reg_no),
        DataType::UByte => arg = LtacArg::Reg8(reg_no),
        
        DataType::Ptr
        if *sub_type == DataType::Byte || *sub_type == DataType::UByte => arg = LtacArg::Reg8(reg_no),
        
        // Short
        DataType::Short => arg = LtacArg::Reg16(reg_no),
        DataType::UShort => arg = LtacArg::Reg16(reg_no),
        
        DataType::Ptr
        if *sub_type == DataType::Short || *sub_type == DataType::UShort => arg = LtacArg::Reg16(reg_no),
        
        // Int
        DataType::Int => arg = LtacArg::Reg32(reg_no),
        DataType::UInt => arg = LtacArg::Reg32(reg_no),
        
        DataType::Ptr
        if *sub_type == DataType::Int || *sub_type == DataType::UInt => arg = LtacArg::Reg32(reg_no),
        
        // Int-64
        DataType::Int64 => arg = LtacArg::Reg64(reg_no),
        DataType::UInt64 => arg = LtacArg::Reg64(reg_no),
        
        DataType::Ptr
        if *sub_type == DataType::Int64 || *sub_type == DataType::UInt64 => arg = LtacArg::Reg64(reg_no),
        
        // Float
        DataType::Float => arg = LtacArg::FltReg(reg_no),
        DataType::Double => arg = LtacArg::FltReg64(reg_no),
        
        DataType::Ptr if *sub_type == DataType::Float => arg = LtacArg::FltReg(reg_no),
        DataType::Ptr if *sub_type == DataType::Double => arg = LtacArg::FltReg64(reg_no),
        
        // String
        DataType::Char | DataType::Str => arg = LtacArg::Reg8(reg_no),
        
        DataType::Ptr
        if *sub_type == DataType::Str => arg = LtacArg::Reg64(reg_no),
        
        _ => {},
    }
    
    arg
}

// Returns a ldarg statement for a given type
pub fn ldarg_for_type(data_type : &DataType, dest : LtacArg, pos : i32) -> LtacInstr {
    let mut arg = ltac::create_instr(LtacType::None);
    
    match data_type {
        DataType::Byte => arg = ltac::create_instr(LtacType::LdArgI8),
        DataType::UByte => arg = ltac::create_instr(LtacType::LdArgU8),
        
        DataType::Short => arg = ltac::create_instr(LtacType::LdArgI16),
        DataType::UShort => arg = ltac::create_instr(LtacType::LdArgU16),
        
        DataType::Int => arg = ltac::create_instr(LtacType::LdArgI32),
        DataType::UInt => arg = ltac::create_instr(LtacType::LdArgU32),
        
        DataType::Int64 => arg = ltac::create_instr(LtacType::LdArgI64),
        DataType::UInt64 => arg = ltac::create_instr(LtacType::LdArgU64),
        
        DataType::Float => arg = ltac::create_instr(LtacType::LdArgF32),
        DataType::Double => arg = ltac::create_instr(LtacType::LdArgF64),
        
        DataType::Ptr | DataType::Str => arg = ltac::create_instr(LtacType::LdArgPtr),
        
        _ => return arg,
    }
    
    arg.arg1 = dest;
    arg.arg2_val = pos;
    
    arg
}


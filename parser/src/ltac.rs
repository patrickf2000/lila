//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

// Represents a data entry type
#[derive(Clone, PartialEq)]
pub enum LtacDataType {
    StringL,
    FloatL,
    DoubleL,
}

// Represents an instruction type
#[derive(Debug, Clone, PartialEq)]
pub enum LtacType {
    None,
    
    Extern,
    Label,
    Func,
    Ret,
    
    MovB,       MovUB,      // Move byte (byte)
    MovW,       MovUW,      // Move word (short)
    Mov,        MovU,       // Move double word (int)
    MovQ,       MovUQ,      // Move qword (int64)
    MovF32,
    MovF64,
    MovI32Vec,
    
    LdAddr,
    
    // Push/pop
    // These are mainly used on stack machines, but x86 has these instructions
    Push,
    Pop,
    
    // Load-store instructions- RISC specific
    LdB,        LdUB,
    LdW,        LdUW,
    Ld,         LdU,
    LdQ,        LdUQ,
    LdF32,
    LdF64,
    
    StrB,       StrUB,
    StrW,       StrUW,
    Str,        StrU,
    StrQ,       StrUQ,
    StrF32,
    StrF64,
    StrPtr,
    
    // Argument load instructions
    LdArgI8,    LdArgU8,
    LdArgI16,   LdArgU16,
    LdArgI32,   LdArgU32,
    LdArgI64,   LdArgU64,
    LdArgF32,
    LdArgF64,
    LdArgPtr,
    
    PushArg,
    KPushArg,
    Call,
    Syscall,
    
    Malloc,
    Free,
    Exit,
    
    // Comparison and flow instructions
    I8Cmp,      U8Cmp,
    I16Cmp,     U16Cmp,
    I32Cmp,     U32Cmp,
    I64Cmp,     U64Cmp,
    F32Cmp,
    F64Cmp,
    StrCmp,
    
    Br,
    Be,
    Bne,
    Bl,
    Ble,
    Bfl,        // Jump if float is less
    Bfle,       // Jump if float is less or equal
    Bg,
    Bge,
    Bfg,        // Jump if float is greater
    Bfge,       // Jump if float is greater or equal
    
    // Math operations
    I8Add,      U8Add,
    I8Sub,
    I8Mul,      U8Mul,
    I8Div,      U8Div,
    I8Mod,      U8Mod,
    
    I16Add,     U16Add,
    I16Sub,
    I16Mul,     U16Mul,
    I16Div,     U16Div,
    I16Mod,     U16Mod,
    
    I32Add,     U32Add,
    I32Sub,
    I32Mul,     U32Mul,
    I32Div,     U32Div,
    I32Mod,     U32Mod,
    
    I64Add,     U64Add,
    I64Sub,
    I64Mul,     U64Mul,
    I64Div,     U64Div,
    I64Mod,     U64Mod,
    
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    
    // Bitwise instructions
    // I don't think these need to be typed, though
    // if I'm wrong we'll have to change this.
    And,
    Or,
    Xor,
    Lsh,
    Rsh,
    
    // Vector instructions
    I32VAdd,
    
    // Type conversion instructions
    // These aren't used by default, but some of the transform layers might
    CvtF32F64,
    MovF64Int,    // Move float-64 register to int register
}

// Represents an instruction argument type
#[derive(Debug, Clone, PartialEq)]
pub enum LtacArg {
    Empty,
    
    Reg8(i32),
    Reg16(i32),
    Reg32(i32),
    Reg64(i32),
    FltReg(i32),
    FltReg64(i32),
    
    // 10/30/2020
    // The reason for separate types is because on some architectures, you have to
    // have a specific instruction, even if the registers are the same
    //
    // For a justifiable example, see the main build_instr function in the x86 layer
    RetRegI8,       RetRegU8,
    RetRegI16,      RetRegU16,
    RetRegI32,      RetRegU32,
    RetRegI64,      RetRegU64,
    RetRegF32,
    RetRegF64,
    
    Mem(i32),
    MemOffsetImm(i32, i32),
    MemOffsetMem(i32, i32, i32),    // Dest, var, size
    MemOffsetReg(i32, i32, i32),    // Dest, reg _no, size
    
    Byte(i8),       UByte(u8),
    I16(i16),       U16(u16),
    I32(i32),       U32(u32),
    I64(i64),       U64(u64),
    F32(String),
    F64(String),
    
    Ptr(i32),
    PtrLcl(String)
}

// Represents an LTAC file
#[derive(Clone)]
pub struct LtacFile {
    pub name : String,
    pub data : Vec<LtacData>,
    pub code : Vec<LtacInstr>,
}

// Represents data for the ELF .data entry
#[derive(Clone)]
pub struct LtacData {
    pub data_type : LtacDataType,
    pub name : String,
    pub val : String,
}

// Represents an instruction
#[derive(Clone)]
pub struct LtacInstr {
    pub instr_type : LtacType,
    pub name : String,
    
    pub arg1 : LtacArg,
    pub arg1_val : i32,
    
    pub arg2 : LtacArg,
    pub arg2_val : i32,
}

//=====================================
// Creates an LTAC instruction

pub fn create_instr(instr_type : LtacType) -> LtacInstr {
    LtacInstr {
        instr_type : instr_type,
        name : String::new(),
        
        arg1 : LtacArg::Empty,
        arg1_val : 0,
        
        arg2 : LtacArg::Empty,
        arg2_val : 0,
    }
}



// Represents a data entry type
#[derive(Clone, PartialEq)]
pub enum LtacDataType {
    StringL,
    FloatL,
    DoubleL,
}

// Represents an instruction type
#[derive(Clone, PartialEq)]
pub enum LtacType {
    Extern,
    Label,
    Func,
    Ret,
    
    MovB,       MovUB,
    MovW,       MovUW,
    Mov,        MovU,
    MovF32,
    MovF64,
    MovOffImm,
    MovOffMem,
    MovI32Vec,
    
    Ld,
    LdB,
    LdUB,
    LdW,
    Str,
    StrB,
    StrUB,
    StrW,
    StrPtr,
    
    LdArgI8,    LdArgU8,
    LdArgI16,   LdArgU16,
    LdArgI32,
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
    I32Cmp,
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
    BAdd,
    BSub,
    BMul,
    BDiv,
    BMod,
    
    U8Add,
    U8Mul,
    U8Div,
    U8Mod,
    
    I16Add,
    I16Sub,
    I16Mul,
    I16Div,
    I16Mod,
    
    U16Add,
    U16Mul,
    U16Div,
    U16Mod,
    
    I32Add,
    I32Sub,
    I32Mul,
    I32Div,
    I32Mod,
    
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    
    // Bitwise instructions
    // I don't think these need to be by signed/unsigned, but
    // if I'm wrong we'll have to change this.
    BAnd,
    BOr,
    BXor,
    BLsh,
    BRsh,
    
    WAnd,
    WOr,
    WXor,
    WLsh,
    WRsh,
    
    I32And,
    I32Or,
    I32Xor,
    I32Lsh,
    I32Rsh,
    
    // Vector instructions
    I32VAdd,
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
    
    RetRegI8,       RetRegU8,
    RetRegI16,      RetRegU16,
    RetRegI32,
    RetRegI64,
    RetRegF32,
    RetRegF64,
    
    Mem(i32),
    
    Byte(i8),       UByte(u8),
    I16(i16),       U16(u16),
    I32(i32),       U32(u32),
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
    
    pub arg1_type : LtacArg,
    pub arg1_val : i32,
    pub arg1_offset : i32,
    pub arg1_offset_size : i32,
    
    pub arg2_type : LtacArg,
    pub arg2_val : i32,
    pub arg2_offset : i32,
    pub arg2_offset_size : i32,
}

//=====================================
// Creates an LTAC instruction

pub fn create_instr(instr_type : LtacType) -> LtacInstr {
    LtacInstr {
        instr_type : instr_type,
        name : String::new(),
        
        arg1_type : LtacArg::Empty,
        arg1_val : 0,
        arg1_offset : 0,
        arg1_offset_size : 0,
        
        arg2_type : LtacArg::Empty,
        arg2_val : 0,
        arg2_offset : 0,
        arg2_offset_size : 0,
    }
}


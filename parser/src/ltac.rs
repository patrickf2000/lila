
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
    
    Mov,
    MovB,
    MovF32,
    MovF64,
    MovOffImm,
    MovOffMem,
    MovI32Vec,
    
    Ld,
    Str,
    StrB,
    StrPtr,
    
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
    
    I32And,
    I32Or,
    I32Xor,
    I32Lsh,
    I32Rsh,
    
    I32VAdd,
}

// Represents an instruction argument type
#[derive(Clone, PartialEq)]
pub enum LtacArg {
    Empty,
    Reg,
    Reg8,
    Reg64,
    FltReg,
    FltReg64,
    RetRegI32,
    RetRegI64,
    RetRegF32,
    RetRegF64,
    Mem,
    Byte,
    I32,
    F32,
    F64,
    Ptr,
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
    pub arg1_bval : u8,
    pub arg1_sval : String,
    pub arg1_offset : i32,
    pub arg1_offset_size : i32,
    
    pub arg2_type : LtacArg,
    pub arg2_val : i32,
    pub arg2_bval : u8,
    pub arg2_sval : String,
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
        arg1_bval : 0,
        arg1_sval : String::new(),
        arg1_offset : 0,
        arg1_offset_size : 0,
        
        arg2_type : LtacArg::Empty,
        arg2_val : 0,
        arg2_bval : 0,
        arg2_sval : String::new(),
        arg2_offset : 0,
        arg2_offset_size : 0,
    }
}

//=====================================
// The various debug areas

// LtacFile
impl LtacFile {
    pub fn print(&self) {
        println!("File: {}", self.name);
        
        println!("");
        println!(".data");
        
        for data in self.data.iter() {
            data.print();
        }
        
        println!("");
        println!(".code");
        
        for code in self.code.iter() {
            code.print();
        }
    }
}

// LtacData
impl LtacData {
    pub fn print(&self) {
        print!("    ");
        
        match &self.data_type {
            LtacDataType::StringL => println!("{} .string \"{}\"", self.name, self.val),
            LtacDataType::FloatL => println!("{} .float {}", self.name, self.val),
            LtacDataType::DoubleL => println!("{} .double {}", self.name, self.val),
        }
    }
}

// LtacCode
impl LtacInstr {
    pub fn print(&self) { 
        match &self.instr_type {
            LtacType::Extern => {
                println!("extern {}", self.name);
                return;
            }
            
            LtacType::Label => {
                println!("lbl {}", self.name);
                return;
            }
            
            LtacType::Func => {
                println!("func {}", self.name);
                println!("  setup {}", self.arg1_val);
                println!("");
                return;
            }
            
            LtacType::Ret => {
                println!("  leave");
                println!("  ret");
                println!("");
                return;
            },
            
            LtacType::LdArgI32 => {
                println!("  i32.ldarg [bp-{}], r{}", self.arg1_val, self.arg2_val);
                println!("");
                return;
            },
            
            LtacType::LdArgF32 => {
                println!("  f32.ldarg [bp-{}], fr{}", self.arg1_val, self.arg2_val);
                println!("");
                return;
            },
            
            LtacType::LdArgF64 => {
                println!("  f64.ldarg [bp-{}], dr{}", self.arg1_val, self.arg2_val);
                println!("");
                return;
            },
            
            LtacType::LdArgPtr => {
                println!("  ptr.ldarg [bp-{}], r{}", self.arg1_val, self.arg2_val);
                println!("");
                return;
            },
            
            LtacType::Mov => print!("  mov "),
            LtacType::MovB => print!("  mov.b "),
            LtacType::MovF32 => print!("  mov.f32 "),
            LtacType::MovF64 => print!("  mov.f64 "),
            LtacType::MovOffImm => print!("  mov.imm "),
            LtacType::MovOffMem => print!("  mov.mem "),
            LtacType::MovI32Vec => print!("  mov.i32.vec "),
            
            LtacType::Ld => print!("  ld "),
            LtacType::Str => print!("  str "),
            LtacType::StrB => print!("  str.b "),
            LtacType::StrPtr => print!("  str.ptr "),
            
            LtacType::PushArg => print!("  pusharg "),
            LtacType::KPushArg => print!("  kpusharg "),
            
            LtacType::Call => {
                println!("  call {}", self.name);
                println!("");
                return;
            },
            
            LtacType::Syscall => {
                println!("  syscall");
                println!("");
                return;
            },
            
            LtacType::Malloc => {
                println!("  malloc");
                println!("");
                return;
            },
            
            LtacType::Free => {
                println!("  free");
                println!("");
            },
            
            LtacType::Exit => print!("  exit "),
            
            LtacType::I32Cmp => print!("  i32.cmp "),
            LtacType::F32Cmp => print!("  f32.cmp "),
            LtacType::F64Cmp => print!("  f64.cmp "),
            LtacType::StrCmp => {
                println!("  str.cmp");
                return;
            },
            
            LtacType::Br => println!("  br {}\n", self.name),
            LtacType::Be => println!("  be {}\n", self.name),
            LtacType::Bne => println!("  bne {}\n", self.name),
            LtacType::Bl => println!("  bl {}\n", self.name),
            LtacType::Ble => println!("  ble {}\n", self.name),
            LtacType::Bfl => println!("  bfl {}\n", self.name),
            LtacType::Bfle => println!("  bfle {}\n", self.name),
            LtacType::Bg => println!("  bg {}\n", self.name),
            LtacType::Bge => println!("  bge {}\n", self.name),
            LtacType::Bfg => println!("  bfg {}\n", self.name),
            LtacType::Bfge => println!("  bfge {}\n", self.name),
            
            LtacType::I32Add => print!("  i32.add "),
            LtacType::I32Sub => print!("  i32.sub "),
            LtacType::I32Mul => print!("  i32.mul "),
            LtacType::I32Div => print!("  i32.div "),
            LtacType::I32Mod => print!("  i32.mod "),
            
            LtacType::F32Add => print!("  f32.add "),
            LtacType::F32Sub => print!("  f32.sub "),
            LtacType::F32Mul => print!("  f32.mul "),
            LtacType::F32Div => print!("  f32.div "),
            
            LtacType::F64Add => print!("  f64.add "),
            LtacType::F64Sub => print!("  f64.sub "),
            LtacType::F64Mul => print!("  f64.mul "),
            LtacType::F64Div => print!("  f64.div "),
            
            LtacType::I32And => print!("  i32.and "),
            LtacType::I32Or => print!("  i32.or "),
            LtacType::I32Xor => print!("  i32.xor "),
            LtacType::I32Lsh => print!("  i32.lsh "),
            LtacType::I32Rsh => print!("  i32.rsh "),
            
            LtacType::I32VAdd => print!("  i32.vadd "),
        }
        
        match &self.arg1_type {
            LtacArg::Empty => print!(" "),
            
            LtacArg::Reg => print!("r{}", self.arg1_val),
            LtacArg::Reg8 => print!("rl{}", self.arg1_val),
            LtacArg::Reg64 => print!("xr{}", self.arg1_val),
            LtacArg::FltReg => print!("fr{}", self.arg1_val),
            LtacArg::FltReg64 => print!("dr{}", self.arg1_val),
            
            LtacArg::RetRegI32 => print!("i32.ret"),
            LtacArg::RetRegI64 => print!("i64.ret"),
            LtacArg::RetRegF32 => print!("f32.ret"),
            LtacArg::RetRegF64 => print!("f64.ret"),
            
            LtacArg::Mem => {
                if self.arg1_offset > 0 && self.arg1_offset_size > 0 {
                    print!("[bp-{}+({}*{})]", self.arg1_val, self.arg1_offset, self.arg1_offset_size);
                } else if self.arg1_offset > 0 {
                    print!("[bp-{}+{}]", self.arg1_val, self.arg1_offset);
                } else {
                    print!("[bp-{}]", self.arg1_val);
                }
            },
            
            LtacArg::Byte => print!("{}", self.arg1_bval),
            LtacArg::I32 => print!("{}", self.arg1_val),
            LtacArg::F32 => print!("{}", self.arg1_sval),
            LtacArg::F64 => print!("{}", self.arg1_sval),
            
            LtacArg::Ptr => {
                if self.arg1_sval.len() > 0 {
                    print!("{}", self.arg1_sval);
                } else {
                    print!("[bp-{}]", self.arg1_val);
                }
            },
        }
        
        match &self.arg2_type {
            LtacArg::Empty => println!(""),
            
            LtacArg::Reg => println!(", r{}", self.arg2_val),
            LtacArg::Reg8 => println!(", rl{}", self.arg2_val),
            LtacArg::Reg64 => println!(", xr{}", self.arg2_val),
            LtacArg::FltReg => println!(", fr{}", self.arg2_val),
            LtacArg::FltReg64 => println!(", dr{}", self.arg2_val),
            
            LtacArg::RetRegI32 => println!(", i32.ret"),
            LtacArg::RetRegI64 => println!(", i64.ret"),
            LtacArg::RetRegF32 => println!(", f32.ret"),
            LtacArg::RetRegF64 => println!(", f64.ret"),
            
            LtacArg::Mem => {
                if self.arg2_offset > 0 && self.arg2_offset_size > 0 {
                    println!(", [bp-{}+({}*{})]", self.arg2_val, self.arg2_offset, self.arg2_offset_size);
                } else if self.arg2_offset > 0 {
                    println!(", [bp-{}+{}]", self.arg2_val, self.arg2_offset);
                } else {
                    println!(", [bp-{}]", self.arg2_val);
                }
            },
            
            LtacArg::Byte => println!(", {}", self.arg2_bval),
            LtacArg::I32 => println!(", {}", self.arg2_val),
            LtacArg::F32 => println!(", {}", self.arg2_sval),
            LtacArg::F64 => println!(", {}", self.arg2_sval),
            LtacArg::Ptr => println!(", {}", self.arg2_sval),
        }
    }
}


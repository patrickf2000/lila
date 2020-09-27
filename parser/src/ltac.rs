
// Represents a data entry type
#[derive(Clone, PartialEq)]
pub enum LtacDataType {
    StringL,
}

// Represents an instruction type
#[derive(Clone, PartialEq)]
pub enum LtacType {
    Extern,
    Label,
    Func,
    Ret,
    Mov,
    
    PushArg,
    KPushArg,
    Call,
    Syscall,
    
    I32Cmp,
    Br,
    Be,
    Bne,
    
    I32Add,
    I32Mul,
}

// Represents an instruction argument type
#[derive(Clone, PartialEq)]
pub enum LtacArg {
    Empty,
    Reg,
    RetRegI32,
    Mem,
    I32,
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
    pub arg1_sval : String,
    
    pub arg2_type : LtacArg,
    pub arg2_val : i32,
    pub arg2_sval : String,
}

//=====================================
// Creates an LTAC instruction

pub fn create_instr(instr_type : LtacType) -> LtacInstr {
    LtacInstr {
        instr_type : instr_type,
        name : String::new(),
        
        arg1_type : LtacArg::Empty,
        arg1_val : 0,
        arg1_sval : String::new(),
        
        arg2_type : LtacArg::Empty,
        arg2_val : 0,
        arg2_sval : String::new(),
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
            }
            
            LtacType::Func => {
                println!("func {}", self.name);
                println!("  setup {}", self.arg1_val);
                println!("");
                return;
            }
            
            LtacType::Ret => {
                println!("  ret");
                println!("");
                return;
            },
            
            LtacType::Mov => print!("  mov "),
            
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
            
            LtacType::I32Cmp => print!("  i32.cmp "),
            
            LtacType::Br => println!("  br {}\n", self.name),
            LtacType::Be => println!("  be {}\n", self.name),
            LtacType::Bne => println!("  bne {}\n", self.name),
            
            LtacType::I32Add => print!("  i32.add "),
            LtacType::I32Mul => print!("  i32.mul "),
        }
        
        match &self.arg1_type {
            LtacArg::Empty => print!(" "),
            LtacArg::Reg => print!("r{}", self.arg1_val),
            LtacArg::RetRegI32 => print!("i32.ret"),
            LtacArg::Mem => print!("[bp-{}]", self.arg1_val),
            LtacArg::I32 => print!("{}", self.arg1_val),
            LtacArg::Ptr => print!("{}", self.arg1_sval),
        }
        
        match &self.arg2_type {
            LtacArg::Empty => println!(""),
            LtacArg::Reg => println!(", r{}", self.arg2_val),
            LtacArg::RetRegI32 => println!(", i32.ret"),
            LtacArg::Mem => println!(", [bp-{}]", self.arg2_val),
            LtacArg::I32 => println!(", {}", self.arg2_val),
            LtacArg::Ptr => println!(", {}", self.arg2_sval),
        }
    }
}


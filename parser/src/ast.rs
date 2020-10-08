
use crate::lex::*;

// Represents AST statement types
#[derive(PartialEq, Clone)]
pub enum AstStmtType {
    VarDec,
    VarAssign,
    ArrayAssign,
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,
    FuncCall,
    Return,
    End,
}

// Represents AST argument types
#[derive(PartialEq, Clone)]
pub enum AstArgType {
    IntL,
    StringL,
    Id,
    Array,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpEq,
    OpNeq,
    OpLt,
    OpLte,
    OpGt,
    OpGte,
    OpNot,
    OpAnd,
    OpOr,
    OpXor,
    OpLeftShift,
    OpRightShift,
}

// Represents modifiers
#[derive(PartialEq, Clone)]
pub enum AstModType {
    Int,
    IntDynArray,
    Str,
}

// Represents the top of an AST tree
pub struct AstTree {
    pub file_name : String,
    pub functions : Vec<AstFunc>,
}

// Represents a function in a tree
pub struct AstFunc {
    pub name : String,
    pub is_extern : bool,
    pub statements : Vec<AstStmt>,
    pub args : Vec<AstStmt>,
    pub modifiers : Vec<AstMod>,
}

// Represents a statement
#[derive(Clone)]
pub struct AstStmt {
    pub stmt_type : AstStmtType,
    pub name : String,
    
    pub sub_args : Vec<AstArg>,
    pub args : Vec<AstArg>,
    pub modifiers : Vec<AstMod>,
    
    pub line : String,
    pub line_no : i32,
}

// Represents an argument
// Arguments are constants, variables, operators, etc
#[derive(Clone)]
pub struct AstArg {
    pub arg_type : AstArgType,
    pub str_val : String,
    pub i32_val : i32,
    
    pub sub_args : Vec<AstArg>,
}

// Represents an statement modifier
#[derive(Clone)]
pub struct AstMod {
    pub mod_type : AstModType,
}

// Tree implementation
impl AstTree {
    pub fn print(&self) {
        println!("Tree: {}", self.file_name);
    
        for func in self.functions.iter() {
            func.print();
        }
    }
}

// Function implementation
impl AstFunc {
    pub fn print(&self) {
        print!("  ");
        if self.is_extern {
            print!("EXTERN ");
        }
        
        print!("FUNC {}", self.name);
        
        for m in self.modifiers.iter() {
            m.print(true);
        }
        
        println!("");
        
        for arg in self.args.iter() {
            arg.print(true);
        }
        
        for stmt in self.statements.iter() {
            stmt.print(false);
        }
    }
}

// Statement implementation
impl AstStmt {
    pub fn print(&self, is_arg : bool) {
        print!("    ");
        
        if is_arg {
            print!("FUNC_ARG ");
        }
        
        match &self.stmt_type {
            AstStmtType::VarDec => println!("VAR DEC {}", self.name),
            AstStmtType::VarAssign => println!("VAR ASSIGN {}", self.name),
            AstStmtType::ArrayAssign => println!("ARRAY ASSIGN {}", self.name),
            AstStmtType::If => println!("IF"),
            AstStmtType::Elif => println!("ELIF"),
            AstStmtType::Else => println!("ELSE"),
            AstStmtType::While => println!("WHILE"),
            AstStmtType::Break => println!("BREAK"),
            AstStmtType::Continue => println!("CONTINUE"),
            AstStmtType::FuncCall => println!("FUNC CALL {}", self.name),
            AstStmtType::Return => println!("RETURN"),
            AstStmtType::End => println!("END"),
        }
        
        for m in self.modifiers.iter() {
            m.print(false);
        }
        
        if self.args.len() > 0 {
            print!("        ARG ");
            
            for arg in self.args.iter() {
                arg.print();
            }
            
            println!("");
        }
        
        if self.sub_args.len() > 0 {
            print!("        SUB_ARG ");
            
            for arg in self.sub_args.iter() {
                arg.print();
            }
            
            println!("");
        }
    }
}

// Argument implementation
impl AstArg {
    pub fn print(&self) {
        match &self.arg_type {
            AstArgType::IntL => print!("{} ", self.i32_val),
            AstArgType::StringL => print!("\"{}\" ", self.str_val),
            AstArgType::Id => print!("{} ", self.str_val),
            AstArgType::Array => print!("ARRAY "),
            AstArgType::OpAdd => print!("+ "),
            AstArgType::OpSub => print!("- "),
            AstArgType::OpMul => print!("* "),
            AstArgType::OpDiv => print!("/ "),
            AstArgType::OpMod => print!("% "),
            AstArgType::OpEq => print!("== "),
            AstArgType::OpNeq => print!("!= "),
            AstArgType::OpLt => print!("< "),
            AstArgType::OpLte => print!("<= "),
            AstArgType::OpGt => print!("> "),
            AstArgType::OpGte => print!(">= "),
            AstArgType::OpNot => print!("! "),
            AstArgType::OpAnd => print!("& "),
            AstArgType::OpOr => print!("| "),
            AstArgType::OpXor => print!("^ "),
            AstArgType::OpLeftShift => print!("<< "),
            AstArgType::OpRightShift => print!(">> "),
        }
        
        if self.sub_args.len() > 0 {
            print!("(");
            for arg in self.sub_args.iter() {
                arg.print();
            }
            print!(") ");
        }
    }
}

// Modifier implementation
impl AstMod {
    pub fn print(&self, is_func : bool) {
        if is_func {
            print!(" (");
        } else {
            print!("        MOD ");
        }
        
        match &self.mod_type {
            AstModType::Int => print!("Int"),
            AstModType::IntDynArray => print!("IntDynArr"),
            AstModType::Str => print!("Str"),
        }
        
        if is_func {
            print!(")");
        } else {
            println!("");
        }
    }
}

// Helper functions
pub fn create_extern_func(name : String) -> AstFunc {
    AstFunc {
        name : name,
        is_extern : true,
        statements : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
    }
}

pub fn create_func(name : String) -> AstFunc {
    AstFunc {
        name : name,
        is_extern : false,
        statements : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
    }
}

pub fn create_stmt(stmt_type : AstStmtType, scanner : &mut Lex) -> AstStmt {
    AstStmt {
        stmt_type : stmt_type,
        name : String::new(),
        
        sub_args : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
        
        line_no : scanner.get_line_no(),
        line : scanner.get_current_line(),
    }
}

// This should only be used by the LTAC layer; do not use unless absolutely necessary
pub fn create_orphan_stmt(stmt_type : AstStmtType) -> AstStmt {
    AstStmt {
        stmt_type : stmt_type,
        name : String::new(),
        
        sub_args : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
        
        line_no : 0,
        line : String::new(),
    }
}

pub fn add_stmt(tree : &mut AstTree, stmt : AstStmt) {
    let top_func_pos = tree.functions.len() - 1;
    let top_func = &mut tree.functions[top_func_pos];
    &top_func.statements.push(stmt);
}

pub fn create_int(val : i32) -> AstArg {
    AstArg {
        arg_type : AstArgType::IntL,
        str_val : String::new(),
        i32_val : val,
        sub_args : Vec::new(),
    }
}

pub fn create_string(val : String) -> AstArg {
    AstArg {
        arg_type : AstArgType::StringL,
        str_val : val,
        i32_val : 0,
        sub_args : Vec::new(),
    }
}

pub fn create_arg(arg_type : AstArgType) -> AstArg {
    AstArg {
        arg_type : arg_type,
        str_val : String::new(),
        i32_val : 0,
        sub_args : Vec::new(),
    }
}


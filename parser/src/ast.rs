
// Represents AST statement types
pub enum AstStmtType {
    VarDec,
    FuncCall,
    End,
}

// Represents AST argument types
pub enum AstArgType {
    IntL,
    StringL,
    Id,
    OpMul,
}

// Represents modifiers
pub enum AstModType {
    Int
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
}

// Represents a statement
pub struct AstStmt {
    pub stmt_type : AstStmtType,
    pub name : String,
    
    pub sub_statements : Vec<AstStmt>,
    pub args : Vec<AstArg>,
    pub modifiers : Vec<AstMod>,
}

// Represents an argument
// Arguments are constants, variables, operators, etc
pub struct AstArg {
    pub arg_type : AstArgType,
    pub str_val : String,
    pub i32_val : i32,
}

// Represents an statement modifier
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
        
        println!("FUNC {}", self.name);
        
        for stmt in self.statements.iter() {
            stmt.print();
        }
    }
}

// Statement implementation
impl AstStmt {
    pub fn print(&self) {
        print!("    ");
        
        match &self.stmt_type {
            AstStmtType::VarDec => println!("VAR DEC {}", self.name),
            AstStmtType::FuncCall => println!("FUNC CALL {}", self.name),
            AstStmtType::End => println!("END"),
        }
        
        for m in self.modifiers.iter() {
            m.print();
        }
        
        for arg in self.args.iter() {
            arg.print();
        }
    }
}

// Argument implementation
impl AstArg {
    pub fn print(&self) {
        print!("        ARG ");
        
        match &self.arg_type {
            AstArgType::IntL => println!("{} ", self.i32_val),
            AstArgType::StringL => println!("\"{}\" ", self.str_val),
            AstArgType::Id => println!("{} ", self.str_val),
            AstArgType::OpMul => println!("* "),
        }
    }
}

// Modifier implementation
impl AstMod {
    pub fn print(&self) {
        print!("        MOD ");
        
        match &self.mod_type {
            AstModType::Int => println!("Int"),
        }
    }
}

// Helper functions
pub fn create_extern_func(name : String) -> AstFunc {
    AstFunc {
        name : name,
        is_extern : true,
        statements : Vec::new(),
    }
}

pub fn create_func(name : String) -> AstFunc {
    AstFunc {
        name : name,
        is_extern : false,
        statements : Vec::new(),
    }
}

pub fn create_stmt(stmt_type : AstStmtType) -> AstStmt {
    AstStmt {
        stmt_type : stmt_type,
        name : String::new(),
        
        sub_statements : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
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
    }
}

pub fn create_string(val : String) -> AstArg {
    AstArg {
        arg_type : AstArgType::StringL,
        str_val : val,
        i32_val : 0,
    }
}


use crate::ast::*;
use crate::ltac;
use crate::ltac::*;

pub struct LtacBuilder {
    file : LtacFile,
    str_pos : i32,
}

pub fn new_ltac_builder(name : String) -> LtacBuilder {
    LtacBuilder {
        file : LtacFile {
            name : name,
            data : Vec::new(),
            code : Vec::new(),
        },
        str_pos : 0,
    }
}

// The LTAC builder
impl LtacBuilder {

    // Builds the main LTAC file
    pub fn build_ltac(&mut self, tree : &AstTree) -> LtacFile {
        // Build functions
        self.build_functions(tree);
        
        self.file.clone()
    }

    // Converts AST functions to LTAC functions
    fn build_functions(&mut self, tree : &AstTree) {
        for func in tree.functions.iter() {
            if func.is_extern {
                let mut fc = ltac::create_instr(LtacType::Extern);
                fc.name = func.name.clone();
                self.file.code.push(fc);
            } else {
                let mut fc = ltac::create_instr(LtacType::Func);
                fc.name = func.name.clone();
                fc.arg1_val = 0;
                self.file.code.push(fc);
                
                self.build_block(&func.statements);
            }
        }
    }

    // Builds function body
    fn build_block(&mut self, statements : &Vec<AstStmt>) {
        for line in statements {
            match &line.stmt_type {
                AstStmtType::VarDec => {},
                AstStmtType::FuncCall => self.build_func_call(&line),
                AstStmtType::End => self.build_end(),
            }
        }
    }

    // Builds an LTAC function call
    fn build_func_call(&mut self, line : &AstStmt) {
        // Build the arguments
        for arg in line.args.iter() {
            match &arg.arg_type {
                AstArgType::IntL => {},
                
                AstArgType::StringL => {
                    let name = self.build_string(arg.str_val.clone());
                    
                    let mut push = ltac::create_instr(LtacType::PushArg);
                    push.arg1_type = LtacArg::Ptr;
                    push.arg1_sval = name;
                    self.file.code.push(push);
                },
                
                AstArgType::Id => {},
                _ => {},
            }
        }
        
        // Build the call
        let mut fc = ltac::create_instr(LtacType::Call);
        fc.name = line.name.clone();
        self.file.code.push(fc);
    }
    
    // Builds a void return
    // TODO: We will eventually need better handling of this
    fn build_end(&mut self) {
        let ret = ltac::create_instr(LtacType::Ret);
        self.file.code.push(ret);
    }

    // Builds a string and adds it to the data section
    fn build_string(&mut self, val : String) -> String {
        // Create the string name
        let spos = self.str_pos.to_string();
        self.str_pos = self.str_pos + 1;
        
        let mut name = "STR".to_string();
        name.push_str(&spos);
        
        // Create the data
        let string = LtacData {
            data_type : LtacDataType::StringL,
            name : name.clone(),
            val : val.clone(),
        };
        
        self.file.data.push(string);
        
        name
    }

}

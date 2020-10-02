
use std::collections::HashMap;

use crate::ast::*;
use crate::ltac;
use crate::ltac::*;

use crate::ltac_array::*;
use crate::ltac_flow::*;
use crate::ltac_func::*;
use crate::ltac_var::*;

#[derive(PartialEq, Clone)]
pub enum DataType {
    Int,
    IntDynArray,
}

#[derive(Clone)]
pub struct Var {
    pub pos : i32,
    pub data_type : DataType,
}

pub struct LtacBuilder {
    pub file : LtacFile,
    pub str_pos : i32,
    
    // Variable-related values
    pub vars : HashMap<String, Var>,
    pub stack_pos : i32,
    
    // For labels and blocks
    pub block_layer : i32,
    pub label_stack : Vec<String>,
    pub top_label_stack : Vec<String>,
    pub code_stack : Vec<Vec<LtacInstr>>,
    
    //For loops
    pub loop_layer : i32,
    pub loop_labels : Vec<String>,      // Needed for continue
    pub end_labels : Vec<String>,       // Needed for break
}

pub fn new_ltac_builder(name : String) -> LtacBuilder {
    LtacBuilder {
        file : LtacFile {
            name : name,
            data : Vec::new(),
            code : Vec::new(),
        },
        str_pos : 0,
        vars : HashMap::new(),
        stack_pos : 0,
        block_layer : 0,
        label_stack : Vec::new(),
        top_label_stack : Vec::new(),
        code_stack : Vec::new(),
        loop_layer : 0,
        loop_labels : Vec::new(),
        end_labels : Vec::new(),
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
                // Create the function and load the arguments
                let mut fc = ltac::create_instr(LtacType::Func);
                fc.name = func.name.clone();
                fc.arg1_val = 0;
                
                let pos = self.file.code.len();        // The position of the code before we add anything
                let mut arg_pos = 1;                   // Needed for function arguments
                
                for arg in func.args.iter() {
                    build_var_dec(self, &arg, arg_pos);
                    arg_pos += 1;
                }
                
                // Build the body and calculate the stack size
                self.build_block(&func.statements);
                
                if self.vars.len() > 0 {
                    let mut stack_size = 0;
                    while stack_size < (self.stack_pos + 1) {
                        stack_size = stack_size + 16;
                    }
                    
                    fc.arg1_val = stack_size;
                    fc.arg2_val = self.stack_pos;    // At this point, only needed by Arm
                }
                
                self.file.code.insert(pos, fc);
            }
        }
    }

    // Builds function body
    fn build_block(&mut self, statements : &Vec<AstStmt>) {
        for line in statements {
            match &line.stmt_type {
                AstStmtType::VarDec => build_var_dec(self, &line, 0),
                AstStmtType::VarAssign => build_var_assign(self, &line),
                AstStmtType::ArrayAssign => build_array_assign(self, &line),
                AstStmtType::If => build_cond(self, &line),
                AstStmtType::Elif => build_cond(self, &line),
                AstStmtType::Else => build_cond(self, &line),
                AstStmtType::While => build_while(self, &line),
                AstStmtType::Break => build_break(self),
                AstStmtType::Continue => build_continue(self),
                AstStmtType::FuncCall => build_func_call(self, &line),
                AstStmtType::Return => build_return(self, &line),
                AstStmtType::End => build_end(self),
            }
        }
    }

    // Builds a string and adds it to the data section
    pub fn build_string(&mut self, val : String) -> String {
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



use std::collections::HashMap;
use std::mem;

use crate::ast::*;
use crate::ltac;
use crate::ltac::*;
use crate::syntax::*;

use crate::ltac_array::*;
use crate::ltac_flow::*;
use crate::ltac_func::*;
use crate::ltac_var::*;

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Void,
    Byte,
    ByteDynArray,
    UByte,
    UByteDynArray,
    Short,
    UShort,
    ShortDynArray,
    UShortDynArray,
    Int,
    UInt,
    IntDynArray,
    Int64,
    Float,
    Double,
    FloatDynArray,
    DoubleDynArray,
    Str,
}

#[derive(Clone)]
pub struct Var {
    pub pos : i32,
    pub data_type : DataType,
    pub is_param : bool,
}

#[derive(Clone)]
pub struct LtacBuilder {
    pub file : LtacFile,
    pub syntax : ErrorManager,
    
    pub str_pos : i32,
    pub flt_pos : i32,
    
    // Function-related values
    pub functions : HashMap<String, DataType>,
    pub current_func : String,
    pub current_type : DataType,
    
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

pub fn new_ltac_builder(name : String, syntax : &mut ErrorManager) -> LtacBuilder {
    LtacBuilder {
        file : LtacFile {
            name : name,
            data : Vec::new(),
            code : Vec::new(),
        },
        syntax : syntax.clone(),
        str_pos : 0,
        flt_pos : 0,
        functions : HashMap::new(),
        current_func : String::new(),
        current_type : DataType::Void,
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
    pub fn build_ltac(&mut self, tree : &AstTree) -> Result<LtacFile, ()> {
        // Build functions
        if !self.build_functions(tree) {
            self.syntax.print_errors();
            return Err(());
        }
        
        Ok(self.file.clone())
    }

    // Converts AST functions to LTAC functions
    // Make two passes; the first collects information, and the second does construction
    fn build_functions(&mut self, tree : &AstTree) -> bool {
        // Collect information- for now, only names
        for func in tree.functions.iter() {
            let name = func.name.clone();
            let mut func_type = DataType::Void;
            
            if func.modifiers.len() > 0 {
                let func_mod = func.modifiers.first().unwrap();
                match &func_mod.mod_type {
                    AstModType::Byte => func_type = DataType::Byte,
                    AstModType::ByteDynArray => func_type = DataType::ByteDynArray,
                    AstModType::UByte => func_type = DataType::UByte,
                    AstModType::UByteDynArray => func_type = DataType::UByteDynArray,
                    AstModType::Short => func_type = DataType::Short,
                    AstModType::UShort => func_type = DataType::UShort,
                    AstModType::ShortDynArray => func_type = DataType::ShortDynArray,
                    AstModType::UShortDynArray => func_type = DataType::UShortDynArray,
                    AstModType::Int => func_type = DataType::Int,
                    AstModType::UInt => func_type = DataType::UInt,
                    AstModType::IntDynArray => func_type = DataType::IntDynArray,
                    AstModType::Int64 => func_type = DataType::Int64,
                    AstModType::Float => func_type = DataType::Float,
                    AstModType::Double => func_type = DataType::Double,
                    AstModType::FloatDynArray => func_type = DataType::FloatDynArray,
                    AstModType::DoubleDynArray => func_type = DataType::DoubleDynArray,
                    AstModType::Str => func_type = DataType::Str,
                    
                    // Do we need an error here? Really, it should never get to this pointer
                    AstModType::None => {},
                }
            }
        
            self.functions.insert(name, func_type);
        }
        
        // Build everything
        for func in tree.functions.iter() {
            if func.is_extern {
                let mut fc = ltac::create_instr(LtacType::Extern);
                fc.name = func.name.clone();
                self.file.code.push(fc);
            } else {
                // Set the current function and type
                self.current_func = func.name.clone();
                
                match self.functions.get(&self.current_func) {
                    Some(t) => self.current_type = t.clone(),
                    None => self.current_type = DataType::Void,
                };
            
                // Create the function and load the arguments
                let mut fc = ltac::create_instr(LtacType::Func);
                fc.name = func.name.clone();
                fc.arg1_val = 0;
                
                let pos = self.file.code.len();        // The position of the code before we add anything
                let mut arg_pos = 1;                   // Needed for function arguments
                let mut flt_arg_pos = 1;               // Needed for floating-point function arguments
                
                for arg in func.args.iter() {
                    let ret = build_var_dec(self, &arg, arg_pos, flt_arg_pos);
                    arg_pos = ret.1;
                    flt_arg_pos = ret.2;
                }
                
                // Build the body and calculate the stack size
                if !self.build_block(&func.statements) {
                    return false;
                }
                
                if self.vars.len() > 0 {
                    let mut stack_size = 0;
                    while stack_size < (self.stack_pos + 1) {
                        stack_size = stack_size + 16;
                    }
                    
                    fc.arg1_val = stack_size;
                    fc.arg2_val = self.stack_pos;    // At this point, only needed by Arm
                }
                
                self.file.code.insert(pos, fc);
                self.stack_pos = 0;
                self.vars.clear();
            }
        }
        
        true
    }

    // Builds function body
    fn build_block(&mut self, statements : &Vec<AstStmt>) -> bool {
        let mut code = true;
    
        for line in statements {
            match &line.stmt_type {
                AstStmtType::VarDec => code = build_var_dec(self, &line, 0, 0).0,
                AstStmtType::VarAssign => code = build_var_assign(self, &line),
                AstStmtType::ArrayAssign => code = build_array_assign(self, &line),
                AstStmtType::If => build_cond(self, &line),
                AstStmtType::Elif => build_cond(self, &line),
                AstStmtType::Else => build_cond(self, &line),
                AstStmtType::While => build_while(self, &line),
                AstStmtType::Break => build_break(self),
                AstStmtType::Continue => build_continue(self),
                AstStmtType::FuncCall => code = build_func_call(self, &line),
                AstStmtType::Return => code = build_return(self, &line),
                AstStmtType::Exit => code = build_exit(self, &line),
                AstStmtType::End => code = build_end(self, &line),
            }
            
            if !code {
                break;
            }
        }
        
        code
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
    
    // Builds a float literal and adds it to the data section
    // https://stackoverflow.com/questions/40030551/how-to-decode-and-encode-a-float-in-rust
    pub fn build_float(&mut self, val : f64, is_double : bool) -> String {
        // Create the float name
        let fpos = self.flt_pos.to_string();
        self.flt_pos = self.flt_pos + 1;
        
        let mut name = "FLT".to_string();
        name.push_str(&fpos);
        
        let value : String;
        let data_type : LtacDataType;
        
        if is_double {
            data_type = LtacDataType::DoubleL;
            let as_int : u64 = unsafe { mem::transmute(val) };
            value = as_int.to_string();
        } else {
            data_type = LtacDataType::FloatL;
            let as_int: u32 = unsafe { mem::transmute(val as f32) };
            value = as_int.to_string();
        }
        
        // Create the data
        let flt = LtacData {
            data_type : data_type,
            name : name.clone(),
            val : value,
        };
        
        self.file.data.push(flt);
        
        name
    }

}


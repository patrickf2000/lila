
use std::collections::HashMap;

use crate::ast::*;
use crate::ltac;
use crate::ltac::*;

#[derive(PartialEq, Clone)]
enum DataType {
    Int,
}

#[derive(Clone)]
struct Var {
    pos : i32,
    data_type : DataType,
}

pub struct LtacBuilder {
    file : LtacFile,
    str_pos : i32,
    vars : HashMap<String, Var>,
    stack_pos : i32,
    
    // For labels and blocks
    block_layer : i32,
    label_stack : Vec<String>,
    top_label_stack : Vec<String>,
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
                
                let pos = self.file.code.len();
                
                self.build_block(&func.statements);
                
                if self.vars.len() > 0 {
                    let mut stack_size = 0;
                    while stack_size < (self.stack_pos + 1) {
                        stack_size = stack_size + 16;
                    }
                    
                    fc.arg1_val = stack_size;
                }
                
                self.file.code.insert(pos, fc);
            }
        }
    }

    // Builds function body
    fn build_block(&mut self, statements : &Vec<AstStmt>) {
        for line in statements {
            match &line.stmt_type {
                AstStmtType::VarDec => self.build_var_dec(&line),
                AstStmtType::If => self.build_cond(&line),
                AstStmtType::FuncCall => self.build_func_call(&line),
                AstStmtType::Return => {},
                AstStmtType::End => self.build_end(),
            }
        }
    }
    
    // Builds an LTAC variable declaration
    fn build_var_dec(&mut self, line : &AstStmt) {
        let name = line.name.clone();
        let data_type = &line.modifiers[0];
        
        match &data_type.mod_type {
            AstModType::Int => self.stack_pos += 4,
        }
        
        let v = Var {
            pos : self.stack_pos,
            data_type : DataType::Int,
        };
        
        self.vars.insert(name, v);
        self.build_var_assign(line);
    }
    
    // Builds an LTAC variable assignment
    fn build_var_assign(&mut self, line : &AstStmt) {
        let var : Var;
        match self.vars.get(&line.name) {
            Some(v) => var = v.clone(),
            None => return,
        }
        
        if var.data_type == DataType::Int {
            if line.args.len() == 1 {
                self.build_i32var_single_assign(&line.args, &var);
            } else {
                self.build_i32var_math(&line.args, &var);
            }
        }
    }
    
    // Builds a single int32 variable assignment
    fn build_i32var_single_assign(&mut self, args : &Vec<AstArg>, var : &Var) {
        let arg = &args[0];
        
        let mut instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        
        match &arg.arg_type {
            AstArgType::IntL => {
                instr.arg2_type = LtacArg::I32;
                instr.arg2_val = arg.i32_val;
            },
            
            AstArgType::Id => {},
            _ => { /* TODO ERROR */ },
        }
        
        self.file.code.push(instr);
    }
    
    // Builds an int32 math assignment
    fn build_i32var_math(&mut self, args : &Vec<AstArg>, var : &Var) {
        let mut instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Reg;
        instr.arg1_val = 1;
        
        for arg in args.iter() {
            match &arg.arg_type {
                AstArgType::IntL => {
                    instr.arg2_type = LtacArg::I32;
                    instr.arg2_val = arg.i32_val;
                    self.file.code.push(instr.clone());
                },
                
                AstArgType::StringL => {},
                
                AstArgType::Id => {
                    match self.vars.get(&arg.str_val) {
                        Some(v) => instr.arg2_val = v.pos,
                        None => instr.arg2_val = 0,
                    }
                    
                    instr.arg2_type = LtacArg::Mem;
                    self.file.code.push(instr.clone());
                },
                
                AstArgType::OpAdd => {
                    instr = ltac::create_instr(LtacType::I32Add);
                    instr.arg1_type = LtacArg::Reg;
                    instr.arg1_val = 1;
                },
                
                AstArgType::OpMul => {
                    instr = ltac::create_instr(LtacType::I32Mul);
                    instr.arg1_type = LtacArg::Reg;
                    instr.arg1_val = 1;
                },
                
                AstArgType::OpEq => {},
            }
        }
        
        //Store the result back
        instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        instr.arg2_type = LtacArg::Reg;
        instr.arg2_val = 1;
        self.file.code.push(instr);
    }
    
    // A utility function to create a label
    fn create_label(&mut self, is_top : bool) {
        let lbl_pos = self.str_pos.to_string();
        self.str_pos += 1;
        
        let mut name = "L".to_string();
        name.push_str(&lbl_pos);
        
        if is_top {
            self.top_label_stack.push(name);
        } else {
            self.label_stack.push(name);
        }
    }
    
    // Builds an LTAC conditional block
    fn build_cond(&mut self, line : &AstStmt) {
        if self.block_layer > 0 {
            let mut jmp = ltac::create_instr(LtacType::Br);
            jmp.name = self.top_label_stack.last().unwrap().to_string();
            self.file.code.push(jmp);
        }
    
        self.block_layer += 1;
        
        if line.stmt_type == AstStmtType::If {
            self.create_label(true);
        }
        
        self.create_label(false);
        
        // Build the conditional statement
        let arg1 = &line.args.iter().nth(0).unwrap();
        let arg2 = &line.args.iter().nth(2).unwrap();
        
        let mut cmp = ltac::create_instr(LtacType::I32Cmp);
        
        match &arg1.arg_type {
            AstArgType::IntL => {
                cmp.arg1_type = LtacArg::I32;
                cmp.arg1_val = arg1.i32_val;
            },
            
            AstArgType::StringL => {},
            
            AstArgType::Id => {
                let mut mov = ltac::create_instr(LtacType::Mov);
                mov.arg1_type = LtacArg::Reg;
                mov.arg1_val = 0;
                mov.arg2_type = LtacArg::Mem;
                
                match &self.vars.get(&arg1.str_val) {
                    Some(v) => mov.arg2_val = v.pos,
                    None => mov.arg2_val = 0,
                }
                
                self.file.code.push(mov);
                
                cmp.arg1_type = LtacArg::Reg;
                cmp.arg1_val = 0;
            },
            
            _ => {},
        }
        
        match &arg2.arg_type {
            AstArgType::IntL => {
                cmp.arg2_type = LtacArg::I32;
                cmp.arg2_val = arg2.i32_val;
            },
            
            AstArgType::StringL => {},
            
            AstArgType::Id => {
                let mut mov = ltac::create_instr(LtacType::Mov);
                mov.arg1_type = LtacArg::Reg;
                mov.arg1_val = 0;
                mov.arg2_type = LtacArg::Mem;
                
                match &self.vars.get(&arg1.str_val) {
                    Some(v) => mov.arg2_val = v.pos,
                    None => mov.arg2_val = 0,
                }
                
                self.file.code.push(mov);
                
                cmp.arg1_type = LtacArg::Reg;
                cmp.arg1_val = 0;
            },
            
            _ => {},
        }
        
        self.file.code.push(cmp);
        
        // Now the operator
        let op = &line.args.iter().nth(1).unwrap();
        let mut br = ltac::create_instr(LtacType::Br);
        br.name = self.label_stack.last().unwrap().to_string();
        
        match &op.arg_type {
            AstArgType::OpEq => br.instr_type = LtacType::Bne,
            _ => {},
        }
        
        self.file.code.push(br);
    }

    // Builds an LTAC function call
    fn build_func_call(&mut self, line : &AstStmt) {
        let mut arg_type = LtacType::PushArg;
        let mut call_type = LtacType::Call;
        
        if line.name == "syscall" {
            arg_type = LtacType::KPushArg;
            call_type = LtacType::Syscall;
        }
        
        // Represents the current argument position
        let mut arg_no : i32 = 1;
    
        // Build the arguments
        for arg in line.args.iter() {
            match &arg.arg_type {
                AstArgType::IntL => {
                    let mut push = ltac::create_instr(arg_type.clone());
                    push.arg1_type = LtacArg::I32;
                    push.arg1_val = arg.i32_val.clone();
                    push.arg2_val = arg_no;
                    self.file.code.push(push);
                },
                
                AstArgType::StringL => {
                    let name = self.build_string(arg.str_val.clone());
                    
                    let mut push = ltac::create_instr(arg_type.clone());
                    push.arg1_type = LtacArg::Ptr;
                    push.arg1_sval = name;
                    push.arg2_val = arg_no;
                    self.file.code.push(push);
                },
                
                AstArgType::Id => {
                    let mut push = ltac::create_instr(arg_type.clone());
                    push.arg2_val = arg_no;
                    push.arg1_type = LtacArg::Mem;
                    
                    match &self.vars.get(&arg.str_val) {
                        Some(v) => push.arg1_val = v.pos,
                        None => push.arg1_val = 0,
                    }
                    
                    self.file.code.push(push);
                },
                
                _ => {},
            }
            
            arg_no = arg_no + 1;
        }
        
        // Build the call
        let mut fc = ltac::create_instr(call_type);
        fc.name = line.name.clone();
        self.file.code.push(fc);
    }
    
    // Builds a void return
    // TODO: We will eventually need better handling of this
    fn build_end(&mut self) {
        if self.block_layer == 0 {
            let ret = ltac::create_instr(LtacType::Ret);
            self.file.code.push(ret);
        } else {
            self.block_layer -= 1;
            
            if self.label_stack.len() > 0 {
                let mut label = ltac::create_instr(LtacType::Label);
                label.name = self.label_stack.pop().unwrap();
                self.file.code.push(label);
            }
            
            if self.top_label_stack.len() > 0 {
                let mut label = ltac::create_instr(LtacType::Label);
                label.name = self.top_label_stack.pop().unwrap();
                self.file.code.push(label);
            }
        }
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

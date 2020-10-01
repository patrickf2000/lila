
use std::collections::HashMap;

use crate::ast::*;
use crate::ltac;
use crate::ltac::*;

#[derive(PartialEq, Clone)]
enum DataType {
    Int,
    IntDynArray,
}

#[derive(Clone)]
struct Var {
    pos : i32,
    data_type : DataType,
}

pub struct LtacBuilder {
    file : LtacFile,
    str_pos : i32,
    
    // Variable-related values
    vars : HashMap<String, Var>,
    stack_pos : i32,
    
    // For labels and blocks
    block_layer : i32,
    label_stack : Vec<String>,
    top_label_stack : Vec<String>,
    code_stack : Vec<Vec<LtacInstr>>,
    
    //For loops
    loop_layer : i32,
    loop_labels : Vec<String>,      // Needed for continue
    end_labels : Vec<String>,       // Needed for break
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
                    self.build_var_dec(&arg, arg_pos);
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
                AstStmtType::VarDec => self.build_var_dec(&line, 0),
                AstStmtType::VarAssign => self.build_var_assign(&line),
                AstStmtType::ArrayAssign => self.build_array_assign(&line),
                AstStmtType::If => self.build_cond(&line),
                AstStmtType::Elif => self.build_cond(&line),
                AstStmtType::Else => self.build_cond(&line),
                AstStmtType::While => self.build_while(&line),
                AstStmtType::Break => self.build_break(),
                AstStmtType::Continue => self.build_continue(),
                AstStmtType::FuncCall => self.build_func_call(&line),
                AstStmtType::Return => self.build_return(&line),
                AstStmtType::End => self.build_end(),
            }
        }
    }
    
    // Builds an LTAC variable declaration
    fn build_var_dec(&mut self, line : &AstStmt, arg_no : i32) {
        let name = line.name.clone();
        let ast_data_type = &line.modifiers[0];
        let data_type : DataType;
        
        match &ast_data_type.mod_type {
            AstModType::Int => {
                data_type = DataType::Int;
                self.stack_pos += 4;
            },
            
            AstModType::IntDynArray => {
                data_type = DataType::IntDynArray;
                self.stack_pos += 8
            },
        }
        
        let v = Var {
            pos : self.stack_pos,
            data_type : data_type,
        };
        
        self.vars.insert(name, v);
        
        // If we have a function argument, add the load instruction
        if arg_no > 0 {
            let mut ld = ltac::create_instr(LtacType::LdArgI32);
            ld.arg1_val = self.stack_pos;
            ld.arg2_val = arg_no;
            self.file.code.push(ld);
        } else {
            self.build_var_assign(line);
        }
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
        } else if var.data_type == DataType::IntDynArray {
            self.build_i32dyn_array(&line.sub_args, &var);
        }
    }
    
    // Assigns a value to an array
    fn build_array_assign(&mut self, line : &AstStmt) {
        let var : Var;
        match self.vars.get(&line.name) {
            Some(v) => var = v.clone(),
            None => return,
        }
        
        if var.data_type == DataType::IntDynArray {
            if line.args.len() == 1 {
                self.build_i32array_single_assign(&line, &var);
            } else {
                //TODO
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
    
    // Builds a single int32 array assignment
    fn build_i32array_single_assign(&mut self, line : &AstStmt, var : &Var) {
        let arg = &line.args[0];
        let mut instr : LtacInstr;
        
        if line.sub_args.len() == 1 && line.sub_args.last().unwrap().arg_type == AstArgType::IntL {
            let imm = line.sub_args.last().unwrap();
            
            instr = ltac::create_instr(LtacType::MovOffImm);
            instr.arg1_type = LtacArg::Mem;
            instr.arg1_val = var.pos;
            instr.arg1_offset = imm.i32_val * 4;
        } else {
            // TODO: This is wrong
            instr = ltac::create_instr(LtacType::Mov);
        }
        
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
                
                _ => {},
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
    
    // Initializes a 32-bit integer array in the heap
    fn build_i32dyn_array(&mut self, args : &Vec<AstArg>, var : &Var) {
        if args.len() == 1 && args.last().unwrap().arg_type == AstArgType::IntL {
            let arg = args.last().unwrap();
            
            let mut pusharg = ltac::create_instr(LtacType::PushArg);
            pusharg.arg1_type = LtacArg::I32;
            pusharg.arg1_val = arg.i32_val * 4;
            pusharg.arg2_val = 1;
            
            self.file.code.push(pusharg);
        } else {
            //TODO
        }
        
        let mut instr = ltac::create_instr(LtacType::Call);
        instr.name = "malloc".to_string();
        self.file.code.push(instr);
        
        // Move the return register back to the variable
        instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        instr.arg2_type = LtacArg::RetRegI64;
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
        if line.stmt_type == AstStmtType::If {
            self.block_layer += 1;
            self.create_label(true);
            
            // A dummy placeholder
            let code_block : Vec<LtacInstr> = Vec::new();
            self.code_stack.push(code_block);
        } else {
            let mut jmp = ltac::create_instr(LtacType::Br);
            jmp.name = self.top_label_stack.last().unwrap().to_string();
            self.file.code.push(jmp);
        
            let mut label = ltac::create_instr(LtacType::Label);
            label.name = self.label_stack.pop().unwrap();
            self.file.code.push(label);
            
            if line.stmt_type == AstStmtType::Else {
                return;
            }
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
            AstArgType::OpNeq => br.instr_type = LtacType::Be,
            AstArgType::OpLt => br.instr_type = LtacType::Bge,
            AstArgType::OpLte => br.instr_type = LtacType::Bg,
            AstArgType::OpGt => br.instr_type = LtacType::Ble,
            AstArgType::OpGte => br.instr_type = LtacType::Bl,
            _ => {},
        }
        
        self.file.code.push(br);
    }
    
    // Builds a while loop block
    fn build_while(&mut self, line : &AstStmt) {
        self.block_layer += 1;
        self.loop_layer += 1;
        
        self.create_label(false);    // Goes at the very end
        self.create_label(false);    // Add a comparison label
        self.create_label(false);   // Add a loop label
        
        let end_label = self.label_stack.pop().unwrap();
        let loop_label = self.label_stack.pop().unwrap();
        let cmp_label = self.label_stack.pop().unwrap();
        
        self.loop_labels.push(cmp_label.clone());
        self.end_labels.push(end_label.clone());
        
        // Jump to the comparsion label, and add the loop label
        let mut br = ltac::create_instr(LtacType::Br);
        br.name = cmp_label.clone();
        self.file.code.push(br);
        
        let mut lbl = ltac::create_instr(LtacType::Label);
        lbl.name = loop_label.clone();
        self.file.code.push(lbl);
        
        // Now build the comparison
        let mut cmp_block : Vec<LtacInstr> = Vec::new();
        
        let mut lbl2 = ltac::create_instr(LtacType::Label);
        lbl2.name = cmp_label.clone();
        cmp_block.push(lbl2);
        
        // Now for the arguments
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
                
                cmp_block.push(mov);
                
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
                mov.arg1_val = 1;
                mov.arg2_type = LtacArg::Mem;
                
                match &self.vars.get(&arg2.str_val) {
                    Some(v) => mov.arg2_val = v.pos,
                    None => mov.arg2_val = 0,
                }
                
                cmp_block.push(mov);
                
                cmp.arg2_type = LtacArg::Reg;
                cmp.arg2_val = 1;
            },
            
            _ => {},
        }
        
        cmp_block.push(cmp);
        
        // Now the operator
        let op = &line.args.iter().nth(1).unwrap();
        let mut br = ltac::create_instr(LtacType::Br);
        br.name = loop_label.clone();
        
        match &op.arg_type {
            AstArgType::OpEq => br.instr_type = LtacType::Be,
            AstArgType::OpNeq => br.instr_type = LtacType::Bne,
            AstArgType::OpLt => br.instr_type = LtacType::Bl,
            AstArgType::OpLte => br.instr_type = LtacType::Ble,
            AstArgType::OpGt => br.instr_type = LtacType::Bg,
            AstArgType::OpGte => br.instr_type = LtacType::Bge,
            _ => {},
        }
        
        cmp_block.push(br);
        
        // The end label
        let mut end_lbl = ltac::create_instr(LtacType::Label);
        end_lbl.name = end_label.clone();
        cmp_block.push(end_lbl);
        
        self.code_stack.push(cmp_block);
    }
    
    // Break out of a current loop
    fn build_break(&mut self) {
        let mut br = ltac::create_instr(LtacType::Br);
        br.name = self.end_labels.last().unwrap().to_string();
        self.file.code.push(br);
    }
    
    // Continue through the rest of the loop
    fn build_continue(&mut self) {
        let mut br = ltac::create_instr(LtacType::Br);
        br.name = self.loop_labels.last().unwrap().to_string();
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
    
    // Builds a function return
    fn build_return(&mut self, line : &AstStmt) {
        if line.args.len() == 1 {
            let arg1 = line.args.first().unwrap();
            let mut mov = ltac::create_instr(LtacType::Mov);
            mov.arg1_type = LtacArg::RetRegI32;
            
            match &arg1.arg_type {
                AstArgType::IntL => {
                    mov.arg2_type = LtacArg::I32;
                    mov.arg2_val = arg1.i32_val;
                },
                
                AstArgType::StringL => {},
                AstArgType::Id => {},
                _ => {},
            }
            
            self.file.code.push(mov);
        } else if line.args.len() > 1 {
            // TODO
        }
        
        let ret = ltac::create_instr(LtacType::Ret);
        self.file.code.push(ret);
    }
    
    // Builds a void return
    fn build_end(&mut self) {
        if self.block_layer == 0 {
            let last = self.file.code.last().unwrap();
            
            if last.instr_type != LtacType::Ret {
                let ret = ltac::create_instr(LtacType::Ret);
                self.file.code.push(ret);
            }
        } else {
            self.block_layer -= 1;
            
            if self.loop_layer > 0 {
                self.loop_layer -= 1;
                
                self.end_labels.pop();
                self.loop_labels.pop();
            }
            
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
            
            if self.code_stack.len() > 0 {
                let sub_block = self.code_stack.pop().unwrap();
                
                for item in sub_block.iter() {
                    self.file.code.push(item.clone());
                }
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

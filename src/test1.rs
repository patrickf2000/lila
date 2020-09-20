use parser::ast::{AstTree, AstFunc, AstStmt, AstArg, AstMod};
use parser::ast::{AstStmtType, AstArgType, AstModType};

use parser::ltac;
use parser::ltac::{LtacFile, LtacData, LtacArg};
use parser::ltac::{LtacDataType, LtacType};

// Test the printing of a simple program:
/*
func main
    int x = 5
    int y = 3 * x
    println "Hello!"
end

// Expected
FUNC main
    VAR DEC x
        MOD int
        ARGS 5
    VAR DEC y
        MOD int
        ARGS 3 * x
    FUNC CALL println
        ARGS "Hello!"
*/

pub fn build_ast() {
    let mut tree = AstTree {
        file_name : "test.qk".to_string(),
        functions : Vec::new(),
    };
    
    let mut func = AstFunc {
        name : "main".to_string(),
        is_extern : false,
        statements : Vec::new(),
    };
    
    // VAR x = 5
    let mut var_x = AstStmt {
        stmt_type : AstStmtType::VarDec,
        name : "x".to_string(),
        
        sub_statements : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
    };
    
    let modx = AstMod {
        mod_type : AstModType::Int,
    };
    
    let x_arg = AstArg {
        arg_type : AstArgType::IntL,
        str_val : String::new(),
        i32_val : 5,
    };
    
    // VAR y = 3 * x
    let mut var_y = AstStmt {
        stmt_type : AstStmtType::VarDec,
        name : "y".to_string(),
        
        sub_statements : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
    };
    
    let mody = AstMod {
        mod_type : AstModType::Int,
    };
    
    let y1_arg = AstArg {
        arg_type : AstArgType::IntL,
        str_val : String::new(),
        i32_val : 3,
    };
    
    let y2_arg = AstArg {
        arg_type : AstArgType::OpMul,
        str_val : String::new(),
        i32_val : 0,
    };
    
    let y3_arg = AstArg {
        arg_type : AstArgType::Id,
        str_val : "x".to_string(),
        i32_val : 0,
    };
    
    // FUNC CALL puts
    let mut fc = AstStmt {
        stmt_type : AstStmtType::FuncCall,
        name : "puts".to_string(),
        
        sub_statements : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
    };
    
    let stringl = AstArg {
        arg_type : AstArgType::StringL,
        str_val : "Hello!".to_string(),
        i32_val : 0,
    };
    
    // Put it all together
    var_x.args.push(x_arg);
    var_x.modifiers.push(modx);
    
    var_y.args.push(y1_arg);
    var_y.args.push(y2_arg);
    var_y.args.push(y3_arg);
    var_y.modifiers.push(mody);
    
    fc.args.push(stringl);
    
    func.statements.push(var_x);
    func.statements.push(var_y);
    func.statements.push(fc);
    tree.functions.push(func);
    
    tree.print();
}

pub fn build_ltac() {
    // The file
    let mut file = LtacFile {
        name : "test.asm".to_string(),
        data : Vec::new(),
        code : Vec::new(),
    };
    
    // The string data
    let string = LtacData {
        data_type : LtacDataType::StringL,
        name : "STR0".to_string(),
        val : "Hello!".to_string(),
    };
    
    // extern puts
    let mut ext = ltac::create_instr(LtacType::Extern);
    ext.name = "puts".to_string();
    file.code.push(ext);
    
    // func main
    let mut func = ltac::create_instr(LtacType::Func);
    func.name = "main".to_string();
    func.arg1_val = 16;
    file.code.push(func);
    
    // int x = 5
    // mov [bp-4], 5
    let mut instr1 = ltac::create_instr(LtacType::Mov);
    instr1.arg1_type = LtacArg::Mem;
    instr1.arg1_val = 4;
    instr1.arg2_type = LtacArg::I32;
    instr1.arg2_val = 5;
    file.code.push(instr1);
    
    // int y = 3 * x
    // mov r1, 3
    // imul r1, [bp-4]
    // mov [bp-8], r1
    let mut ld1 = ltac::create_instr(LtacType::Mov);
    ld1.arg1_type = LtacArg::Reg;
    ld1.arg1_val = 1;
    ld1.arg2_type = LtacArg::I32;
    ld1.arg2_val = 3;
    file.code.push(ld1);
    
    let mut add1 = ltac::create_instr(LtacType::I32Add);
    add1.arg1_type = LtacArg::Reg;
    add1.arg1_val = 1;
    add1.arg2_type = LtacArg::Mem;
    add1.arg2_val = 4;
    file.code.push(add1);
    
    let mut str1 = ltac::create_instr(LtacType::Mov);
    str1.arg1_type = LtacArg::Mem;
    str1.arg1_val = 8;
    str1.arg2_type = LtacArg::Reg;
    str1.arg2_val = 1;
    file.code.push(str1);
    
    // puts("Hello!")
    // pusharg STR0
    // call puts
    let mut push1 = ltac::create_instr(LtacType::PushArg);
    push1.arg1_type = LtacArg::Ptr;
    push1.arg1_sval = "STR0".to_string();
    file.code.push(push1);
    
    let mut call = ltac::create_instr(LtacType::Call);
    call.name = "puts".to_string();
    file.code.push(call);
    
    // ret
    let ret = ltac::create_instr(LtacType::Ret);
    file.code.push(ret);
    
    // Put it all together
    file.data.push(string);
    
    file.print();
    
    // compile
    x86::compile(&file).expect("Codegen failed with unknown error.");
    x86::build_asm();
}

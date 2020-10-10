
use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex};
use crate::syntax::ErrorManager;

use crate::ast_utils::*;

// Builds an external function
pub fn build_extern(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager) -> bool {
    // Syntax check
    // The first token should be the "func" keyword, and the second token should be an ID
    let token1 = scanner.get_token();
    let token2 = scanner.get_token();
    let name : String;
    
    match token1 {
        Token::Func => {},
        _ => { 
            syntax.syntax_error(scanner, "Expected \"func\" keyword.".to_string());
            return false;
        }
    }
    
    match token2 {
        Token::Id(ref val) => name = val.to_string(),
        _ => {
            syntax.syntax_error(scanner, "Invalid extern-> Expected function name.".to_string());
            return false;
        }
    }
    
    let func = ast::create_extern_func(name);
    tree.functions.push(func);
    
    true
}

// A helper function for the function declaration builder
fn build_func_return(scanner : &mut Lex, func : &mut AstFunc, syntax : &mut ErrorManager) -> bool {
    let token = scanner.get_token();
        
    match token {
        Token::Int => {
            let func_type = AstMod { mod_type : AstModType::Int, };
            func.modifiers.push(func_type);
        },
        
        _ => {
            syntax.syntax_error(scanner, "Invalid function return type.".to_string());
            return false;
        },
    }
    
    true
}

// Builds a regular function declaration
pub fn build_func(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager) -> bool {
    // The first token should be the function name
    let mut token = scanner.get_token();
    let name : String;
    
    match token {
        Token::Id(ref val) => name = val.to_string(),
        _ => {
            syntax.syntax_error(scanner, "Expected function name.".to_string());
            return false;
        },
    }
    
    let mut func = ast::create_func(name);
    
    // Check for arguments, and get them if so
    token = scanner.get_token();
    
    if token != Token::LParen {
        if token == Token::Arrow {
            let ret = build_func_return(scanner, &mut func, syntax);
            
            if !ret {
                return false;
            }
        }
        
        tree.functions.push(func);
        return true;
    }
    
    let mut last_token = Token::LParen;
    
    while token != Token::RParen {
        let name_token = scanner.get_token();
        
        let mut arg = ast::create_stmt(AstStmtType::VarDec, scanner);
        
        match name_token {
            Token::Id(ref val) => arg.name = val.to_string(),
            
            Token::RParen => {
                if last_token != Token::LParen {
                    syntax.syntax_error(scanner, "Invalid function arguments list.".to_string());
                    return false;
                } else {
                    break;
                }
            },
            
            _ => {
                syntax.syntax_error(scanner, "Expected function argument name.".to_string());
                return false;
            },
        }
        
        let sym_token = scanner.get_token();
        let type_token = scanner.get_token();
        
        last_token = name_token.clone();
        
        if sym_token != Token::Colon {
            syntax.syntax_error(scanner, "Arguments should have a colon between name and type.".to_string());
            return false;
        }
        
        token = scanner.get_token();
        
        match type_token {
            Token::Int => {
                let mut data_type = AstModType::Int;
                
                if token == Token::LBracket {
                    token = scanner.get_token();
                    
                    if token == Token::RBracket {
                        data_type = AstModType::IntDynArray;
                        token = scanner.get_token();
                    } else {
                        syntax.syntax_error(scanner, "Expected closing \']\'.".to_string());
                        return false;
                    }
                }
                
                let val_type = AstMod { mod_type : data_type, };
                arg.modifiers.push(val_type);
            },
            
            Token::Float => {
                let val_type = AstMod { mod_type : AstModType::Float, };
                arg.modifiers.push(val_type);
            },
            
            Token::Double => {
                let val_type = AstMod { mod_type : AstModType::Double, };
                arg.modifiers.push(val_type);
            },
            
            Token::TStr => {
                let val_type = AstMod { mod_type : AstModType::Str, };
                arg.modifiers.push(val_type);
            },
            
            _ => {
                syntax.syntax_error(scanner, "Invalid or missing function argument type.".to_string());
                return false;
            },
        }
        
        func.args.push(arg);
        
        if token != Token::Comma && token != Token::RParen {
            syntax.syntax_error(scanner, "Invalid function arguments list.".to_string());
            return false;
        }
    }
    
    token = scanner.get_token();
    
    if token == Token::Arrow {
        let ret = build_func_return(scanner, &mut func, syntax);
        
        if !ret {
            return false;
        }
    }
    
    tree.functions.push(func);
    
    true
}

// Builds a return statement
pub fn build_return(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager) -> bool {
    let mut ret = ast::create_stmt(AstStmtType::Return, scanner);
    
    if !build_args(scanner, &mut ret, Token::Eof, syntax) {
        return false;
    }
    
    ast::add_stmt(tree, ret);
    
    true
}

// Builds the exit statement
pub fn build_exit(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager) -> bool {
    let mut exit = ast::create_stmt(AstStmtType::Exit, scanner);
    
    // Build arguments
    if !build_args(scanner, &mut exit, Token::Eof, syntax) {
        return false;
    }
    
    ast::add_stmt(tree, exit);
    
    true
}

// Builds the end statement
pub fn build_end(scanner: &mut Lex, tree : &mut AstTree) {
    let stmt = ast::create_stmt(AstStmtType::End, scanner);
    ast::add_stmt(tree, stmt);
}

// Builds function calls
pub fn build_func_call(scanner : &mut Lex, tree : &mut AstTree, id_val : String, syntax : &mut ErrorManager) -> bool {
    let mut fc = ast::create_stmt(AstStmtType::FuncCall, scanner);
    fc.name = id_val;
    
    // Build arguments
    if !build_args(scanner, &mut fc, Token::RParen, syntax) {
        return false;
    }
    
    // Add the call
    ast::add_stmt(tree, fc);
    
    true
}



use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex};

// A common function for building statement arguments
// TODO: If there's a way to not make parts of this so repetative, that would be great
pub fn build_args(scanner : &mut Lex, stmt : &mut AstStmt, end : Token) {
    let mut token = scanner.get_token();
    let mut args : Vec<AstArg> = Vec::new();
    
    let mut current_arg = ast::create_arg(AstArgType::Id);
    let mut in_array = false;
    
    while token != end {
        match token {
            Token::IntL(val) => {
                let arg = ast::create_int(val);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::StringL(ref val) => {
                let arg = ast::create_string(val.to_string());
                args.push(arg);
            },
            
            Token::Id(ref val) => {
                let mut arg = ast::create_arg(AstArgType::Id);
                arg.str_val = val.to_string();
                args.push(arg);
            },
            
            Token::Array => {
                let arg = ast::create_arg(AstArgType::Array);
                args.push(arg);
            },
            
            Token::OpAdd => {
                let arg = ast::create_arg(AstArgType::OpAdd);
                args.push(arg);
            },
            
            Token::OpMul => {
                let arg = ast::create_arg(AstArgType::OpMul);
                args.push(arg);
            },
            
            Token::OpEq => {
                let arg = ast::create_arg(AstArgType::OpEq);
                args.push(arg);
            },
            
            Token::OpNeq => {
                let arg = ast::create_arg(AstArgType::OpNeq);
                args.push(arg);
            },
            
            Token::OpLt => {
                let arg = ast::create_arg(AstArgType::OpLt);
                args.push(arg);
            },
            
            Token::OpLte => {
                let arg = ast::create_arg(AstArgType::OpLte);
                args.push(arg);
            },
            
            Token::OpGt => {
                let arg = ast::create_arg(AstArgType::OpGt);
                args.push(arg);
            },
            
            Token::OpGte => {
                let arg = ast::create_arg(AstArgType::OpGte);
                args.push(arg);
            },
            
            Token::LBracket => {
                in_array = true;
                
                let arg = args.pop().unwrap();
                current_arg = arg;
            },
            
            Token::RBracket => {
                in_array = false;
                args.push(current_arg.clone());
            },
            
            Token::Comma => {},
            
            // TODO: Better syntax error
            _ => println!("Invalid expression argument: {:?}", token),
        }
    
        token = scanner.get_token();
    }
    
    for arg in args.iter() {
        if end == Token::RBracket {
            stmt.sub_args.push(arg.clone());
        } else {
            stmt.args.push(arg.clone());
        }
    }
}


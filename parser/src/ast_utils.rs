
use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex};

// A common function for building statement arguments
pub fn build_args(scanner : &mut Lex, stmt : &mut AstStmt, end : Token) {
    let mut token = scanner.get_token();
    
    while token != end {
        match token {
            Token::IntL(val) => {
                let arg = ast::create_int(val);
                stmt.args.push(arg);
            },
            
            Token::StringL(ref val) => {
                let arg = ast::create_string(val.to_string());
                stmt.args.push(arg);
            },
            
            Token::Id(ref val) => {
                let mut arg = ast::create_arg(AstArgType::Id);
                arg.str_val = val.to_string();
                stmt.args.push(arg);
            },
            
            Token::OpAdd => {
                let arg = ast::create_arg(AstArgType::OpAdd);
                stmt.args.push(arg);
            },
            
            Token::OpMul => {
                let arg = ast::create_arg(AstArgType::OpMul);
                stmt.args.push(arg);
            },
            
            Token::OpEq => {
                let arg = ast::create_arg(AstArgType::OpEq);
                stmt.args.push(arg);
            },
            
            Token::OpNeq => {
                let arg = ast::create_arg(AstArgType::OpNeq);
                stmt.args.push(arg);
            },
            
            Token::OpLt => {
                let arg = ast::create_arg(AstArgType::OpLt);
                stmt.args.push(arg);
            },
            
            Token::OpLte => {
                let arg = ast::create_arg(AstArgType::OpLte);
                stmt.args.push(arg);
            },
            
            Token::OpGt => {
                let arg = ast::create_arg(AstArgType::OpGt);
                stmt.args.push(arg);
            },
            
            Token::OpGte => {
                let arg = ast::create_arg(AstArgType::OpGte);
                stmt.args.push(arg);
            },
            
            Token::Comma => {},
            
            // TODO: Better syntax error
            _ => println!("Invalid expression argument: {:?}", token),
        }
    
        token = scanner.get_token();
    }
}


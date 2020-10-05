
use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex};
use crate::syntax::*;

// A common function for building statement arguments
// TODO: If there's a way to not make parts of this so repetative, that would be great
pub fn build_args(scanner : &mut Lex, stmt : &mut AstStmt, end : Token, syntax : &mut ErrorManager) -> bool {
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
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::Array => {
                let arg = ast::create_arg(AstArgType::Array);
                args.push(arg);
            },
            
            Token::OpAdd => {
                let arg = ast::create_arg(AstArgType::OpAdd);
                args.push(arg);
            },
            
            Token::OpSub => {
                let arg = ast::create_arg(AstArgType::OpSub);
                args.push(arg);
            },
            
            Token::OpMul => {
                let arg = ast::create_arg(AstArgType::OpMul);
                args.push(arg);
            },
            
            Token::OpDiv => {
                let arg = ast::create_arg(AstArgType::OpDiv);
                args.push(arg);
            },
            
            Token::OpMod => {
                let arg = ast::create_arg(AstArgType::OpMod);
                args.push(arg);
            },
            
            Token::OpExponent => {
                let arg = ast::create_arg(AstArgType::OpExponent);
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
            
            Token::OpAnd => {
                let arg = ast::create_arg(AstArgType::OpAnd);
                args.push(arg);
            },
            
            Token::OpOr => {
                let arg = ast::create_arg(AstArgType::OpOr);
                args.push(arg);
            },
            
            Token::OpXor => {
                let arg = ast::create_arg(AstArgType::OpXor);
                args.push(arg);
            },
            
            Token::OpLeftShift => {
                let arg = ast::create_arg(AstArgType::OpLeftShift);
                args.push(arg);
            },
            
            Token::OpRightShift => {
                let arg = ast::create_arg(AstArgType::OpRightShift);
                args.push(arg);
            },
            
            Token::LBracket | Token::LParen => {
                in_array = true;
                
                let arg = args.pop().unwrap();
                current_arg = arg;
            },
            
            Token::RBracket | Token::RParen => {
                in_array = false;
                args.push(current_arg.clone());
            },
            
            Token::Comma => {},
            
            _ => {
                syntax.syntax_error(scanner, "Invalid token in expression.".to_string());
                return false;
            },
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
    
    true
}


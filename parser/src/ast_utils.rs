
// This file is part of the Dash compiler
// Copyright (C) 2020 Patrick Flynn
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use crate::ast;
use crate::ast::*;
use crate::ast_func::*;
use crate::lex::{Token, Lex};
use crate::syntax::*;

// Checks to see if a given token is an operator
fn is_operator(token : Token) -> bool {
    match token {
        Token::Assign |
        Token::Comma |
        Token::Any |
        Token::OpAdd |
        Token::OpSub |
        Token::OpMul |
        Token::OpDiv |
        Token::OpMod |
        Token::OpEq |
        Token::OpNeq |
        Token::OpLt |
        Token::OpLte |
        Token::OpGt |
        Token::OpGte |
        Token::OpNot |
        Token::OpAnd |
        Token::OpOr |
        Token::OpXor |
        Token::OpLeftShift |
        Token::OpRightShift => return true,
        
        _ => return false,
    }
}

// A common function for building statement arguments
// TODO: If there's a way to not make parts of this so repetative, that would be great
pub fn build_args(scanner : &mut Lex, stmt : &mut AstStmt, end : Token, syntax : &mut ErrorManager) -> bool {
    let mut token = scanner.get_token();
    let mut args : Vec<AstArg> = Vec::new();
    let mut last = Token::Unknown;
    
    let mut current_arg = ast::create_arg(AstArgType::Id);
    let mut in_array = false;
    
    while token != end {
        match token {
            Token::ByteL(val) => {
                let arg = ast::create_byte(val);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::ShortL(val) => {
                let arg = ast::create_short(val);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::IntL(val) => {
                let arg = ast::create_int(val);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::FloatL(val) => {
                let arg = ast::create_float(val);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::CharL(val) => {
                let arg = ast::create_char(val);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::StringL(ref val) => {
                let arg = ast::create_string(val.to_string());
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
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
            
            Token::LdArg => {
                let arg = build_ldarg(scanner, syntax);
                
                if arg.arg_type == AstArgType::None {
                    return false;
                }
                
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
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::OpSub => {
                let arg : AstArg;
                if last == Token::Unknown || is_operator(last) {
                    arg = ast::create_arg(AstArgType::OpNeg);
                } else {
                    arg = ast::create_arg(AstArgType::OpSub);
                }
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::OpMul => {
                let arg = ast::create_arg(AstArgType::OpMul);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::OpDiv => {
                let arg = ast::create_arg(AstArgType::OpDiv);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
            },
            
            Token::OpMod => {
                let arg = ast::create_arg(AstArgType::OpMod);
                
                if in_array {
                    current_arg.sub_args.push(arg);
                } else {
                    args.push(arg);
                }
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
    
        last = token.clone();
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

// Checks the order of operations in an expression
pub fn check_operations(original_args : &Vec<AstArg>) -> Vec<AstArg> {
    if original_args.len() < 4 {
        return original_args.to_vec();
    }

    let mut args : Vec<AstArg> = Vec::new();
    let mut operations : Vec<AstArg> = Vec::new();
    
    for arg in original_args.iter() {
        match arg.arg_type {
            AstArgType::OpAdd |
            AstArgType::OpSub |
            AstArgType::OpMul |
            AstArgType::OpDiv |
            AstArgType::OpMod => {
                loop {
                    if operations.len() == 0 {
                        break;
                    }
                    
                    let top_type = &operations.last().unwrap().arg_type;
                    
                    if arg.arg_type == AstArgType::OpAdd || arg.arg_type == AstArgType::OpSub {
                        let top = operations.pop().unwrap();
                        args.push(top);
                    } else {
                        match top_type {
                            AstArgType::OpAdd | AstArgType::OpSub => {
                                break;
                            },
                            
                            _ => {
                                let top = operations.pop().unwrap();
                                args.push(top);
                            },
                        }
                    }
                }
                
                operations.push(arg.clone());
            },
            
            _ => args.push(arg.clone()),
        }
    }
    
    while operations.len() > 0 {
        let op = operations.pop().unwrap();
        args.push(op.clone());
    }
    
    // Now, convert back to infix
    let mut stack : Vec<Vec<AstArg>> = Vec::new();
    let mut negate_next = false;
    
    for arg in args {
        match arg.arg_type {
            AstArgType::OpAdd |
            AstArgType::OpSub |
            AstArgType::OpMul |
            AstArgType::OpDiv |
            AstArgType::OpMod => {
                let arg2 = stack.pop().unwrap();
                let arg1 = stack.pop().unwrap();
                let mut ln : Vec<AstArg> = Vec::new();
                
                let lparen = ast::create_arg(AstArgType::OpLParen);
                let rparen = ast::create_arg(AstArgType::OpRParen);
                
                if arg1.len() > 2 {
                    ln.push(lparen.clone());
                    
                    for a in arg1.iter() {
                        ln.push(a.clone());
                    }
                    
                    ln.push(rparen.clone());
                } else {
                    for a in arg1.iter() {
                        ln.push(a.clone());
                    }
                }
                
                ln.push(arg.clone());
                
                if arg2.len() > 2 {
                    ln.push(lparen);
                    
                    for a in arg2.iter() {
                        ln.push(a.clone());
                    }
                    
                    ln.push(rparen);
                } else {
                    for a in arg2.iter() {
                        ln.push(a.clone());
                    }
                }
                
                stack.push(ln);
            },
            
            AstArgType::OpNeg => negate_next = true,
            
            _ => {
                let mut ln : Vec<AstArg> = Vec::new();
                
                if negate_next {
                    let arg = ast::create_arg(AstArgType::OpNeg);
                    ln.push(arg);
                }
                
                ln.push(arg.clone());
                stack.push(ln);
                
                negate_next = false;
            },
        }
    }
    
    // TODO: This is dangerous
    stack.pop().unwrap()
}



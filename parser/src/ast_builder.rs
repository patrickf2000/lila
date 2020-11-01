
// Import what we need
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex, create_lex};
use crate::syntax::ErrorManager;

use crate::ast_func::*;
use crate::ast_utils::*;

// The AST building function
// This function opens the file and reads a line; 
// the line is then passed to another function which lexically analyzes
// it and builds an AST node.
//
// In Quik, each line is a self-contained expression; as a result, we read a line
// and then lexically analyze and build an AST node from it
//
pub fn build_ast(path : String, name : String, syntax : &mut ErrorManager) -> Result<AstTree, ()> {   
    let mut tree = AstTree {
        file_name : name,
        functions : Vec::new(),
    };
    
    // Open the file
    let file = File::open(&path)
        .expect("Error: Unable to open input file.");
    let reader = BufReader::new(file);
    
    // Read the thing line by line
    let mut line_no = 0;
    
    for line in reader.lines() {
        let mut current = line.unwrap();
        current = current.trim().to_string();
        line_no += 1;
        
        if current.len() == 0 {
            continue;
        }
        
        let ret = build_line(current, line_no, &mut tree, syntax);
        
        if !ret {
            syntax.print_errors();
            return Err(());
        }
    }
    
    Ok(tree)
}

// Converts a line to an AST node
fn build_line(line : String, line_no : i32, tree : &mut AstTree, syntax : &mut ErrorManager) -> bool {
    let mut scanner = create_lex(line);
    scanner.tokenize(line_no);
    
    let mut code = true;
    
    // Get the first token
    let mut token = scanner.get_token();
    
    match token {
        Token::Extern => {
            token = scanner.get_token();
            match token {
                Token::Func => {},
                _ => {
                    syntax.syntax_error(&mut scanner, "Expected \"func\" keyword.".to_string());
                    return false;
                }
            }
                
            code = build_func(&mut scanner, tree, syntax, true)
        },
        
        Token::Func => code = build_func(&mut scanner, tree, syntax, false),
        Token::Return => code = build_return(&mut scanner, tree, syntax),
        Token::Exit => code = build_exit(&mut scanner, tree, syntax),
        Token::End => build_end(&mut scanner, tree),
        Token::Byte => code = build_var_dec(&mut scanner, tree, syntax, AstModType::Byte),
        Token::UByte => code = build_var_dec(&mut scanner, tree, syntax, AstModType::UByte),
        Token::Short => code = build_var_dec(&mut scanner, tree, syntax, AstModType::Short),
        Token::UShort => code = build_var_dec(&mut scanner, tree, syntax, AstModType::UShort),
        Token::Int => code = build_var_dec(&mut scanner, tree, syntax, AstModType::Int),
        Token::UInt => code = build_var_dec(&mut scanner, tree, syntax, AstModType::UInt),
        Token::Int64 => code = build_var_dec(&mut scanner, tree, syntax, AstModType::Int64),
        Token::UInt64 => code = build_var_dec(&mut scanner, tree, syntax, AstModType::UInt64),
        Token::Float => code = build_var_dec(&mut scanner, tree, syntax, AstModType::Float),
        Token::Double => code = build_var_dec(&mut scanner, tree, syntax, AstModType::Double),
        Token::TStr => code = build_var_dec(&mut scanner, tree, syntax, AstModType::Str),
        Token::Id(ref val) => code = build_id(&mut scanner, tree, val.to_string(), syntax),
        Token::If => code = build_cond(&mut scanner, tree, Token::If, syntax),
        Token::Elif => code = build_cond(&mut scanner, tree, Token::Elif, syntax),
        Token::Else => code = build_cond(&mut scanner, tree, Token::Else, syntax),
        Token::While => code = build_cond(&mut scanner, tree, Token::While, syntax),
        
        Token::Break => {
            let br = ast::create_stmt(AstStmtType::Break, &mut scanner);
            ast::add_stmt(tree, br);
        },
        
        Token::Continue => {
            let cont = ast::create_stmt(AstStmtType::Continue, &mut scanner);
            ast::add_stmt(tree, cont);
        },
        
        Token::Eof => {},
        
        _ => {
            syntax.syntax_error(&mut scanner, "Invalid token.".to_string());
            code = false;
        }
    }
    
    code
}

// Builds an integer variable declaration
fn build_var_dec(scanner : &mut Lex, tree : &mut AstTree, syntax : &mut ErrorManager, dtype : AstModType) -> bool {
    let mut var_dec = ast::create_stmt(AstStmtType::VarDec, scanner);
    let mut is_array = false;
        
    let mut data_type = AstMod {
        mod_type : dtype.clone(),
    };
    
    // Gather information
    // The first token should be the name
    let mut token = scanner.get_token();
    
    match token {
        Token::Id(ref val) => var_dec.name = val.to_string(),
        
        Token::LBracket => {
            is_array = true;
            if !build_args(scanner, &mut var_dec, Token::RBracket, syntax) {
                return false;
            }
            
            token = scanner.get_token();
            match token {
                Token::Id(ref val) => var_dec.name = val.to_string(),
                _ => {
                    syntax.syntax_error(scanner, "Expected array name.".to_string());
                    return false;
                },
            }
        },
        
        _ => {
            syntax.syntax_error(scanner, "Expected variable name.".to_string());
            return false;
        },
    }
    
    // The next token should be the assign operator
    token = scanner.get_token();
    
    match token {
        Token::Assign => {},
        _ => {
            syntax.syntax_error(scanner, "Expected assignment operator.".to_string());
            return false;
        },
    }
    
    // Build the remaining arguments
    if !build_args(scanner, &mut var_dec, Token::Eof, syntax) {
        return false;
    }
    
    // If we have the array, check the array type
    if is_array {
        if var_dec.args.len() == 1 && var_dec.args.last().unwrap().arg_type == AstArgType::Array {
            match &dtype {
                AstModType::Byte => data_type.mod_type = AstModType::ByteDynArray,
                AstModType::UByte => data_type.mod_type = AstModType::UByteDynArray,
                AstModType::Short => data_type.mod_type = AstModType::ShortDynArray,
                AstModType::UShort => data_type.mod_type = AstModType::UShortDynArray,
                AstModType::Int => data_type.mod_type = AstModType::IntDynArray,
                AstModType::UInt => data_type.mod_type = AstModType::UInt,
                AstModType::Float => data_type.mod_type = AstModType::FloatDynArray,
                AstModType::Double => data_type.mod_type = AstModType::DoubleDynArray,
                
                _ => {},
            }
        } else {
            //TODO
        }
    }
    
    var_dec.modifiers.push(data_type);
    ast::add_stmt(tree, var_dec);
    
    true
}

// Builds a variable assignment
fn build_var_assign(scanner : &mut Lex, tree : &mut AstTree, name : String, syntax : &mut ErrorManager) -> bool {
    let mut var_assign = ast::create_stmt(AstStmtType::VarAssign, scanner);
    var_assign.name = name;
    
    if !build_args(scanner, &mut var_assign, Token::Eof, syntax) {
        return false;
    }
    
    ast::add_stmt(tree, var_assign);
    
    true
}

// Builds an array assignment
fn build_array_assign(scanner : &mut Lex, tree : &mut AstTree, id_val : String, syntax : &mut ErrorManager) -> bool {
    let mut array_assign = ast::create_stmt(AstStmtType::ArrayAssign, scanner);
    array_assign.name = id_val;
    
    // For the array index
    if !build_args(scanner, &mut array_assign, Token::RBracket, syntax) {
        return false;
    }
    
    if scanner.get_token() != Token::Assign {
        syntax.syntax_error(scanner, "Expected \'=\' in array assignment.".to_string());
        return false;
    }
    
    // Tokens being assigned to the array
    if !build_args(scanner, &mut array_assign, Token::Eof, syntax) {
        return false;
    }
    
    ast::add_stmt(tree, array_assign);
    
    true
}

// Handles cases when an identifier is the first token
fn build_id(scanner : &mut Lex, tree : &mut AstTree, id_val : String, syntax : &mut ErrorManager) -> bool {
    // If the next token is an assignment, we have a variable assignment
    // If the next token is a parantheses, we have a function call
    let token = scanner.get_token();
    let code : bool;
    
    match token {
        Token::Assign => code = build_var_assign(scanner, tree, id_val, syntax),
        Token::LParen => code = build_func_call(scanner, tree, id_val, syntax),
        Token::LBracket => code = build_array_assign(scanner, tree, id_val, syntax),
        _ => {
            syntax.syntax_error(scanner, "Invalid assignment or call.".to_string());
            return false;
        },
    }
    
    code
}

// Builds conditional statements
fn build_cond(scanner : &mut Lex, tree : &mut AstTree, cond_type : Token, syntax : &mut ErrorManager) -> bool {
    let mut ast_cond_type : AstStmtType = AstStmtType::If;
    match cond_type {
        Token::If => ast_cond_type = AstStmtType::If,
        Token::Elif => ast_cond_type = AstStmtType::Elif,
        Token::Else => ast_cond_type = AstStmtType::Else,
        Token::While => ast_cond_type = AstStmtType::While,
        _ => {},
    }
    
    let mut cond = ast::create_stmt(ast_cond_type, scanner);
    
    // Build the rest arguments
    if cond_type != Token::Else {
        if !build_args(scanner, &mut cond, Token::Eof, syntax) {
            return false;
        }
    }
    
    ast::add_stmt(tree, cond);
    
    true
}


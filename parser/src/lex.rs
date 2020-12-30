
// This file is part of the Lila compiler
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


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
    EoI,
    
    Module,
    Use,
    
    Enum,
    
    Extern,
    Func,
    LdArg,
    Begin,
    Return,
    Exit,
    End,
    
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,
    
    Const,
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Int64,
    UInt64,
    Float,
    Double,
    Char,
    TStr,
    
    LParen,
    RParen,
    LBracket,
    RBracket,
    Assign,
    Colon,
    Comma,
    Semicolon,
    Arrow,
    Any,
    Sizeof,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpEq,
    OpNeq,
    OpLt,
    OpLte,
    OpGt,
    OpGte,
    OpNot,
    OpAnd,
    OpOr,
    OpXor,
    OpLeftShift,
    OpRightShift,
    
    Id(String),
    ByteL(u8),
    ShortL(u16),
    IntL(u64),
    FloatL(f64),
    CharL(char),
    StringL(String),
}

pub struct Line {
    input : String,
    line_no : i32,
    all_tokens : Vec<Token>,
}

pub struct Lex {
    current_input : String,
    current_line_no : i32,
    current_line : usize,
    pos : usize,
    index : usize,
    lines : Vec<Line>,
    current_tokens : Vec<Token>,
}

impl Lex {
    pub fn get_current_line(&mut self) -> String {
        self.current_input.clone()
    }
    
    pub fn get_line_no(&mut self) -> i32 {
        self.current_line_no
    }

    pub fn tokenize(&mut self, input : String, line_no : i32) {
        let mut line = Line {
            input : input.clone(),
            line_no : line_no,
            all_tokens : Vec::new(),
        };
    
        self.index = 0;
        let mut current = String::new();
        let mut in_quote = false;
        
        let length = input.len();
        
        while self.index < length {
            let c = input.chars().nth(self.index).unwrap();
            let c2 : char;
            if self.index + 1 < length {
                c2 = input.chars().nth(self.index+1).unwrap();
            } else {
                c2 = '\0';
            }
            
            // Return if we have a comment
            if c == '#' {
                return;
            }
            
            // Check to see if we have a char literal
            if c == '\'' {
                let c = input.chars().nth(self.index+1).unwrap();
                
                let token = Token::CharL(c);
                line.all_tokens.push(token);
                
                self.index += 3;
                continue;
            }
            
            // Check to see if we are in a quote (string literal)
            if c == '\"' {
                if in_quote {
                    in_quote = false;
                    
                    let token = Token::StringL(current);
                    line.all_tokens.push(token);
                    current = String::new();
                } else {
                    in_quote = true;
                }
                
                self.index += 1;
                continue;
            }
            
            if in_quote {
                current.push(c);
                self.index += 1;
                continue;
            }
        
            // Otherwise, do other checks
            if self.is_symbol(c) {
                if current.len() > 0 {
                    let token = self.get_keyword(current);
                    line.all_tokens.push(token);
                    current = String::new();
                }
                
                let symbol = self.get_symbol(c, c2);
                line.all_tokens.push(symbol);
            } else if c == ' ' || c == '\t' {
                if current.len() > 0 {
                    let token = self.get_keyword(current);
                    line.all_tokens.push(token);
                    current = String::new();
                }
            } else {
                current.push(c);
            }
            
            self.index += 1;
        }
        
        if current.len() > 0 {
            let token = self.get_keyword(current);
            line.all_tokens.push(token);
        }
        
        self.lines.push(line);
    }
    
    pub fn get_token(&mut self) -> Token {
        let token : Token;
        
        if self.pos >= self.current_tokens.len() && self.current_line >= self.lines.len() {
            token = Token::EoI;
        } else if self.pos >= self.current_tokens.len() {
            self.current_tokens = self.lines[self.current_line].all_tokens.clone();
            
            self.pos = 0;
            self.current_line += 1;
            
            token = Token::Eof;
        } else {
            if self.pos == 0 && self.current_line > 0 {
                self.current_input = self.lines[self.current_line-1].input.clone();
                self.current_line_no = self.lines[self.current_line-1].line_no;
            }
            
            token = self.current_tokens[self.pos].clone();
            self.pos += 1;
        }
        
        token
    }
    
    // Checks to see if a given character is a symbol or part of one
    fn is_symbol(&self, c : char) -> bool {
        match c {
            '(' => return true,
            ')' => return true,
            '[' => return true,
            ']' => return true,
            '=' => return true,
            ':' => return true,
            ',' => return true,
            ';' => return true,
            '+' => return true,
            '-' => return true,
            '*' => return true,
            '/' => return true,
            '%' => return true,
            '!' => return true,
            '<' => return true,
            '>' => return true,
            '&' => return true,
            '|' => return true,
            '^' => return true,
            _ => return false,
        }
    }
    
    // Returns the symbol for a given character
    fn get_symbol(&mut self, c : char, c2 : char) -> Token {
        match c {
            '(' => return Token::LParen,
            ')' => return Token::RParen,
            '[' => return Token::LBracket,
            ']' => return Token::RBracket,
            
            '=' => {
                if c2 == '=' {
                    self.index += 1;
                    return Token::OpEq;
                }
                
                return Token::Assign;
            },
            
            '!' => {
                if c2 == '=' {
                    self.index += 1;
                    return Token::OpNeq;
                }
                
                return Token::OpNot;
            },
            
            '<' => {
                if c2 == '=' {
                    self.index += 1;
                    return Token::OpLte;
                } else if c2 == '<' {
                    self.index += 1;
                    return Token::OpLeftShift;
                }
                
                return Token::OpLt;
            },
            
            '>' => {
                if c2 == '=' {
                    self.index += 1;
                    return Token::OpGte;
                } else if c2 == '>' {
                    self.index += 1;
                    return Token::OpRightShift;
                }
                
                return Token::OpGt;
            },
            
            '-' => {
                if c2 == '>' {
                    self.index += 1;
                    return Token::Arrow;
                }
                
                return Token::OpSub;
            },
            
            ':' => return Token::Colon,
            ',' => return Token::Comma,
            ';' => return Token::Semicolon,
            '+' => return Token::OpAdd,
            '*' => return Token::OpMul,
            '/' => return Token::OpDiv,
            '%' => return Token::OpMod,
            
            '&' => return Token::OpAnd,
            '|' => return Token::OpOr,
            '^' => return Token::OpXor,
            
            _ => return Token::Unknown,
        }
    }
    
    // Returns a keyword for a given buffer
    fn get_keyword(&self, current : String) -> Token {
        // Check to see if we have a literal
        // Start with hex
        match current.get(..2) {
            Some("0x") => {
                let base = current.trim_start_matches("0x");
                
                if base.len() > 4 && base.len() <= 16 {
                    return Token::IntL(u64::from_str_radix(base, 16).unwrap());
                } else if base.len() == 4 || base.len() == 3 {
                    return Token::ShortL(u16::from_str_radix(base, 16).unwrap());
                } else {
                    return Token::ByteL(u8::from_str_radix(base, 16).unwrap());
                }
            },
            
            _ => {},
        }
        
        // Check other literals
        if current.parse::<u64>().is_ok() {
            return Token::IntL(current.parse::<u64>().unwrap());
        } else if current.parse::<f64>().is_ok() {
            return Token::FloatL(current.parse::<f64>().unwrap());
        }
    
        // If not, it must be a keyword
        let token : Token;
        
        match current.as_ref() {
            "..." => token = Token::Any,
            "module" => token = Token::Module,
            "use" => token = Token::Use,
            "enum" => token = Token::Enum,
            "extern" => token = Token::Extern,
            "func" => token = Token::Func,
            "ldarg" => token = Token::LdArg,
            "begin" => token = Token::Begin,
            "return" => token = Token::Return,
            "exit" => token = Token::Exit,
            "end" => token = Token::End,
            "const" => token = Token::Const,
            "byte" => token = Token::Byte,
            "ubyte" => token = Token::UByte,
            "short" => token = Token::Short,
            "ushort" => token = Token::UShort,
            "int" => token = Token::Int,
            "uint" => token = Token::UInt,
            "int64" => token = Token::Int64,
            "uint64" => token = Token::UInt64,
            "float" => token = Token::Float,
            "double" => token = Token::Double,
            "char" => token = Token::Char,
            "str" => token = Token::TStr,
            "if" => token = Token::If,
            "elif" => token = Token::Elif,
            "else" => token = Token::Else,
            "while" => token = Token::While,
            "break" => token = Token::Break,
            "continue" => token = Token::Continue,
            "sizeof" => token = Token::Sizeof,
            _ => token = Token::Id(current.clone()),
        };
        
        token
    }
}

pub fn create_lex() -> Lex {
    Lex {
        current_line_no : 0,
        current_input : String::new(),
        current_line : 0,
        pos : 0,
        index : 0,
        lines : Vec::new(),
        current_tokens : Vec::new(),
    }
}

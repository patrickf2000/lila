
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
    
    Extern,
    Func,
    Return,
    Exit,
    End,
    
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,
    
    Byte,
    Short,
    Int,
    Float,
    Double,
    TStr,
    Array,
    
    LParen,
    RParen,
    LBracket,
    RBracket,
    Assign,
    Colon,
    Comma,
    Arrow,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpExponent,
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
    IntL(i32),
    FloatL(f64),
    StringL(String),
}

pub struct Lex {
    input : String,
    line_no : i32,
    pos : usize,
    index : usize,
    all_tokens : Vec<Token>,
}

impl Lex {
    pub fn get_current_line(&mut self) -> String {
        self.input.clone()
    }
    
    pub fn get_line_no(&mut self) -> i32 {
        self.line_no
    }

    pub fn tokenize(&mut self, line_no : i32) {
        self.line_no = line_no;
    
        self.index = 0;
        let mut current = String::new();
        let mut in_quote = false;
        
        let length = self.input.len();
        
        while self.index < length {
            let c = self.input.chars().nth(self.index).unwrap();
            let c2 : char;
            if self.index + 1 < length {
                c2 = self.input.chars().nth(self.index+1).unwrap();
            } else {
                c2 = '\0';
            }
            
            // Return if we have a comment
            if c == '#' {
                return;
            }
            
            // Check to see if we are in a quote (string literal)
            if c == '\"' {
                if in_quote {
                    in_quote = false;
                    
                    let token = Token::StringL(current);
                    self.all_tokens.push(token);
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
                    self.all_tokens.push(token);
                    current = String::new();
                }
                
                let symbol = self.get_symbol(c, c2);
                self.all_tokens.push(symbol);
            } else if c == ' ' || c == '\t' {
                if current.len() > 0 {
                    let token = self.get_keyword(current);
                    self.all_tokens.push(token);
                    current = String::new();
                }
            } else {
                current.push(c);
            }
            
            self.index += 1;
        }
        
        if current.len() > 0 {
            let token = self.get_keyword(current);
            self.all_tokens.push(token);
        }
    }
    
    pub fn get_token(&mut self) -> Token {
        let token : Token;
        
        if self.pos >= self.all_tokens.len() {
            token = Token::Eof;
        } else {
            token = self.all_tokens[self.pos].clone();
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
                
                if base.len() == 4 || base.len() == 3 {
                    return Token::ShortL(u16::from_str_radix(base, 16).unwrap());
                } else {
                    return Token::ByteL(u8::from_str_radix(base, 16).unwrap());
                }
            },
            
            _ => {},
        }
        
        // Check other literals
        if current.parse::<i32>().is_ok() {
            return Token::IntL(current.parse::<i32>().unwrap());
        } else if current.parse::<f64>().is_ok() {
            return Token::FloatL(current.parse::<f64>().unwrap());
        }
    
        // If not, it must be a keyword
        let token : Token;
        
        match current.as_ref() {
            "extern" => token = Token::Extern,
            "func" => token = Token::Func,
            "return" => token = Token::Return,
            "exit" => token = Token::Exit,
            "end" => token = Token::End,
            "byte" => token = Token::Byte,
            "short" => token = Token::Short,
            "int" => token = Token::Int,
            "float" => token = Token::Float,
            "double" => token = Token::Double,
            "str" => token = Token::TStr,
            "if" => token = Token::If,
            "elif" => token = Token::Elif,
            "else" => token = Token::Else,
            "while" => token = Token::While,
            "break" => token = Token::Break,
            "continue" => token = Token::Continue,
            "array" => token = Token::Array,
            _ => token = Token::Id(current.clone()),
        };
        
        token
    }
}

pub fn create_lex(input : String) -> Lex {
    Lex {
        input : input,
        line_no : 0,
        pos : 0,
        index : 0,
        all_tokens : Vec::new(),
    }
}


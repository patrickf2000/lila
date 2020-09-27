
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
    
    Extern,
    Func,
    Return,
    End,
    
    If,
    Elif,
    Else,
    
    Int,
    TStr,
    
    LParen,
    RParen,
    Assign,
    Colon,
    Comma,
    OpAdd,
    OpMul,
    OpEq,
    
    Id(String),
    IntL(i32),
    FloatL(f64),
    StringL(String),
}

pub struct Lex {
    input : String,
    pos : usize,
    index : usize,
    all_tokens : Vec<Token>,
}

impl Lex {
    pub fn tokenize(&mut self) {
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
            '=' => return true,
            ':' => return true,
            ',' => return true,
            '+' => return true,
            '*' => return true,
            _ => return false,
        }
    }
    
    // Returns the symbol for a given character
    fn get_symbol(&mut self, c : char, c2 : char) -> Token {
        match c {
            '(' => return Token::LParen,
            ')' => return Token::RParen,
            
            '=' => {
                if c2 == '=' {
                    self.index += 1;
                    return Token::OpEq;
                }
                
                return Token::Assign;
            },
            
            ':' => return Token::Colon,
            ',' => return Token::Comma,
            '+' => return Token::OpAdd,
            '*' => return Token::OpMul,
            _ => return Token::Unknown,
        }
    }
    
    // Returns a keyword for a given buffer
    fn get_keyword(&self, current : String) -> Token {
        // Check to see if we have a literal
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
            "end" => token = Token::End,
            "int" => token = Token::Int,
            "str" => token = Token::TStr,
            "if" => token = Token::If,
            "elif" => token = Token::Elif,
            "else" => token = Token::Else,
            _ => token = Token::Id(current.clone()),
        };
        
        token
    }
}

pub fn create_lex(input : String) -> Lex {
    Lex {
        input : input,
        pos : 0,
        index : 0,
        all_tokens : Vec::new(),
    }
}


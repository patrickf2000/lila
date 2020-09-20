
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Unknown,
    Eof,
    
    Extern,
    Func,
    End,
    
    Int,
    TStr,
    
    LParen,
    RParen,
    Assign,
    Colon,
    Comma,
    OpAdd,
    OpMul,
    
    Id(String),
    IntL(i32),
    FloatL(f64),
    StringL(String),
}

pub struct Lex {
    input : String,
    pos : usize,
    all_tokens : Vec<Token>,
}

impl Lex {
    pub fn tokenize(&mut self) {
        let mut current = String::new();
        let mut index = 0;
        let mut in_quote = false;
        
        for c in self.input.chars() {
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
                
                index = index + 1;
                continue;
            }
            
            if in_quote {
                current.push(c);
                index = index + 1;
                continue;
            }
        
            // Otherwise, do other checks
            if self.is_symbol(c) {
                if current.len() > 0 {
                    let token = self.get_keyword(current);
                    self.all_tokens.push(token);
                    current = String::new();
                }
                
                let symbol = self.get_symbol(c, index);
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
            
            index = index + 1;
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
    fn get_symbol(&self, c : char, _pos : i32) -> Token {
        match c {
            '(' => return Token::LParen,
            ')' => return Token::RParen,
            '=' => return Token::Assign,
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
            "end" => token = Token::End,
            "int" => token = Token::Int,
            "str" => token = Token::TStr,
            _ => token = Token::Id(current.clone()),
        };
        
        token
    }
}

pub fn create_lex(input : String) -> Lex {
    Lex {
        input : input,
        pos : 0,
        all_tokens : Vec::new(),
    }
}

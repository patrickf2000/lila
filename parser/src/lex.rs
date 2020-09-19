
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
    
    Id(String),
    IntL(i32),
}

pub struct Lex {
    input : String,
    pos : usize,
    all_tokens : Vec<Token>,
}

impl Lex {
    pub fn tokenize(&mut self) {
        let mut current = String::new();
        
        for c in self.input.chars() {
            if (c == ' ' || c == '\t') && current.len() > 0 {
                let token = get_keyword(current);
                self.all_tokens.push(token);
                current = String::new();
            } else {
                current.push(c);
            }
        }
        
        if current.len() > 0 {
            let token = get_keyword(current);
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
}

pub fn create_lex(input : String) -> Lex {
    Lex {
        input : input,
        pos : 0,
        all_tokens : Vec::new(),
    }
}

fn get_keyword(current : String) -> Token {
    let mut token = Token::Unknown;
    
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

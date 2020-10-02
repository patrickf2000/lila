
use crate::lex::*;

pub struct SyntaxError {
    pub line_no : i32,
    pub line : String,
    pub message : String,
}

pub struct ErrorManager {
    pub errors : Vec<SyntaxError>,
}

pub fn create_error_manager() -> ErrorManager {
    ErrorManager {
        errors : Vec::new(),
    }
}

impl ErrorManager {

    // Called when the AST is being built
    pub fn syntax_error(&mut self, scanner : &mut Lex, msg : String) {
        let error = SyntaxError {
            line_no : scanner.get_line_no(),
            line : scanner.get_current_line(),
            message : msg,
        };
        
        self.errors.push(error);
    }
    
    // Called to print any syntax errors
    pub fn print_errors(&mut self) {
        for error in self.errors.iter() {
            println!("Syntax Error: {}", error.message);
            println!(" -> [{}] {}", error.line_no, error.line);
            println!("");
        }
    }
}


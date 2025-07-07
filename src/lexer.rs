use std::error::Error;

use crate::token::Token;
macro_rules! curr {
    (self) => {
        self.char_vec[self.char_index]
    };
}

pub struct Lexer {
    last_char: char,
    char_vec: Vec<char>,
    char_index: usize,
    number_string: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(char_vec: Vec<char>) -> Self {
        let number_string = "".to_string();
        let tokens: Vec<Token> = Vec::new();
        let char_index: usize = 0;
        let last_char: char = ' ';
        Self {
            last_char,
            char_vec,
            char_index,
            number_string,
            tokens,
        }
    }
    fn get_tok(&self) -> Token {
        self.skip_whitespace();

        if curr!(self).is_alphabetical() {
            return self.get_string();
        }

        if curr!(self).is_numeric() || curr!(self) == '.' {
            return self.get_number();
        }

        if curr!(self) == '#' {
            while curr!(self) != EOF && curr!(self) != '\n' && curr!(self) != '\r' {
                move_read_head();
            }
            if curr!(self) != EOF {
                return self.get_tok();
            }
        }

        // Check for end of file.  Don't eat the EOF.
        if curr!(self) == EOF {
            return Token::Eof;
        }

        // Otherwise, just return the character as its ascii value.
        let current_char = curr!(self);
        move_read_head();
        return current_char;
    }

    fn get_string(&self) -> Token {
        let identifier_string = "";

        while curr!(self).is_alphanumeric() {
            identifier_string += curr!(self);
            move_read_head();
        }

        return match identifier_string {
            "def" => Token::Def,
            "extern" => Token::Eof,
            _ => Token::Identifier(identifier_string.to_string()),
        };
    }

    fn skip_whitespace(&self) {
        while curr!(self).is_whitespace() {
            move_read_head();
        }
    }

    fn get_number(&self) -> Option<Token> {
        let number_string = "";
        while curr!(self).is_numeric() || curr!(self) == '.' {
            number_string += curr!(self);
            move_read_head();
        }
        return match number_string.parse::<f32>() {
            Ok(n) => Some(Token::Number(n)),
            Err(_) => panic!("Number not parsable!"),
        };
    }
}

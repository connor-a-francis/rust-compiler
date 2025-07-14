use crate::token::Token;

const EOF: char = '\u{5B8C}'; // Japanese character for end cause why not. Beats epsilon.
pub struct Lexer {
    char_vec: Vec<char>,
    char_index: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(char_vec: Vec<char>) -> Self {
        let tokens: Vec<Token> = Vec::new();
        let char_index: usize = 0;
        Self {
            char_vec,
            char_index,
            tokens,
        }
    }

    fn get_tokens(&mut self) -> Vec<Token> {
        if self.char_vec.len() == 0 {
            return vec![Token::Eof];
        }
        loop {
            let token = self.get_tok();
            self.tokens.push(token.clone());
            if token == Token::Eof {
                break;
            }
        }
        self.tokens.clone()
    }

    fn get_tok(&mut self) -> Token {
        self.skip_whitespace();

        if self.get_current() == self::EOF {
            Token::Eof
        } else if self.get_current().is_numeric() || self.get_current() == '.' {
            self.get_number()
        } else if self.get_current() == '#' {
            self.skip_comment();
            self.get_tok()
        } else if self.get_current().is_alphabetic() {
            self.get_string()
        } else {
            let current_char = self.get_current();
            self.move_read_head();
            self.get_symbol(current_char)
        }
    }

    fn get_symbol(&self, c: char) -> Token {
        match c {
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mult,
            '/' => Token::Div,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '=' => Token::Eq,
            _ => Token::Misc(c),
        }
    }

    fn get_string(&mut self) -> Token {
        let mut identifier_string = String::new();

        while self.get_current().is_alphanumeric() && self.get_current() != self::EOF {
            identifier_string.push(self.get_current());
            self.move_read_head();
        }

        match identifier_string.as_str() {
            "def" => Token::Def,
            "extern" => Token::Extern,
            _ => Token::Identifier(identifier_string),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.get_current().is_whitespace() {
            self.move_read_head();
        }
    }

    fn get_number(&mut self) -> Token {
        let mut number_string = String::new();
        while self.get_current().is_numeric() || self.get_current() == '.' {
            number_string.push(self.get_current());
            self.move_read_head();
        }
        match number_string.parse::<f32>() {
            Ok(n) => Token::Number(n),
            Err(_) => panic!("Number not parsable!"),
        }
    }
    fn move_read_head(&mut self) {
        self.char_index += 1;
    }
    fn skip_comment(&mut self) {
        while self.get_current() != self::EOF
            && self.get_current() != '\n'
            && self.get_current() != '\r'
        {
            self.move_read_head();
        }
    }
    fn get_current(&self) -> char {
        if self.char_index >= self.char_vec.len() - 1 {
            self::EOF
        } else {
            self.char_vec[self.char_index]
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
    #[test]
    fn lexer_builds() {
        let _result = Lexer::new("a".chars().collect());
    }
    #[test]
    fn lexer_reads_eof() {
        let result = Lexer::new("".chars().collect()).get_tokens();
        assert_eq!(result, vec![Token::Eof])
    }
    #[test]
    fn lexer_reads_space_eof() {
        let result = Lexer::new(" ".chars().collect()).get_tokens();
        assert_eq!(result, vec![Token::Eof])
    }
    #[test]
    fn lexer_reads_def() {
        let program: Vec<char> = r###"
            def hello()
        "###
        .chars()
        .collect();
        let result = Lexer::new(program).get_tokens();
        let expected = vec![
            Token::Def,
            Token::Identifier("hello".to_string()),
            Token::LParen,
            Token::RParen,
            Token::Eof,
        ];
        assert_eq!(result, expected)
    }
    #[test]
    fn lexer_reads_extern() {
        let program: Vec<char> = r###"
                extern hello
            "###
        .chars()
        .collect();
        let result = Lexer::new(program).get_tokens();
        let expected = vec![
            Token::Extern,
            Token::Identifier("hello".to_string()),
            Token::Eof,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn lexer_skips_comment() {
        let program: Vec<char> = r###"
            extern hello
            # skip me!
            def goodbye()
        "###
        .chars()
        .collect();
        let result = Lexer::new(program).get_tokens();
        let expected = vec![
            Token::Extern,
            Token::Identifier("hello".to_string()),
            Token::Def,
            Token::Identifier("goodbye".to_string()),
            Token::LParen,
            Token::RParen,
            Token::Eof,
        ];
        assert_eq!(result, expected)
    }
    #[test]
    fn lexer_reads_math() {
        let program: Vec<char> = r###"
            a=(b+c)-d*e/(f+$)
        "###
        .chars()
        .collect();

        let result = Lexer::new(program).get_tokens();
        let expected = vec![
            Token::Identifier('a'.to_string()),
            Token::Eq,
            Token::LParen,
            Token::Identifier('b'.to_string()),
            Token::Add,
            Token::Identifier('c'.to_string()),
            Token::RParen,
            Token::Sub,
            Token::Identifier('d'.to_string()),
            Token::Mult,
            Token::Identifier('e'.to_string()),
            Token::Div,
            Token::LParen,
            Token::Identifier('f'.to_string()),
            Token::Add,
            Token::Misc('$'),
            Token::RParen,
            Token::Eof
        ];
        assert_eq!(result, expected)
    }
    #[test]
    fn lexer_reads_def_and_impl() {
        let program: Vec<char> = r###"
            def hello()
            a=b+c
        "###
        .chars()
        .collect();
        let result = Lexer::new(program).get_tokens();
        let expected = vec![
            Token::Def,
            Token::Identifier("hello".to_string()),
            Token::LParen,
            Token::RParen,
            Token::Identifier("a".to_string()),
            Token::Eq,
            Token::Identifier("b".to_string()),
            Token::Add,
            Token::Identifier("c".to_string()),
            Token::Eof,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn lexer_reads_numbers() {
        let program: Vec<char> = r###"
            def hello()
            a=45+69.2
        "###
        .chars()
        .collect();
        let result = Lexer::new(program).get_tokens();
        let expected = vec![
            Token::Def,
            Token::Identifier("hello".to_string()),
            Token::LParen,
            Token::RParen,
            Token::Identifier("a".to_string()),
            Token::Eq,
            Token::Number(45f32),
            Token::Add,
            Token::Number(69.2),
            Token::Eof,
        ];
        assert_eq!(result, expected)
    }
    #[test]
    #[should_panic]
    fn lexer_hates_weird_numbers() {
        let program: Vec<char> = r###"
            def hello()
            a=45+69.2.3
        "###
        .chars()
        .collect();
        let _ = Lexer::new(program).get_tokens();
    }
}

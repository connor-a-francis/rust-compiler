use crate::ast::{Expression, Function, Program, Statement};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    statements: Vec<Statement>,
    functions: Vec<Function>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let statements: Vec<Statement> = Vec::new();
        let functions: Vec<Function> = Vec::new();
        Self {
            tokens,
            statements,
            functions,
        }
    }
    pub fn parse(&mut self) -> Program {
        &self.rec_parse();
        self.build_program()
    }
    fn rec_parse(&mut self) {
        for token in &self.tokens {
            match token {
                Token::EOF => return,
                Token::Def => {
                    &self.handle_definition();
                    return self.rec_parse();
                }
                Token::Let => {
                    &self.handle_variable();
                    return self.rec_parse();
                }
                Token::Extern => {
                    &self.handle_external();
                    return self.rec_parse();
                }
                Token::Identifier(s) => {
                    &self.handle_identifier();
                    return self.rec_parse();
                }
                _ => panic!("Unparsable input!"),
            }
        }
    }

    fn handle_definition(&self) {
        unimplemented!()
    }

    fn handle_variable(&mut self) {
        let s: Statement = Statement::Let {
            name: "five".to_string(),
            value: Expression::Literal(5.0),
        };

        let mut lhs: Vec<Token> = Vec::new();

        lhs.extend(
            self.tokens.drain(
                ..self
                    .tokens
                    .clone()
                    .into_iter()
                    .position(|x| matches!(x, Token::EOL))
                    .expect("Missing Semi-colon")
                    + 1,
            ),
        );

        let rhs: Vec<Token> = lhs.split_off(
            lhs.clone()
                .into_iter()
                .position(|x| matches!(x, Token::Eq))
                .expect("Missing equal sign"),
        );
        let exp = handle_expression(rhs[1..]);

        if let Token::Identifier(name) = lhs[1] {
            self.statements.push(Statement {
                name: name,
                expression: exp,
            })
        }

        self.statements.push(s);
    }

    fn handle_expression(input_tokens: [Token]) -> Expression {
        unimplemented!()
    }

    fn handle_external(&self) {
        unimplemented!()
    }

    fn handle_identifier(&self) {
        unimplemented!()
    }

    fn build_program(&self) -> Program {
        Program {
            statements: self.statements.to_vec(),
            functions: self.functions.to_vec(),
        }
    }
}
#[cfg(test)]
mod parser_tests {
    use super::*;
    #[test]
    fn parser_builds() {
        let _result = Parser::new(vec![Token::Number(5.0), Token::EOF]);
    }
    #[test]
    fn parser_builds_program() {
        let result = Parser::new(vec![Token::EOF]).parse();
        let expected = Program {
            statements: vec![],
            functions: vec![],
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn parser_converts_token() {
        let result = Parser::new(vec![
            Token::Let,
            Token::Identifier("five".to_string()),
            Token::Eq,
            Token::Number(5.0),
            Token::EOL,
            Token::EOF,
        ])
        .parse();
        let expected = Program {
            statements: vec![Statement::Let {
                name: "five".to_string(),
                value: Expression::Literal(5.0),
            }],
            functions: vec![],
        };
        assert_eq!(result, expected)
    }
}

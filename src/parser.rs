use crate::ast::Program;
use crate::token::Token;

pub struct Parser {
    program: Program,
}

impl Parser {
    fn parse(&self, tokens: Vec<Token>) -> Program {
        let p = Program::new();
        return p;
    }
    fn process(&self, cur_tok: Token) {
        fprintf(stderr, "ready> ");
        match (cur_tok) {
            Token::EOF => return,
            Token::EOL => getNextToken(),
            Token::Def => HandleDefinition(),
            Token::Extern => HandleExtern(),
            _ => HandleTopLevelExpression(),
        }
    }
}


use crate::token::Token;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Expression {
    Literal(f64),
    Variable(String),
    BinaryOp {
        op: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Call {
        callee: String,
        args: Vec<Expression>,
    },
}
#[derive(Debug)]
pub struct Prototype {
    name: String,
    args: Vec<String>,
}
#[derive(Debug)]
pub struct Function {
    proto: Prototype,
    body: Program,
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub enum Statement {
    Let { name: String, value: Expression },
    Expr(Expression),
    Return(Expression),
}

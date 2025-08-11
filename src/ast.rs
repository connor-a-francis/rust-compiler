use crate::token::Token;
use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone)]
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
#[derive(PartialEq, Debug, Clone)]
pub struct Prototype {
    name: String,
    args: Vec<String>,
}
#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    proto: Prototype,
    body: Program,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub functions: Vec<Function>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Let { name: String, value: Expression },
    Expression(Expression),
    Return(Expression),
}

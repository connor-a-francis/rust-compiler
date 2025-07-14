#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    Number(f32),
    Misc(char)
}
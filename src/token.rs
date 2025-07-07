pub enum Token {
    Eof,
    Def,
    Extern,
    Identifier(String),
    Number(f32),
    Misc(char)
}